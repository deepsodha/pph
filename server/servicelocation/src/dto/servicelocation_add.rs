use common::dto::{address::AddressInput, last_updated_input::LastUpdatedInput};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Default, Clone, validator::Validate, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/ServiceLocationAdd.ts")]
pub struct ServiceLocationAdd {
    pub service_location_name: String,
    pub address: AddressInput,
    pub start_time: String,
    pub end_time: String,
    pub last_updated_input: LastUpdatedInput,
}
