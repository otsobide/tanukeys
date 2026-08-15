use kernel::users::domain::value_objects::user_name::UserName;
use uuid::Uuid;

pub struct UserNameMother;

#[allow(dead_code)]
impl UserNameMother {
    pub fn create(value: impl Into<String>) -> UserName {
        UserName::new(value).unwrap()
    }

    pub fn random() -> UserName {
        UserName::new(format!("user-{}", Uuid::new_v4())).unwrap()
    }
}
