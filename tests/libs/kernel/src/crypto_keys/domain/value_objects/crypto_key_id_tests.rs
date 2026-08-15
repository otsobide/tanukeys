use kernel::crypto_keys::domain::value_objects::crypto_key_id::CryptoKeyId;

use crate::src::crypto_keys::domain::value_objects::mothers::crypto_key_id_mother::CryptoKeyIdMother;

#[test]
fn it_creates_a_random_valid_id() {
    let id = CryptoKeyIdMother::random();
    assert!(!id.value().is_empty());
}

#[test]
fn it_accepts_a_canonical_uuid_v4() {
    let result = CryptoKeyId::new("6f772c15-e0ea-4e0d-a01c-9c8f8c86d6ee");
    assert!(result.is_ok());
}

#[test]
fn it_normalizes_uppercase_input_to_lowercase() {
    let id = CryptoKeyId::new("6F772C15-E0EA-4E0D-A01C-9C8F8C86D6EE").unwrap();
    assert_eq!(id.value(), "6f772c15-e0ea-4e0d-a01c-9c8f8c86d6ee");
}

#[test]
fn it_rejects_an_empty_id() {
    assert!(CryptoKeyId::new("").is_err());
}

#[test]
fn it_rejects_a_non_v4_uuid() {
    // Version 1 (time-based) UUID.
    assert!(CryptoKeyId::new("550e8400-e29b-11d4-a716-446655440000").is_err());
}

#[test]
fn it_rejects_a_malformed_uuid() {
    assert!(CryptoKeyId::new("6f772c15-e0ea-4e0d-a01c").is_err());
}

#[test]
fn two_ids_from_the_same_value_are_equal() {
    let raw = "6f772c15-e0ea-4e0d-a01c-9c8f8c86d6ee";
    assert_eq!(
        CryptoKeyId::new(raw).unwrap(),
        CryptoKeyId::new(raw).unwrap()
    );
}
