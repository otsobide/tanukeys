use std::time::{Duration, SystemTime};

use kernel::crypto_keys::domain::value_objects::crypto_key_timestamps::CryptoKeyTimestamps;

#[test]
fn it_creates_now_with_equal_instants() {
    let timestamps = CryptoKeyTimestamps::now();
    assert_eq!(timestamps.created_at(), timestamps.updated_at());
}

#[test]
fn it_accepts_updated_at_equal_to_created_at() {
    let instant = SystemTime::now();
    assert!(CryptoKeyTimestamps::new(instant, instant).is_ok());
}

#[test]
fn it_accepts_updated_at_after_created_at() {
    let created_at = SystemTime::now();
    let updated_at = created_at + Duration::from_secs(60);
    assert!(CryptoKeyTimestamps::new(created_at, updated_at).is_ok());
}

#[test]
fn it_rejects_updated_at_before_created_at() {
    let created_at = SystemTime::now();
    let updated_at = created_at - Duration::from_secs(60);
    assert!(CryptoKeyTimestamps::new(created_at, updated_at).is_err());
}

#[test]
fn touch_preserves_the_creation_instant() {
    let created_at = SystemTime::now() - Duration::from_secs(3600);
    let timestamps = CryptoKeyTimestamps::new(created_at, created_at).unwrap();

    let touched = timestamps.touch();

    assert_eq!(touched.created_at(), created_at);
}

#[test]
fn touch_moves_updated_at_forward() {
    let created_at = SystemTime::now() - Duration::from_secs(3600);
    let timestamps = CryptoKeyTimestamps::new(created_at, created_at).unwrap();

    let touched = timestamps.touch();

    assert!(touched.updated_at() > timestamps.updated_at());
}

#[test]
fn touch_never_breaks_the_invariant_even_with_a_future_creation_instant() {
    let created_at = SystemTime::now() + Duration::from_secs(3600);
    let timestamps = CryptoKeyTimestamps::new(created_at, created_at).unwrap();

    let touched = timestamps.touch();

    assert!(touched.updated_at() >= touched.created_at());
}
