use crate::dto::immunization::administer_types::Brand;
use chrono::{DateTime, Utc};
use common::dto::types::Types;
use doctor::dto::doctor_type::DoctorType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AdministerCommand {
    CreateAdminister(CreateAdminister),
    UpdateAdminister(UpdateAdminister),
    DeleteAdminister(DeleteAdminister),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateAdminister {
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
pub struct UpdateAdminister {
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
pub struct DeleteAdminister {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub patient_id: String,
}
