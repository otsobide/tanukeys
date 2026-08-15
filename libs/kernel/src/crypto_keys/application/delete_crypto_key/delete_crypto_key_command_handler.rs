//! [`CommandHandler`] for the delete-crypto-key use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::crypto_keys::application::find_crypto_key::find_crypto_key_response::CryptoKeyErrorEntry;
use crate::crypto_keys::domain::errors::crypto_key_repository_error::CryptoKeyRepositoryError;

use super::crypto_key_deleter::CryptoKeyDeleter;
use super::delete_crypto_key_command::DeleteCryptoKeyCommand;
use super::delete_crypto_key_response::DeleteCryptoKeyResponse;

/// [`CommandHandler`] that processes [`DeleteCryptoKeyCommand`]s by
/// delegating to [`CryptoKeyDeleter`].
pub struct DeleteCryptoKeyCommandHandler {
    deleter: CryptoKeyDeleter,
}

impl DeleteCryptoKeyCommandHandler {
    pub fn new(deleter: CryptoKeyDeleter) -> Self {
        Self { deleter }
    }
}

#[async_trait]
impl CommandHandler<DeleteCryptoKeyCommand> for DeleteCryptoKeyCommandHandler {
    type Response = DeleteCryptoKeyResponse;

    async fn handle(
        &self,
        command: DeleteCryptoKeyCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self.deleter.execute(command.id).await {
            Ok(()) => Ok(DeleteCryptoKeyResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    CryptoKeyRepositoryError::NotFound => "NotFound",
                    CryptoKeyRepositoryError::AlreadyExists => "AlreadyExists",
                    CryptoKeyRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(DeleteCryptoKeyResponse {
                    error: Some(CryptoKeyErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
