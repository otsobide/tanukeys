use std::sync::Mutex;

use async_trait::async_trait;

use kernel::users::domain::entities::user::User;
use kernel::users::domain::errors::user_repository_error::UserRepositoryError;
use kernel::users::domain::repositories::user_repository::UserRepository;
use kernel::users::domain::value_objects::user_id::UserId;

pub enum SaveBehavior {
    Succeeds,
    FailsWithAlreadyExists,
}

#[allow(dead_code)]
pub enum FindByIdBehavior {
    ReturnsNone,
    ReturnsUser(Mutex<Option<User>>),
    FailsWithUnexpected(String),
}

#[allow(dead_code)]
pub enum UpdateBehavior {
    Succeeds,
    FailsWithNotFound,
    FailsWithUnexpected(String),
}

pub struct UserRepositoryMock {
    save_behavior: SaveBehavior,
    find_by_id_behavior: FindByIdBehavior,
    update_behavior: UpdateBehavior,
    saved_ids: Mutex<Vec<String>>,
    update_call_count: Mutex<u32>,
    delete_call_count: Mutex<u32>,
}

#[allow(dead_code)]
impl UserRepositoryMock {
    pub fn that_succeeds() -> Self {
        Self {
            save_behavior: SaveBehavior::Succeeds,
            find_by_id_behavior: FindByIdBehavior::ReturnsNone,
            update_behavior: UpdateBehavior::Succeeds,
            saved_ids: Mutex::new(vec![]),
            update_call_count: Mutex::new(0),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn that_fails_with_already_exists() -> Self {
        Self {
            save_behavior: SaveBehavior::FailsWithAlreadyExists,
            find_by_id_behavior: FindByIdBehavior::ReturnsNone,
            update_behavior: UpdateBehavior::Succeeds,
            saved_ids: Mutex::new(vec![]),
            update_call_count: Mutex::new(0),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn that_returns_user(user: User) -> Self {
        Self {
            save_behavior: SaveBehavior::Succeeds,
            find_by_id_behavior: FindByIdBehavior::ReturnsUser(Mutex::new(Some(user))),
            update_behavior: UpdateBehavior::Succeeds,
            saved_ids: Mutex::new(vec![]),
            update_call_count: Mutex::new(0),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn that_returns_user_but_update_fails(user: User) -> Self {
        Self {
            save_behavior: SaveBehavior::Succeeds,
            find_by_id_behavior: FindByIdBehavior::ReturnsUser(Mutex::new(Some(user))),
            update_behavior: UpdateBehavior::FailsWithNotFound,
            saved_ids: Mutex::new(vec![]),
            update_call_count: Mutex::new(0),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn that_finds_nothing() -> Self {
        Self {
            save_behavior: SaveBehavior::Succeeds,
            find_by_id_behavior: FindByIdBehavior::ReturnsNone,
            update_behavior: UpdateBehavior::Succeeds,
            saved_ids: Mutex::new(vec![]),
            update_call_count: Mutex::new(0),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn that_fails_on_find(message: String) -> Self {
        Self {
            save_behavior: SaveBehavior::Succeeds,
            find_by_id_behavior: FindByIdBehavior::FailsWithUnexpected(message),
            update_behavior: UpdateBehavior::Succeeds,
            saved_ids: Mutex::new(vec![]),
            update_call_count: Mutex::new(0),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn saved_ids(&self) -> Vec<String> {
        self.saved_ids.lock().unwrap().clone()
    }

    pub fn update_call_count(&self) -> u32 {
        *self.update_call_count.lock().unwrap()
    }

    pub fn delete_call_count(&self) -> u32 {
        *self.delete_call_count.lock().unwrap()
    }
}

#[async_trait]
impl UserRepository for UserRepositoryMock {
    async fn save(&self, user: &User) -> Result<(), UserRepositoryError> {
        match &self.save_behavior {
            SaveBehavior::FailsWithAlreadyExists => Err(UserRepositoryError::AlreadyExists),
            SaveBehavior::Succeeds => {
                self.saved_ids
                    .lock()
                    .unwrap()
                    .push(user.id().value().to_string());
                Ok(())
            }
        }
    }

    async fn find_by_id(&self, _id: &UserId) -> Result<Option<User>, UserRepositoryError> {
        match &self.find_by_id_behavior {
            FindByIdBehavior::ReturnsNone => Ok(None),
            FindByIdBehavior::ReturnsUser(cell) => Ok(cell.lock().unwrap().take()),
            FindByIdBehavior::FailsWithUnexpected(msg) => {
                Err(UserRepositoryError::Unexpected(msg.clone()))
            }
        }
    }

    async fn update(&self, _user: &User) -> Result<(), UserRepositoryError> {
        *self.update_call_count.lock().unwrap() += 1;
        match &self.update_behavior {
            UpdateBehavior::Succeeds => Ok(()),
            UpdateBehavior::FailsWithNotFound => Err(UserRepositoryError::NotFound),
            UpdateBehavior::FailsWithUnexpected(msg) => {
                Err(UserRepositoryError::Unexpected(msg.clone()))
            }
        }
    }

    async fn delete(&self, _id: &UserId) -> Result<(), UserRepositoryError> {
        *self.delete_call_count.lock().unwrap() += 1;
        Ok(())
    }
}
