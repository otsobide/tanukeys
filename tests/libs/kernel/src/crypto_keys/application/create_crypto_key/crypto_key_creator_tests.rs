use std::sync::Arc;

use kernel::crypto_keys::application::create_crypto_key::crypto_key_creator::CryptoKeyCreator;
use kernel::crypto_keys::domain::errors::crypto_key_repository_error::CryptoKeyRepositoryError;
use kernel::crypto_keys::domain::events::crypto_key_created_event::CryptoKeyCreatedEvent;
use kernel::crypto_keys::domain::repositories::crypto_key_repository::CryptoKeyRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::crypto_keys::domain::value_objects::mothers::crypto_key_algorithm_mother::CryptoKeyAlgorithmMother;
use crate::src::crypto_keys::domain::value_objects::mothers::crypto_key_id_mother::CryptoKeyIdMother;
use crate::src::crypto_keys::domain::value_objects::mothers::crypto_key_name_mother::CryptoKeyNameMother;
use crate::src::crypto_keys::domain::value_objects::mothers::crypto_key_payload_mother::CryptoKeyPayloadMother;
use crate::src::crypto_keys::domain::value_objects::mothers::crypto_key_protocol_mother::CryptoKeyProtocolMother;
use crate::src::mocks::crypto_key_repository_mock::CryptoKeyRepositoryMock;
use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::users::domain::value_objects::mothers::user_id_mother::UserIdMother;

fn make_creator(repo: Arc<CryptoKeyRepositoryMock>, bus: Arc<EventBusMock>) -> CryptoKeyCreator {
    let repo: Arc<dyn CryptoKeyRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    CryptoKeyCreator::new(repo, bus)
}

#[tokio::test]
async fn it_saves_the_crypto_key() {
    let id = CryptoKeyIdMother::random();
    let expected_id = id.value().to_string();

    let repo = Arc::new(CryptoKeyRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator
        .execute(
            id,
            UserIdMother::random(),
            CryptoKeyNameMother::random(),
            CryptoKeyProtocolMother::random(),
            CryptoKeyAlgorithmMother::random(),
            CryptoKeyPayloadMother::random(),
        )
        .await
        .unwrap();

    assert_eq!(repo.saved_ids(), vec![expected_id]);
}

#[tokio::test]
async fn it_publishes_a_created_event() {
    let repo = Arc::new(CryptoKeyRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator
        .execute(
            CryptoKeyIdMother::random(),
            UserIdMother::random(),
            CryptoKeyNameMother::random(),
            CryptoKeyProtocolMother::random(),
            CryptoKeyAlgorithmMother::random(),
            CryptoKeyPayloadMother::random(),
        )
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![CryptoKeyCreatedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_ok() {
    let repo = Arc::new(CryptoKeyRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator
        .execute(
            CryptoKeyIdMother::random(),
            UserIdMother::random(),
            CryptoKeyNameMother::random(),
            CryptoKeyProtocolMother::random(),
            CryptoKeyAlgorithmMother::random(),
            CryptoKeyPayloadMother::random(),
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn it_returns_already_exists_error_when_crypto_key_already_exists() {
    let repo = Arc::new(CryptoKeyRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator
        .execute(
            CryptoKeyIdMother::random(),
            UserIdMother::random(),
            CryptoKeyNameMother::random(),
            CryptoKeyProtocolMother::random(),
            CryptoKeyAlgorithmMother::random(),
            CryptoKeyPayloadMother::random(),
        )
        .await;

    assert!(matches!(
        result,
        Err(CryptoKeyRepositoryError::AlreadyExists)
    ));
}

#[tokio::test]
async fn it_does_not_publish_event_when_save_fails() {
    let repo = Arc::new(CryptoKeyRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let _ = creator
        .execute(
            CryptoKeyIdMother::random(),
            UserIdMother::random(),
            CryptoKeyNameMother::random(),
            CryptoKeyProtocolMother::random(),
            CryptoKeyAlgorithmMother::random(),
            CryptoKeyPayloadMother::random(),
        )
        .await;

    assert!(bus.published_event_names().is_empty());
}
