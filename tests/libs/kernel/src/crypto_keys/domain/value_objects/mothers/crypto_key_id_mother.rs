use kernel::crypto_keys::domain::value_objects::crypto_key_id::CryptoKeyId;

pub struct CryptoKeyIdMother;

#[allow(dead_code)]
impl CryptoKeyIdMother {
    pub fn create(value: impl Into<String>) -> CryptoKeyId {
        CryptoKeyId::new(value).unwrap()
    }

    pub fn random() -> CryptoKeyId {
        CryptoKeyId::generate()
    }
}
