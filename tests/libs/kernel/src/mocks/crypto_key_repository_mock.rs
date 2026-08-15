use std::sync::Mutex;

use async_trait::async_trait;

use kernel::crypto_keys::domain::entities::crypto_key::CryptoKey;
use kernel::crypto_keys::domain::errors::crypto_key_repository_error::CryptoKeyRepositoryError;
use kernel::crypto_keys::domain::repositories::crypto_key_repository::CryptoKeyRepository;
use kernel::crypto_keys::domain::value_objects::crypto_key_id::CryptoKeyId;
use kernel::users::domain::value_objects::user_id::UserId;

pub enum SaveBehavior {
    Succeeds,
    FailsWithAlreadyExists,
}

#[allow(dead_code)]
pub enum FindByIdBehavior {
    ReturnsNone,
    ReturnsCryptoKey(Mutex<Option<CryptoKey>>),
    FailsWithUnexpected(String),
}

#[allow(dead_code)]
pub enum FindByUserIdBehavior {
    ReturnsCryptoKeys(Mutex<Vec<CryptoKey>>),
    FailsWithUnexpected(String),
}

#[allow(dead_code)]
pub enum UpdateBehavior {
    Succeeds,
    FailsWithUnexpected(String),
}

#[allow(dead_code)]
pub enum DeleteBehavior {
    Succeeds,
    FailsWithUnexpected(String),
}

pub struct CryptoKeyRepositoryMock {
    save_behavior: SaveBehavior,
    find_by_id_behavior: FindByIdBehavior,
    find_by_user_id_behavior: FindByUserIdBehavior,
    update_behavior: UpdateBehavior,
    delete_behavior: DeleteBehavior,
    saved_ids: Mutex<Vec<String>>,
    updated_keys: Mutex<Vec<CryptoKey>>,
    delete_call_count: Mutex<u32>,
}

#[allow(dead_code)]
impl CryptoKeyRepositoryMock {
    pub fn that_succeeds() -> Self {
        Self {
            save_behavior: SaveBehavior::Succeeds,
            find_by_id_behavior: FindByIdBehavior::ReturnsNone,
            find_by_user_id_behavior: FindByUserIdBehavior::ReturnsCryptoKeys(Mutex::new(vec![])),
            update_behavior: UpdateBehavior::Succeeds,
            delete_behavior: DeleteBehavior::Succeeds,
            saved_ids: Mutex::new(vec![]),
            updated_keys: Mutex::new(vec![]),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn that_fails_with_already_exists() -> Self {
        Self {
            save_behavior: SaveBehavior::FailsWithAlreadyExists,
            ..Self::that_succeeds()
        }
    }

    pub fn that_returns_crypto_key(crypto_key: CryptoKey) -> Self {
        Self {
            find_by_id_behavior: FindByIdBehavior::ReturnsCryptoKey(Mutex::new(Some(crypto_key))),
            ..Self::that_succeeds()
        }
    }

    pub fn that_returns_crypto_key_but_update_fails(crypto_key: CryptoKey) -> Self {
        Self {
            find_by_id_behavior: FindByIdBehavior::ReturnsCryptoKey(Mutex::new(Some(crypto_key))),
            update_behavior: UpdateBehavior::FailsWithUnexpected("mock error".to_string()),
            ..Self::that_succeeds()
        }
    }

    pub fn that_returns_crypto_key_but_delete_fails(crypto_key: CryptoKey) -> Self {
        Self {
            find_by_id_behavior: FindByIdBehavior::ReturnsCryptoKey(Mutex::new(Some(crypto_key))),
            delete_behavior: DeleteBehavior::FailsWithUnexpected("mock error".to_string()),
            ..Self::that_succeeds()
        }
    }

    pub fn that_finds_nothing() -> Self {
        Self::that_succeeds()
    }

    pub fn that_fails_on_find(message: impl Into<String>) -> Self {
        Self {
            find_by_id_behavior: FindByIdBehavior::FailsWithUnexpected(message.into()),
            ..Self::that_succeeds()
        }
    }

    pub fn that_returns_crypto_keys(crypto_keys: Vec<CryptoKey>) -> Self {
        Self {
            find_by_user_id_behavior: FindByUserIdBehavior::ReturnsCryptoKeys(Mutex::new(
                crypto_keys,
            )),
            ..Self::that_succeeds()
        }
    }

    pub fn that_fails_on_find_by_user_id(message: impl Into<String>) -> Self {
        Self {
            find_by_user_id_behavior: FindByUserIdBehavior::FailsWithUnexpected(message.into()),
            ..Self::that_succeeds()
        }
    }

    pub fn saved_ids(&self) -> Vec<String> {
        self.saved_ids.lock().unwrap().clone()
    }

    pub fn updated_keys(&self) -> Vec<CryptoKey> {
        self.updated_keys.lock().unwrap().clone()
    }

    pub fn delete_call_count(&self) -> u32 {
        *self.delete_call_count.lock().unwrap()
    }
}

#[async_trait]
impl CryptoKeyRepository for CryptoKeyRepositoryMock {
    async fn save(&self, crypto_key: &CryptoKey) -> Result<(), CryptoKeyRepositoryError> {
        match &self.save_behavior {
            SaveBehavior::FailsWithAlreadyExists => Err(CryptoKeyRepositoryError::AlreadyExists),
            SaveBehavior::Succeeds => {
                self.saved_ids
                    .lock()
                    .unwrap()
                    .push(crypto_key.id().value().to_string());
                Ok(())
            }
        }
    }

    async fn find_by_id(
        &self,
        _id: &CryptoKeyId,
    ) -> Result<Option<CryptoKey>, CryptoKeyRepositoryError> {
        match &self.find_by_id_behavior {
            FindByIdBehavior::ReturnsNone => Ok(None),
            FindByIdBehavior::ReturnsCryptoKey(cell) => Ok(cell.lock().unwrap().take()),
            FindByIdBehavior::FailsWithUnexpected(msg) => {
                Err(CryptoKeyRepositoryError::Unexpected(msg.clone()))
            }
        }
    }

    async fn find_by_user_id(
        &self,
        _user_id: &UserId,
    ) -> Result<Vec<CryptoKey>, CryptoKeyRepositoryError> {
        match &self.find_by_user_id_behavior {
            FindByUserIdBehavior::ReturnsCryptoKeys(cell) => Ok(cell.lock().unwrap().clone()),
            FindByUserIdBehavior::FailsWithUnexpected(msg) => {
                Err(CryptoKeyRepositoryError::Unexpected(msg.clone()))
            }
        }
    }

    async fn update(&self, crypto_key: &CryptoKey) -> Result<(), CryptoKeyRepositoryError> {
        self.updated_keys.lock().unwrap().push(crypto_key.clone());
        match &self.update_behavior {
            UpdateBehavior::Succeeds => Ok(()),
            UpdateBehavior::FailsWithUnexpected(msg) => {
                Err(CryptoKeyRepositoryError::Unexpected(msg.clone()))
            }
        }
    }

    async fn delete(&self, _id: &CryptoKeyId) -> Result<(), CryptoKeyRepositoryError> {
        *self.delete_call_count.lock().unwrap() += 1;
        match &self.delete_behavior {
            DeleteBehavior::Succeeds => Ok(()),
            DeleteBehavior::FailsWithUnexpected(msg) => {
                Err(CryptoKeyRepositoryError::Unexpected(msg.clone()))
            }
        }
    }
}
