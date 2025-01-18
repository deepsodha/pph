use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, TS, Eq, sqlx::Type)]
#[ts(export, export_to = "../../bindings/Status.ts")]
pub enum Status {
    #[default]
    Active,
    InActive,
    MarkAsError,
}

impl Status {
    pub fn as_str(&self) -> String {
        match self {
            Status::Active => String::from("Active"),
            Status::InActive => String::from("InActive"),
            Status::MarkAsError => String::from("MarkAsError"),
        }
    }
}
