//! Value Object for the cryptographic algorithm of a crypto key.

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

use super::crypto_key_kind::CryptoKeyKind;

/// The cryptographic algorithm behind the key material of a
/// [`CryptoKey`](crate::crypto_keys::domain::entities::crypto_key::CryptoKey).
///
/// This is a closed set: unknown algorithms are rejected at construction,
/// and supporting a new one means adding a variant here (which also forces
/// the [`kind`](CryptoKeyAlgorithm::kind) mapping to be defined for it).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum CryptoKeyAlgorithm {
    /// RSA (asymmetric).
    Rsa,
    /// Ed25519 (asymmetric, signing).
    Ed25519,
    /// ECDSA over a NIST curve (asymmetric, signing).
    Ecdsa,
    /// X25519 (asymmetric, key agreement).
    X25519,
    /// AES-256 (symmetric).
    Aes256,
    /// ChaCha20 (symmetric).
    ChaCha20,
    /// HMAC (symmetric, authentication).
    Hmac,
}

impl CryptoKeyAlgorithm {
    /// Creates a `CryptoKeyAlgorithm` from a raw string.
    ///
    /// Input is matched case-insensitively against the canonical names.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the value does not name a
    /// supported algorithm.
    pub fn new(value: impl Into<String>) -> Result<Self, ValueObjectValidationError> {
        let raw = value.into();
        match raw.to_lowercase().as_str() {
            "rsa" => Ok(CryptoKeyAlgorithm::Rsa),
            "ed25519" => Ok(CryptoKeyAlgorithm::Ed25519),
            "ecdsa" => Ok(CryptoKeyAlgorithm::Ecdsa),
            "x25519" => Ok(CryptoKeyAlgorithm::X25519),
            "aes256" => Ok(CryptoKeyAlgorithm::Aes256),
            "chacha20" => Ok(CryptoKeyAlgorithm::ChaCha20),
            "hmac" => Ok(CryptoKeyAlgorithm::Hmac),
            _ => Err(ValueObjectValidationError::new(format!(
                "crypto key algorithm <{raw}> is not supported; valid values are: \
                 rsa, ed25519, ecdsa, x25519, aes256, chacha20, hmac"
            ))),
        }
    }

    /// Returns the canonical string representation of the algorithm.
    pub fn as_str(&self) -> &'static str {
        match self {
            CryptoKeyAlgorithm::Rsa => "rsa",
            CryptoKeyAlgorithm::Ed25519 => "ed25519",
            CryptoKeyAlgorithm::Ecdsa => "ecdsa",
            CryptoKeyAlgorithm::X25519 => "x25519",
            CryptoKeyAlgorithm::Aes256 => "aes256",
            CryptoKeyAlgorithm::ChaCha20 => "chacha20",
            CryptoKeyAlgorithm::Hmac => "hmac",
        }
    }

    /// Returns the [`CryptoKeyKind`] this algorithm belongs to.
    pub fn kind(&self) -> CryptoKeyKind {
        match self {
            CryptoKeyAlgorithm::Rsa
            | CryptoKeyAlgorithm::Ed25519
            | CryptoKeyAlgorithm::Ecdsa
            | CryptoKeyAlgorithm::X25519 => CryptoKeyKind::Asymmetric,
            CryptoKeyAlgorithm::Aes256
            | CryptoKeyAlgorithm::ChaCha20
            | CryptoKeyAlgorithm::Hmac => CryptoKeyKind::Symmetric,
        }
    }
}

impl std::fmt::Display for CryptoKeyAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
