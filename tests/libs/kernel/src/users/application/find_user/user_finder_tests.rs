use std::sync::Arc;

use kernel::users::application::find_user::user_finder::UserFinder;
use kernel::users::domain::errors::user_repository_error::UserRepositoryError;
use kernel::users::domain::repositories::user_repository::UserRepository;

use crate::src::mocks::user_repository_mock::UserRepositoryMock;
use crate::src::users::domain::entities::mothers::user_mother::UserMother;
use crate::src::users::domain::value_objects::mothers::user_id_mother::UserIdMother;

fn make_finder(repo: Arc<UserRepositoryMock>) -> UserFinder {
    let repo: Arc<dyn UserRepository> = repo;
    UserFinder::new(repo)
}

#[tokio::test]
async fn it_returns_not_found_when_user_does_not_exist() {
    let repo = Arc::new(UserRepositoryMock::that_finds_nothing());
    let finder = make_finder(repo);

    let result = finder.execute(UserIdMother::random()).await;

    assert!(matches!(result, Err(UserRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_returns_the_user_when_it_exists() {
    let id = "6f772c15-e0ea-4e0d-a01c-9c8f8c86d6ee";
    let user = UserMother::create(id, "tanuki", Some("a tanuki".to_string()));
    let repo = Arc::new(UserRepositoryMock::that_returns_user(user));
    let finder = make_finder(repo);

    let result = finder.execute(UserIdMother::create(id)).await.unwrap();

    assert_eq!(result.id().value(), id);
    assert_eq!(result.name().value(), "tanuki");
    assert_eq!(result.description().value(), Some("a tanuki"));
}

#[tokio::test]
async fn it_returns_error_on_storage_failure() {
    let repo = Arc::new(UserRepositoryMock::that_fails_on_find(
        "storage error".to_string(),
    ));
    let finder = make_finder(repo);

    let result = finder.execute(UserIdMother::random()).await;

    assert!(matches!(result, Err(UserRepositoryError::Unexpected(_))));
}
