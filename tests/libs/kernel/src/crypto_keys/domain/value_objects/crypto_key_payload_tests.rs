use kernel::crypto_keys::domain::value_objects::crypto_key_payload::CryptoKeyPayload;

use crate::src::crypto_keys::domain::value_objects::mothers::crypto_key_payload_mother::CryptoKeyPayloadMother;

#[test]
fn it_creates_a_random_valid_payload() {
    let payload = CryptoKeyPayloadMother::random();
    assert!(!payload.value().is_empty());
}

#[test]
fn it_accepts_arbitrary_binary_content() {
    assert!(CryptoKeyPayload::new(vec![0u8, 255, 10, 13, 128]).is_ok());
}

#[test]
fn it_accepts_a_single_byte() {
    assert!(CryptoKeyPayload::new(vec![42u8]).is_ok());
}

#[test]
fn it_accepts_a_payload_at_the_maximum_size() {
    assert!(CryptoKeyPayload::new(vec![7u8; 64 * 1024]).is_ok());
}

#[test]
fn it_rejects_a_payload_over_the_maximum_size() {
    assert!(CryptoKeyPayload::new(vec![7u8; 64 * 1024 + 1]).is_err());
}

#[test]
fn it_rejects_an_empty_payload() {
    assert!(CryptoKeyPayload::new(Vec::new()).is_err());
}

#[test]
fn it_preserves_the_exact_bytes() {
    let bytes = vec![1u8, 2, 3, 0, 255];
    let payload = CryptoKeyPayload::new(bytes.clone()).unwrap();
    assert_eq!(payload.value(), bytes.as_slice());
}

#[test]
fn two_payloads_from_the_same_bytes_are_equal() {
    let bytes = vec![9u8, 8, 7];
    assert_eq!(
        CryptoKeyPayload::new(bytes.clone()).unwrap(),
        CryptoKeyPayload::new(bytes).unwrap()
    );
}
