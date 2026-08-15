//! Domain service for updating an existing user.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info, warn};

use crate::users::domain::entities::user::User;
use crate::users::domain::errors::user_repository_error::UserRepositoryError;
use crate::users::domain::events::create_user_updated_event::create_user_updated_event;
use crate::users::domain::repositories::user_repository::UserRepository;
use crate::users::domain::value_objects::user_description::UserDescription;
use crate::users::domain::value_objects::user_id::UserId;
use crate::users::domain::value_objects::user_name::UserName;

/// Domain service that updates an existing [`User`] and publishes a
/// [`UserUpdatedEvent`] via the event bus.
///
/// [`UserUpdatedEvent`]: crate::users::domain::events::user_updated_event::UserUpdatedEvent
pub struct UserUpdater {
    repository: Arc<dyn UserRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl UserUpdater {
    pub fn new(repository: Arc<dyn UserRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(
        &self,
        id: UserId,
        name: UserName,
        description: UserDescription,
    ) -> Result<(), UserRepositoryError> {
        debug!(id = %id, "Updating user");

        let previous = self.repository.find_by_id(&id).await?.ok_or_else(|| {
            warn!(id = %id, "User not found for update");
            UserRepositoryError::NotFound
        })?;

        let updated = User::new(id, name, description);
        self.repository.update(&updated).await?;

        let event = create_user_updated_event(&updated, &previous)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| UserRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %updated.id(), "User updated");
        Ok(())
    }
}
