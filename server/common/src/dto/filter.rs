use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/Filter.ts")]

pub struct Filter {
    pub page_index: Option<usize>,
    pub page_size: Option<usize>,
}
