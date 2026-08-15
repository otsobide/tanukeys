use kernel::users::domain::entities::user::User;

use crate::src::users::domain::value_objects::mothers::user_description_mother::UserDescriptionMother;
use crate::src::users::domain::value_objects::mothers::user_id_mother::UserIdMother;
use crate::src::users::domain::value_objects::mothers::user_name_mother::UserNameMother;

pub struct UserMother;

#[allow(dead_code)]
impl UserMother {
    pub fn random() -> User {
        User::new(
            UserIdMother::random(),
            UserNameMother::random(),
            UserDescriptionMother::random(),
        )
    }

    pub fn create(
        id: impl Into<String>,
        name: impl Into<String>,
        description: Option<String>,
    ) -> User {
        User::new(
            UserIdMother::create(id),
            UserNameMother::create(name),
            UserDescriptionMother::create(description),
        )
    }
}
