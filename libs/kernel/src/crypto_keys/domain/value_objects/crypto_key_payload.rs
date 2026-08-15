//! Value Object for the raw crypto key material.

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// Maximum size of a crypto key payload, in bytes.
const MAX_BYTES: usize = 64 * 1024;

/// An immutable Value Object wrapping the raw bytes of a
/// [`CryptoKey`](crate::crypto_keys::domain::entities::crypto_key::CryptoKey).
///
/// The payload is opaque to the platform: it can hold DER, PEM, an
/// ASCII-armored OpenPGP block, an OpenSSH public key line, or any other
/// encoding, as raw bytes. It is non-empty and at most [`MAX_BYTES`] bytes.
/// Transport encodings (e.g. base64 over HTTP) belong to the delivery layer.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct CryptoKeyPayload(Vec<u8>);

impl CryptoKeyPayload {
    /// Creates a `CryptoKeyPayload` from raw bytes.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the payload is empty or
    /// larger than [`MAX_BYTES`] bytes.
    pub fn new(value: impl Into<Vec<u8>>) -> Result<Self, ValueObjectValidationError> {
        let raw = value.into();
        if raw.is_empty() {
            return Err(ValueObjectValidationError::new(
                "crypto key payload must not be empty".to_string(),
            ));
        }
        if raw.len() > MAX_BYTES {
            return Err(ValueObjectValidationError::new(format!(
                "crypto key payload must not be larger than {MAX_BYTES} bytes"
            )));
        }
        Ok(Self(raw))
    }

    /// Returns the underlying bytes.
    pub fn value(&self) -> &[u8] {
        &self.0
    }
}
