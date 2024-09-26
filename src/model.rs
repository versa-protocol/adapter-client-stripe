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
    pub schema_version: String,
    pub currency: String,
    pub amount: i64,
    pub subtotal: i64,
    pub date_time: i64,
    pub sender_client_id: String,
    pub mcc: Option<String>,
    pub third_party: Option<ThirdParty>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Envelope {
    pub encrypted: String,
    pub hash: u64,
    pub nonce: String,
}

#[derive(Debug, Serialize)]
pub struct RegistrationData {
    pub hash: u64,
    pub key: String,
}

#[derive(Serialize, Debug, Default)]
pub struct RoutingInfo {
    pub customer_email: Option<String>,
    pub authorization_bin: Option<String>,
    pub authorization_par: Option<String>,
}
