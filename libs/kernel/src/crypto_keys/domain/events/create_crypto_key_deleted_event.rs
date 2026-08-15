//! Factory function for [`CryptoKeyDeletedEvent`].

use crate::crypto_keys::domain::entities::crypto_key::CryptoKey;
use crate::crypto_keys::domain::errors::crypto_key_repository_error::CryptoKeyRepositoryError;
use crate::crypto_keys::domain::events::crypto_key_deleted_event::CryptoKeyDeletedEvent;

/// Creates a [`CryptoKeyDeletedEvent`] from the deleted crypto key.
pub fn create_crypto_key_deleted_event(
    crypto_key: &CryptoKey,
) -> Result<CryptoKeyDeletedEvent, CryptoKeyRepositoryError> {
    Ok(CryptoKeyDeletedEvent::new(
        crypto_key.id().clone(),
        crypto_key.user_id().clone(),
        crypto_key.name().clone(),
    ))
}
