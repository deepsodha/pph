use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, Default, PartialEq, Deserialize, TS, Copy, Eq, sqlx::Type)]
#[ts(export, export_to = "../../bindings/VisitType.ts")]
pub enum VisitType {
    #[default]
    SickVisit,
    RegularVisit,
}
impl VisitType {
    pub fn as_str(&self) -> &'static str {
        match self {
            VisitType::SickVisit => "SickVisit",
            VisitType::RegularVisit => "RegularVisit",
        }
    }
}
