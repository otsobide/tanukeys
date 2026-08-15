//! Value Object for the free-form user description.

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// Maximum length of a user description, in characters.
const MAX_LENGTH: usize = 600;

/// An immutable Value Object wrapping the optional free-form description of
/// a [`User`](crate::users::domain::entities::user::User).
///
/// The description may be absent (`None`) and, when present, is limited to
/// [`MAX_LENGTH`] characters. Any content is allowed, including Unicode.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UserDescription(Option<String>);

impl UserDescription {
    /// Creates a `UserDescription` from an optional raw string.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the description exceeds
    /// [`MAX_LENGTH`] characters.
    pub fn new(value: Option<String>) -> Result<Self, ValueObjectValidationError> {
        if let Some(ref content) = value {
            if content.chars().count() > MAX_LENGTH {
                return Err(ValueObjectValidationError::new(format!(
                    "user description must not be longer than {MAX_LENGTH} characters"
                )));
            }
        }
        Ok(Self(value))
    }

    /// Creates an empty `UserDescription`.
    pub fn none() -> Self {
        Self(None)
    }

    /// Returns the underlying description, if any.
    pub fn value(&self) -> Option<&str> {
        self.0.as_deref()
    }
}
