use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Default, Validate, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/AddressUpdate.ts")]

pub struct AddressUpdate {
    pub id: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub pin_code: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub city: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub state: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub address_line: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub country: String,
}
