use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Merchant {
    pub id: String,
    pub name: String,
    pub brand_color: String,
    pub logo: String,
    pub mcc: String,
    pub website: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThirdParty {
    pub first_party_relation: String,
    pub make_primary: bool,
    pub merchant: Merchant,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SenderReceiptHeader {
    pub id: String,
    pub currency: String,
    pub amount: i64,
    pub subtotal: i64,
    pub date_time: i64,
    pub sender_client_id: String,
    pub third_party: Option<ThirdParty>,
}

#[derive(Debug)]
pub struct PreparedData {
    pub encrypted: Vec<u8>,
    pub hash: u64,
    pub key: Vec<u8>,
    pub nonce: Vec<u8>,
}

#[derive(Debug, Serialize)]
pub struct Envelope {
    pub encrypted: Vec<u8>,
    pub hash: u64,
    pub nonce: Vec<u8>,
}

#[derive(Debug, Serialize)]
pub struct RegistrationData {
    pub hash: u64,
    pub key: Vec<u8>,
}

impl PreparedData {
    pub fn to_envelope_and_registration_data(self) -> (Envelope, RegistrationData) {
        (
            Envelope {
                encrypted: self.encrypted,
                hash: self.hash,
                nonce: self.nonce,
            },
            RegistrationData {
                hash: self.hash,
                key: self.key,
            },
        )
    }
}

#[derive(Serialize, Debug)]
pub struct RoutingInfo {
    pub email: Option<String>,
    pub bin: Option<String>,
    pub par: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Receiver {
    pub address: String,
    pub org_id: String,
    pub version: String,
}
