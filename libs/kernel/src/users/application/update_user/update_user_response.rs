//! Response for the update-user use case.

use crate::users::application::find_user::find_user_response::UserErrorEntry;

/// Response envelope returned by [`UpdateUserCommandHandler`].
///
/// On success, `error` is `None`. On failure, `error` contains the structured error.
///
/// [`UpdateUserCommandHandler`]: super::update_user_command_handler::UpdateUserCommandHandler
pub struct UpdateUserResponse {
    pub error: Option<UserErrorEntry>,
}
