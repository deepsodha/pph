use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;
#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/Address.ts")]
pub struct Address {
    pub pin_code: String,
    pub city: String,
    pub state: String,
    pub address_line: String,
    pub country: String,
}

#[derive(
    Deserialize, Serialize, Debug, Clone, Default, Validate, TS, PartialEq, Eq, sqlx::FromRow,
)]
#[ts(export, export_to = "../../bindings/AddressInput.ts")]
pub struct AddressInput {
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
