//! Value Object for the human-readable crypto key name.

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// Maximum length of a crypto key name, in characters.
const MAX_LENGTH: usize = 100;

/// An immutable Value Object wrapping the human-readable label of a
/// [`CryptoKey`](crate::crypto_keys::domain::entities::crypto_key::CryptoKey)
/// (e.g. `"laptop ssh key"`).
///
/// Unlike a user handle, any content is allowed, including Unicode; the name
/// is non-empty, at most [`MAX_LENGTH`] characters, and carries no leading
/// or trailing whitespace.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct CryptoKeyName(String);

impl CryptoKeyName {
    /// Creates a `CryptoKeyName` from a raw string.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if any invariant is violated.
    pub fn new(value: impl Into<String>) -> Result<Self, ValueObjectValidationError> {
        let raw = value.into();
        if raw.is_empty() {
            return Err(ValueObjectValidationError::new(
                "crypto key name must not be empty".to_string(),
            ));
        }
        if raw.chars().count() > MAX_LENGTH {
            return Err(ValueObjectValidationError::new(format!(
                "crypto key name must not be longer than {MAX_LENGTH} characters"
            )));
        }
        if raw.starts_with(char::is_whitespace) || raw.ends_with(char::is_whitespace) {
            return Err(ValueObjectValidationError::new(
                "crypto key name must not start or end with whitespace".to_string(),
            ));
        }
        Ok(Self(raw))
    }

    /// Returns the underlying name string.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for CryptoKeyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
