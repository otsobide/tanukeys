//! Command for deleting a user.

use shared_cqrs::command::domain::command::Command;

use crate::users::domain::value_objects::user_id::UserId;

/// Command that requests the deletion of a user by id.
pub struct DeleteUserCommand {
    pub id: UserId,
}

impl Command for DeleteUserCommand {}
