//! Response for the delete-crypto-key use case.

use crate::crypto_keys::application::find_crypto_key::find_crypto_key_response::CryptoKeyErrorEntry;

/// Response envelope returned by [`DeleteCryptoKeyCommandHandler`].
///
/// On success, `error` is `None`. On failure, `error` contains the structured error.
///
/// [`DeleteCryptoKeyCommandHandler`]: super::delete_crypto_key_command_handler::DeleteCryptoKeyCommandHandler
pub struct DeleteCryptoKeyResponse {
    pub error: Option<CryptoKeyErrorEntry>,
}
