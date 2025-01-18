use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Default, Validate, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/AddressDelete.ts")]

pub struct AddressDelete {
    pub id: String,
}
