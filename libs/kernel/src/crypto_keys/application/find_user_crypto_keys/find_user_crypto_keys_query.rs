//! Query for listing the crypto keys of a user.

use shared_cqrs::query::domain::query::Query;

use crate::users::domain::value_objects::user_id::UserId;

/// Query that requests every crypto key owned by the given [`UserId`].
pub struct FindUserCryptoKeysQuery {
    pub user_id: UserId,
}

impl Query for FindUserCryptoKeysQuery {}
