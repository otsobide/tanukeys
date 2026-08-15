//! Repository trait for the crypto key aggregate.

use async_trait::async_trait;

use crate::crypto_keys::domain::entities::crypto_key::CryptoKey;
use crate::crypto_keys::domain::errors::crypto_key_repository_error::CryptoKeyRepositoryError;
use crate::crypto_keys::domain::value_objects::crypto_key_id::CryptoKeyId;
use crate::users::domain::value_objects::user_id::UserId;

/// Async persistence contract for [`CryptoKey`] aggregates.
///
/// Concrete implementations are found in the `infrastructure` layer.
#[async_trait]
pub trait CryptoKeyRepository: Send + Sync {
    /// Persists a new crypto key.
    ///
    /// # Errors
    ///
    /// Returns [`CryptoKeyRepositoryError::AlreadyExists`] if a key with
    /// the same id already exists, or
    /// [`CryptoKeyRepositoryError::Unexpected`] on storage failure.
    async fn save(&self, crypto_key: &CryptoKey) -> Result<(), CryptoKeyRepositoryError>;

    /// Retrieves a crypto key by its [`CryptoKeyId`].
    ///
    /// Returns `Ok(None)` if no key is found.
    ///
    /// # Errors
    ///
    /// Returns [`CryptoKeyRepositoryError::Unexpected`] on storage failure.
    async fn find_by_id(
        &self,
        id: &CryptoKeyId,
    ) -> Result<Option<CryptoKey>, CryptoKeyRepositoryError>;

    /// Retrieves every crypto key owned by the given user.
    ///
    /// Returns an empty list if the user owns no keys; an unknown user id
    /// is indistinguishable from a user with no keys at this level.
    ///
    /// # Errors
    ///
    /// Returns [`CryptoKeyRepositoryError::Unexpected`] on storage failure.
    async fn find_by_user_id(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<CryptoKey>, CryptoKeyRepositoryError>;

    /// Updates an existing crypto key.
    ///
    /// # Errors
    ///
    /// Returns [`CryptoKeyRepositoryError::NotFound`] if the key does not
    /// exist, or [`CryptoKeyRepositoryError::Unexpected`] on storage
    /// failure.
    async fn update(&self, crypto_key: &CryptoKey) -> Result<(), CryptoKeyRepositoryError>;

    /// Deletes a crypto key by its [`CryptoKeyId`].
    ///
    /// # Errors
    ///
    /// Returns [`CryptoKeyRepositoryError::NotFound`] if the key does not
    /// exist, or [`CryptoKeyRepositoryError::Unexpected`] on storage
    /// failure.
    async fn delete(&self, id: &CryptoKeyId) -> Result<(), CryptoKeyRepositoryError>;
}
