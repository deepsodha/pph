use chrono::{DateTime, Utc};
use common::dto::{last_updated_input::LastUpdatedInput, types::Types};
use doctor::dto::doctor_type::DoctorType;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/AdministerAdd.ts")]
pub struct AdministerAdd {
    pub patient_id: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub vaccine: String,
    pub types: Types,
    pub brand: Brand,
    pub generic: String,
    pub ordered: DateTime<Utc>,
    pub recorded: DateTime<Utc>,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub dose: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub site: String,
    #[validate(range(min = 1, message = "field can't be empty"))]
    pub number_of_serial: i32,
    pub lot: i32,
    pub expiration: DateTime<Utc>,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub consent_obtain: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub administrated_by: String,
    pub clinic_location: String,
    pub provider: DoctorType,
    pub vis_date: DateTime<Utc>,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub vfs_financial_class: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub comments: String,
    pub last_updated_input: LastUpdatedInput,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/AdministerUpdate.ts")]
pub struct AdministerUpdate {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
    pub input: AdministerAdd,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/AdministerDelete.ts")]
pub struct AdministerDelete {
    pub id: String,
    pub patient_id: String,
    pub last_updated_input: LastUpdatedInput,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, TS, Eq, sqlx::Type)]
#[ts(export, export_to = "../../bindings/Brand.ts")]
pub enum Brand {
    #[default]
    Brand1,
    Brand2,
}

impl Brand {
    pub fn as_str(&self) -> String {
        match self {
            Brand::Brand1 => String::from("Brand1"),
            Brand::Brand2 => String::from("Brand2"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, TS, Eq, sqlx::Type)]
#[ts(export, export_to = "../../bindings/Location.ts")]
pub enum Location {
    #[default]
    Office,
    Home,
}

impl Location {
    pub fn as_str(&self) -> String {
        match self {
            Location::Office => String::from("Office"),
            Location::Home => String::from("Home"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, TS, Eq)]
#[ts(export, export_to = "../../bindings/ConsentObtainedTypes.ts")]
pub enum ConsentObtainedTypes {
    #[default]
    Verbal,
    Written,
}

impl ConsentObtainedTypes {
    pub fn as_str(&self) -> String {
        match self {
            ConsentObtainedTypes::Verbal => String::from("Verbal"),
            ConsentObtainedTypes::Written => String::from("Written"),
        }
    }
}
