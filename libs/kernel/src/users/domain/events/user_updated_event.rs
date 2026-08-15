//! Domain event raised when a user is updated.

use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};

use crate::users::domain::value_objects::user_description::UserDescription;
use crate::users::domain::value_objects::user_id::UserId;
use crate::users::domain::value_objects::user_name::UserName;

/// Domain event raised when an existing [`User`] is updated.
///
/// [`User`]: crate::users::domain::entities::user::User
pub struct UserUpdatedEvent {
    base: DomainEventBase,
    /// Id of the updated user.
    pub id: UserId,
    /// Handle after the update.
    pub new_name: UserName,
    /// Handle before the update.
    pub old_name: UserName,
    /// Description after the update.
    pub new_description: UserDescription,
    /// Description before the update.
    pub old_description: UserDescription,
}

impl UserUpdatedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "tanukeys.kernel.user.updated";

    /// Creates a new `UserUpdatedEvent` with auto-generated metadata.
    pub fn new(
        id: UserId,
        new_name: UserName,
        old_name: UserName,
        new_description: UserDescription,
        old_description: UserDescription,
    ) -> Self {
        Self {
            base: DomainEventBase::new(id.value().to_string()),
            id,
            new_name,
            old_name,
            new_description,
            old_description,
        }
    }
}

impl DomainEvent for UserUpdatedEvent {
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
