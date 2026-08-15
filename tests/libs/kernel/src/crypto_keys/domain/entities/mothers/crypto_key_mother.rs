use kernel::crypto_keys::domain::entities::crypto_key::CryptoKey;
use kernel::users::domain::value_objects::user_id::UserId;

use crate::src::crypto_keys::domain::value_objects::mothers::crypto_key_algorithm_mother::CryptoKeyAlgorithmMother;
use crate::src::crypto_keys::domain::value_objects::mothers::crypto_key_id_mother::CryptoKeyIdMother;
use crate::src::crypto_keys::domain::value_objects::mothers::crypto_key_name_mother::CryptoKeyNameMother;
use crate::src::crypto_keys::domain::value_objects::mothers::crypto_key_payload_mother::CryptoKeyPayloadMother;
use crate::src::crypto_keys::domain::value_objects::mothers::crypto_key_protocol_mother::CryptoKeyProtocolMother;
use crate::src::crypto_keys::domain::value_objects::mothers::crypto_key_timestamps_mother::CryptoKeyTimestampsMother;
use crate::src::users::domain::value_objects::mothers::user_id_mother::UserIdMother;

pub struct CryptoKeyMother;

#[allow(dead_code)]
impl CryptoKeyMother {
    pub fn random() -> CryptoKey {
        Self::random_with_user(&UserIdMother::random())
    }

    pub fn random_with_user(user_id: &UserId) -> CryptoKey {
        CryptoKey::new(
            CryptoKeyIdMother::random(),
            user_id.clone(),
            CryptoKeyNameMother::random(),
            CryptoKeyProtocolMother::random(),
            CryptoKeyAlgorithmMother::random(),
            CryptoKeyPayloadMother::random(),
            CryptoKeyTimestampsMother::random(),
        )
    }

    pub fn create(
        id: impl Into<String>,
        user_id: impl Into<String>,
        name: impl Into<String>,
        protocol: impl Into<String>,
        algorithm: impl Into<String>,
        payload: impl Into<Vec<u8>>,
    ) -> CryptoKey {
        CryptoKey::new(
            CryptoKeyIdMother::create(id),
            UserIdMother::create(user_id),
            CryptoKeyNameMother::create(name),
            CryptoKeyProtocolMother::create(protocol),
            CryptoKeyAlgorithmMother::create(algorithm),
            CryptoKeyPayloadMother::create(payload),
            CryptoKeyTimestampsMother::random(),
        )
    }
}
