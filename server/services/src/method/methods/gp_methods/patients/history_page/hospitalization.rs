use crate::method::convention::{ErrorData, Request};
use crate::rpc::{create_request, Rpc};
use patient::dto::history::hospitalization::hospitalization_add::HospitalizationAdd;
use patient::dto::history::hospitalization::hospitalization_delete::HospitalizationDelete;
use patient::dto::history::hospitalization::hospitalization_update::HospitalizationUpdate;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum HospitalizationMethods {
    Add(HospitalizationAdd),
    Update(HospitalizationUpdate),
    Delete(HospitalizationDelete),
}

impl Rpc for HospitalizationMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<HospitalizationAdd>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(HospitalizationMethods::Add(input))
                }
                "Update" => {
                    let input = serde_json::from_value::<HospitalizationUpdate>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(HospitalizationMethods::Update(input))
                }
                "Delete" => {
                    let input = serde_json::from_value::<HospitalizationDelete>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(HospitalizationMethods::Delete(input))
                }

                _ => Err(ErrorData::std(-32601)),
            }
        } else {
            Err(ErrorData::std(-32601))
        };

        res
    }

    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        match self {
            HospitalizationMethods::Add(input) => create_request(namespace, "Add", input),

            HospitalizationMethods::Update(input) => create_request(namespace, "Update", input),

            HospitalizationMethods::Delete(input) => create_request(namespace, "Delete", input),
        }
    }
}
