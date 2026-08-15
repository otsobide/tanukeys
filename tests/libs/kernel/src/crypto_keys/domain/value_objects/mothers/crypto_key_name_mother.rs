use kernel::crypto_keys::domain::value_objects::crypto_key_name::CryptoKeyName;
use uuid::Uuid;

pub struct CryptoKeyNameMother;

#[allow(dead_code)]
impl CryptoKeyNameMother {
    pub fn create(value: impl Into<String>) -> CryptoKeyName {
        CryptoKeyName::new(value).unwrap()
    }

    pub fn random() -> CryptoKeyName {
        CryptoKeyName::new(format!("key {}", Uuid::new_v4())).unwrap()
    }
}
