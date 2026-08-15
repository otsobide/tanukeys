//! Factory function for [`UserUpdatedEvent`].

use crate::users::domain::entities::user::User;
use crate::users::domain::errors::user_repository_error::UserRepositoryError;
use crate::users::domain::events::user_updated_event::UserUpdatedEvent;

/// Creates a [`UserUpdatedEvent`] from the updated and previous users.
pub fn create_user_updated_event(
    updated: &User,
    previous: &User,
) -> Result<UserUpdatedEvent, UserRepositoryError> {
    Ok(UserUpdatedEvent::new(
        updated.id().clone(),
        updated.name().clone(),
        previous.name().clone(),
        updated.description().clone(),
        previous.description().clone(),
    ))
}
