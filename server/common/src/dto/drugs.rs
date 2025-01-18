use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "../../bindings/Drugs.ts")]
pub struct Drugs {
    #[serde(rename = "Brand")]
    pub brand: String,
    #[serde(rename = "Generic")]
    pub generic: String,
    #[serde(rename = "Details")]
    pub details: String,
    #[serde(rename = "Category")]
    pub category: String,
    #[serde(rename = "SideEffects")]
    pub side_effects: String,
    #[serde(rename = "DrugDoseInfo")]
    pub drug_dose_info: String,
    #[serde(rename = "Precautions")]
    pub precautions: String,
    #[serde(rename = "ManufacturerName")]
    pub manufacturer_name: String,
    #[serde(rename = "Medicines")]
    pub medicines: String,
    #[serde(rename = "ContraIndications")]
    pub contra_indications: String,
    #[serde(rename = "Diseases")]
    pub diseases: String,
    #[serde(rename = "Interactions")]
    pub interactions: String,
    #[serde(rename = "Contains")]
    pub contains: String,
    pub id: i64,
}
