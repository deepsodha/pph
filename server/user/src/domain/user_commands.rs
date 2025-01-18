use chrono::{DateTime, Utc};
use common::dto::{
    address::AddressInput, contact::ContactInput, gov_info::GovInfoInput, user::UserInput,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub id: String,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub user: UserInput,
    pub phone: ContactInput,
    pub address: AddressInput,
    pub government_info: GovInfoInput,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateUserPassword {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateUserContact {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub phone: ContactInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateUserAddress {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub address: AddressInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateUserGovDetails {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub government_info: GovInfoInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateUserDetails {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub user: UserInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteUser {
    pub id: String,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UserCommand {
    CreateUser(Box<CreateUser>),
    UpdateUserPassword(UpdateUserPassword),
    UpdateUserContact(UpdateUserContact),
    UpdateUserAddress(UpdateUserAddress),
    UpdateUserGovDetails(UpdateUserGovDetails),
    UpdateUserDetails(UpdateUserDetails),
    DeleteUser(DeleteUser),
}
