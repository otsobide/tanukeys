use std::sync::Arc;

use kernel::users::application::create_user::user_creator::UserCreator;
use kernel::users::domain::errors::user_repository_error::UserRepositoryError;
use kernel::users::domain::events::user_created_event::UserCreatedEvent;
use kernel::users::domain::repositories::user_repository::UserRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::user_repository_mock::UserRepositoryMock;
use crate::src::users::domain::value_objects::mothers::user_description_mother::UserDescriptionMother;
use crate::src::users::domain::value_objects::mothers::user_id_mother::UserIdMother;
use crate::src::users::domain::value_objects::mothers::user_name_mother::UserNameMother;

fn make_creator(repo: Arc<UserRepositoryMock>, bus: Arc<EventBusMock>) -> UserCreator {
    let repo: Arc<dyn UserRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    UserCreator::new(repo, bus)
}

#[tokio::test]
async fn it_saves_the_user() {
    let id = UserIdMother::random();
    let expected_id = id.value().to_string();

    let repo = Arc::new(UserRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator
        .execute(
            id,
            UserNameMother::random(),
            UserDescriptionMother::random(),
        )
        .await
        .unwrap();

    assert_eq!(repo.saved_ids(), vec![expected_id]);
}

#[tokio::test]
async fn it_publishes_a_created_event() {
    let repo = Arc::new(UserRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator
        .execute(
            UserIdMother::random(),
            UserNameMother::random(),
            UserDescriptionMother::random(),
        )
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![UserCreatedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_ok() {
    let repo = Arc::new(UserRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator
        .execute(
            UserIdMother::random(),
            UserNameMother::random(),
            UserDescriptionMother::random(),
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn it_returns_already_exists_error_when_user_already_exists() {
    let repo = Arc::new(UserRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator
        .execute(
            UserIdMother::random(),
            UserNameMother::random(),
            UserDescriptionMother::random(),
        )
        .await;

    assert!(matches!(result, Err(UserRepositoryError::AlreadyExists)));
}

#[tokio::test]
async fn it_does_not_publish_event_when_save_fails() {
    let repo = Arc::new(UserRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let _ = creator
        .execute(
            UserIdMother::random(),
            UserNameMother::random(),
            UserDescriptionMother::random(),
        )
        .await;

    assert!(bus.published_event_names().is_empty());
}
