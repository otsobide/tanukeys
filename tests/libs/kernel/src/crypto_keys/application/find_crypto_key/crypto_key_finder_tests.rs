use std::sync::Arc;

use kernel::crypto_keys::application::find_crypto_key::crypto_key_finder::CryptoKeyFinder;
use kernel::crypto_keys::domain::errors::crypto_key_repository_error::CryptoKeyRepositoryError;
use kernel::crypto_keys::domain::repositories::crypto_key_repository::CryptoKeyRepository;

use crate::src::crypto_keys::domain::entities::mothers::crypto_key_mother::CryptoKeyMother;
use crate::src::crypto_keys::domain::value_objects::mothers::crypto_key_id_mother::CryptoKeyIdMother;
use crate::src::mocks::crypto_key_repository_mock::CryptoKeyRepositoryMock;

fn make_finder(repo: Arc<CryptoKeyRepositoryMock>) -> CryptoKeyFinder {
    let repo: Arc<dyn CryptoKeyRepository> = repo;
    CryptoKeyFinder::new(repo)
}

#[tokio::test]
async fn it_returns_the_crypto_key_when_found() {
    let crypto_key = CryptoKeyMother::random();
    let expected_id = crypto_key.id().clone();

    let repo = Arc::new(CryptoKeyRepositoryMock::that_returns_crypto_key(crypto_key));
    let finder = make_finder(repo);

    let found = finder.execute(expected_id.clone()).await.unwrap();

    assert_eq!(found.id(), &expected_id);
}

#[tokio::test]
async fn it_returns_not_found_when_crypto_key_does_not_exist() {
    let repo = Arc::new(CryptoKeyRepositoryMock::that_finds_nothing());
    let finder = make_finder(repo);

    let result = finder.execute(CryptoKeyIdMother::random()).await;

    assert!(matches!(result, Err(CryptoKeyRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_returns_unexpected_when_storage_fails() {
    let repo = Arc::new(CryptoKeyRepositoryMock::that_fails_on_find("boom"));
    let finder = make_finder(repo);

    let result = finder.execute(CryptoKeyIdMother::random()).await;

    assert!(matches!(
        result,
        Err(CryptoKeyRepositoryError::Unexpected(_))
    ));
}
