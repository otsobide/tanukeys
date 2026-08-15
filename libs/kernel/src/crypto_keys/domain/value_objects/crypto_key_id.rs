//! Value Object for the platform-wide crypto key identifier.

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;
use uuid::Uuid;

/// An immutable Value Object wrapping the UUID v4 that identifies a
/// [`CryptoKey`](crate::crypto_keys::domain::entities::crypto_key::CryptoKey)
/// across the platform.
///
/// The value is normalized to the canonical lowercase hyphenated form, so
/// two `CryptoKeyId`s created from equivalent representations compare equal.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct CryptoKeyId(String);

impl CryptoKeyId {
    /// Creates a `CryptoKeyId` from a raw string.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the value is not a valid
    /// UUID or is not version 4.
    pub fn new(value: impl Into<String>) -> Result<Self, ValueObjectValidationError> {
        let raw = value.into();
        let uuid = Uuid::parse_str(&raw).map_err(|_| {
            ValueObjectValidationError::new(format!("crypto key id <{raw}> is not a valid UUID"))
        })?;
        if uuid.get_version_num() != 4 {
            return Err(ValueObjectValidationError::new(format!(
                "crypto key id <{raw}> is not a version 4 UUID"
            )));
        }
        Ok(Self(uuid.to_string()))
    }

    /// Generates a new random `CryptoKeyId`.
    pub fn generate() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Returns the canonical string representation of the id.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for CryptoKeyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
