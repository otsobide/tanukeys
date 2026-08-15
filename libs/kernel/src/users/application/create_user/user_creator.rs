//! Domain service for creating users.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info};

use crate::users::domain::entities::user::User;
use crate::users::domain::errors::user_repository_error::UserRepositoryError;
use crate::users::domain::events::create_user_created_event::create_user_created_event;
use crate::users::domain::repositories::user_repository::UserRepository;
use crate::users::domain::value_objects::user_description::UserDescription;
use crate::users::domain::value_objects::user_id::UserId;
use crate::users::domain::value_objects::user_name::UserName;

/// Domain service that persists a new [`User`] and publishes a
/// [`UserCreatedEvent`] via the event bus.
///
/// [`UserCreatedEvent`]: crate::users::domain::events::user_created_event::UserCreatedEvent
pub struct UserCreator {
    repository: Arc<dyn UserRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl UserCreator {
    pub fn new(repository: Arc<dyn UserRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self { repository, event_bus }
    }

    pub async fn execute(
        &self,
        id: UserId,
        name: UserName,
        description: UserDescription,
    ) -> Result<(), UserRepositoryError> {
        let user = User::new(id, name, description);
        debug!(id = %user.id(), "Creating user");

        self.repository.save(&user).await?;

        let event = create_user_created_event(&user)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| UserRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %user.id(), "User created");
        Ok(())
    }
}
