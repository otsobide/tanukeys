//! Value Object for the lifecycle timestamps of a crypto key.

use std::time::SystemTime;

use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// An immutable Value Object holding the lifecycle timestamps of a
/// [`CryptoKey`](crate::crypto_keys::domain::entities::crypto_key::CryptoKey).
///
/// Keeping both instants in one Value Object lets the invariant
/// `updated_at >= created_at` live in a single place. Timestamps are plain
/// [`SystemTime`]s; formatting them (e.g. RFC 3339) belongs to the
/// delivery layer.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CryptoKeyTimestamps {
    created_at: SystemTime,
    updated_at: SystemTime,
}

impl CryptoKeyTimestamps {
    /// Creates a `CryptoKeyTimestamps` from explicit instants.
    ///
    /// Useful when reconstituting a key from a persistent store.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] if `updated_at` is earlier
    /// than `created_at`.
    pub fn new(
        created_at: SystemTime,
        updated_at: SystemTime,
    ) -> Result<Self, ValueObjectValidationError> {
        if updated_at < created_at {
            return Err(ValueObjectValidationError::new(
                "crypto key updated_at must not be earlier than created_at".to_string(),
            ));
        }
        Ok(Self {
            created_at,
            updated_at,
        })
    }

    /// Creates a `CryptoKeyTimestamps` for a key born right now.
    ///
    /// Both instants are set to the same current time.
    pub fn now() -> Self {
        let now = SystemTime::now();
        Self {
            created_at: now,
            updated_at: now,
        }
    }

    /// Returns a copy with `updated_at` moved to the current time.
    ///
    /// The creation instant is preserved. If the clock reports a time
    /// earlier than `created_at`, the creation instant is used instead so
    /// the invariant always holds.
    pub fn touch(&self) -> Self {
        Self {
            created_at: self.created_at,
            updated_at: SystemTime::now().max(self.created_at),
        }
    }

    /// Returns the instant the key was created.
    pub fn created_at(&self) -> SystemTime {
        self.created_at
    }

    /// Returns the instant the key was last updated.
    pub fn updated_at(&self) -> SystemTime {
        self.updated_at
    }
}
