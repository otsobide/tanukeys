//! Domain event raised when a new crypto key is created.

use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};

use crate::crypto_keys::domain::value_objects::crypto_key_algorithm::CryptoKeyAlgorithm;
use crate::crypto_keys::domain::value_objects::crypto_key_id::CryptoKeyId;
use crate::crypto_keys::domain::value_objects::crypto_key_name::CryptoKeyName;
use crate::crypto_keys::domain::value_objects::crypto_key_payload::CryptoKeyPayload;
use crate::crypto_keys::domain::value_objects::crypto_key_protocol::CryptoKeyProtocol;
use crate::users::domain::value_objects::user_id::UserId;

/// Domain event raised when a new [`CryptoKey`] is successfully persisted.
///
/// [`CryptoKey`]: crate::crypto_keys::domain::entities::crypto_key::CryptoKey
pub struct CryptoKeyCreatedEvent {
    base: DomainEventBase,
    /// Id of the newly created key.
    pub id: CryptoKeyId,
    /// Id of the user that owns the key.
    pub user_id: UserId,
    /// Label of the key.
    pub name: CryptoKeyName,
    /// Protocol of the key.
    pub protocol: CryptoKeyProtocol,
    /// Algorithm of the key.
    pub algorithm: CryptoKeyAlgorithm,
    /// Raw key material.
    pub payload: CryptoKeyPayload,
}

impl CryptoKeyCreatedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "tanukeys.kernel.crypto_key.created";

    /// Creates a new `CryptoKeyCreatedEvent` with auto-generated metadata.
    pub fn new(
        id: CryptoKeyId,
        user_id: UserId,
        name: CryptoKeyName,
        protocol: CryptoKeyProtocol,
        algorithm: CryptoKeyAlgorithm,
        payload: CryptoKeyPayload,
    ) -> Self {
        Self {
            base: DomainEventBase::new(id.value().to_string()),
            id,
            user_id,
            name,
            protocol,
            algorithm,
            payload,
        }
    }
}

impl DomainEvent for CryptoKeyCreatedEvent {
    fn event_name(&self) -> &'static str {
        Self::EVENT_NAME
    }

    fn aggregate_id(&self) -> &str {
        &self.base.aggregate_id
    }

    fn event_id(&self) -> &str {
        &self.base.event_id
    }

    fn occurred_on(&self) -> SystemTime {
        self.base.occurred_on
    }
}
