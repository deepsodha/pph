use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{
    pastmedical_history_commands::{
        CreatePastMedicalHistory, DeletePastMedicalHistory, UpdatePastMedicalHistory,
    },
    pastmedical_history_events::{PastMedicalHistoryCreated, PastMedicalHistoryUpdated},
};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export, export_to = "../../bindings/PastMedicalHistoryState.ts")]
pub struct PastMedicalHistoryState {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub blood_type: Option<String>,
    pub head: Option<Vec<String>>,
    pub respiratory: Option<Vec<String>>,
    pub musculoskeletal: Option<Vec<String>>,
    pub endocrine: Option<Vec<String>>,
    pub eyes: Option<Vec<String>>,
    pub gastrointestinal: Option<Vec<String>>,
    pub skin: Option<Vec<String>>,
    pub ears: Option<Vec<String>>,
    pub noses: Option<Vec<String>>,
    pub neurological: Option<Vec<String>>,
    pub heme: Option<Vec<String>>,
    pub mouth: Option<Vec<String>>,
    pub infectious: Option<Vec<String>>,
    pub cardiovascular: Option<Vec<String>>,
    pub genitourinary: Option<Vec<String>>,
    pub psychiatric: Option<Vec<String>>,
    pub comments: Option<String>,
    pub is_deleted: bool,
}

impl From<PastMedicalHistoryCreated> for PastMedicalHistoryState {
    fn from(u: PastMedicalHistoryCreated) -> Self {
        PastMedicalHistoryState {
            id: String::from(&u.id),
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            blood_type: u.blood_type,
            head: u.head,
            respiratory: u.respiratory,
            musculoskeletal: u.musculoskeletal,
            endocrine: u.endocrine,
            eyes: u.eyes,
            gastrointestinal: u.gastrointestinal,
            skin: u.skin,
            ears: u.ears,
            noses: u.noses,
            neurological: u.neurological,
            heme: u.heme,
            mouth: u.mouth,
            infectious: u.infectious,
            cardiovascular: u.cardiovascular,
            genitourinary: u.genitourinary,
            psychiatric: u.psychiatric,
            comments: u.comments,
            is_deleted: false,
        }
    }
}

impl From<PastMedicalHistoryUpdated> for PastMedicalHistoryState {
    fn from(u: PastMedicalHistoryUpdated) -> Self {
        PastMedicalHistoryState {
            id: String::from(&u.id),
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            blood_type: u.blood_type,
            head: u.head,
            respiratory: u.respiratory,
            musculoskeletal: u.musculoskeletal,
            endocrine: u.endocrine,
            eyes: u.eyes,
            gastrointestinal: u.gastrointestinal,
            skin: u.skin,
            ears: u.ears,
            noses: u.noses,
            neurological: u.neurological,
            heme: u.heme,
            mouth: u.mouth,
            infectious: u.infectious,
            cardiovascular: u.cardiovascular,
            genitourinary: u.genitourinary,
            psychiatric: u.psychiatric,
            comments: u.comments,
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
    pub blood_type: Option<String>,
    pub head: Option<Vec<String>>,
    pub respiratory: Option<Vec<String>>,
    pub musculoskeletal: Option<Vec<String>>,
    pub endocrine: Option<Vec<String>>,
    pub eyes: Option<Vec<String>>,
    pub gastrointestinal: Option<Vec<String>>,
    pub skin: Option<Vec<String>>,
    pub ears: Option<Vec<String>>,
    pub noses: Option<Vec<String>>,
    pub neurological: Option<Vec<String>>,
    pub heme: Option<Vec<String>>,
    pub mouth: Option<Vec<String>>,
    pub infectious: Option<Vec<String>>,
    pub cardiovascular: Option<Vec<String>>,
    pub genitourinary: Option<Vec<String>>,
    pub psychiatric: Option<Vec<String>>,
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
    pub blood_type: Option<String>,
    pub head: Option<Vec<String>>,
    pub respiratory: Option<Vec<String>>,
    pub musculoskeletal: Option<Vec<String>>,
    pub endocrine: Option<Vec<String>>,
    pub eyes: Option<Vec<String>>,
    pub gastrointestinal: Option<Vec<String>>,
    pub skin: Option<Vec<String>>,
    pub ears: Option<Vec<String>>,
    pub noses: Option<Vec<String>>,
    pub neurological: Option<Vec<String>>,
    pub heme: Option<Vec<String>>,
    pub mouth: Option<Vec<String>>,
    pub infectious: Option<Vec<String>>,
    pub cardiovascular: Option<Vec<String>>,
    pub genitourinary: Option<Vec<String>>,
    pub psychiatric: Option<Vec<String>>,
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
    pub fn parse(a: &CreatePastMedicalHistory) -> Result<Create> {
        Ok(Create {
            id: a.id.to_owned(),
            org_id: String::from(&a.org_id),
            patient_id: String::from(&a.patient_id),
            created_by: String::from(&a.created_by),
            updated_by: String::from(&a.updated_by),
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            blood_type: (a.blood_type).to_owned(),
            head: (a.head).to_owned(),
            respiratory: (a.respiratory).to_owned(),
            musculoskeletal: (a.musculoskeletal).to_owned(),
            endocrine: (a.endocrine).to_owned(),
            eyes: (a.eyes).to_owned(),
            gastrointestinal: (a.gastrointestinal).to_owned(),
            skin: (a.skin).to_owned(),
            ears: (a.ears).to_owned(),
            noses: (a.noses).to_owned(),
            neurological: (a.neurological).to_owned(),
            heme: (a.heme).to_owned(),
            mouth: (a.mouth).to_owned(),
            infectious: (a.infectious).to_owned(),
            cardiovascular: (a.cardiovascular).to_owned(),
            genitourinary: (a.genitourinary).to_owned(),
            psychiatric: (a.psychiatric).to_owned(),
            comments: (a.comments).to_owned(),
        })
    }
}
impl Update {
    pub fn parse(a: &UpdatePastMedicalHistory) -> Result<Update> {
        Ok(Update {
            id: String::from(&a.id),
            org_id: String::from(&a.org_id),
            patient_id: String::from(&a.patient_id),
            created_by: String::from(&a.created_by),
            updated_by: String::from(&a.updated_by),
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            blood_type: (a.blood_type).to_owned(),
            head: (a.head).to_owned(),
            respiratory: (a.respiratory).to_owned(),
            musculoskeletal: (a.musculoskeletal).to_owned(),
            endocrine: (a.endocrine).to_owned(),
            eyes: (a.eyes).to_owned(),
            gastrointestinal: (a.gastrointestinal).to_owned(),
            skin: (a.skin).to_owned(),
            ears: (a.ears).to_owned(),
            noses: (a.noses).to_owned(),
            neurological: (a.neurological).to_owned(),
            heme: (a.heme).to_owned(),
            mouth: (a.mouth).to_owned(),
            infectious: (a.infectious).to_owned(),
            cardiovascular: (a.cardiovascular).to_owned(),
            genitourinary: (a.genitourinary).to_owned(),
            psychiatric: (a.psychiatric).to_owned(),
            comments: (a.comments).to_owned(),
        })
    }
}
impl Delete {
    pub fn parse(a: &DeletePastMedicalHistory) -> Result<Delete> {
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
