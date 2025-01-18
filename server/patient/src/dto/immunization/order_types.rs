use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

use common::dto::{last_updated_input::LastUpdatedInput, types::Types};
#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/OrderAdd.ts")]

pub struct OrderAdd {
    pub patient_id: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub vaccine: String,
    pub types: Types,
    pub ordered: DateTime<Utc>,
    pub provider: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub comments: String,
    pub last_updated_input: LastUpdatedInput,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/OrderUpdate.ts")]
pub struct OrderUpdate {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
    pub input: OrderAdd,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/OrderDelete.ts")]
pub struct OrderDelete {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub patient_id: String,
    pub last_updated_input: LastUpdatedInput,
}
