//! Factory function for [`UserDeletedEvent`].

use crate::users::domain::entities::user::User;
use crate::users::domain::errors::user_repository_error::UserRepositoryError;
use crate::users::domain::events::user_deleted_event::UserDeletedEvent;

/// Creates a [`UserDeletedEvent`] from the deleted user.
pub fn create_user_deleted_event(user: &User) -> Result<UserDeletedEvent, UserRepositoryError> {
    Ok(UserDeletedEvent::new(
        user.id().clone(),
        user.name().clone(),
    ))
}
