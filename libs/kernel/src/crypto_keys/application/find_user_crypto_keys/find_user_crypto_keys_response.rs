//! Response for the find-user-crypto-keys use case.

use crate::crypto_keys::application::find_crypto_key::find_crypto_key_response::{
    CryptoKeyEntry, CryptoKeyErrorEntry,
};

/// Response envelope returned by [`FindUserCryptoKeysQueryHandler`].
///
/// On success, `crypto_keys` contains the (possibly empty) list and `error`
/// is `None`. On failure, `crypto_keys` is `None` and `error` contains the
/// structured error.
///
/// [`FindUserCryptoKeysQueryHandler`]: super::find_user_crypto_keys_query_handler::FindUserCryptoKeysQueryHandler
pub struct FindUserCryptoKeysResponse {
    pub crypto_keys: Option<Vec<CryptoKeyEntry>>,
    pub error: Option<CryptoKeyErrorEntry>,
}
