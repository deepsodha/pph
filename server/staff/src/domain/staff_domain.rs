use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use common::domain::emergency_access::EmergencyAccess;
use common::domain::non_empty_vec::NonEmptyVec;
use common::domain::required_string::RequiredString;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use user::domain::user_domain::UserState;
use uuid::Uuid;

use crate::dto::staff_type::StaffType;

use super::{
    staff_commands::{CreateStaff, DeleteStaff, UpdateStaff},
    staff_events::StaffCreated,
};

#[derive(Debug, Clone)]
pub struct StaffRoles(Vec<StaffType>);
impl StaffRoles {
    pub fn parse(roles: NonEmptyVec<StaffType>) -> Result<StaffRoles> {
        // doctor roles should always contain doctor as role
        let r = Vec::from(roles);
        let res = r.iter().any(|x| x == &StaffType::Consultation);
        if res {
            Ok(StaffRoles(r))
        } else {
            bail!("At least staff roles is must")
        }
    }
}

impl From<StaffRoles> for Vec<StaffType> {
    fn from(r: StaffRoles) -> Self {
        r.0
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/StaffState.ts")]
pub struct StaffState {
    pub id: String,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub user: UserState,
    pub staff_department: Vec<StaffType>,
    pub emergency: bool,
    pub is_deleted: bool,
}

impl From<StaffCreated> for StaffState {
    fn from(u: StaffCreated) -> Self {
        StaffState {
            org_id: String::from(&u.org_id),
            id: String::from(&u.id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            emergency: u.emergency,
            is_deleted: false,
            user: u.user,
            staff_department: u.staff_department,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Create {
    pub id: String,
    pub org_id: RequiredString,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub user: UserState,
    pub staff_department: StaffRoles,
    pub emergency: EmergencyAccess,
}

#[derive(Debug, Clone)]
pub struct Update {
    pub id: RequiredString,
    pub org_id: RequiredString,
    pub user: UserState,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub staff_department: StaffRoles,
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
    pub fn parse(a: &CreateStaff) -> Result<Create> {
        Ok(Create {
            id: Uuid::new_v4().to_string(),
            org_id: RequiredString::parse(&a.org_id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            user: a.user.clone(),
            staff_department: StaffRoles::parse(NonEmptyVec::parse(
                (a.staff_department).to_owned(),
            )?)?,
            emergency: EmergencyAccess::parse(&a.emergency)?,
        })
    }
}

impl Update {
    pub fn parse(a: &UpdateStaff) -> Result<Update> {
        Ok(Update {
            id: RequiredString::parse(&a.id)?,
            org_id: RequiredString::parse(&a.org_id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            staff_department: StaffRoles::parse(NonEmptyVec::parse(
                (a.staff_department).to_owned(),
            )?)?,
            emergency: EmergencyAccess::parse(&a.emergency)?,
            user: a.user.clone(),
        })
    }
}

impl Delete {
    pub fn parse(a: &DeleteStaff) -> Result<Delete> {
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
