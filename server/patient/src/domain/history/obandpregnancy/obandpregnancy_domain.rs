use super::{
    obandpregnancy_commands::{CreateOBandPregnancy, DeleteOBandPregnancy, UpdateOBandPregnancy},
    obandpregnancy_events::{OBandPregnancyCreated, OBandPregnancyUpdated},
};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export, export_to = "../../bindings/OBandPregnancyState.ts")]
pub struct OBandPregnancyState {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub age_onset_of_menses: u32,
    pub age_at_menopause: u32,
    pub comments_ob: Option<String>,
    pub total_pregnancy: Option<u32>,
    pub full_term: Option<u32>,
    pub pre_term: Option<u32>,
    pub miscarriages: Option<u32>,
    pub living: Option<u32>,
    pub comments_pregnancy: Option<String>,
    pub is_deleted: bool,
}

impl From<OBandPregnancyCreated> for OBandPregnancyState {
    fn from(u: OBandPregnancyCreated) -> Self {
        OBandPregnancyState {
            id: String::from(&u.id),
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            age_onset_of_menses: u.age_onset_of_menses,
            age_at_menopause: u.age_at_menopause,
            comments_ob: u.comments_ob,
            total_pregnancy: u.total_pregnancy,
            full_term: u.full_term,
            pre_term: u.pre_term,
            miscarriages: u.miscarriages,
            living: u.living,
            comments_pregnancy: u.comments_pregnancy,
            is_deleted: false,
        }
    }
}

impl From<OBandPregnancyUpdated> for OBandPregnancyState {
    fn from(u: OBandPregnancyUpdated) -> Self {
        OBandPregnancyState {
            id: String::from(&u.id),
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            age_onset_of_menses: u.age_onset_of_menses,
            age_at_menopause: u.age_at_menopause,
            comments_ob: u.comments_ob,
            total_pregnancy: u.total_pregnancy,
            full_term: u.full_term,
            pre_term: u.pre_term,
            miscarriages: u.miscarriages,
            living: u.living,
            comments_pregnancy: u.comments_pregnancy,
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
    pub age_onset_of_menses: u32,
    pub age_at_menopause: u32,
    pub comments_ob: Option<String>,
    pub total_pregnancy: Option<u32>,
    pub full_term: Option<u32>,
    pub pre_term: Option<u32>,
    pub miscarriages: Option<u32>,
    pub living: Option<u32>,
    pub comments_pregnancy: Option<String>,
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
    pub age_onset_of_menses: u32,
    pub age_at_menopause: u32,
    pub comments_ob: Option<String>,
    pub total_pregnancy: Option<u32>,
    pub full_term: Option<u32>,
    pub pre_term: Option<u32>,
    pub miscarriages: Option<u32>,
    pub living: Option<u32>,
    pub comments_pregnancy: Option<String>,
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
    pub fn parse(a: &CreateOBandPregnancy) -> Result<Create> {
        Ok(Create {
            id: a.id.to_owned(),
            org_id: String::from(&a.org_id),
            patient_id: String::from(&a.patient_id),
            created_by: String::from(&a.created_by),
            updated_by: String::from(&a.updated_by),
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            age_onset_of_menses: (a.age_onset_of_menses).to_owned(),
            age_at_menopause: (a.age_at_menopause).to_owned(),
            comments_ob: (a.comments_ob).to_owned(),
            total_pregnancy: (a.total_pregnancy).to_owned(),
            full_term: (a.full_term).to_owned(),
            pre_term: (a.pre_term).to_owned(),
            miscarriages: (a.miscarriages).to_owned(),
            living: (a.living).to_owned(),
            comments_pregnancy: (a.comments_pregnancy).to_owned(),
        })
    }
}
impl Update {
    pub fn parse(a: &UpdateOBandPregnancy) -> Result<Update> {
        Ok(Update {
            id: String::from(&a.id),
            org_id: String::from(&a.org_id),
            patient_id: String::from(&a.patient_id),
            created_by: String::from(&a.created_by),
            updated_by: String::from(&a.updated_by),
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            age_onset_of_menses: (a.age_onset_of_menses).to_owned(),
            age_at_menopause: (a.age_at_menopause).to_owned(),
            comments_ob: (a.comments_ob).to_owned(),
            total_pregnancy: (a.total_pregnancy).to_owned(),
            full_term: (a.full_term).to_owned(),
            pre_term: (a.pre_term).to_owned(),
            miscarriages: (a.miscarriages).to_owned(),
            living: (a.living).to_owned(),
            comments_pregnancy: (a.comments_pregnancy).to_owned(),
        })
    }
}
impl Delete {
    pub fn parse(a: &DeleteOBandPregnancy) -> Result<Delete> {
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
