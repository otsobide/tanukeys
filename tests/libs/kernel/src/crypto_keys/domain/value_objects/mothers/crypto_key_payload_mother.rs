use kernel::crypto_keys::domain::value_objects::crypto_key_payload::CryptoKeyPayload;
use uuid::Uuid;

pub struct CryptoKeyPayloadMother;

#[allow(dead_code)]
impl CryptoKeyPayloadMother {
    pub fn create(value: impl Into<Vec<u8>>) -> CryptoKeyPayload {
        CryptoKeyPayload::new(value).unwrap()
    }

    pub fn random() -> CryptoKeyPayload {
        CryptoKeyPayload::new(Uuid::new_v4().as_bytes().to_vec()).unwrap()
    }
}
