use crate::method::convention::{ErrorData, Request};
use crate::rpc::{create_request, Rpc};
use patient::dto::history::obandpregnancy::obandpregnancy_add::OBandPregnancyAdd;
use patient::dto::history::obandpregnancy::obandpregnancy_delete::OBandPregnancyDelete;
use patient::dto::history::obandpregnancy::obandpregnancy_update::OBandPregnancyUpdate;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum OBAndPregnancyMethods {
    Add(OBandPregnancyAdd),
    Update(OBandPregnancyUpdate),
    Delete(OBandPregnancyDelete),
}

impl Rpc for OBAndPregnancyMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        //Add as str
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<OBandPregnancyAdd>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(OBAndPregnancyMethods::Add(input))
                }
                "Update" => {
                    let input = serde_json::from_value::<OBandPregnancyUpdate>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(OBAndPregnancyMethods::Update(input))
                }
                "Delete" => {
                    let input = serde_json::from_value::<OBandPregnancyDelete>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(OBAndPregnancyMethods::Delete(input))
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
            OBAndPregnancyMethods::Add(input) => create_request(namespace, "Add", input),

            OBAndPregnancyMethods::Update(input) => create_request(namespace, "Update", input),

            OBAndPregnancyMethods::Delete(input) => create_request(namespace, "Delete", input),
        }
    }
}
