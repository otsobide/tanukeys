use kernel::crypto_keys::domain::value_objects::crypto_key_protocol::CryptoKeyProtocol;

pub struct CryptoKeyProtocolMother;

#[allow(dead_code)]
impl CryptoKeyProtocolMother {
    pub fn create(value: impl Into<String>) -> CryptoKeyProtocol {
        CryptoKeyProtocol::new(value).unwrap()
    }

    pub fn random() -> CryptoKeyProtocol {
        CryptoKeyProtocol::OpenPgp
    }
}
