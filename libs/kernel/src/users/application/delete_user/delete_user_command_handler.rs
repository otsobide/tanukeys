//! [`CommandHandler`] for the delete-user use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::users::application::find_user::find_user_response::UserErrorEntry;
use crate::users::domain::errors::user_repository_error::UserRepositoryError;

use super::delete_user_command::DeleteUserCommand;
use super::delete_user_response::DeleteUserResponse;
use super::user_deleter::UserDeleter;

/// [`CommandHandler`] that processes [`DeleteUserCommand`]s by delegating
/// to [`UserDeleter`].
pub struct DeleteUserCommandHandler {
    deleter: UserDeleter,
}

impl DeleteUserCommandHandler {
    pub fn new(deleter: UserDeleter) -> Self {
        Self { deleter }
    }
}

#[async_trait]
impl CommandHandler<DeleteUserCommand> for DeleteUserCommandHandler {
    type Response = DeleteUserResponse;

    async fn handle(&self, command: DeleteUserCommand) -> Result<Self::Response, CommandBusError> {
        match self.deleter.execute(command.id).await {
            Ok(()) => Ok(DeleteUserResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    UserRepositoryError::NotFound => "NotFound",
                    UserRepositoryError::AlreadyExists => "AlreadyExists",
                    UserRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(DeleteUserResponse {
                    error: Some(UserErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
