use chrono::{DateTime, Utc};
use common::dto::gender::GenderType;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct CreateSocialHistory {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub birth_gender: GenderType,
    pub tobacco: Option<Vec<String>>,
    pub alcohol: Option<Vec<String>>,
    pub cardiovascular: Option<Vec<String>>,
    pub sexual_activity: Option<Vec<String>>,
    pub drug_abuse: Option<Vec<String>>,
    pub safety: Option<Vec<String>>,
    pub comments: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct UpdateSocialHistory {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub birth_gender: GenderType,
    pub tobacco: Option<Vec<String>>,
    pub alcohol: Option<Vec<String>>,
    pub cardiovascular: Option<Vec<String>>,
    pub sexual_activity: Option<Vec<String>>,
    pub drug_abuse: Option<Vec<String>>,
    pub safety: Option<Vec<String>>,
    pub comments: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct DeleteSocialHistory {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SocialHistoryCommand {
    CreateSocialHistory(CreateSocialHistory),
    UpdateSocialHistory(UpdateSocialHistory),
    DeleteSocialHistory(DeleteSocialHistory),
}
