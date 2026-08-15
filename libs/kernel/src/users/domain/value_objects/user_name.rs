//! Value Object for the public user name (handle).

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// Characters allowed in a user name.
const ALLOWED_CHARS: &str = "abcdefghijklmnopqrstuvwxyz0123456789_.-";

/// Maximum length of a user name, in characters.
const MAX_LENGTH: usize = 50;

/// An immutable Value Object wrapping the public handle of a
/// [`User`](crate::users::domain::entities::user::User).
///
/// A user name is non-empty, at most [`MAX_LENGTH`] characters, and made of
/// lowercase letters, digits, `-`, `_` or `.` only.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct UserName(String);

impl UserName {
    /// Creates a `UserName` from a raw string.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if any invariant is violated.
    pub fn new(value: impl Into<String>) -> Result<Self, ValueObjectValidationError> {
        let raw = value.into();
        if raw.is_empty() {
            return Err(ValueObjectValidationError::new(
                "user name must not be empty".to_string(),
            ));
        }
        if raw.chars().count() > MAX_LENGTH {
            return Err(ValueObjectValidationError::new(format!(
                "user name must not be longer than {MAX_LENGTH} characters"
            )));
        }
        if raw.contains(char::is_whitespace) {
            return Err(ValueObjectValidationError::new(
                "user name must not contain whitespace".to_string(),
            ));
        }
        if raw.chars().any(|c| c.is_uppercase()) {
            return Err(ValueObjectValidationError::new(
                "user name must not contain uppercase characters".to_string(),
            ));
        }
        if raw.chars().any(|c| !ALLOWED_CHARS.contains(c)) {
            return Err(ValueObjectValidationError::new(
                "user name may only contain lowercase letters, digits or <-._>".to_string(),
            ));
        }
        Ok(Self(raw))
    }

    /// Returns the underlying user name string.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for UserName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
