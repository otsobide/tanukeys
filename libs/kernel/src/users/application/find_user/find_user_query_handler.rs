//! [`QueryHandler`] for the find-user use case.

use async_trait::async_trait;
use shared_cqrs::query::domain::query_bus_error::QueryBusError;
use shared_cqrs::query::domain::query_handler::QueryHandler;

use crate::users::application::find_user::find_user_response::{UserEntry, UserErrorEntry};
use crate::users::domain::errors::user_repository_error::UserRepositoryError;

use super::find_user_query::FindUserQuery;
use super::find_user_response::FindUserResponse;
use super::user_finder::UserFinder;

/// [`QueryHandler`] that processes [`FindUserQuery`]s by delegating to
/// [`UserFinder`].
///
/// The finder returns a domain entity. This handler maps it to a response DTO.
pub struct FindUserQueryHandler {
    finder: UserFinder,
}

impl FindUserQueryHandler {
    pub fn new(finder: UserFinder) -> Self {
        Self { finder }
    }
}

#[async_trait]
impl QueryHandler<FindUserQuery> for FindUserQueryHandler {
    type Response = FindUserResponse;

    async fn handle(&self, query: FindUserQuery) -> Result<Self::Response, QueryBusError> {
        match self.finder.execute(query.id).await {
            Ok(user) => Ok(FindUserResponse {
                user: Some(UserEntry {
                    id: user.id().value().to_string(),
                    name: user.name().value().to_string(),
                    description: user.description().value().map(ToString::to_string),
                }),
                error: None,
            }),
            Err(e) => {
                let concept = match &e {
                    UserRepositoryError::NotFound => "NotFound",
                    UserRepositoryError::AlreadyExists => "AlreadyExists",
                    UserRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(FindUserResponse {
                    user: None,
                    error: Some(UserErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
