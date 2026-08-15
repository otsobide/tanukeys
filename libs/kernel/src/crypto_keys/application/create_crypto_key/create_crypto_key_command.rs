//! Command for creating a crypto key.

use shared_cqrs::command::domain::command::Command;

use crate::crypto_keys::domain::value_objects::crypto_key_algorithm::CryptoKeyAlgorithm;
use crate::crypto_keys::domain::value_objects::crypto_key_id::CryptoKeyId;
use crate::crypto_keys::domain::value_objects::crypto_key_name::CryptoKeyName;
use crate::crypto_keys::domain::value_objects::crypto_key_payload::CryptoKeyPayload;
use crate::crypto_keys::domain::value_objects::crypto_key_protocol::CryptoKeyProtocol;
use crate::users::domain::value_objects::user_id::UserId;

/// Command that requests the creation of a new crypto key.
///
/// Timestamps are not part of the command: the domain stamps them at
/// creation time.
pub struct CreateCryptoKeyCommand {
    pub id: CryptoKeyId,
    pub user_id: UserId,
    pub name: CryptoKeyName,
    pub protocol: CryptoKeyProtocol,
    pub algorithm: CryptoKeyAlgorithm,
    pub payload: CryptoKeyPayload,
}

impl Command for CreateCryptoKeyCommand {}
