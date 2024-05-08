use serde::{Deserialize, Serialize};

use crate::{
    encryption::encrypt_envelope,
    model::{Envelope, Receiver, RoutingInfo},
};

#[derive(Serialize)]
pub struct TransactionRegistration {
    pub transaction_hash: u64,
    pub authorization_bin: Option<String>,
    pub customer_email: Option<String>,
    pub customer_id: Option<String>,
}

#[derive(Deserialize)]
pub struct RegistrationResponse {
    pub receivers: Vec<Receiver>,
    pub encryption_key: Vec<u8>,
}

pub async fn register(
    client_id: &str,
    client_secret: &str,
    routing_info: RoutingInfo,
    registration_hash: u64,
) -> Result<RegistrationResponse, ()> {
    let registry_url = std::env::var("REGISTRY_URL").unwrap_or_default();
    let credential = format!("{}:{}", client_id, client_secret);

    let payload = TransactionRegistration {
        transaction_hash: registration_hash,
        authorization_bin: routing_info.authorization_bin,
        customer_email: routing_info.customer_email,
        customer_id: None,
    };

    let payload_json = serde_json::to_string(&payload).unwrap();

    let url = format!("{}/http/register", registry_url);
    info!("Sending registration request to: {}", url);
    let client = reqwest::Client::new();
    let response_result = client
        .post(url)
        .header("Accept", "application/json")
        .header("Authorization", credential)
        .header("Content-Type", "application/json")
        .body(payload_json)
        .send()
        .await;

    let res = match response_result {
        Ok(res) => res,
        Err(e) => {
            info!("Error placing request: {:?}", e);
            return Err(());
        }
    };
    info!("Registration response received");

    if res.status().is_success() {
        let data: RegistrationResponse = match res.json().await {
            Ok(val) => val,
            Err(e) => {
                info!("Failed to deserialize due to error: {}", e);
                return Err(());
            }
        };
        return Ok(data);
    } else {
        info!("Received error status from registry: {}", res.status());
    }

    return Err(());
}

#[derive(Serialize)]
pub struct ReceiverPayload {
    sender_client_id: String,
    envelope: Envelope,
}

pub async fn encrypt_and_send<T>(
    receiver: &Receiver,
    client_id: &str,
    encryption_key: &Vec<u8>,
    data: T,
) -> Result<(), ()>
where
    T: Serialize,
{
    let envelope = encrypt_envelope(&data, encryption_key);

    let payload = ReceiverPayload {
        sender_client_id: client_id.to_string(),
        envelope: envelope,
    };

    let payload_json = serde_json::to_string(&payload).unwrap();

    let client = reqwest::Client::new();
    let response_result = client
        .post(&receiver.address)
        // .header("Authorization", credential) // TODO ?
        .header("Content-Type", "application/json")
        .body(payload_json)
        .send()
        .await;

    let res = match response_result {
        Ok(res) => res,
        Err(e) => {
            info!("Error placing request: {:?}", e);
            return Err(());
        }
    };

    if res.status().is_success() {
        info!("Successfully sent data to receiver: {}", receiver.address);
        // TODO: process response from each receiver
        return Ok(());
    } else {
        let status = res.status();
        let text = res.text().await.unwrap_or_default();
        info!("Received an error from the receiver: {} {}", status, text);
    }
    // info!("Received an error from the receiver: {:?}", res);

    return Err(());
}
