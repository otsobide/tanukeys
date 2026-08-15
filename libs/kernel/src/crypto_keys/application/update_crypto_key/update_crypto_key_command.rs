//! Command for updating a crypto key.

use shared_cqrs::command::domain::command::Command;

use crate::crypto_keys::domain::value_objects::crypto_key_algorithm::CryptoKeyAlgorithm;
use crate::crypto_keys::domain::value_objects::crypto_key_id::CryptoKeyId;
use crate::crypto_keys::domain::value_objects::crypto_key_name::CryptoKeyName;
use crate::crypto_keys::domain::value_objects::crypto_key_payload::CryptoKeyPayload;
use crate::crypto_keys::domain::value_objects::crypto_key_protocol::CryptoKeyProtocol;

/// Command that requests an update of an existing crypto key.
///
/// The owner cannot be changed: keys do not move between users.
pub struct UpdateCryptoKeyCommand {
    pub id: CryptoKeyId,
    pub name: CryptoKeyName,
    pub protocol: CryptoKeyProtocol,
    pub algorithm: CryptoKeyAlgorithm,
    pub payload: CryptoKeyPayload,
}

impl Command for UpdateCryptoKeyCommand {}
