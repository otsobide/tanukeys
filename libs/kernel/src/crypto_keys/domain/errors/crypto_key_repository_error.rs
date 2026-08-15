//! Error types for crypto key repository operations.

use thiserror::Error;

/// Errors that can occur during [`CryptoKeyRepository`] operations.
///
/// [`CryptoKeyRepository`]: crate::crypto_keys::domain::repositories::crypto_key_repository::CryptoKeyRepository
#[derive(Debug, Error)]
pub enum CryptoKeyRepositoryError {
    /// The requested crypto key was not found.
    #[error("crypto key not found")]
    NotFound,

    /// A crypto key with the same id already exists.
    #[error("crypto key already exists")]
    AlreadyExists,

    /// An unexpected storage error occurred.
    #[error("unexpected error: {0}")]
    Unexpected(String),
}
