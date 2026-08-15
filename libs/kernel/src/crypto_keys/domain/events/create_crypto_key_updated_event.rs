//! Factory function for [`CryptoKeyUpdatedEvent`].

use crate::crypto_keys::domain::entities::crypto_key::CryptoKey;
use crate::crypto_keys::domain::errors::crypto_key_repository_error::CryptoKeyRepositoryError;
use crate::crypto_keys::domain::events::crypto_key_updated_event::CryptoKeyUpdatedEvent;

/// Creates a [`CryptoKeyUpdatedEvent`] from the updated and previous keys.
pub fn create_crypto_key_updated_event(
    updated: &CryptoKey,
    previous: &CryptoKey,
) -> Result<CryptoKeyUpdatedEvent, CryptoKeyRepositoryError> {
    Ok(CryptoKeyUpdatedEvent::new(
        updated.id().clone(),
        updated.user_id().clone(),
        updated.name().clone(),
        previous.name().clone(),
        updated.protocol(),
        previous.protocol(),
        updated.algorithm(),
        previous.algorithm(),
        updated.payload().clone(),
        previous.payload().clone(),
    ))
}
