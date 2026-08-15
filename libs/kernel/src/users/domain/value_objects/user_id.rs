//! Value Object for the platform-wide user identifier.

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;
use uuid::Uuid;

/// An immutable Value Object wrapping the UUID v4 that is the true identity
/// of a [`User`](crate::users::domain::entities::user::User) across the
/// platform.
///
/// The value is normalized to the canonical lowercase hyphenated form, so
/// two `UserId`s created from equivalent representations compare equal.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct UserId(String);

impl UserId {
    /// Creates a `UserId` from a raw string.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the value is not a valid
    /// UUID or is not version 4.
    pub fn new(value: impl Into<String>) -> Result<Self, ValueObjectValidationError> {
        let raw = value.into();
        let uuid = Uuid::parse_str(&raw).map_err(|_| {
            ValueObjectValidationError::new(format!("user id <{raw}> is not a valid UUID"))
        })?;
        if uuid.get_version_num() != 4 {
            return Err(ValueObjectValidationError::new(format!(
                "user id <{raw}> is not a version 4 UUID"
            )));
        }
        Ok(Self(uuid.to_string()))
    }

    /// Generates a new random `UserId`.
    pub fn generate() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Returns the canonical string representation of the id.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
