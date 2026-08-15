use kernel::users::domain::value_objects::user_description::UserDescription;

use crate::src::users::domain::value_objects::mothers::user_description_mother::UserDescriptionMother;

#[test]
fn it_creates_a_random_valid_description() {
    let description = UserDescriptionMother::random();
    assert!(description.value().is_some());
}

#[test]
fn it_accepts_an_absent_description() {
    let description = UserDescription::new(None).unwrap();
    assert!(description.value().is_none());
}

#[test]
fn it_accepts_an_empty_description() {
    assert!(UserDescription::new(Some(String::new())).is_ok());
}

#[test]
fn it_accepts_unicode_content() {
    assert!(UserDescription::new(Some("こんにちは 안녕하세요 你好 مرحبا".to_string())).is_ok());
}

#[test]
fn it_accepts_a_description_of_exactly_six_hundred_characters() {
    assert!(UserDescription::new(Some("a".repeat(600))).is_ok());
}

#[test]
fn it_rejects_a_description_longer_than_six_hundred_characters() {
    assert!(UserDescription::new(Some("a".repeat(601))).is_err());
}
