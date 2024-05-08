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

#[derive(Serialize, Debug, Default)]
pub struct RoutingInfo {
    pub customer_email: Option<String>,
    pub authorization_bin: Option<String>,
    pub authorization_par: Option<String>,
}

// #[derive(Deserialize, Debug)]
// pub struct Receiver {
//     pub address: String,
//     pub org_id: String,
//     pub version: String,
// }

#[derive(Deserialize)]
pub struct Receiver {
    pub id: String,
    pub org_id: String,
    pub handle: String,
    pub handle_type: String,
    pub name: String,
    pub address: String,
    pub created: i64,
    pub decommissioned: Option<i64>,
}
