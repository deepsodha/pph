use super::doctor_commands::{CreateDoctor, DeleteDoctor, UpdateDoctor};
use super::doctor_events::DoctorCreated;
use crate::dto::doctor_type::DoctorType;
use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use common::domain::emergency_access::EmergencyAccess;
use common::domain::non_empty_vec::NonEmptyVec;
use common::domain::required_string::RequiredString;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use user::domain::user_domain::UserState;

#[derive(Debug, Clone)]
pub struct DoctorRoles(Vec<DoctorType>);
impl DoctorRoles {
    pub fn parse(roles: NonEmptyVec<DoctorType>) -> Result<DoctorRoles> {
        // doctor roles should always contain doctor as role
        let r = Vec::from(roles);
        if !r.is_empty() {
            Ok(DoctorRoles(r))
        } else {
            bail!("At least doctor roles is must")
        }
    }
}

impl From<DoctorRoles> for Vec<DoctorType> {
    fn from(r: DoctorRoles) -> Self {
        r.0
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/DoctorState.ts")]
pub struct DoctorState {
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
    pub is_deleted: bool,
}

impl From<DoctorCreated> for DoctorState {
    fn from(u: DoctorCreated) -> Self {
        DoctorState {
            org_id: String::from(&u.org_id),
            id: String::from(&u.id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            doctor_role: u.doctor_role,
            doctor_register_number: u.doctor_register_number,
            doctor_department: u.doctor_department,
            doctor_speciality: u.doctor_speciality,
            emergency: u.emergency,
            is_deleted: false,
            user: u.user.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Create {
    pub id: String,
    pub user: UserState,
    pub org_id: RequiredString,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub doctor_register_number: RequiredString,
    pub doctor_role: DoctorRoles,
    pub doctor_department: RequiredString,
    pub doctor_speciality: RequiredString,
    pub emergency: EmergencyAccess,
}

#[derive(Debug, Clone)]
pub struct Update {
    pub id: RequiredString,
    pub user: UserState,
    pub org_id: RequiredString,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub doctor_register_number: RequiredString,
    pub doctor_role: DoctorRoles,
    pub doctor_department: RequiredString,
    pub doctor_speciality: RequiredString,
    pub emergency: EmergencyAccess,
}

#[derive(Debug, Clone)]
pub struct Delete {
    pub id: RequiredString,
    pub org_id: RequiredString,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl Create {
    pub fn parse(a: &CreateDoctor) -> Result<Create> {
        Ok(Create {
            id: a.id.to_owned(),
            user: a.user.clone(),
            org_id: RequiredString::parse(&a.org_id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            doctor_register_number: RequiredString::parse(&a.doctor_register_number)?,
            doctor_role: DoctorRoles::parse(NonEmptyVec::parse((a.doctor_role).to_owned())?)?,
            doctor_department: RequiredString::parse(&a.doctor_department)?,
            emergency: EmergencyAccess::parse(&a.emergency)?,
            doctor_speciality: RequiredString::parse(&a.doctor_speciality)?,
        })
    }
}

impl Update {
    pub fn parse(a: &UpdateDoctor) -> Result<Update> {
        Ok(Update {
            id: RequiredString::parse(&a.id)?,
            org_id: RequiredString::parse(&a.org_id)?,
            user: a.user.clone(),
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            doctor_register_number: RequiredString::parse(&a.doctor_register_number)?,
            doctor_role: DoctorRoles::parse(NonEmptyVec::parse((a.doctor_role).to_owned())?)?,
            doctor_department: RequiredString::parse(&a.doctor_department)?,
            emergency: EmergencyAccess::parse(&a.emergency)?,
            doctor_speciality: RequiredString::parse(&a.doctor_speciality)?,
        })
    }
}

impl Delete {
    pub fn parse(a: &DeleteDoctor) -> Result<Delete> {
        Ok(Delete {
            id: RequiredString::parse(&a.id)?,
            org_id: RequiredString::parse(&a.org_id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
        })
    }
}
