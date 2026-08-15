//! Factory function for [`CryptoKeyCreatedEvent`].

use crate::crypto_keys::domain::entities::crypto_key::CryptoKey;
use crate::crypto_keys::domain::errors::crypto_key_repository_error::CryptoKeyRepositoryError;
use crate::crypto_keys::domain::events::crypto_key_created_event::CryptoKeyCreatedEvent;

/// Creates a [`CryptoKeyCreatedEvent`] from the given crypto key.
pub fn create_crypto_key_created_event(
    crypto_key: &CryptoKey,
) -> Result<CryptoKeyCreatedEvent, CryptoKeyRepositoryError> {
    Ok(CryptoKeyCreatedEvent::new(
        crypto_key.id().clone(),
        crypto_key.user_id().clone(),
        crypto_key.name().clone(),
        crypto_key.protocol(),
        crypto_key.algorithm(),
        crypto_key.payload().clone(),
    ))
}
