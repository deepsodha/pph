use common::dto::{last_updated_input::LastUpdatedInput, status::Status};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

use super::allergy_severities::AllergySeveritiesType;

#[derive(Deserialize, Serialize, Debug, Clone, Default, Validate, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/AllergyAdd.ts")]
pub struct AllergyAdd {
    pub patient_id: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub allergen: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub reaction: String,
    pub allergy_severities: AllergySeveritiesType,
    pub input_date: String,
    pub status: Status,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub comments: String,
    pub last_updated_input: LastUpdatedInput,
}
