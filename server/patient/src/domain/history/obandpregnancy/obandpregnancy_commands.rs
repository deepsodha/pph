use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct CreateOBandPregnancy {
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

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct UpdateOBandPregnancy {
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

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct DeleteOBandPregnancy {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OBandPregnancyCommand {
    CreateOBandPregnancy(CreateOBandPregnancy),
    UpdateOBandPregnancy(UpdateOBandPregnancy),
    DeleteOBandPregnancy(DeleteOBandPregnancy),
}
