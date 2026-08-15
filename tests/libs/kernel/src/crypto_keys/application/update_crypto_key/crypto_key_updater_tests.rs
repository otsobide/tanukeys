use std::sync::Arc;

use kernel::crypto_keys::application::update_crypto_key::crypto_key_updater::CryptoKeyUpdater;
use kernel::crypto_keys::domain::errors::crypto_key_repository_error::CryptoKeyRepositoryError;
use kernel::crypto_keys::domain::events::crypto_key_updated_event::CryptoKeyUpdatedEvent;
use kernel::crypto_keys::domain::repositories::crypto_key_repository::CryptoKeyRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::crypto_keys::domain::entities::mothers::crypto_key_mother::CryptoKeyMother;
use crate::src::crypto_keys::domain::value_objects::mothers::crypto_key_algorithm_mother::CryptoKeyAlgorithmMother;
use crate::src::crypto_keys::domain::value_objects::mothers::crypto_key_id_mother::CryptoKeyIdMother;
use crate::src::crypto_keys::domain::value_objects::mothers::crypto_key_name_mother::CryptoKeyNameMother;
use crate::src::crypto_keys::domain::value_objects::mothers::crypto_key_payload_mother::CryptoKeyPayloadMother;
use crate::src::crypto_keys::domain::value_objects::mothers::crypto_key_protocol_mother::CryptoKeyProtocolMother;
use crate::src::mocks::crypto_key_repository_mock::CryptoKeyRepositoryMock;
use crate::src::mocks::event_bus_mock::EventBusMock;

fn make_updater(repo: Arc<CryptoKeyRepositoryMock>, bus: Arc<EventBusMock>) -> CryptoKeyUpdater {
    let repo: Arc<dyn CryptoKeyRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    CryptoKeyUpdater::new(repo, bus)
}

#[tokio::test]
async fn it_calls_update_on_the_repository() {
    let crypto_key = CryptoKeyMother::random();
    let repo = Arc::new(CryptoKeyRepositoryMock::that_returns_crypto_key(crypto_key));
    let bus = Arc::new(EventBusMock::new());
    let updater = make_updater(repo.clone(), bus.clone());

    updater
        .execute(
            CryptoKeyIdMother::random(),
            CryptoKeyNameMother::random(),
            CryptoKeyProtocolMother::random(),
            CryptoKeyAlgorithmMother::random(),
            CryptoKeyPayloadMother::random(),
        )
        .await
        .unwrap();

    assert_eq!(repo.updated_keys().len(), 1);
}

#[tokio::test]
async fn it_publishes_an_updated_event() {
    let crypto_key = CryptoKeyMother::random();
    let repo = Arc::new(CryptoKeyRepositoryMock::that_returns_crypto_key(crypto_key));
    let bus = Arc::new(EventBusMock::new());
    let updater = make_updater(repo.clone(), bus.clone());

    updater
        .execute(
            CryptoKeyIdMother::random(),
            CryptoKeyNameMother::random(),
            CryptoKeyProtocolMother::random(),
            CryptoKeyAlgorithmMother::random(),
            CryptoKeyPayloadMother::random(),
        )
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![CryptoKeyUpdatedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_preserves_the_owner_and_the_creation_instant() {
    let previous = CryptoKeyMother::random();
    let expected_owner = previous.user_id().clone();
    let expected_created_at = previous.timestamps().created_at();

    let repo = Arc::new(CryptoKeyRepositoryMock::that_returns_crypto_key(previous));
    let bus = Arc::new(EventBusMock::new());
    let updater = make_updater(repo.clone(), bus.clone());

    updater
        .execute(
            CryptoKeyIdMother::random(),
            CryptoKeyNameMother::random(),
            CryptoKeyProtocolMother::random(),
            CryptoKeyAlgorithmMother::random(),
            CryptoKeyPayloadMother::random(),
        )
        .await
        .unwrap();

    let updated = repo.updated_keys();
    assert_eq!(updated.len(), 1);
    assert_eq!(updated[0].user_id(), &expected_owner);
    assert_eq!(updated[0].timestamps().created_at(), expected_created_at);
    assert!(updated[0].timestamps().updated_at() >= expected_created_at);
}

#[tokio::test]
async fn it_returns_not_found_when_crypto_key_does_not_exist() {
    let repo = Arc::new(CryptoKeyRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let updater = make_updater(repo.clone(), bus.clone());

    let result = updater
        .execute(
            CryptoKeyIdMother::random(),
            CryptoKeyNameMother::random(),
            CryptoKeyProtocolMother::random(),
            CryptoKeyAlgorithmMother::random(),
            CryptoKeyPayloadMother::random(),
        )
        .await;

    assert!(matches!(result, Err(CryptoKeyRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_does_not_publish_event_when_update_fails() {
    let crypto_key = CryptoKeyMother::random();
    let repo =
        Arc::new(CryptoKeyRepositoryMock::that_returns_crypto_key_but_update_fails(crypto_key));
    let bus = Arc::new(EventBusMock::new());
    let updater = make_updater(repo.clone(), bus.clone());

    let _ = updater
        .execute(
            CryptoKeyIdMother::random(),
            CryptoKeyNameMother::random(),
            CryptoKeyProtocolMother::random(),
            CryptoKeyAlgorithmMother::random(),
            CryptoKeyPayloadMother::random(),
        )
        .await;

    assert!(bus.published_event_names().is_empty());
}
