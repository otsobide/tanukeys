//! Response for the create-user use case.

use crate::users::application::find_user::find_user_response::UserErrorEntry;

/// Response envelope returned by [`CreateUserCommandHandler`].
///
/// On success, `error` is `None`. On failure, `error` contains the structured error.
///
/// [`CreateUserCommandHandler`]: super::create_user_command_handler::CreateUserCommandHandler
pub struct CreateUserResponse {
    pub error: Option<UserErrorEntry>,
}
