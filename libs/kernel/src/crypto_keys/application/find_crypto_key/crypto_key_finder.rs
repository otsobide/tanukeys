//! Domain service for finding a single crypto key.

use std::sync::Arc;

use tracing::debug;

use crate::crypto_keys::domain::entities::crypto_key::CryptoKey;
use crate::crypto_keys::domain::errors::crypto_key_repository_error::CryptoKeyRepositoryError;
use crate::crypto_keys::domain::repositories::crypto_key_repository::CryptoKeyRepository;
use crate::crypto_keys::domain::value_objects::crypto_key_id::CryptoKeyId;

/// Domain service that looks up a single [`CryptoKey`] by id.
///
/// Returns the domain entity directly. The handler is responsible for
/// mapping it to a response DTO.
pub struct CryptoKeyFinder {
    repository: Arc<dyn CryptoKeyRepository>,
}

impl CryptoKeyFinder {
    pub fn new(repository: Arc<dyn CryptoKeyRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: CryptoKeyId) -> Result<CryptoKey, CryptoKeyRepositoryError> {
        debug!(id = %id, "Finding crypto key");
        let crypto_key = self.repository.find_by_id(&id).await?;

        crypto_key.ok_or(CryptoKeyRepositoryError::NotFound)
    }
}
