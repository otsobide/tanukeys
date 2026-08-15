//! In-memory implementation of [`UserRepository`].

use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;

use crate::users::domain::entities::user::User;
use crate::users::domain::errors::user_repository_error::UserRepositoryError;
use crate::users::domain::repositories::user_repository::UserRepository;
use crate::users::domain::value_objects::user_id::UserId;

/// An in-memory implementation of [`UserRepository`] backed by a
/// [`HashMap`] protected by a [`Mutex`], keyed by the user id.
///
/// Intended for use in tests and local development.
pub struct InMemoryUserRepository {
    store: Mutex<HashMap<String, User>>,
}

impl InMemoryUserRepository {
    /// Creates a new empty `InMemoryUserRepository`.
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn save(&self, user: &User) -> Result<(), UserRepositoryError> {
        let mut store = self.store.lock().unwrap();
        let id = user.id().value().to_string();
        if store.contains_key(&id) {
            return Err(UserRepositoryError::AlreadyExists);
        }
        store.insert(id, user.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, UserRepositoryError> {
        let store = self.store.lock().unwrap();
        Ok(store.get(id.value()).cloned())
    }

    async fn update(&self, user: &User) -> Result<(), UserRepositoryError> {
        let mut store = self.store.lock().unwrap();
        let id = user.id().value().to_string();
        if !store.contains_key(&id) {
            return Err(UserRepositoryError::NotFound);
        }
        store.insert(id, user.clone());
        Ok(())
    }

    async fn delete(&self, id: &UserId) -> Result<(), UserRepositoryError> {
        let mut store = self.store.lock().unwrap();
        if store.remove(id.value()).is_none() {
            return Err(UserRepositoryError::NotFound);
        }
        Ok(())
    }
}
