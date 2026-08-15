use std::sync::Arc;

use kernel::crypto_keys::application::find_user_crypto_keys::user_crypto_keys_finder::UserCryptoKeysFinder;
use kernel::crypto_keys::domain::errors::crypto_key_repository_error::CryptoKeyRepositoryError;
use kernel::crypto_keys::domain::repositories::crypto_key_repository::CryptoKeyRepository;

use crate::src::crypto_keys::domain::entities::mothers::crypto_key_mother::CryptoKeyMother;
use crate::src::mocks::crypto_key_repository_mock::CryptoKeyRepositoryMock;
use crate::src::users::domain::value_objects::mothers::user_id_mother::UserIdMother;

fn make_finder(repo: Arc<CryptoKeyRepositoryMock>) -> UserCryptoKeysFinder {
    let repo: Arc<dyn CryptoKeyRepository> = repo;
    UserCryptoKeysFinder::new(repo)
}

#[tokio::test]
async fn it_returns_the_keys_of_the_user() {
    let user_id = UserIdMother::random();
    let keys = vec![
        CryptoKeyMother::random_with_user(&user_id),
        CryptoKeyMother::random_with_user(&user_id),
    ];

    let repo = Arc::new(CryptoKeyRepositoryMock::that_returns_crypto_keys(keys));
    let finder = make_finder(repo);

    let found = finder.execute(user_id.clone()).await.unwrap();

    assert_eq!(found.len(), 2);
    assert!(found.iter().all(|key| key.user_id() == &user_id));
}

#[tokio::test]
async fn it_returns_an_empty_list_when_the_user_has_no_keys() {
    let repo = Arc::new(CryptoKeyRepositoryMock::that_succeeds());
    let finder = make_finder(repo);

    let found = finder.execute(UserIdMother::random()).await.unwrap();

    assert!(found.is_empty());
}

#[tokio::test]
async fn it_returns_unexpected_when_storage_fails() {
    let repo = Arc::new(CryptoKeyRepositoryMock::that_fails_on_find_by_user_id(
        "boom",
    ));
    let finder = make_finder(repo);

    let result = finder.execute(UserIdMother::random()).await;

    assert!(matches!(
        result,
        Err(CryptoKeyRepositoryError::Unexpected(_))
    ));
}
