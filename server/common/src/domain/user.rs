use super::{date_of_birth::DateOfBirth, email::ValidEmail, name::Name, photo_url::PhotoUrl};
use crate::dto::{gender::GenderType, user::UserInput};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct User {
    pub first_name: Name,
    pub middle_name: Name,
    pub last_name: Name,
    pub dob: DateOfBirth,
    pub email: ValidEmail,
    pub gender: GenderType,
    pub photo_url: PhotoUrl,
}

impl User {
    pub fn parse(a: &UserInput) -> Result<User> {
        let res = User {
            first_name: Name::parse(&a.first_name, "First")?,
            middle_name: Name::parse(&a.middle_name, "Middle")?,
            last_name: Name::parse(&a.last_name, "Last")?,
            dob: DateOfBirth::parse(&a.dob)?,
            email: ValidEmail::parse(&a.email)?,
            gender: a.gender,
            photo_url: PhotoUrl::parse(&a.photo_url)?,
        };
        Ok(res)
    }
}

impl From<User> for UserInput {
    fn from(s: User) -> Self {
        UserInput {
            first_name: String::from(s.first_name.as_ref()),
            middle_name: String::from(s.middle_name.as_ref()),
            last_name: String::from(s.last_name.as_ref()),
            dob: *s.dob.as_ref(),
            email: String::from(s.email.as_ref()),
            gender: s.gender,
            photo_url: String::from(s.photo_url.as_ref()),
        }
    }
}
