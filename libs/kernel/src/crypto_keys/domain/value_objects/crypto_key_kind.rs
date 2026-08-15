//! Value Object for the kind (category) of a crypto key.

/// The category a crypto key belongs to.
///
/// The kind is never stored or supplied by callers: it is **derived** from
/// the key's
/// [`CryptoKeyAlgorithm`](super::crypto_key_algorithm::CryptoKeyAlgorithm),
/// so a key can never claim a kind that contradicts its algorithm.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum CryptoKeyKind {
    /// Public/private key pair algorithms (the stored payload is expected
    /// to be the public half).
    Asymmetric,
    /// Single shared secret algorithms.
    Symmetric,
}

impl CryptoKeyKind {
    /// Returns the canonical string representation of the kind.
    pub fn as_str(&self) -> &'static str {
        match self {
            CryptoKeyKind::Asymmetric => "asymmetric",
            CryptoKeyKind::Symmetric => "symmetric",
        }
    }
}

impl std::fmt::Display for CryptoKeyKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
