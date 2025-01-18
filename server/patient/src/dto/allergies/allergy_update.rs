use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

use super::allergy_add::AllergyAdd;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/AllergyUpdate.ts")]
pub struct AllergyUpdate {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
    pub input: AllergyAdd,
}
