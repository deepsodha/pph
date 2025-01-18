use super::organization_commands::{
    CreateOrganization, DeleteOrganization, OrganizationSelect, UpdateOrganization,
};
use super::organization_events::{OrganizationCreated, OrganizationUpdated};
use anyhow::Result;
use chrono::{DateTime, Utc};
use common::domain::address::Address;
use common::domain::contact::PhoneNo;
use common::domain::email::ValidEmail;
use common::domain::name::Name;
use common::domain::required_string::RequiredString;
use common::dto::address::AddressInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Default, Serialize, Deserialize, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/OrganizationState.ts")]
pub struct OrganizationState {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub name: String,
    pub details: String,
    pub phone_number: String,
    pub email: String,
    pub address: AddressInput,
    pub is_deleted: bool,
}

impl From<OrganizationCreated> for OrganizationState {
    fn from(u: OrganizationCreated) -> Self {
        OrganizationState {
            id: String::from(&u.id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            name: String::from(&u.org_name),
            details: String::from(&u.details),
            phone_number: String::from(&u.phone_number),
            email: String::from(&u.email),
            address: u.address,
            is_deleted: false,
        }
    }
}

impl From<OrganizationUpdated> for OrganizationState {
    fn from(u: OrganizationUpdated) -> Self {
        OrganizationState {
            id: String::from(&u.id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            name: String::from(&u.org_name),
            details: String::from(&u.details),
            phone_number: String::from(&u.phone_number),
            email: String::from(&u.email),
            address: u.address,
            is_deleted: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Create {
    pub id: String,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub name: Name,
    pub details: RequiredString,
    pub phone_number: PhoneNo,
    pub email: ValidEmail,
    pub address: Address,
}

#[derive(Debug, Clone)]
pub struct Update {
    pub id: RequiredString,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub name: Name,
    pub details: RequiredString,
    pub phone_number: PhoneNo,
    pub email: ValidEmail,
    pub address: Address,
}

#[derive(Debug, Clone)]
pub struct Delete {
    pub id: RequiredString,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Select {
    pub id: RequiredString,
    pub name: RequiredString,
}

impl Create {
    pub fn parse(a: &CreateOrganization) -> Result<Create> {
        // Here all the validation will come
        Ok(Create {
            id: a.id.to_owned(),
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            name: Name::parse(&a.org_name, "Organization")?,
            details: RequiredString::parse(&a.details)?,
            phone_number: PhoneNo::parse(&a.phone_number)?,
            email: ValidEmail::parse(&a.email)?,
            address: Address::parse(&a.address)?,
        })
    }
}

impl Update {
    pub fn parse(a: &UpdateOrganization) -> Result<Update> {
        // Here all the validation will come
        Ok(Update {
            id: RequiredString::parse(&a.id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            name: Name::parse(&a.org_name, "Organization")?,
            details: RequiredString::parse(&a.details)?,
            phone_number: PhoneNo::parse(&a.phone_number)?,
            email: ValidEmail::parse(&a.email)?,
            address: Address::parse(&a.address)?,
        })
    }
}

impl Delete {
    pub fn parse(a: &DeleteOrganization) -> Result<Delete> {
        // Here all the validation will come
        Ok(Delete {
            id: RequiredString::parse(&a.id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
        })
    }
}

impl Select {
    pub fn parse(a: &OrganizationSelect) -> Result<Select> {
        Ok(Select {
            id: RequiredString::parse(&a.id)?,
            name: RequiredString::parse(&a.name)?,
        })
    }
}
