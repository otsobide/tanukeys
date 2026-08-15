//! Domain event raised when a user is deleted.

use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};

use crate::users::domain::value_objects::user_id::UserId;
use crate::users::domain::value_objects::user_name::UserName;

/// Domain event raised when a [`User`] is deleted.
///
/// [`User`]: crate::users::domain::entities::user::User
pub struct UserDeletedEvent {
    base: DomainEventBase,
    /// Id of the deleted user.
    pub id: UserId,
    /// Handle the user had when it was deleted.
    pub name: UserName,
}

impl UserDeletedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "tanukeys.kernel.user.deleted";

    /// Creates a new `UserDeletedEvent` with auto-generated metadata.
    pub fn new(id: UserId, name: UserName) -> Self {
        Self {
            base: DomainEventBase::new(id.value().to_string()),
            id,
            name,
        }
    }
}

impl DomainEvent for UserDeletedEvent {
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
