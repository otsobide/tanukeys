//! [`CommandHandler`] for the update-crypto-key use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::crypto_keys::application::find_crypto_key::find_crypto_key_response::CryptoKeyErrorEntry;
use crate::crypto_keys::domain::errors::crypto_key_repository_error::CryptoKeyRepositoryError;

use super::crypto_key_updater::CryptoKeyUpdater;
use super::update_crypto_key_command::UpdateCryptoKeyCommand;
use super::update_crypto_key_response::UpdateCryptoKeyResponse;

/// [`CommandHandler`] that processes [`UpdateCryptoKeyCommand`]s by
/// delegating to [`CryptoKeyUpdater`].
pub struct UpdateCryptoKeyCommandHandler {
    updater: CryptoKeyUpdater,
}

impl UpdateCryptoKeyCommandHandler {
    pub fn new(updater: CryptoKeyUpdater) -> Self {
        Self { updater }
    }
}

#[async_trait]
impl CommandHandler<UpdateCryptoKeyCommand> for UpdateCryptoKeyCommandHandler {
    type Response = UpdateCryptoKeyResponse;

    async fn handle(
        &self,
        command: UpdateCryptoKeyCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self
            .updater
            .execute(
                command.id,
                command.name,
                command.protocol,
                command.algorithm,
                command.payload,
            )
            .await
        {
            Ok(()) => Ok(UpdateCryptoKeyResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    CryptoKeyRepositoryError::NotFound => "NotFound",
                    CryptoKeyRepositoryError::AlreadyExists => "AlreadyExists",
                    CryptoKeyRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(UpdateCryptoKeyResponse {
                    error: Some(CryptoKeyErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
