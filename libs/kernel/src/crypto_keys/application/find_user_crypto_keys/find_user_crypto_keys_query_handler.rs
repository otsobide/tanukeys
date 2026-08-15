//! [`QueryHandler`] for the find-user-crypto-keys use case.

use async_trait::async_trait;
use shared_cqrs::query::domain::query_bus_error::QueryBusError;
use shared_cqrs::query::domain::query_handler::QueryHandler;

use crate::crypto_keys::application::find_crypto_key::find_crypto_key_response::{
    CryptoKeyEntry, CryptoKeyErrorEntry,
};
use crate::crypto_keys::domain::errors::crypto_key_repository_error::CryptoKeyRepositoryError;

use super::find_user_crypto_keys_query::FindUserCryptoKeysQuery;
use super::find_user_crypto_keys_response::FindUserCryptoKeysResponse;
use super::user_crypto_keys_finder::UserCryptoKeysFinder;

/// [`QueryHandler`] that processes [`FindUserCryptoKeysQuery`]s by
/// delegating to [`UserCryptoKeysFinder`].
///
/// The finder returns domain entities. This handler maps them to DTOs.
pub struct FindUserCryptoKeysQueryHandler {
    finder: UserCryptoKeysFinder,
}

impl FindUserCryptoKeysQueryHandler {
    pub fn new(finder: UserCryptoKeysFinder) -> Self {
        Self { finder }
    }
}

#[async_trait]
impl QueryHandler<FindUserCryptoKeysQuery> for FindUserCryptoKeysQueryHandler {
    type Response = FindUserCryptoKeysResponse;

    async fn handle(
        &self,
        query: FindUserCryptoKeysQuery,
    ) -> Result<Self::Response, QueryBusError> {
        match self.finder.execute(query.user_id).await {
            Ok(crypto_keys) => Ok(FindUserCryptoKeysResponse {
                crypto_keys: Some(
                    crypto_keys
                        .iter()
                        .map(CryptoKeyEntry::from_domain)
                        .collect(),
                ),
                error: None,
            }),
            Err(e) => {
                let concept = match &e {
                    CryptoKeyRepositoryError::NotFound => "NotFound",
                    CryptoKeyRepositoryError::AlreadyExists => "AlreadyExists",
                    CryptoKeyRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(FindUserCryptoKeysResponse {
                    crypto_keys: None,
                    error: Some(CryptoKeyErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
