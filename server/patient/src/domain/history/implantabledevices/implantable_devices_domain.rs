use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{
    implantable_devices_commands::{
        CreateImplantableDevices, DeleteImplantableDevices, UpdateImplantableDevices,
    },
    implantable_devices_events::{ImplantableDevicesCreated, ImplantableDevicesUpdated},
};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export, export_to = "../../bindings/ImplantableDevicesState.ts")]
pub struct ImplantableDevicesState {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub status: Option<String>,
    pub udi: Option<String>,
    pub udi_unknown: Option<bool>,
    pub comments: Option<String>,
    pub is_deleted: bool,
}

impl From<ImplantableDevicesCreated> for ImplantableDevicesState {
    fn from(u: ImplantableDevicesCreated) -> Self {
        ImplantableDevicesState {
            id: String::from(&u.id),
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            comments: u.comments,
            status: u.status,
            udi: u.udi,
            udi_unknown: u.udi_unknown,
            is_deleted: false,
        }
    }
}

impl From<ImplantableDevicesUpdated> for ImplantableDevicesState {
    fn from(u: ImplantableDevicesUpdated) -> Self {
        ImplantableDevicesState {
            id: String::from(&u.id),
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            comments: u.comments,
            status: u.status,
            udi: u.udi,
            udi_unknown: u.udi_unknown,
            is_deleted: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Create {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub status: Option<String>,
    pub udi: Option<String>,
    pub udi_unknown: Option<bool>,
    pub comments: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Update {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub status: Option<String>,
    pub udi: Option<String>,
    pub udi_unknown: Option<bool>,
    pub comments: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Delete {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl Create {
    pub fn parse(a: &CreateImplantableDevices) -> Result<Create> {
        Ok(Create {
            id: a.id.to_owned(),
            org_id: String::from(&a.org_id),
            patient_id: String::from(&a.patient_id),
            created_by: String::from(&a.created_by),
            updated_by: String::from(&a.updated_by),
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            comments: (a.comments).to_owned(),
            status: (a.status).to_owned(),
            udi: (a.udi).to_owned(),
            udi_unknown: (a.udi_unknown).to_owned(),
        })
    }
}
impl Update {
    pub fn parse(a: &UpdateImplantableDevices) -> Result<Update> {
        Ok(Update {
            id: String::from(&a.id),
            org_id: String::from(&a.org_id),
            patient_id: String::from(&a.patient_id),
            created_by: String::from(&a.created_by),
            updated_by: String::from(&a.updated_by),
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            comments: (a.comments).to_owned(),
            status: (a.status).to_owned(),
            udi: (a.udi).to_owned(),
            udi_unknown: (a.udi_unknown).to_owned(),
        })
    }
}
impl Delete {
    pub fn parse(a: &DeleteImplantableDevices) -> Result<Delete> {
        Ok(Delete {
            id: String::from(&a.id),
            org_id: String::from(&a.org_id),
            patient_id: String::from(&a.patient_id),
            created_by: String::from(&a.created_by),
            updated_by: String::from(&a.updated_by),
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
        })
    }
}
