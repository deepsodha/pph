use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../../bindings/SearchQuery.ts")]
pub struct SearchQuery {
    pub query: String,
}
