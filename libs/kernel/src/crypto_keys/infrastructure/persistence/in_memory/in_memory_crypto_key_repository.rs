//! In-memory implementation of [`CryptoKeyRepository`].

use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;

use crate::crypto_keys::domain::entities::crypto_key::CryptoKey;
use crate::crypto_keys::domain::errors::crypto_key_repository_error::CryptoKeyRepositoryError;
use crate::crypto_keys::domain::repositories::crypto_key_repository::CryptoKeyRepository;
use crate::crypto_keys::domain::value_objects::crypto_key_id::CryptoKeyId;
use crate::users::domain::value_objects::user_id::UserId;

/// An in-memory implementation of [`CryptoKeyRepository`] backed by a
/// [`HashMap`] protected by a [`Mutex`], keyed by the crypto key id.
///
/// Intended for use in tests and local development.
pub struct InMemoryCryptoKeyRepository {
    store: Mutex<HashMap<String, CryptoKey>>,
}

impl InMemoryCryptoKeyRepository {
    /// Creates a new empty `InMemoryCryptoKeyRepository`.
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryCryptoKeyRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CryptoKeyRepository for InMemoryCryptoKeyRepository {
    async fn save(&self, crypto_key: &CryptoKey) -> Result<(), CryptoKeyRepositoryError> {
        let mut store = self.store.lock().unwrap();
        let id = crypto_key.id().value().to_string();
        if store.contains_key(&id) {
            return Err(CryptoKeyRepositoryError::AlreadyExists);
        }
        store.insert(id, crypto_key.clone());
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &CryptoKeyId,
    ) -> Result<Option<CryptoKey>, CryptoKeyRepositoryError> {
        let store = self.store.lock().unwrap();
        Ok(store.get(id.value()).cloned())
    }

    async fn find_by_user_id(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<CryptoKey>, CryptoKeyRepositoryError> {
        let store = self.store.lock().unwrap();
        let mut keys: Vec<CryptoKey> = store
            .values()
            .filter(|key| key.user_id() == user_id)
            .cloned()
            .collect();
        // HashMap iteration order is arbitrary; sort so listings are stable.
        keys.sort_by(|a, b| {
            a.timestamps()
                .created_at()
                .cmp(&b.timestamps().created_at())
                .then_with(|| a.id().value().cmp(b.id().value()))
        });
        Ok(keys)
    }

    async fn update(&self, crypto_key: &CryptoKey) -> Result<(), CryptoKeyRepositoryError> {
        let mut store = self.store.lock().unwrap();
        let id = crypto_key.id().value().to_string();
        if !store.contains_key(&id) {
            return Err(CryptoKeyRepositoryError::NotFound);
        }
        store.insert(id, crypto_key.clone());
        Ok(())
    }

    async fn delete(&self, id: &CryptoKeyId) -> Result<(), CryptoKeyRepositoryError> {
        let mut store = self.store.lock().unwrap();
        if store.remove(id.value()).is_none() {
            return Err(CryptoKeyRepositoryError::NotFound);
        }
        Ok(())
    }
}
