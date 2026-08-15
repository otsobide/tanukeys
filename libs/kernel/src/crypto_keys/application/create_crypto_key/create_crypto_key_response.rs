//! Response for the create-crypto-key use case.

use crate::crypto_keys::application::find_crypto_key::find_crypto_key_response::CryptoKeyErrorEntry;

/// Response envelope returned by [`CreateCryptoKeyCommandHandler`].
///
/// On success, `error` is `None`. On failure, `error` contains the structured error.
///
/// [`CreateCryptoKeyCommandHandler`]: super::create_crypto_key_command_handler::CreateCryptoKeyCommandHandler
pub struct CreateCryptoKeyResponse {
    pub error: Option<CryptoKeyErrorEntry>,
}
