//! Command for updating a user.

use shared_cqrs::command::domain::command::Command;

use crate::users::domain::value_objects::user_description::UserDescription;
use crate::users::domain::value_objects::user_id::UserId;
use crate::users::domain::value_objects::user_name::UserName;

/// Command that requests an update of an existing user.
pub struct UpdateUserCommand {
    pub id: UserId,
    pub name: UserName,
    pub description: UserDescription,
}

impl Command for UpdateUserCommand {}
