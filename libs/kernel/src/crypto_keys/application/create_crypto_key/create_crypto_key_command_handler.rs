//! [`CommandHandler`] for the create-crypto-key use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::crypto_keys::application::find_crypto_key::find_crypto_key_response::CryptoKeyErrorEntry;
use crate::crypto_keys::domain::errors::crypto_key_repository_error::CryptoKeyRepositoryError;

use super::create_crypto_key_command::CreateCryptoKeyCommand;
use super::create_crypto_key_response::CreateCryptoKeyResponse;
use super::crypto_key_creator::CryptoKeyCreator;

/// [`CommandHandler`] that processes [`CreateCryptoKeyCommand`]s by
/// delegating to [`CryptoKeyCreator`].
pub struct CreateCryptoKeyCommandHandler {
    creator: CryptoKeyCreator,
}

impl CreateCryptoKeyCommandHandler {
    pub fn new(creator: CryptoKeyCreator) -> Self {
        Self { creator }
    }
}

#[async_trait]
impl CommandHandler<CreateCryptoKeyCommand> for CreateCryptoKeyCommandHandler {
    type Response = CreateCryptoKeyResponse;

    async fn handle(
        &self,
        command: CreateCryptoKeyCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self
            .creator
            .execute(
                command.id,
                command.user_id,
                command.name,
                command.protocol,
                command.algorithm,
                command.payload,
            )
            .await
        {
            Ok(()) => Ok(CreateCryptoKeyResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    CryptoKeyRepositoryError::NotFound => "NotFound",
                    CryptoKeyRepositoryError::AlreadyExists => "AlreadyExists",
                    CryptoKeyRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(CreateCryptoKeyResponse {
                    error: Some(CryptoKeyErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
