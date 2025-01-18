use anyhow::Result;
use chrono::{DateTime, Utc};
use common::{
    domain::{min_max_words::RequiredMax300Words, required_string::RequiredString},
    dto::types::Types,
};
use doctor::dto::doctor_type::DoctorType;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::dto::immunization::administer_types::Brand;

use super::{
    administer_commands::{CreateAdminister, DeleteAdminister, UpdateAdminister},
    administer_events::{AdministerCreated, AdministerUpdated},
};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export, export_to = "../../bindings/AdministerState.ts")]
pub struct AdministerState {
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
    pub is_deleted: bool,
}

impl From<AdministerCreated> for AdministerState {
    fn from(u: AdministerCreated) -> Self {
        AdministerState {
            id: String::from(&u.id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            vaccine: String::from(&u.vaccine),
            types: u.types,
            brand: u.brand,
            ordered: u.ordered,
            recorded: u.recorded,
            dose: String::from(&u.dose),
            site: String::from(&u.site),
            number_of_series: u.number_of_series,
            lot: u.lot,
            expiration: u.expiration,
            consent_obtain: String::from(&u.consent_obtain),
            administrated_by: String::from(&u.administrated_by),
            clinic_location: u.clinic_location,
            provider: u.provider,
            vis_date: u.vis_date,
            vfs_financial_class: String::from(&u.vfs_financial_class),
            comments: String::from(&u.comments),
            generic: String::from(&u.generic),
            is_deleted: false,
        }
    }
}

impl From<AdministerUpdated> for AdministerState {
    fn from(u: AdministerUpdated) -> Self {
        AdministerState {
            id: String::from(&u.id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            vaccine: String::from(&u.vaccine),
            types: u.types,
            brand: u.brand,
            generic: String::from(&u.generic),
            ordered: u.ordered,
            recorded: u.recorded,
            dose: String::from(&u.dose),
            site: String::from(&u.site),
            number_of_series: u.number_of_series,
            lot: u.lot,
            expiration: u.expiration,
            consent_obtain: String::from(&u.consent_obtain),
            administrated_by: String::from(&u.administrated_by),
            clinic_location: u.clinic_location,
            provider: u.provider,
            vis_date: u.vis_date,
            vfs_financial_class: String::from(&u.vfs_financial_class),
            comments: String::from(&u.comments),
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
    pub org_id: RequiredString,
    pub patient_id: RequiredString,
    pub vaccine: RequiredString,
    pub types: Types,
    pub brand: Brand,
    pub generic: RequiredString,
    pub ordered: DateTime<Utc>,
    pub recorded: DateTime<Utc>,
    pub dose: RequiredString,
    pub site: RequiredString,
    pub number_of_series: i32,
    pub lot: i32,
    pub expiration: DateTime<Utc>,
    pub consent_obtain: RequiredString,
    pub administrated_by: RequiredString,
    pub clinic_location: String,
    pub provider: DoctorType,
    pub vis_date: DateTime<Utc>,
    pub vfs_financial_class: RequiredString,
    pub comments: RequiredMax300Words,
}

#[derive(Debug, Clone)]
pub struct Update {
    pub id: RequiredString,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: RequiredString,
    pub patient_id: RequiredString,
    pub vaccine: RequiredString,
    pub types: Types,
    pub brand: Brand,
    pub generic: RequiredString,
    pub ordered: DateTime<Utc>,
    pub recorded: DateTime<Utc>,
    pub dose: RequiredString,
    pub site: RequiredString,
    pub number_of_series: i32,
    pub lot: i32,
    pub expiration: DateTime<Utc>,
    pub consent_obtain: RequiredString,
    pub administrated_by: RequiredString,
    pub clinic_location: String,
    pub provider: DoctorType,
    pub vis_date: DateTime<Utc>,
    pub vfs_financial_class: RequiredString,
    pub comments: RequiredMax300Words,
}

#[derive(Debug, Clone)]
pub struct Delete {
    pub id: RequiredString,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: RequiredString,
    pub patient_id: RequiredString,
}

impl Create {
    pub fn parse(a: &CreateAdminister) -> Result<Create> {
        Ok(Create {
            id: a.id.to_owned(),
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            org_id: RequiredString::parse(&a.org_id)?,
            patient_id: RequiredString::parse(&a.patient_id)?,
            vaccine: RequiredString::parse(&a.vaccine)?,
            types: (a.types).to_owned(),
            generic: RequiredString::parse(&a.generic)?,
            ordered: (a.ordered).to_owned(),
            brand: (a.brand).to_owned(),
            recorded: (a.recorded).to_owned(),
            dose: RequiredString::parse(&a.dose)?,
            site: RequiredString::parse(&a.site)?,
            number_of_series: (a.number_of_series).to_owned(),
            lot: (a.lot).to_owned(),
            expiration: (a.expiration).to_owned(),
            consent_obtain: RequiredString::parse(&a.consent_obtain)?,
            administrated_by: RequiredString::parse(&a.administrated_by)?,
            clinic_location: (a.clinic_location).to_owned(),
            provider: (a.provider).to_owned(),
            vis_date: (a.vis_date).to_owned(),
            vfs_financial_class: RequiredString::parse(&a.vfs_financial_class)?,
            comments: RequiredMax300Words::parse(&a.comments)?,
        })
    }
}

impl Update {
    pub fn parse(a: &UpdateAdminister) -> Result<Update> {
        Ok(Update {
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            id: RequiredString::parse(&a.id)?,
            org_id: RequiredString::parse(&a.org_id)?,
            patient_id: RequiredString::parse(&a.patient_id)?,
            vaccine: RequiredString::parse(&a.vaccine)?,
            types: (a.types).to_owned(),
            generic: RequiredString::parse(&a.generic)?,
            ordered: (a.ordered).to_owned(),
            brand: (a.brand).to_owned(),
            recorded: (a.recorded).to_owned(),
            dose: RequiredString::parse(&a.dose)?,
            site: RequiredString::parse(&a.site)?,
            number_of_series: (a.number_of_series).to_owned(),
            lot: (a.lot).to_owned(),
            expiration: (a.expiration).to_owned(),
            consent_obtain: RequiredString::parse(&a.consent_obtain)?,
            administrated_by: RequiredString::parse(&a.administrated_by)?,
            clinic_location: (a.clinic_location).to_owned(),
            provider: (a.provider).to_owned(),
            vis_date: (a.vis_date).to_owned(),
            vfs_financial_class: RequiredString::parse(&a.vfs_financial_class)?,
            comments: RequiredMax300Words::parse(&a.comments)?,
        })
    }
}

impl Delete {
    pub fn parse(a: &DeleteAdminister) -> Result<Delete> {
        Ok(Delete {
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            id: RequiredString::parse(&a.id)?,
            org_id: RequiredString::parse(&a.org_id)?,
            patient_id: RequiredString::parse(&a.patient_id)?,
        })
    }
}
