//! Response for the delete-user use case.

use crate::users::application::find_user::find_user_response::UserErrorEntry;

/// Response envelope returned by [`DeleteUserCommandHandler`].
///
/// On success, `error` is `None`. On failure, `error` contains the structured error.
///
/// [`DeleteUserCommandHandler`]: super::delete_user_command_handler::DeleteUserCommandHandler
pub struct DeleteUserResponse {
    pub error: Option<UserErrorEntry>,
}
