//! Query for finding a crypto key by id.

use shared_cqrs::query::domain::query::Query;

use crate::crypto_keys::domain::value_objects::crypto_key_id::CryptoKeyId;

/// Query that requests a single crypto key by its [`CryptoKeyId`].
pub struct FindCryptoKeyQuery {
    pub id: CryptoKeyId,
}

impl Query for FindCryptoKeyQuery {}
