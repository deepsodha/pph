use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "../../bindings/Drugs.ts")]
pub struct Drugs {
    pub brand: String,
    pub generic: String,
    pub details: String,
    pub category: String,
    pub side_effects: String,
    pub drugsdose_info: String,
    pub precautions: String,
    pub manufacturer_name: String,
    pub medicines: String,
    pub contra_indications: String,
    pub diseases: String,
    pub interactions: String,
    pub contains: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "../../bindings/GetDrugs.ts")]
pub struct GetDrugs {
    pub id: String,
    pub brand: String,
    pub generic: String,
    pub details: String,
    pub category: String,
    pub side_effects: String,
    pub drugsdose_info: String,
    pub precautions: String,
    pub manufacturer_name: String,
    pub medicines: String,
    pub contra_indications: String,
    pub diseases: String,
    pub interactions: String,
    pub contains: String,
    pub is_deleted: bool,
}
