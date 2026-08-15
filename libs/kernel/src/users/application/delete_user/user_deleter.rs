//! Domain service for deleting a user.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info, warn};

use crate::users::domain::errors::user_repository_error::UserRepositoryError;
use crate::users::domain::events::create_user_deleted_event::create_user_deleted_event;
use crate::users::domain::repositories::user_repository::UserRepository;
use crate::users::domain::value_objects::user_id::UserId;

/// Domain service that deletes a [`User`] and publishes a
/// [`UserDeletedEvent`] via the event bus.
///
/// [`User`]: crate::users::domain::entities::user::User
/// [`UserDeletedEvent`]: crate::users::domain::events::user_deleted_event::UserDeletedEvent
pub struct UserDeleter {
    repository: Arc<dyn UserRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl UserDeleter {
    pub fn new(repository: Arc<dyn UserRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self { repository, event_bus }
    }

    pub async fn execute(&self, id: UserId) -> Result<(), UserRepositoryError> {
        debug!(id = %id, "Deleting user");

        let user = self.repository.find_by_id(&id).await?.ok_or_else(|| {
            warn!(id = %id, "User not found for deletion");
            UserRepositoryError::NotFound
        })?;

        self.repository.delete(&id).await?;

        let event = create_user_deleted_event(&user)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| UserRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %id, "User deleted");
        Ok(())
    }
}
