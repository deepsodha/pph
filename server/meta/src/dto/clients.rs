use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "../../bindings/Clients.ts")]
pub struct Clients {
    pub name: String,
    pub address: String,
    pub mobile_number: String,
    pub email: String,
    pub gst_no: String,
    pub pan_no: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "../../bindings/GetClients.ts")]
pub struct GetClients {
    pub id: String,
    pub name: String,
    pub address: String,
    pub mobile_number: String,
    pub email: String,
    pub gst_no: String,
    pub pan_no: String,
    pub is_deleted: bool,
}
