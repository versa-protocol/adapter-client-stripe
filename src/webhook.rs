use axum::extract::Request;
use axum::http::HeaderMap;
use stripe::{EventObject, EventType};

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
    let receipt = crate::data_adapter::transform_stripe_invoice(invoice);
    let sender_client_id = receipt.sender_client_id.clone();

    // 3. Encrypt, hash, and register with Versa registry
    let prepared = crate::encryption::encrypt_and_hash(receipt);
    let (_envelope, registration) = prepared.to_envelope_and_registration_data();
    // TODO: Fill this in from invoice payment details
    let routing_info = crate::model::RoutingInfo {
        email: None,
        bin: None,
        par: None,
    };

    let _receivers = crate::protocol::register(&sender_client_id, routing_info, registration);

    // 4. Send encrypted data to receiver endpoints returned by the registry
    Ok(http::StatusCode::ACCEPTED)
}
