//! Factory function for [`UserCreatedEvent`].

use crate::users::domain::entities::user::User;
use crate::users::domain::errors::user_repository_error::UserRepositoryError;
use crate::users::domain::events::user_created_event::UserCreatedEvent;

/// Creates a [`UserCreatedEvent`] from the given user.
pub fn create_user_created_event(user: &User) -> Result<UserCreatedEvent, UserRepositoryError> {
    Ok(UserCreatedEvent::new(
        user.id().clone(),
        user.name().clone(),
        user.description().clone(),
    ))
}
