use chrono::{DateTime, Utc};
use common::dto::{
    contact::ContactInput, gov_info::GovInfoInput, user::UserInput, user_role::SystemAdminRole,
};
use serde::{Deserialize, Serialize};

pub type StreamId = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateSystemAdmin {
    pub id: String,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub user: UserInput,
    pub phone: ContactInput,
    pub government_info: GovInfoInput,
    pub roles: Vec<SystemAdminRole>,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateSystemAdmin {
    pub id: String,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub roles: Vec<SystemAdminRole>,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateSystemAdminContactDetails {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub phone: ContactInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateSystemAdminGovDetails {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub government_info: GovInfoInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateSystemAdminUserDetails {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub user: UserInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteSystemAdmin {
    pub id: String,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SystemAdminCommands {
    CreateSystemAdmin(Box<CreateSystemAdmin>),
    UpdateSystemAdmin(UpdateSystemAdmin),
    DeleteSystemAdmin(DeleteSystemAdmin),
    UpdateSystemAdminUserDetails(Box<UpdateSystemAdminUserDetails>),
    UpdateSystemAdminContactDetails(UpdateSystemAdminContactDetails),
    UpdateSystemAdminGovDetails(UpdateSystemAdminGovDetails),
}
