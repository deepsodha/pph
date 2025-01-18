use crate::method::convention::ErrorData;
use ml::medicine_prediction::get_disease_prediction;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Clone, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/SymptomsNames.ts")]
pub struct SymptomsNames {
    pub name: Vec<String>,
}

pub async fn disease_prediction(params: SymptomsNames) -> Result<Value, ErrorData> {
    let result = get_disease_prediction(params.name);

    println!("result: {:#?}", result);
    serde_json::to_value(result).map_err(ErrorData::from)
}
