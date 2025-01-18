use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, Default, Serialize, Debug, Clone, PartialEq, TS, Copy, Eq)]
#[ts(export, export_to = "../../bindings/HealthStauts.ts")]
pub enum HealthStauts {
    #[default]
    Alive,
    Deceased,
    Unknown,
}

impl HealthStauts {
    pub fn as_str(&self) -> &'static str {
        match self {
            HealthStauts::Alive => "Alive",
            HealthStauts::Deceased => "Deceased",
            HealthStauts::Unknown => "Unknown",
        }
    }
}
