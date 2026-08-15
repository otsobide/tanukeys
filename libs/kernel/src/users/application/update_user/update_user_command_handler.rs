//! [`CommandHandler`] for the update-user use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::users::application::find_user::find_user_response::UserErrorEntry;
use crate::users::domain::errors::user_repository_error::UserRepositoryError;

use super::update_user_command::UpdateUserCommand;
use super::update_user_response::UpdateUserResponse;
use super::user_updater::UserUpdater;

/// [`CommandHandler`] that processes [`UpdateUserCommand`]s by delegating
/// to [`UserUpdater`].
pub struct UpdateUserCommandHandler {
    updater: UserUpdater,
}

impl UpdateUserCommandHandler {
    pub fn new(updater: UserUpdater) -> Self {
        Self { updater }
    }
}

#[async_trait]
impl CommandHandler<UpdateUserCommand> for UpdateUserCommandHandler {
    type Response = UpdateUserResponse;

    async fn handle(&self, command: UpdateUserCommand) -> Result<Self::Response, CommandBusError> {
        match self
            .updater
            .execute(command.id, command.name, command.description)
            .await
        {
            Ok(()) => Ok(UpdateUserResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    UserRepositoryError::NotFound => "NotFound",
                    UserRepositoryError::AlreadyExists => "AlreadyExists",
                    UserRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(UpdateUserResponse {
                    error: Some(UserErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
