use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use user::domain::user_domain::UserState;

use crate::dto::doctor_type::DoctorType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateDoctor {
    pub id: String,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub user: UserState,
    pub doctor_role: Vec<DoctorType>,
    pub doctor_register_number: String,
    pub doctor_department: String,
    pub doctor_speciality: String,
    pub emergency: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateDoctor {
    pub id: String,
    pub user: UserState,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub doctor_role: Vec<DoctorType>,
    pub doctor_register_number: String,
    pub doctor_department: String,
    pub doctor_speciality: String,
    pub emergency: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteDoctor {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DoctorCommand {
    CreateDoctor(Box<CreateDoctor>),
    UpdateDoctor(Box<UpdateDoctor>),
    DeleteDoctor(DeleteDoctor),
}
