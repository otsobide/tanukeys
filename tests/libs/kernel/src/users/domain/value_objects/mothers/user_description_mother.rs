use kernel::users::domain::value_objects::user_description::UserDescription;
use uuid::Uuid;

pub struct UserDescriptionMother;

#[allow(dead_code)]
impl UserDescriptionMother {
    pub fn create(value: Option<String>) -> UserDescription {
        UserDescription::new(value).unwrap()
    }

    pub fn random() -> UserDescription {
        UserDescription::new(Some(format!("description {}", Uuid::new_v4()))).unwrap()
    }

    pub fn none() -> UserDescription {
        UserDescription::none()
    }
}
