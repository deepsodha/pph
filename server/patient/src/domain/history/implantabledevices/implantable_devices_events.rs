use chrono::{DateTime, Utc};
use cosmo_store::types::event_write::EventWrite;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::implantable_devices_domain::{Create, Delete, Update};

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct ImplantableDevicesCreated {
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

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct ImplantableDevicesUpdated {
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

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct ImplantableDevicesDeleted {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ImplantableDevicesEvent {
    ImplantableDevicesCreated(ImplantableDevicesCreated),
    ImplantableDevicesUpdated(ImplantableDevicesUpdated),
    ImplantableDevicesDeleted(ImplantableDevicesDeleted),
}

impl From<Create> for ImplantableDevicesCreated {
    fn from(s: Create) -> Self {
        ImplantableDevicesCreated {
            id: s.id,
            org_id: String::from(&s.org_id),
            patient_id: String::from(&s.patient_id),
            created_by: String::from(&s.created_by),
            updated_by: String::from(&s.updated_by),
            created_at: s.created_at,
            last_updated: s.last_updated,
            comments: s.comments,
            status: s.status,
            udi: s.udi,
            udi_unknown: s.udi_unknown,
        }
    }
}

impl From<Update> for ImplantableDevicesUpdated {
    fn from(s: Update) -> Self {
        ImplantableDevicesUpdated {
            id: s.id,
            org_id: String::from(&s.org_id),
            patient_id: String::from(&s.patient_id),
            created_by: String::from(&s.created_by),
            updated_by: String::from(&s.updated_by),
            created_at: s.created_at,
            last_updated: s.last_updated,
            comments: s.comments,
            status: s.status,
            udi: s.udi,
            udi_unknown: s.udi_unknown,
        }
    }
}

impl From<Delete> for ImplantableDevicesDeleted {
    fn from(s: Delete) -> Self {
        ImplantableDevicesDeleted {
            id: s.id,
            org_id: String::from(&s.org_id),
            patient_id: String::from(&s.patient_id),
            created_by: String::from(&s.created_by),
            updated_by: String::from(&s.updated_by),
            created_at: s.created_at,
            last_updated: s.last_updated,
        }
    }
}

impl From<ImplantableDevicesEvent>
    for EventWrite<ImplantableDevicesEvent, ImplantableDevicesEvent>
{
    fn from(u: ImplantableDevicesEvent) -> Self {
        EventWrite {
            id: uuid::Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name: String::from("ImplantableDevices_event"),
            data: u,
            metadata: None,
        }
    }
}
