//! Domain service for deleting a crypto key.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info, warn};

use crate::crypto_keys::domain::errors::crypto_key_repository_error::CryptoKeyRepositoryError;
use crate::crypto_keys::domain::events::create_crypto_key_deleted_event::create_crypto_key_deleted_event;
use crate::crypto_keys::domain::repositories::crypto_key_repository::CryptoKeyRepository;
use crate::crypto_keys::domain::value_objects::crypto_key_id::CryptoKeyId;

/// Domain service that deletes a [`CryptoKey`] and publishes a
/// [`CryptoKeyDeletedEvent`] via the event bus.
///
/// [`CryptoKey`]: crate::crypto_keys::domain::entities::crypto_key::CryptoKey
/// [`CryptoKeyDeletedEvent`]: crate::crypto_keys::domain::events::crypto_key_deleted_event::CryptoKeyDeletedEvent
pub struct CryptoKeyDeleter {
    repository: Arc<dyn CryptoKeyRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl CryptoKeyDeleter {
    pub fn new(repository: Arc<dyn CryptoKeyRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(&self, id: CryptoKeyId) -> Result<(), CryptoKeyRepositoryError> {
        debug!(id = %id, "Deleting crypto key");

        let crypto_key = self.repository.find_by_id(&id).await?.ok_or_else(|| {
            warn!(id = %id, "Crypto key not found for deletion");
            CryptoKeyRepositoryError::NotFound
        })?;

        self.repository.delete(&id).await?;

        let event = create_crypto_key_deleted_event(&crypto_key)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| CryptoKeyRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %id, "Crypto key deleted");
        Ok(())
    }
}
