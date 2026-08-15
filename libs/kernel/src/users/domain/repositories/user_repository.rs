//! Repository trait for the user aggregate.

use async_trait::async_trait;

use crate::users::domain::entities::user::User;
use crate::users::domain::errors::user_repository_error::UserRepositoryError;
use crate::users::domain::value_objects::user_id::UserId;

/// Async persistence contract for [`User`] aggregates.
///
/// Concrete implementations are found in the `infrastructure` layer.
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Persists a new user.
    ///
    /// # Errors
    ///
    /// Returns [`UserRepositoryError::AlreadyExists`] if a user with the
    /// same id already exists, or [`UserRepositoryError::Unexpected`] on
    /// storage failure.
    async fn save(&self, user: &User) -> Result<(), UserRepositoryError>;

    /// Retrieves a user by its [`UserId`].
    ///
    /// Returns `Ok(None)` if no user is found.
    ///
    /// # Errors
    ///
    /// Returns [`UserRepositoryError::Unexpected`] on storage failure.
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, UserRepositoryError>;

    /// Updates an existing user.
    ///
    /// # Errors
    ///
    /// Returns [`UserRepositoryError::NotFound`] if the user does not
    /// exist, or [`UserRepositoryError::Unexpected`] on storage failure.
    async fn update(&self, user: &User) -> Result<(), UserRepositoryError>;

    /// Deletes a user by its [`UserId`].
    ///
    /// # Errors
    ///
    /// Returns [`UserRepositoryError::NotFound`] if the user does not
    /// exist, or [`UserRepositoryError::Unexpected`] on storage failure.
    async fn delete(&self, id: &UserId) -> Result<(), UserRepositoryError>;
}
