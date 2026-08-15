//! Domain service for updating an existing crypto key.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info, warn};

use crate::crypto_keys::domain::entities::crypto_key::CryptoKey;
use crate::crypto_keys::domain::errors::crypto_key_repository_error::CryptoKeyRepositoryError;
use crate::crypto_keys::domain::events::create_crypto_key_updated_event::create_crypto_key_updated_event;
use crate::crypto_keys::domain::repositories::crypto_key_repository::CryptoKeyRepository;
use crate::crypto_keys::domain::value_objects::crypto_key_algorithm::CryptoKeyAlgorithm;
use crate::crypto_keys::domain::value_objects::crypto_key_id::CryptoKeyId;
use crate::crypto_keys::domain::value_objects::crypto_key_name::CryptoKeyName;
use crate::crypto_keys::domain::value_objects::crypto_key_payload::CryptoKeyPayload;
use crate::crypto_keys::domain::value_objects::crypto_key_protocol::CryptoKeyProtocol;

/// Domain service that updates an existing [`CryptoKey`] and publishes a
/// [`CryptoKeyUpdatedEvent`] via the event bus.
///
/// The owner and the creation instant are immutable: they are carried over
/// from the previous state, and `updated_at` is touched to the current time.
///
/// [`CryptoKeyUpdatedEvent`]: crate::crypto_keys::domain::events::crypto_key_updated_event::CryptoKeyUpdatedEvent
pub struct CryptoKeyUpdater {
    repository: Arc<dyn CryptoKeyRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl CryptoKeyUpdater {
    pub fn new(repository: Arc<dyn CryptoKeyRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(
        &self,
        id: CryptoKeyId,
        name: CryptoKeyName,
        protocol: CryptoKeyProtocol,
        algorithm: CryptoKeyAlgorithm,
        payload: CryptoKeyPayload,
    ) -> Result<(), CryptoKeyRepositoryError> {
        debug!(id = %id, "Updating crypto key");

        let previous = self.repository.find_by_id(&id).await?.ok_or_else(|| {
            warn!(id = %id, "Crypto key not found for update");
            CryptoKeyRepositoryError::NotFound
        })?;

        let updated = CryptoKey::new(
            id,
            previous.user_id().clone(),
            name,
            protocol,
            algorithm,
            payload,
            previous.timestamps().touch(),
        );
        self.repository.update(&updated).await?;

        let event = create_crypto_key_updated_event(&updated, &previous)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| CryptoKeyRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %updated.id(), "Crypto key updated");
        Ok(())
    }
}
