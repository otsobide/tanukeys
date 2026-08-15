//! Domain service for creating crypto keys.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info};

use crate::crypto_keys::domain::entities::crypto_key::CryptoKey;
use crate::crypto_keys::domain::errors::crypto_key_repository_error::CryptoKeyRepositoryError;
use crate::crypto_keys::domain::events::create_crypto_key_created_event::create_crypto_key_created_event;
use crate::crypto_keys::domain::repositories::crypto_key_repository::CryptoKeyRepository;
use crate::crypto_keys::domain::value_objects::crypto_key_algorithm::CryptoKeyAlgorithm;
use crate::crypto_keys::domain::value_objects::crypto_key_id::CryptoKeyId;
use crate::crypto_keys::domain::value_objects::crypto_key_name::CryptoKeyName;
use crate::crypto_keys::domain::value_objects::crypto_key_payload::CryptoKeyPayload;
use crate::crypto_keys::domain::value_objects::crypto_key_protocol::CryptoKeyProtocol;
use crate::crypto_keys::domain::value_objects::crypto_key_timestamps::CryptoKeyTimestamps;
use crate::users::domain::value_objects::user_id::UserId;

/// Domain service that persists a new [`CryptoKey`] and publishes a
/// [`CryptoKeyCreatedEvent`] via the event bus.
///
/// The creation timestamps are stamped here: callers supply the key data,
/// the domain records when it was born.
///
/// [`CryptoKeyCreatedEvent`]: crate::crypto_keys::domain::events::crypto_key_created_event::CryptoKeyCreatedEvent
pub struct CryptoKeyCreator {
    repository: Arc<dyn CryptoKeyRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl CryptoKeyCreator {
    pub fn new(repository: Arc<dyn CryptoKeyRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(
        &self,
        id: CryptoKeyId,
        user_id: UserId,
        name: CryptoKeyName,
        protocol: CryptoKeyProtocol,
        algorithm: CryptoKeyAlgorithm,
        payload: CryptoKeyPayload,
    ) -> Result<(), CryptoKeyRepositoryError> {
        let crypto_key = CryptoKey::new(
            id,
            user_id,
            name,
            protocol,
            algorithm,
            payload,
            CryptoKeyTimestamps::now(),
        );
        debug!(id = %crypto_key.id(), user_id = %crypto_key.user_id(), "Creating crypto key");

        self.repository.save(&crypto_key).await?;

        let event = create_crypto_key_created_event(&crypto_key)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| CryptoKeyRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %crypto_key.id(), "Crypto key created");
        Ok(())
    }
}
