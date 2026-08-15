//! Command for deleting a crypto key.

use shared_cqrs::command::domain::command::Command;

use crate::crypto_keys::domain::value_objects::crypto_key_id::CryptoKeyId;

/// Command that requests the deletion of a crypto key by id.
pub struct DeleteCryptoKeyCommand {
    pub id: CryptoKeyId,
}

impl Command for DeleteCryptoKeyCommand {}
