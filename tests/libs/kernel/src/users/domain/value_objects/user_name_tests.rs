use kernel::users::domain::value_objects::user_name::UserName;

use crate::src::users::domain::value_objects::mothers::user_name_mother::UserNameMother;

#[test]
fn it_creates_a_random_valid_name() {
    let name = UserNameMother::random();
    assert!(!name.value().is_empty());
}

#[test]
fn it_accepts_a_simple_handle() {
    assert!(UserName::new("tanuki_box").is_ok());
}

#[test]
fn it_accepts_digits_dots_and_hyphens() {
    assert!(UserName::new("tanuki.box-42").is_ok());
}

#[test]
fn it_accepts_a_name_of_exactly_fifty_characters() {
    assert!(UserName::new("a".repeat(50)).is_ok());
}

#[test]
fn it_rejects_an_empty_name() {
    assert!(UserName::new("").is_err());
}

#[test]
fn it_rejects_a_name_longer_than_fifty_characters() {
    assert!(UserName::new("a".repeat(51)).is_err());
}

#[test]
fn it_rejects_uppercase_characters() {
    assert!(UserName::new("TanukiBox").is_err());
}

#[test]
fn it_rejects_whitespace() {
    assert!(UserName::new("tanuki box").is_err());
}

#[test]
fn it_rejects_a_name_that_is_only_a_space() {
    assert!(UserName::new(" ").is_err());
}

#[test]
fn it_rejects_disallowed_characters() {
    assert!(UserName::new("tanuki;box").is_err());
}
