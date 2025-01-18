use crate::method::convention::ErrorData;
use ml::medicine_prediction::get_medicine_prediction;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Clone, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/DiseaseNames.ts")]
pub struct DiseaseNames {
    pub name: Vec<String>,
}

pub async fn medicine_prediction(params: DiseaseNames) -> Result<Value, ErrorData> {
    // let user_input = vec!["Tuber Culosis".to_string(), "Allergic rhinitis".to_string()];
    let result = get_medicine_prediction(params.name);

    println!("result: {:#?}", result);
    serde_json::to_value(result).map_err(ErrorData::from)
}
