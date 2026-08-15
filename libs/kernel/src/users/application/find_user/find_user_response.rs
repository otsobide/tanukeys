//! Response types for the find-user use case.

/// Data entry DTO for a user.
pub struct UserEntry {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

/// Structured error DTO for user operations.
pub struct UserErrorEntry {
    pub message: String,
    pub concept: String,
}

/// Response envelope returned by [`FindUserQueryHandler`].
///
/// On success, `user` contains the data and `error` is `None`.
/// On failure, `user` is `None` and `error` contains the structured error.
///
/// [`FindUserQueryHandler`]: super::find_user_query_handler::FindUserQueryHandler
pub struct FindUserResponse {
    pub user: Option<UserEntry>,
    pub error: Option<UserErrorEntry>,
}
