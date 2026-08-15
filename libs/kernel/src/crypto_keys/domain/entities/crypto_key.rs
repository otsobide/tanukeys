//! `CryptoKey` aggregate root.

use crate::crypto_keys::domain::value_objects::crypto_key_algorithm::CryptoKeyAlgorithm;
use crate::crypto_keys::domain::value_objects::crypto_key_id::CryptoKeyId;
use crate::crypto_keys::domain::value_objects::crypto_key_kind::CryptoKeyKind;
use crate::crypto_keys::domain::value_objects::crypto_key_name::CryptoKeyName;
use crate::crypto_keys::domain::value_objects::crypto_key_payload::CryptoKeyPayload;
use crate::crypto_keys::domain::value_objects::crypto_key_protocol::CryptoKeyProtocol;
use crate::crypto_keys::domain::value_objects::crypto_key_timestamps::CryptoKeyTimestamps;
use crate::users::domain::value_objects::user_id::UserId;

/// Aggregate root representing a cryptographic key owned by a user.
///
/// The [`CryptoKeyId`] (a UUID v4) is the identity of the key across the
/// platform. The owner is referenced by [`UserId`] only, never by embedding
/// the user aggregate. The key material itself is an opaque
/// [`CryptoKeyPayload`]; the platform stores and serves it but never
/// interprets it.
#[derive(Clone)]
pub struct CryptoKey {
    /// Platform-wide unique identifier.
    id: CryptoKeyId,
    /// Id of the user that owns this key.
    user_id: UserId,
    /// Human-readable label.
    name: CryptoKeyName,
    /// Format ecosystem the key belongs to (OpenPGP, SSH, ...).
    protocol: CryptoKeyProtocol,
    /// Underlying cryptographic algorithm (RSA, Ed25519, ...).
    algorithm: CryptoKeyAlgorithm,
    /// Raw key material.
    payload: CryptoKeyPayload,
    /// Creation and last-update instants.
    timestamps: CryptoKeyTimestamps,
}

impl CryptoKey {
    /// Creates a new `CryptoKey`.
    pub fn new(
        id: CryptoKeyId,
        user_id: UserId,
        name: CryptoKeyName,
        protocol: CryptoKeyProtocol,
        algorithm: CryptoKeyAlgorithm,
        payload: CryptoKeyPayload,
        timestamps: CryptoKeyTimestamps,
    ) -> Self {
        Self {
            id,
            user_id,
            name,
            protocol,
            algorithm,
            payload,
            timestamps,
        }
    }

    /// Returns the platform-wide identifier of this key.
    pub fn id(&self) -> &CryptoKeyId {
        &self.id
    }

    /// Returns the id of the user that owns this key.
    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    /// Returns the human-readable label of this key.
    pub fn name(&self) -> &CryptoKeyName {
        &self.name
    }

    /// Returns the protocol this key belongs to.
    pub fn protocol(&self) -> CryptoKeyProtocol {
        self.protocol
    }

    /// Returns the cryptographic algorithm of this key.
    pub fn algorithm(&self) -> CryptoKeyAlgorithm {
        self.algorithm
    }

    /// Returns the kind of this key, derived from its algorithm.
    pub fn kind(&self) -> CryptoKeyKind {
        self.algorithm.kind()
    }

    /// Returns the raw key material.
    pub fn payload(&self) -> &CryptoKeyPayload {
        &self.payload
    }

    /// Returns the lifecycle timestamps of this key.
    pub fn timestamps(&self) -> &CryptoKeyTimestamps {
        &self.timestamps
    }
}
