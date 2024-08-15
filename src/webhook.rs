use axum::extract::Request;
use axum::http::HeaderMap;
use stripe::{EventObject, EventType};

use crate::protocol::encrypt_and_send;

pub async fn target(
    headers: HeaderMap,
    request: Request,
) -> Result<http::StatusCode, (http::StatusCode, String)> {
    // 0. Extract and validate the event package from Stripe

    let Ok(secret) = std::env::var("WEBHOOK_SECRET") else {
        info!("FATAL: Missing config: WEBHOOK_SECRET");
        return Err((
            http::StatusCode::SERVICE_UNAVAILABLE,
            "Missing WEBHOOK_SECRET".into(),
        ));
    };

    let signature = if let Some(signature) = headers.get("stripe-signature") {
        signature.to_str().map_err(|error| {
            info!("Error extracting signature from headers: {:?}", error);
            (
                http::StatusCode::UNAUTHORIZED,
                format!("Error extracting signature from headers: {:?}", error),
            )
        })?
    } else {
        return Err((
            http::StatusCode::UNAUTHORIZED,
            "Missing header: stripe-signature".into(),
        ));
    };

    let (_, body) = request.into_parts();

    let bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
        Err(e) => {
            info!("Error parsing request body: {:?}", e);
            return Err((
                http::StatusCode::BAD_REQUEST,
                format!("Error parsing request body: {:?}", e).into(),
            ));
        }
    };
    let payload = std::str::from_utf8(&bytes).map_err(|e| {
        info!("Error parsing request body: {:?}", e);
        (
            http::StatusCode::BAD_REQUEST,
            format!("Error parsing request body: {:?}", e),
        )
    })?;

    // 1. Receive Stripe 'invoice.paid' event

    let event =
        stripe::Webhook::construct_event(payload, &signature, &secret).map_err(|error| {
            info!("Error validating Stripe event: {:?}", error);
            (
                http::StatusCode::UNAUTHORIZED,
                format!("Error validating Stripe event: {:?}", error),
            )
        })?;

    if event.type_ != EventType::InvoicePaid {
        info!("Unsupported event type: {}", event.type_);
        return Err((
            http::StatusCode::METHOD_NOT_ALLOWED,
            format!("Unsupported event type: {}", event.type_),
        ));
    }

    let EventObject::Invoice(invoice) = event.data.object else {
        info!("Missing invoice data in event");
        return Err((
            http::StatusCode::BAD_REQUEST,
            "Missing invoice data in event".into(),
        ));
    };

    // 2. Transform into the Versa receipt schema

    let customer_email = invoice.customer_email.clone();
    let receipt = crate::data_adapter::transform_stripe_invoice(invoice);
    let sender_client_id = std::env::var("CLIENT_ID").unwrap();
    info!("Received invoice for customer email: {:?}", customer_email);

    // 3. Encrypt, hash, and register with Versa registry

    let registration_hash = crate::encryption::generate_hash(&receipt);

    // // Authorized receivers subscribed to this email or domain will be returned by the registry
    // let routing_info = crate::model::RoutingInfo {
    //     customer_email,
    //     ..Default::default()
    // };

    let sender_client_secret = std::env::var("CLIENT_SECRET").unwrap_or_default();

    let response = crate::protocol::register(
        &sender_client_id,
        &sender_client_secret,
        customer_email,
        registration_hash,
    )
    .await
    .map_err(|e| {
        info!("Registration failed: {:?}", e);
        (
            http::StatusCode::SERVICE_UNAVAILABLE,
            format!("Registration failed: {:?}", e),
        )
    })?;

    info!(
        "Registration successful, received {} receivers",
        response.receivers.len()
    );

    // 4. Send encrypted data to receiver endpoints returned by the registry
    for receiver in response.receivers {
        info!(
            "Encrypting and sending envelope to receiver {} at {}",
            receiver.org_id, receiver.address
        );
        match encrypt_and_send(
            &receiver,
            &sender_client_id,
            response.receipt_id.clone(),
            &response.encryption_key,
            &receipt,
        )
        .await
        {
            Ok(_) => info!("Successfully sent to receiver: {}", receiver.address),
            Err(e) => {
                info!("Failed to send to receiver: {:?}", e)
            }
        }
    }

    Ok(http::StatusCode::ACCEPTED)
}
