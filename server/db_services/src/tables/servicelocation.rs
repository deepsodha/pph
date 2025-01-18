use serde::{Deserialize, Serialize};
use sqlx;
use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, Eq, sqlx::FromRow, TS)]
#[ts(export, export_to = "../../bindings/GetServiceLocation.ts")]
pub struct GetServiceLocation {
    pub id: String,
    pub org_id: String,
    pub org_name: String,
    pub service_location_name: String,
    pub pin_code: String,
    pub city: String,
    pub state: String,
    pub address_line: String,
    pub country: String,
}
