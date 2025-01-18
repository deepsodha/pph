use chrono::{DateTime, Utc};
use common::dto::{last_updated_input::LastUpdatedInput, types::Types};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/NotAdministeredAdd.ts")]
pub struct NotAdministeredAdd {
    pub patient_id: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub vaccine: String,
    pub types: Types,
    pub recorded: DateTime<Utc>,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub reason_for_non_administration: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub comments: String,
    pub last_updated_input: LastUpdatedInput,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/NotAdministeredUpdate.ts")]
pub struct NotAdministeredUpdate {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
    pub input: NotAdministeredAdd,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/NotAdministeredDelete.ts")]
pub struct NotAdministeredDelete {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub patient_id: String,
    pub last_updated_input: LastUpdatedInput,
}
