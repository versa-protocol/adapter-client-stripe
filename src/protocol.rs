use serde::Serialize;

use crate::{
    encryption::encrypt_envelope,
    model::{Envelope, Receiver, RegistrationData, RoutingInfo},
};

#[derive(Serialize)]
pub struct TransactionRegistration {
    pub transaction_hash: u64,
    pub decryption_key: Vec<u8>,
    pub authorization_bin: Option<String>,
    pub customer_email: Option<String>,
    pub customer_id: Option<String>,
}

pub async fn register(
    client_id: &str,
    client_secret: &str,
    routing_info: RoutingInfo,
    registration_data: &RegistrationData,
) -> Result<Vec<Receiver>, ()> {
    let registry_url = std::env::var("REGISTRY_URL").unwrap_or_default();
    let credential = format!("{}:{}", client_id, client_secret);

    let payload = TransactionRegistration {
        transaction_hash: registration_data.hash,
        decryption_key: registration_data.key.clone(),
        authorization_bin: routing_info.authorization_bin,
        customer_email: routing_info.customer_email,
        customer_id: None,
    };

    let payload_json = serde_json::to_string(&payload).unwrap();

    let client = reqwest::Client::new();
    let response_result = client
        .post(format!("{registry_url}/http/register"))
        .header("Accept", "application/json")
        .header("Authorization", credential)
        .header("Content-Type", "application/json")
        .body(payload_json)
        .send()
        .await;

    let res = match response_result {
        Ok(res) => res,
        Err(_) => return Err(()),
    };

    if res.status().is_success() {
        let data: Vec<Receiver> = match res.json().await {
            Ok(val) => val,
            Err(e) => {
                info!("Failed to deserialize due to error: {}", e);

                return Err(());
            }
        };
        return Ok(data);
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
    registration_data: &RegistrationData,
    data: T,
) -> Result<(), ()>
where
    T: Serialize,
{
    let envelope = encrypt_envelope(&data, &registration_data.key);

    let payload = ReceiverPayload {
        sender_client_id: client_id.to_string(),
        envelope: envelope,
    };

    let payload_json = serde_json::to_string(&payload).unwrap();

    let client = reqwest::Client::new();
    let response_result = client
        .post(&receiver.address)
        .header("Accept", "application/json")
        // .header("Authorization", credential) // TODO ?
        .header("Content-Type", "application/json")
        .body(payload_json)
        .send()
        .await;

    let res = match response_result {
        Ok(res) => res,
        Err(_) => return Err(()),
    };

    if res.status().is_success() {
        // TODO: process response from each receiver
        return Ok(());
    }

    return Err(());
}
