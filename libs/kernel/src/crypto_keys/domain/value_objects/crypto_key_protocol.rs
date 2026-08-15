//! Value Object for the protocol (format/ecosystem) of a crypto key.

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// The protocol or format ecosystem a
/// [`CryptoKey`](crate::crypto_keys::domain::entities::crypto_key::CryptoKey)
/// belongs to.
///
/// A protocol is *not* an algorithm: an OpenPGP key can internally be RSA or
/// Ed25519. The protocol describes the envelope the payload is encoded in
/// and the ecosystem the key is meant for; the
/// [`CryptoKeyAlgorithm`](super::crypto_key_algorithm::CryptoKeyAlgorithm)
/// describes the underlying cryptography.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum CryptoKeyProtocol {
    /// An OpenPGP key (RFC 4880), e.g. an ASCII-armored public key block.
    OpenPgp,
    /// An SSH key, e.g. an `authorized_keys`-style public key line.
    Ssh,
    /// An X.509 ecosystem key, e.g. a PEM/DER SubjectPublicKeyInfo.
    X509,
    /// Raw key material with no envelope.
    Raw,
}

impl CryptoKeyProtocol {
    /// Creates a `CryptoKeyProtocol` from a raw string.
    ///
    /// Input is matched case-insensitively against the canonical names;
    /// `"pgp"` and `"x.509"` are accepted as aliases and normalized.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if the value does not name a
    /// supported protocol.
    pub fn new(value: impl Into<String>) -> Result<Self, ValueObjectValidationError> {
        let raw = value.into();
        match raw.to_lowercase().as_str() {
            "openpgp" | "pgp" => Ok(CryptoKeyProtocol::OpenPgp),
            "ssh" => Ok(CryptoKeyProtocol::Ssh),
            "x509" | "x.509" => Ok(CryptoKeyProtocol::X509),
            "raw" => Ok(CryptoKeyProtocol::Raw),
            _ => Err(ValueObjectValidationError::new(format!(
                "crypto key protocol <{raw}> is not supported; valid values are: \
                 openpgp, ssh, x509, raw"
            ))),
        }
    }

    /// Returns the canonical string representation of the protocol.
    pub fn as_str(&self) -> &'static str {
        match self {
            CryptoKeyProtocol::OpenPgp => "openpgp",
            CryptoKeyProtocol::Ssh => "ssh",
            CryptoKeyProtocol::X509 => "x509",
            CryptoKeyProtocol::Raw => "raw",
        }
    }
}

impl std::fmt::Display for CryptoKeyProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
