use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use user::domain::user_domain::UserState;

use crate::dto::staff_type::StaffType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateStaff {
    pub id: String,
    pub user: UserState,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub staff_department: Vec<StaffType>,
    pub emergency: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateStaff {
    pub id: String,
    pub org_id: String,
    pub user: UserState,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub staff_department: Vec<StaffType>,
    pub emergency: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteStaff {
    pub id: String,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StaffCommand {
    CreateStaff(Box<CreateStaff>),
    UpdateStaff(Box<UpdateStaff>),
    DeleteStaff(DeleteStaff),
}
