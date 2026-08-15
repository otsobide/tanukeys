use std::sync::Arc;

use kernel::crypto_keys::application::delete_crypto_key::crypto_key_deleter::CryptoKeyDeleter;
use kernel::crypto_keys::domain::errors::crypto_key_repository_error::CryptoKeyRepositoryError;
use kernel::crypto_keys::domain::events::crypto_key_deleted_event::CryptoKeyDeletedEvent;
use kernel::crypto_keys::domain::repositories::crypto_key_repository::CryptoKeyRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::crypto_keys::domain::entities::mothers::crypto_key_mother::CryptoKeyMother;
use crate::src::crypto_keys::domain::value_objects::mothers::crypto_key_id_mother::CryptoKeyIdMother;
use crate::src::mocks::crypto_key_repository_mock::CryptoKeyRepositoryMock;
use crate::src::mocks::event_bus_mock::EventBusMock;

fn make_deleter(repo: Arc<CryptoKeyRepositoryMock>, bus: Arc<EventBusMock>) -> CryptoKeyDeleter {
    let repo: Arc<dyn CryptoKeyRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    CryptoKeyDeleter::new(repo, bus)
}

#[tokio::test]
async fn it_calls_delete_on_the_repository() {
    let crypto_key = CryptoKeyMother::random();
    let repo = Arc::new(CryptoKeyRepositoryMock::that_returns_crypto_key(crypto_key));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(CryptoKeyIdMother::random()).await.unwrap();

    assert_eq!(repo.delete_call_count(), 1);
}

#[tokio::test]
async fn it_publishes_a_deleted_event() {
    let crypto_key = CryptoKeyMother::random();
    let repo = Arc::new(CryptoKeyRepositoryMock::that_returns_crypto_key(crypto_key));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(CryptoKeyIdMother::random()).await.unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![CryptoKeyDeletedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_not_found_when_crypto_key_does_not_exist() {
    let repo = Arc::new(CryptoKeyRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(CryptoKeyIdMother::random()).await;

    assert!(matches!(result, Err(CryptoKeyRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_does_not_publish_event_when_delete_fails() {
    let crypto_key = CryptoKeyMother::random();
    let repo =
        Arc::new(CryptoKeyRepositoryMock::that_returns_crypto_key_but_delete_fails(crypto_key));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let _ = deleter.execute(CryptoKeyIdMother::random()).await;

    assert!(bus.published_event_names().is_empty());
}
