use super::{
    hospitalization_commands::{
        CreateHospitalization, DeleteHospitalization, UpdateHospitalization,
    },
    hospitalization_events::{HospitalizationCreated, HospitalizationUpdated},
};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Default, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/HospitalizationState.ts")]
pub struct HospitalizationState {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub admission_date: DateTime<Utc>,
    pub related_to: Option<String>,
    pub status: Option<String>,
    pub length_of_stay: Option<u32>,
    pub procedure: Option<String>,
    pub comments: Option<String>,
    pub is_deleted: bool,
}

impl From<HospitalizationCreated> for HospitalizationState {
    fn from(u: HospitalizationCreated) -> Self {
        HospitalizationState {
            id: String::from(&u.id),
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            comments: u.comments,
            admission_date: u.admission_date,
            related_to: u.related_to,
            status: u.status,
            length_of_stay: u.length_of_stay,
            procedure: u.procedure,
            is_deleted: false,
        }
    }
}

impl From<HospitalizationUpdated> for HospitalizationState {
    fn from(u: HospitalizationUpdated) -> Self {
        HospitalizationState {
            id: String::from(&u.id),
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            comments: u.comments,
            admission_date: u.admission_date,
            related_to: u.related_to,
            status: u.status,
            length_of_stay: u.length_of_stay,
            procedure: u.procedure,
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
    pub admission_date: DateTime<Utc>,
    pub related_to: Option<String>,
    pub status: Option<String>,
    pub length_of_stay: Option<u32>,
    pub procedure: Option<String>,
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
    pub admission_date: DateTime<Utc>,
    pub related_to: Option<String>,
    pub status: Option<String>,
    pub length_of_stay: Option<u32>,
    pub procedure: Option<String>,
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
    pub fn parse(a: &CreateHospitalization) -> Result<Create> {
        Ok(Create {
            id: a.id.to_owned(),
            org_id: String::from(&a.org_id),
            patient_id: String::from(&a.patient_id),
            created_by: String::from(&a.created_by),
            updated_by: String::from(&a.updated_by),
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            comments: (a.comments).to_owned(),
            admission_date: (a.admission_date).to_owned(),
            related_to: (a.related_to).to_owned(),
            status: (a.status).to_owned(),
            length_of_stay: (a.length_of_stay).to_owned(),
            procedure: (a.procedure).to_owned(),
        })
    }
}
impl Update {
    pub fn parse(a: &UpdateHospitalization) -> Result<Update> {
        Ok(Update {
            id: String::from(&a.id),
            org_id: String::from(&a.org_id),
            patient_id: String::from(&a.patient_id),
            created_by: String::from(&a.created_by),
            updated_by: String::from(&a.updated_by),
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            comments: (a.comments).to_owned(),
            admission_date: (a.admission_date).to_owned(),
            related_to: (a.related_to).to_owned(),
            status: (a.status).to_owned(),
            length_of_stay: (a.length_of_stay).to_owned(),
            procedure: (a.procedure).to_owned(),
        })
    }
}
impl Delete {
    pub fn parse(a: &DeleteHospitalization) -> Result<Delete> {
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
