//! [`QueryHandler`] for the find-crypto-key use case.

use async_trait::async_trait;
use shared_cqrs::query::domain::query_bus_error::QueryBusError;
use shared_cqrs::query::domain::query_handler::QueryHandler;

use crate::crypto_keys::application::find_crypto_key::find_crypto_key_response::{
    CryptoKeyEntry, CryptoKeyErrorEntry,
};
use crate::crypto_keys::domain::errors::crypto_key_repository_error::CryptoKeyRepositoryError;

use super::crypto_key_finder::CryptoKeyFinder;
use super::find_crypto_key_query::FindCryptoKeyQuery;
use super::find_crypto_key_response::FindCryptoKeyResponse;

/// [`QueryHandler`] that processes [`FindCryptoKeyQuery`]s by delegating to
/// [`CryptoKeyFinder`].
///
/// The finder returns a domain entity. This handler maps it to a response DTO.
pub struct FindCryptoKeyQueryHandler {
    finder: CryptoKeyFinder,
}

impl FindCryptoKeyQueryHandler {
    pub fn new(finder: CryptoKeyFinder) -> Self {
        Self { finder }
    }
}

#[async_trait]
impl QueryHandler<FindCryptoKeyQuery> for FindCryptoKeyQueryHandler {
    type Response = FindCryptoKeyResponse;

    async fn handle(&self, query: FindCryptoKeyQuery) -> Result<Self::Response, QueryBusError> {
        match self.finder.execute(query.id).await {
            Ok(crypto_key) => Ok(FindCryptoKeyResponse {
                crypto_key: Some(CryptoKeyEntry::from_domain(&crypto_key)),
                error: None,
            }),
            Err(e) => {
                let concept = match &e {
                    CryptoKeyRepositoryError::NotFound => "NotFound",
                    CryptoKeyRepositoryError::AlreadyExists => "AlreadyExists",
                    CryptoKeyRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(FindCryptoKeyResponse {
                    crypto_key: None,
                    error: Some(CryptoKeyErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
