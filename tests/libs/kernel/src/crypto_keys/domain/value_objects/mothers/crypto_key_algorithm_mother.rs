use kernel::crypto_keys::domain::value_objects::crypto_key_algorithm::CryptoKeyAlgorithm;

pub struct CryptoKeyAlgorithmMother;

#[allow(dead_code)]
impl CryptoKeyAlgorithmMother {
    pub fn create(value: impl Into<String>) -> CryptoKeyAlgorithm {
        CryptoKeyAlgorithm::new(value).unwrap()
    }

    pub fn random() -> CryptoKeyAlgorithm {
        CryptoKeyAlgorithm::Ed25519
    }
}
