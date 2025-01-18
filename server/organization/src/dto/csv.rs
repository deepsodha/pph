use serde::{Deserialize, Serialize};
use sqlx;
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/CsvRecord.ts")]
pub struct Location {
    pub pin_code: String,
    pub city_name: String,
    pub district: String,
    pub state_name: String,
    pub country_name: String,
}

#[derive(
    Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq, sqlx::FromRow,
)]
#[ts(export, export_to = "../../bindings/LocationRecord.ts")]
pub struct LocationRecord {
    pub pin_code: String,
    pub city_name: String,
    pub district: String,
    pub state_name: String,
    pub country_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/VaccineRecord.ts")]
pub struct VaccineRecord {
    pub vaccine: String,
    pub vaccine_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/Icd10Record.ts")]
pub struct Icd10Record {
    pub disease_code: String,
    pub disease_subtype: String,
    pub disease_subcode: String,
    pub disease_name: String,
    pub disease_description: String,
    pub disease_category: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/CombinedRecord.ts")]
pub struct CombinedRecord {
    pub brand: String,
    pub generic: String,
    pub details: String,
    pub category: String,
    pub side_effects: String,
    pub drug_dose_info: String,
    pub precautions: String,
    pub manufacturer_name: String,
    pub medicines: String,
    pub contra_indications: String,
    pub diseases: String,
    pub interactions: String,
    pub contains: String,
}

#[derive(
    Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq, sqlx::FromRow,
)]
#[ts(export, export_to = "../../bindings/PinCodeInput.ts")]
pub struct PinCodeInput {
    pub pin_code: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/AllergiesRecord.ts")]
pub struct AllergiesRecord {
    pub allergy: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/PhoneCode.ts")]
pub struct PhoneCode {
    pub phone_code: String,
}
