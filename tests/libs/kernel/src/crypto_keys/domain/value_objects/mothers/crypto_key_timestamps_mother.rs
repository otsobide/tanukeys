use std::time::SystemTime;

use kernel::crypto_keys::domain::value_objects::crypto_key_timestamps::CryptoKeyTimestamps;

pub struct CryptoKeyTimestampsMother;

#[allow(dead_code)]
impl CryptoKeyTimestampsMother {
    pub fn create(created_at: SystemTime, updated_at: SystemTime) -> CryptoKeyTimestamps {
        CryptoKeyTimestamps::new(created_at, updated_at).unwrap()
    }

    pub fn random() -> CryptoKeyTimestamps {
        CryptoKeyTimestamps::now()
    }
}
