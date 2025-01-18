use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, Default, Serialize, Debug, Clone, PartialEq, TS, Copy, Eq, sqlx::Type)]
#[ts(export, export_to = "../../bindings/AllergySeveritiesType.ts")]
pub enum AllergySeveritiesType {
    #[default]
    Mild,
    Moderate,
    Severe,
}

impl AllergySeveritiesType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AllergySeveritiesType::Mild => "Mild",
            AllergySeveritiesType::Moderate => "Moderate",
            AllergySeveritiesType::Severe => "Severe",
        }
    }
}
