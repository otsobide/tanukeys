//! Domain event raised when a crypto key is deleted.

use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};

use crate::crypto_keys::domain::value_objects::crypto_key_id::CryptoKeyId;
use crate::crypto_keys::domain::value_objects::crypto_key_name::CryptoKeyName;
use crate::users::domain::value_objects::user_id::UserId;

/// Domain event raised when a [`CryptoKey`] is deleted.
///
/// [`CryptoKey`]: crate::crypto_keys::domain::entities::crypto_key::CryptoKey
pub struct CryptoKeyDeletedEvent {
    base: DomainEventBase,
    /// Id of the deleted key.
    pub id: CryptoKeyId,
    /// Id of the user that owned the key.
    pub user_id: UserId,
    /// Label the key had when it was deleted.
    pub name: CryptoKeyName,
}

impl CryptoKeyDeletedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "tanukeys.kernel.crypto_key.deleted";

    /// Creates a new `CryptoKeyDeletedEvent` with auto-generated metadata.
    pub fn new(id: CryptoKeyId, user_id: UserId, name: CryptoKeyName) -> Self {
        Self {
            base: DomainEventBase::new(id.value().to_string()),
            id,
            user_id,
            name,
        }
    }
}

impl DomainEvent for CryptoKeyDeletedEvent {
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
