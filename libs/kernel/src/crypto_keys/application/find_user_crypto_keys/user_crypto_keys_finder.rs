//! Domain service for listing the crypto keys of a user.

use std::sync::Arc;

use tracing::debug;

use crate::crypto_keys::domain::entities::crypto_key::CryptoKey;
use crate::crypto_keys::domain::errors::crypto_key_repository_error::CryptoKeyRepositoryError;
use crate::crypto_keys::domain::repositories::crypto_key_repository::CryptoKeyRepository;
use crate::users::domain::value_objects::user_id::UserId;

/// Domain service that lists every [`CryptoKey`] owned by a user.
///
/// A user with no keys yields an empty list, not an error. Returns domain
/// entities directly; the handler maps them to response DTOs.
pub struct UserCryptoKeysFinder {
    repository: Arc<dyn CryptoKeyRepository>,
}

impl UserCryptoKeysFinder {
    pub fn new(repository: Arc<dyn CryptoKeyRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        user_id: UserId,
    ) -> Result<Vec<CryptoKey>, CryptoKeyRepositoryError> {
        debug!(user_id = %user_id, "Finding crypto keys for user");
        self.repository.find_by_user_id(&user_id).await
    }
}
