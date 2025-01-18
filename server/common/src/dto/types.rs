use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, TS, Eq, sqlx::Type)]
#[ts(export, export_to = "../../bindings/Types.ts")]
pub enum Types {
    #[default]
    Type1,
    Type2,
}

impl Types {
    pub fn as_str(&self) -> String {
        match self {
            Types::Type1 => String::from("Type1"),
            Types::Type2 => String::from("Type2"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, TS, Eq, sqlx::Type)]
#[ts(export, export_to = "../../bindings/ProblemTypes.ts")]
pub enum ProblemTypes {
    #[default]
    Acute,
    Chronic,
}

impl ProblemTypes {
    pub fn as_str(&self) -> String {
        match self {
            ProblemTypes::Acute => String::from("Acute"),
            ProblemTypes::Chronic => String::from("Chronic"),
        }
    }
}
