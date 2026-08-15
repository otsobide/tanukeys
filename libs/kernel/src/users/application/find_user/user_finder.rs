//! Domain service for finding a single user.

use std::sync::Arc;

use tracing::debug;

use crate::users::domain::entities::user::User;
use crate::users::domain::errors::user_repository_error::UserRepositoryError;
use crate::users::domain::repositories::user_repository::UserRepository;
use crate::users::domain::value_objects::user_id::UserId;

/// Domain service that looks up a single [`User`] by id.
///
/// Returns the domain entity directly. The handler is responsible for
/// mapping it to a response DTO.
pub struct UserFinder {
    repository: Arc<dyn UserRepository>,
}

impl UserFinder {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: UserId) -> Result<User, UserRepositoryError> {
        debug!(id = %id, "Finding user");
        let user = self.repository.find_by_id(&id).await?;

        user.ok_or(UserRepositoryError::NotFound)
    }
}
