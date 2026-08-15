//! Response types for the find-crypto-key use case.

use std::time::{SystemTime, UNIX_EPOCH};

use crate::crypto_keys::domain::entities::crypto_key::CryptoKey;

/// Data entry DTO for a crypto key.
///
/// Timestamps are expressed as Unix epoch seconds; turning them into a
/// human-facing format (e.g. RFC 3339) is the delivery layer's job.
pub struct CryptoKeyEntry {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub protocol: String,
    pub algorithm: String,
    pub kind: String,
    pub payload: Vec<u8>,
    pub created_at: u64,
    pub updated_at: u64,
}

impl CryptoKeyEntry {
    /// Maps a domain [`CryptoKey`] to its DTO representation.
    pub fn from_domain(crypto_key: &CryptoKey) -> Self {
        Self {
            id: crypto_key.id().value().to_string(),
            user_id: crypto_key.user_id().value().to_string(),
            name: crypto_key.name().value().to_string(),
            protocol: crypto_key.protocol().as_str().to_string(),
            algorithm: crypto_key.algorithm().as_str().to_string(),
            kind: crypto_key.kind().as_str().to_string(),
            payload: crypto_key.payload().value().to_vec(),
            created_at: unix_seconds(crypto_key.timestamps().created_at()),
            updated_at: unix_seconds(crypto_key.timestamps().updated_at()),
        }
    }
}

/// Structured error DTO for crypto key operations.
pub struct CryptoKeyErrorEntry {
    pub message: String,
    pub concept: String,
}

/// Response envelope returned by [`FindCryptoKeyQueryHandler`].
///
/// On success, `crypto_key` contains the data and `error` is `None`.
/// On failure, `crypto_key` is `None` and `error` contains the structured error.
///
/// [`FindCryptoKeyQueryHandler`]: super::find_crypto_key_query_handler::FindCryptoKeyQueryHandler
pub struct FindCryptoKeyResponse {
    pub crypto_key: Option<CryptoKeyEntry>,
    pub error: Option<CryptoKeyErrorEntry>,
}

fn unix_seconds(instant: SystemTime) -> u64 {
    instant
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}
