use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "../../bindings/AuthenticatedUser.ts")]
pub struct AuthenticatedUser {
    pub id: String,
    pub token: String,
    pub selected_language: String,
    pub role: Vec<String>,
    pub user_name: String,
    pub org_id: String,
    pub org_name: String,
    pub service_location_id: String,
}
