//! Error types for user repository operations.

use thiserror::Error;

/// Errors that can occur during [`UserRepository`] operations.
///
/// [`UserRepository`]: crate::users::domain::repositories::user_repository::UserRepository
#[derive(Debug, Error)]
pub enum UserRepositoryError {
    /// The requested user was not found.
    #[error("user not found")]
    NotFound,

    /// A user with the same id already exists.
    #[error("user already exists")]
    AlreadyExists,

    /// An unexpected storage error occurred.
    #[error("unexpected error: {0}")]
    Unexpected(String),
}
