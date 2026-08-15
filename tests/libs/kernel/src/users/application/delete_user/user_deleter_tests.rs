use std::sync::Arc;

use kernel::users::application::delete_user::user_deleter::UserDeleter;
use kernel::users::domain::errors::user_repository_error::UserRepositoryError;
use kernel::users::domain::events::user_deleted_event::UserDeletedEvent;
use kernel::users::domain::repositories::user_repository::UserRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::user_repository_mock::UserRepositoryMock;
use crate::src::users::domain::entities::mothers::user_mother::UserMother;
use crate::src::users::domain::value_objects::mothers::user_id_mother::UserIdMother;

fn make_deleter(repo: Arc<UserRepositoryMock>, bus: Arc<EventBusMock>) -> UserDeleter {
    let repo: Arc<dyn UserRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    UserDeleter::new(repo, bus)
}

#[tokio::test]
async fn it_calls_delete_on_the_repository() {
    let user = UserMother::random();
    let repo = Arc::new(UserRepositoryMock::that_returns_user(user));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(UserIdMother::random()).await.unwrap();

    assert_eq!(repo.delete_call_count(), 1);
}

#[tokio::test]
async fn it_publishes_a_deleted_event() {
    let user = UserMother::random();
    let repo = Arc::new(UserRepositoryMock::that_returns_user(user));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(UserIdMother::random()).await.unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![UserDeletedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_not_found_when_user_does_not_exist() {
    let repo = Arc::new(UserRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(UserIdMother::random()).await;

    assert!(matches!(result, Err(UserRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_does_not_publish_event_when_user_not_found() {
    let repo = Arc::new(UserRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let _ = deleter.execute(UserIdMother::random()).await;

    assert!(bus.published_event_names().is_empty());
}
