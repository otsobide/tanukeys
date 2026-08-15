//! `User` aggregate root.

use crate::users::domain::value_objects::user_description::UserDescription;
use crate::users::domain::value_objects::user_id::UserId;
use crate::users::domain::value_objects::user_name::UserName;

/// Aggregate root representing a platform user.
///
/// The [`UserId`] (a UUID v4) is the true identity of the user across the
/// platform; the [`UserName`] is the public handle shown to other users.
#[derive(Clone)]
pub struct User {
    /// Platform-wide unique identifier.
    id: UserId,
    /// Public handle.
    name: UserName,
    /// Optional free-form description.
    description: UserDescription,
}

impl User {
    /// Creates a new `User`.
    pub fn new(id: UserId, name: UserName, description: UserDescription) -> Self {
        Self { id, name, description }
    }

    /// Returns the platform-wide identifier of this user.
    pub fn id(&self) -> &UserId {
        &self.id
    }

    /// Returns the public handle of this user.
    pub fn name(&self) -> &UserName {
        &self.name
    }

    /// Returns the description of this user.
    pub fn description(&self) -> &UserDescription {
        &self.description
    }
}
