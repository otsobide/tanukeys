//! Domain event raised when a new user is created.

use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};

use crate::users::domain::value_objects::user_description::UserDescription;
use crate::users::domain::value_objects::user_id::UserId;
use crate::users::domain::value_objects::user_name::UserName;

/// Domain event raised when a new [`User`] is successfully persisted.
///
/// [`User`]: crate::users::domain::entities::user::User
pub struct UserCreatedEvent {
    base: DomainEventBase,
    /// Id of the newly created user.
    pub id: UserId,
    /// Public handle of the user.
    pub name: UserName,
    /// Description of the user.
    pub description: UserDescription,
}

impl UserCreatedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "tanukeys.kernel.user.created";

    /// Creates a new `UserCreatedEvent` with auto-generated metadata.
    pub fn new(id: UserId, name: UserName, description: UserDescription) -> Self {
        Self {
            base: DomainEventBase::new(id.value().to_string()),
            id,
            name,
            description,
        }
    }
}

impl DomainEvent for UserCreatedEvent {
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
