use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, Default, Serialize, Debug, Clone, PartialEq, TS, Copy, Eq, sqlx::Type)]
#[ts(export, export_to = "../../bindings/GenderType.ts")]
pub enum GenderType {
    #[default]
    Male,
    Female,
    Other,
    Unknown,
}

impl GenderType {
    pub fn as_str(&self) -> &'static str {
        match self {
            GenderType::Male => "Male",
            GenderType::Female => "Female",
            GenderType::Other => "Other",
            GenderType::Unknown => "UnKnown",
        }
    }
}
