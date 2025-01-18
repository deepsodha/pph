use chrono::{DateTime, Utc};
use common::dto::types::Types;
use cosmo_store::types::event_write::EventWrite;
use doctor::dto::doctor_type::DoctorType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dto::immunization::administer_types::Brand;

use super::administer_domain::{Create, Delete, Update};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AdministerEvent {
    AdministerCreated(AdministerCreated),
    AdministerUpdated(AdministerUpdated),
    AdministerDeleted(AdministerDeleted),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdministerCreated {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub patient_id: String,
    pub vaccine: String,
    pub types: Types,
    pub brand: Brand,
    pub generic: String,
    pub ordered: DateTime<Utc>,
    pub recorded: DateTime<Utc>,
    pub dose: String,
    pub site: String,
    pub number_of_series: i32,
    pub lot: i32,
    pub expiration: DateTime<Utc>,
    pub consent_obtain: String,
    pub administrated_by: String,
    pub clinic_location: String,
    pub provider: DoctorType,
    pub vis_date: DateTime<Utc>,
    pub vfs_financial_class: String,
    pub comments: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdministerUpdated {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub patient_id: String,
    pub vaccine: String,
    pub types: Types,
    pub brand: Brand,
    pub generic: String,
    pub ordered: DateTime<Utc>,
    pub recorded: DateTime<Utc>,
    pub dose: String,
    pub site: String,
    pub number_of_series: i32,
    pub lot: i32,
    pub expiration: DateTime<Utc>,
    pub consent_obtain: String,
    pub administrated_by: String,
    pub clinic_location: String,
    pub provider: DoctorType,
    pub vis_date: DateTime<Utc>,
    pub vfs_financial_class: String,
    pub comments: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdministerDeleted {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub patient_id: String,
}

impl From<AdministerEvent> for EventWrite<AdministerEvent, AdministerEvent> {
    fn from(u: AdministerEvent) -> Self {
        EventWrite {
            id: Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name: String::from("administer_event"),
            data: u,
            metadata: None,
        }
    }
}

impl From<Create> for AdministerCreated {
    fn from(s: Create) -> Self {
        AdministerCreated {
            id: s.id,
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            org_id: String::from(s.org_id.as_ref()),
            patient_id: String::from(s.patient_id.as_ref()),
            vaccine: String::from(s.vaccine.as_ref()),
            types: s.types,
            brand: s.brand,
            generic: String::from(s.generic.as_ref()),
            ordered: s.ordered,
            recorded: s.recorded,
            dose: String::from(s.dose.as_ref()),
            site: String::from(s.site.as_ref()),
            number_of_series: s.number_of_series,
            lot: s.lot,
            expiration: s.expiration,
            consent_obtain: String::from(s.consent_obtain.as_ref()),
            administrated_by: String::from(s.administrated_by.as_ref()),
            clinic_location: s.clinic_location,
            provider: s.provider,
            vis_date: s.vis_date,
            vfs_financial_class: String::from(s.vfs_financial_class.as_ref()),
            comments: String::from(s.comments.as_ref()),
        }
    }
}

impl From<Update> for AdministerUpdated {
    fn from(s: Update) -> Self {
        AdministerUpdated {
            id: String::from(s.id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            org_id: String::from(s.org_id.as_ref()),
            patient_id: String::from(s.patient_id.as_ref()),
            vaccine: String::from(s.vaccine.as_ref()),
            types: s.types,
            brand: s.brand,
            generic: String::from(s.generic.as_ref()),
            ordered: s.ordered,
            recorded: s.recorded,
            dose: String::from(s.dose.as_ref()),
            site: String::from(s.site.as_ref()),
            number_of_series: s.number_of_series,
            lot: s.lot,
            expiration: s.expiration,
            consent_obtain: String::from(s.consent_obtain.as_ref()),
            administrated_by: String::from(s.administrated_by.as_ref()),
            clinic_location: s.clinic_location,
            provider: s.provider,
            vis_date: s.vis_date,
            vfs_financial_class: String::from(s.vfs_financial_class.as_ref()),
            comments: String::from(s.comments.as_ref()),
        }
    }
}

impl From<Delete> for AdministerDeleted {
    fn from(s: Delete) -> Self {
        AdministerDeleted {
            id: String::from(s.id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            org_id: String::from(s.org_id.as_ref()),
            patient_id: String::from(s.patient_id.as_ref()),
        }
    }
}
