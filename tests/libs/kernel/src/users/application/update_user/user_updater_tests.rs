use std::sync::Arc;

use kernel::users::application::update_user::user_updater::UserUpdater;
use kernel::users::domain::errors::user_repository_error::UserRepositoryError;
use kernel::users::domain::events::user_updated_event::UserUpdatedEvent;
use kernel::users::domain::repositories::user_repository::UserRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::user_repository_mock::UserRepositoryMock;
use crate::src::users::domain::entities::mothers::user_mother::UserMother;
use crate::src::users::domain::value_objects::mothers::user_description_mother::UserDescriptionMother;
use crate::src::users::domain::value_objects::mothers::user_id_mother::UserIdMother;
use crate::src::users::domain::value_objects::mothers::user_name_mother::UserNameMother;

fn make_updater(repo: Arc<UserRepositoryMock>, bus: Arc<EventBusMock>) -> UserUpdater {
    let repo: Arc<dyn UserRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    UserUpdater::new(repo, bus)
}

#[tokio::test]
async fn it_calls_update_on_the_repository() {
    let user = UserMother::random();
    let repo = Arc::new(UserRepositoryMock::that_returns_user(user));
    let bus = Arc::new(EventBusMock::new());
    let updater = make_updater(repo.clone(), bus.clone());

    updater
        .execute(
            UserIdMother::random(),
            UserNameMother::random(),
            UserDescriptionMother::random(),
        )
        .await
        .unwrap();

    assert_eq!(repo.update_call_count(), 1);
}

#[tokio::test]
async fn it_publishes_an_updated_event() {
    let user = UserMother::random();
    let repo = Arc::new(UserRepositoryMock::that_returns_user(user));
    let bus = Arc::new(EventBusMock::new());
    let updater = make_updater(repo.clone(), bus.clone());

    updater
        .execute(
            UserIdMother::random(),
            UserNameMother::random(),
            UserDescriptionMother::random(),
        )
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![UserUpdatedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_not_found_when_user_does_not_exist() {
    let repo = Arc::new(UserRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let updater = make_updater(repo.clone(), bus.clone());

    let result = updater
        .execute(
            UserIdMother::random(),
            UserNameMother::random(),
            UserDescriptionMother::random(),
        )
        .await;

    assert!(matches!(result, Err(UserRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_does_not_publish_event_when_update_fails() {
    let user = UserMother::random();
    let repo = Arc::new(UserRepositoryMock::that_returns_user_but_update_fails(user));
    let bus = Arc::new(EventBusMock::new());
    let updater = make_updater(repo.clone(), bus.clone());

    let _ = updater
        .execute(
            UserIdMother::random(),
            UserNameMother::random(),
            UserDescriptionMother::random(),
        )
        .await;

    assert!(bus.published_event_names().is_empty());
}
