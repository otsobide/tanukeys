//! Query for finding a user by id.

use shared_cqrs::query::domain::query::Query;

use crate::users::domain::value_objects::user_id::UserId;

/// Query that requests a single user by its [`UserId`].
pub struct FindUserQuery {
    pub id: UserId,
}

impl Query for FindUserQuery {}
