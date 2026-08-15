//! Domain event raised when a crypto key is updated.

use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};

use crate::crypto_keys::domain::value_objects::crypto_key_algorithm::CryptoKeyAlgorithm;
use crate::crypto_keys::domain::value_objects::crypto_key_id::CryptoKeyId;
use crate::crypto_keys::domain::value_objects::crypto_key_name::CryptoKeyName;
use crate::crypto_keys::domain::value_objects::crypto_key_payload::CryptoKeyPayload;
use crate::crypto_keys::domain::value_objects::crypto_key_protocol::CryptoKeyProtocol;
use crate::users::domain::value_objects::user_id::UserId;

/// Domain event raised when an existing [`CryptoKey`] is updated.
///
/// Carries both the new and the previous values of every mutable field so
/// subscribers do not need a second query to know what changed. The id and
/// the owner are immutable and appear once.
///
/// [`CryptoKey`]: crate::crypto_keys::domain::entities::crypto_key::CryptoKey
pub struct CryptoKeyUpdatedEvent {
    base: DomainEventBase,
    /// Id of the updated key.
    pub id: CryptoKeyId,
    /// Id of the user that owns the key.
    pub user_id: UserId,
    /// Label after the update.
    pub new_name: CryptoKeyName,
    /// Label before the update.
    pub old_name: CryptoKeyName,
    /// Protocol after the update.
    pub new_protocol: CryptoKeyProtocol,
    /// Protocol before the update.
    pub old_protocol: CryptoKeyProtocol,
    /// Algorithm after the update.
    pub new_algorithm: CryptoKeyAlgorithm,
    /// Algorithm before the update.
    pub old_algorithm: CryptoKeyAlgorithm,
    /// Key material after the update.
    pub new_payload: CryptoKeyPayload,
    /// Key material before the update.
    pub old_payload: CryptoKeyPayload,
}

impl CryptoKeyUpdatedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "tanukeys.kernel.crypto_key.updated";

    /// Creates a new `CryptoKeyUpdatedEvent` with auto-generated metadata.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: CryptoKeyId,
        user_id: UserId,
        new_name: CryptoKeyName,
        old_name: CryptoKeyName,
        new_protocol: CryptoKeyProtocol,
        old_protocol: CryptoKeyProtocol,
        new_algorithm: CryptoKeyAlgorithm,
        old_algorithm: CryptoKeyAlgorithm,
        new_payload: CryptoKeyPayload,
        old_payload: CryptoKeyPayload,
    ) -> Self {
        Self {
            base: DomainEventBase::new(id.value().to_string()),
            id,
            user_id,
            new_name,
            old_name,
            new_protocol,
            old_protocol,
            new_algorithm,
            old_algorithm,
            new_payload,
            old_payload,
        }
    }
}

impl DomainEvent for CryptoKeyUpdatedEvent {
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
