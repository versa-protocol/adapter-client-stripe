use aes_siv::{
    aead::{Aead, KeyInit, OsRng},
    Aes256SivAead,
    Nonce, // Or `Aes128SivAead`
};
use json_canon::to_string;
use rand::Rng;
use serde::Serialize;
use serde_json::json;
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::model::PreparedData;

fn generate_nonce() -> [u8; 16] {
    let mut rng = rand::thread_rng();
    let mut nonce = [0u8; 16];
    rng.fill(&mut nonce);
    nonce
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn encrypt_and_hash<T>(data: T) -> PreparedData
where
    T: Serialize,
{
    let serde_json = json!(data);

    let canonicalized = match to_string(&serde_json) {
        Ok(canonicalized) => canonicalized,
        Err(e) => panic!("Error canonicalizing JSON: {}", e),
    };

    let hash = calculate_hash(&canonicalized);

    let bytekey = Aes256SivAead::generate_key(&mut OsRng);
    let cipher = Aes256SivAead::new(&bytekey);
    let nonce_bytes = generate_nonce();
    let nonce = Nonce::from_slice(&nonce_bytes); // normally, this would be unique to each receiver and included in the message
    let encrypted = match cipher.encrypt(nonce, canonicalized.as_bytes()) {
        Ok(ciphertext) => ciphertext,
        Err(e) => panic!("Error encrypting data: {}", e),
    };
    PreparedData {
        key: bytekey.to_vec(),
        hash,
        encrypted,
        nonce: nonce_bytes.to_vec(),
    }
}

#[cfg(test)]
mod encrypt_tests {
    use aes_siv::{
        aead::{Aead, KeyInit, Payload},
        Aes256SivAead,
    };
    use pretty_assertions::assert_eq;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct DummyLineItem;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct DummyReceipt {
        /// The Unix Epoch timestamp of the transaction authorization
        pub authorized: i64,
        pub authorization_id: String,
        pub amount: i64,
        pub merchant_entity_id: String,
        pub details: Vec<DummyLineItem>,
    }

    #[test]
    fn test_encrypt_and_hash() {
        let data = DummyReceipt {
            merchant_entity_id: "Amazon".into(),
            authorized: 4545,
            amount: 3445,
            authorization_id: "foo".into(),
            details: vec![],
        };

        let result = super::encrypt_and_hash(data);

        let cipher = Aes256SivAead::new(result.key[..].into());
        let decrypted = cipher
            .decrypt(
                result.nonce[..].into(),
                Payload::from(&result.encrypted[..]),
            )
            .expect("Decryption works");
        assert_eq!(decrypted, "{\"amount\":3445,\"authorization_id\":\"foo\",\"authorized\":4545,\"details\":[],\"merchant_entity_id\":\"Amazon\"}".as_bytes());
        let canonical_json = String::from_utf8(decrypted).expect("Works");
        let deserialized = serde_json::from_str::<DummyReceipt>(&canonical_json)
            .expect("Deserialization should work");
        assert_eq!(deserialized.merchant_entity_id, "Amazon".to_string());

        let recalculated_hash = super::calculate_hash(&canonical_json);
        assert_eq!(recalculated_hash, result.hash);
    }
}
