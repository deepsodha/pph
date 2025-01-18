use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Default, Validate, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/GovInfoInput.ts")]

pub struct GovInfoInput {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id_no: String,
    pub id_type: GovIdType,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, TS, Copy, Eq, sqlx::Type)]
#[ts(export, export_to = "../../bindings/GovIdType.ts")]
pub enum GovIdType {
    #[default]
    AadhaarCard,
    DrivingLicense,
    Passport,
}

impl GovIdType {
    pub fn as_str(&self) -> &'static str {
        match self {
            GovIdType::AadhaarCard => "AadhaarCard",
            GovIdType::DrivingLicense => "DrivingLicense",
            GovIdType::Passport => "Passport",
        }
    }
}
