use super::service_location_commands::{
    CreateSerivceLocation, DeleteServiceLocation, ServiceLocationSelect, UpdateServiceLocation,
};
use super::service_location_events::{ServiceLocationCreated, ServiceLocationUpdated};
use anyhow::Result;
use chrono::{DateTime, NaiveTime, Utc};
use common::domain::address::Address;
use common::domain::required_string::RequiredString;
use common::dto::address::AddressInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export, export_to = "../../bindings/ServiceLocationState.ts")]
pub struct ServiceLocationState {
    pub id: String,
    pub org_id: String,
    pub service_location_name: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub address: AddressInput,
    pub is_deleted: bool,
}

impl From<ServiceLocationCreated> for ServiceLocationState {
    fn from(u: ServiceLocationCreated) -> Self {
        ServiceLocationState {
            org_id: String::from(&u.org_id),
            id: String::from(&u.id),
            service_location_name: String::from(&u.service_location_name),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            start_time: u.start_time,
            end_time: u.end_time,
            created_at: u.created_at,
            last_updated: u.last_updated,
            address: u.address,
            is_deleted: false,
        }
    }
}

impl From<ServiceLocationUpdated> for ServiceLocationState {
    fn from(u: ServiceLocationUpdated) -> Self {
        ServiceLocationState {
            org_id: String::from(&u.org_id),
            id: String::from(&u.id),
            service_location_name: String::from(&u.service_location_name),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            start_time: u.start_time,
            end_time: u.end_time,
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
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: RequiredString,
    pub service_location_name: RequiredString,
    pub address: Address,
}

#[derive(Debug, Clone)]
pub struct Update {
    pub id: RequiredString,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: RequiredString,
    pub service_location_name: RequiredString,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub address: Address,
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

#[derive(Debug, Clone)]
pub struct Select {
    pub id: RequiredString,
    pub name: RequiredString,
}

impl Create {
    pub fn parse(a: &CreateSerivceLocation) -> Result<Create> {
        Ok(Create {
            id: a.id.to_owned(),
            org_id: RequiredString::parse(&a.org_id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            start_time: (a.start_time).to_owned(),
            end_time: (a.end_time).to_owned(),
            address: Address::parse(&a.address)?,
            service_location_name: RequiredString::parse(&a.service_location_name)?,
        })
    }
}

impl Update {
    pub fn parse(a: &UpdateServiceLocation) -> Result<Update> {
        Ok(Update {
            id: RequiredString::parse(&a.id)?,
            org_id: RequiredString::parse(&a.org_id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            start_time: (a.start_time).to_owned(),
            end_time: (a.end_time).to_owned(),
            address: Address::parse(&a.address)?,
            service_location_name: RequiredString::parse(&a.service_location_name)?,
        })
    }
}

impl Delete {
    pub fn parse(a: &DeleteServiceLocation) -> Result<Delete> {
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

impl Select {
    pub fn parse(a: &ServiceLocationSelect) -> Result<Select> {
        Ok(Select {
            id: RequiredString::parse(&a.id)?,
            name: RequiredString::parse(&a.name)?,
        })
    }
}
