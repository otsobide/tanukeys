use kernel::users::domain::value_objects::user_id::UserId;

pub struct UserIdMother;

#[allow(dead_code)]
impl UserIdMother {
    pub fn create(value: impl Into<String>) -> UserId {
        UserId::new(value).unwrap()
    }

    pub fn random() -> UserId {
        UserId::generate()
    }
}
