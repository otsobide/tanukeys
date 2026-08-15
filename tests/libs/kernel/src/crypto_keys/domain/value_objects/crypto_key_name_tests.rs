use kernel::crypto_keys::domain::value_objects::crypto_key_name::CryptoKeyName;

use crate::src::crypto_keys::domain::value_objects::mothers::crypto_key_name_mother::CryptoKeyNameMother;

#[test]
fn it_creates_a_random_valid_name() {
    let name = CryptoKeyNameMother::random();
    assert!(!name.value().is_empty());
}

#[test]
fn it_accepts_a_simple_name() {
    assert!(CryptoKeyName::new("laptop ssh key").is_ok());
}

#[test]
fn it_accepts_unicode_content() {
    assert!(CryptoKeyName::new("clave del portátil 🔑").is_ok());
}

#[test]
fn it_accepts_a_name_at_the_maximum_length() {
    assert!(CryptoKeyName::new("a".repeat(100)).is_ok());
}

#[test]
fn it_rejects_a_name_over_the_maximum_length() {
    assert!(CryptoKeyName::new("a".repeat(101)).is_err());
}

#[test]
fn it_counts_length_in_characters_not_bytes() {
    // 100 two-byte characters: 200 bytes but exactly 100 characters.
    assert!(CryptoKeyName::new("ñ".repeat(100)).is_ok());
}

#[test]
fn it_rejects_an_empty_name() {
    assert!(CryptoKeyName::new("").is_err());
}

#[test]
fn it_rejects_leading_whitespace() {
    assert!(CryptoKeyName::new(" key").is_err());
}

#[test]
fn it_rejects_trailing_whitespace() {
    assert!(CryptoKeyName::new("key ").is_err());
}

#[test]
fn two_names_from_the_same_value_are_equal() {
    assert_eq!(
        CryptoKeyName::new("laptop ssh key").unwrap(),
        CryptoKeyName::new("laptop ssh key").unwrap()
    );
}
