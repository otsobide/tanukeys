//! [`CommandHandler`] for the create-user use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::users::application::find_user::find_user_response::UserErrorEntry;
use crate::users::domain::errors::user_repository_error::UserRepositoryError;

use super::create_user_command::CreateUserCommand;
use super::create_user_response::CreateUserResponse;
use super::user_creator::UserCreator;

/// [`CommandHandler`] that processes [`CreateUserCommand`]s by delegating
/// to [`UserCreator`].
pub struct CreateUserCommandHandler {
    creator: UserCreator,
}

impl CreateUserCommandHandler {
    pub fn new(creator: UserCreator) -> Self {
        Self { creator }
    }
}

#[async_trait]
impl CommandHandler<CreateUserCommand> for CreateUserCommandHandler {
    type Response = CreateUserResponse;

    async fn handle(&self, command: CreateUserCommand) -> Result<Self::Response, CommandBusError> {
        match self
            .creator
            .execute(command.id, command.name, command.description)
            .await
        {
            Ok(()) => Ok(CreateUserResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    UserRepositoryError::NotFound => "NotFound",
                    UserRepositoryError::AlreadyExists => "AlreadyExists",
                    UserRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(CreateUserResponse {
                    error: Some(UserErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
