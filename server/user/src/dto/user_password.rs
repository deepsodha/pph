use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/UserPassword.ts")]

pub struct UserPassword {
    pub id: String,
    pub password: String,
    pub confirm_password: String,
    pub last_updated_input: LastUpdatedInput,
}
