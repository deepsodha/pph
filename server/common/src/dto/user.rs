use super::contact::ContactInput;
use super::gov_info::GovInfoInput;
use super::{address::AddressInput, gender::GenderType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(
    Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq, sqlx::FromRow,
)]
#[ts(export, export_to = "../../bindings/UserDetails.ts")]

pub struct UserDetails {
    pub user: UserInput,
    pub address: AddressInput,
    pub phone: ContactInput,
    pub government_info: GovInfoInput,
}

#[derive(
    Deserialize, Serialize, Debug, Clone, Default, Validate, TS, PartialEq, Eq, sqlx::FromRow,
)]
#[ts(export, export_to = "../../bindings/UserInput.ts")]
pub struct UserInput {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub first_name: String,
    pub middle_name: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub last_name: String,
    pub dob: DateTime<Utc>,
    pub email: String,
    pub gender: GenderType,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub photo_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/UserSlim.ts")]

pub struct UserSlim {
    pub id: String,
    pub name: String,
    pub image_url: String,
    pub role: String,
}

#[derive(Deserialize, Serialize, Debug, TS, Clone)]
#[ts(export, export_to = "../../bindings/AuthUser.ts")]

pub struct AuthUser {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub email: String,
    pub photo_url: String,
    pub gender: String,
    pub date_of_birth: String,
    pub password_hash: String,
}
