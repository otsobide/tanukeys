//! Response for the update-crypto-key use case.

use crate::crypto_keys::application::find_crypto_key::find_crypto_key_response::CryptoKeyErrorEntry;

/// Response envelope returned by [`UpdateCryptoKeyCommandHandler`].
///
/// On success, `error` is `None`. On failure, `error` contains the structured error.
///
/// [`UpdateCryptoKeyCommandHandler`]: super::update_crypto_key_command_handler::UpdateCryptoKeyCommandHandler
pub struct UpdateCryptoKeyResponse {
    pub error: Option<CryptoKeyErrorEntry>,
}
