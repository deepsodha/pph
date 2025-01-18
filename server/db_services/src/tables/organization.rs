use serde::{Deserialize, Serialize};
use sqlx;
use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, Eq, sqlx::FromRow, TS)]
#[ts(export, export_to = "../../bindings/Getorganization.ts")]
pub struct Getorganization {
    pub id: String,
    pub name: String,
    pub details: String,
    pub mobile_number: String,
    pub email: String,
    pub pin_code: String,
    pub city: String,
    pub state: String,
    pub address_line: String,
    pub country: String,
}
