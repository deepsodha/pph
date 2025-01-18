use common::{
    domain::{
        contact::Contact, goverment::GovInfo, non_empty_vec::NonEmptyVec, password::ValidPassword,
        user::User,
    },
    dto::{
        contact::ContactInput, gov_info::GovInfoInput, user::UserInput, user_role::SystemAdminRole,
    },
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{
    systemadmin_commands::{
        CreateSystemAdmin, DeleteSystemAdmin, UpdateSystemAdmin, UpdateSystemAdminContactDetails,
        UpdateSystemAdminGovDetails, UpdateSystemAdminUserDetails,
    },
    systemadmin_events::SystemAdminCreated,
};
use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use common::domain::required_string::RequiredString;

#[derive(Debug, Clone)]
pub struct SystemAdminRoles(Vec<SystemAdminRole>);
impl SystemAdminRoles {
    pub fn parse(roles: NonEmptyVec<SystemAdminRole>) -> Result<SystemAdminRoles> {
        // doctor roles should always contain doctor as role
        let r = Vec::from(roles);
        let res = r.iter().any(|x| x == &SystemAdminRole::SystemAdmin);
        if res {
            Ok(SystemAdminRoles(r))
        } else {
            bail!("At least systemadmin roles is must")
        }
    }
}

impl From<SystemAdminRoles> for Vec<SystemAdminRole> {
    fn from(r: SystemAdminRoles) -> Self {
        r.0
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/SystemAdminState.ts")]
pub struct SystemAdminState {
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
    pub is_deleted: bool,
}

impl From<SystemAdminCreated> for SystemAdminState {
    fn from(u: SystemAdminCreated) -> Self {
        SystemAdminState {
            id: String::from(&u.id),
            org_id: String::from(&u.org_id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            user: u.user,
            phone: u.phone,
            government_info: u.government_info,
            roles: u.roles,
            password: String::from(&u.password),
            is_deleted: false,
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
    pub user: User,
    pub phone: Contact,
    pub government_info: GovInfo,
    pub roles: SystemAdminRoles,
    pub password: ValidPassword,
}

#[derive(Debug, Clone)]
pub struct Update {
    pub id: RequiredString,
    pub org_id: RequiredString,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub roles: SystemAdminRoles,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct SystemAdminUser {
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub user: User,
}

#[derive(Debug, Clone)]
pub struct SystemAdminPhone {
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub phone: Contact,
}

#[derive(Debug, Clone)]
pub struct SystemAdminGov {
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub government_info: GovInfo,
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
    pub fn parse(a: &CreateSystemAdmin) -> Result<Create> {
        Ok(Create {
            id: a.id.to_owned(),
            org_id: RequiredString::parse(&a.org_id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            user: User::parse(&a.user)?,
            phone: Contact::parse(&a.phone)?,
            government_info: GovInfo::parse(&a.government_info)?,
            last_updated: (a.last_updated).to_owned(),
            roles: SystemAdminRoles::parse(NonEmptyVec::parse((a.roles).to_owned())?)?,
            password: ValidPassword::parse(&a.password)?,
        })
    }
}

impl Update {
    pub fn parse(a: &UpdateSystemAdmin) -> Result<Update> {
        Ok(Update {
            id: RequiredString::parse(&a.id)?,
            org_id: RequiredString::parse(&a.org_id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            roles: SystemAdminRoles::parse(NonEmptyVec::parse((a.roles).to_owned())?)?,
            password: (a.password).to_owned(),
        })
    }
}

impl SystemAdminPhone {
    pub fn parse(a: &UpdateSystemAdminContactDetails) -> Result<SystemAdminPhone> {
        Ok(SystemAdminPhone {
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            phone: Contact::parse(&a.phone)?,
        })
    }
}

impl SystemAdminGov {
    pub fn parse(a: &UpdateSystemAdminGovDetails) -> Result<SystemAdminGov> {
        Ok(SystemAdminGov {
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            government_info: GovInfo::parse(&a.government_info)?,
        })
    }
}

impl SystemAdminUser {
    pub fn parse(a: &UpdateSystemAdminUserDetails) -> Result<SystemAdminUser> {
        Ok(SystemAdminUser {
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            user: User::parse(&a.user)?,
        })
    }
}

impl Delete {
    pub fn parse(a: &DeleteSystemAdmin) -> Result<Delete> {
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
