use crate::method::convention::{ErrorData, Request};
use crate::rpc::create_request;
use crate::rpc::Rpc;
use patient::dto::vital::vital_add::VitalsAdd;
use patient::dto::vital::vital_delete::VitalsDelete;
use patient::dto::vital::vital_update::VitalsUpdate;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum VitalsMethods {
    Add(VitalsAdd, Option<String>),
    Update(VitalsUpdate, Option<String>),
    Delete(VitalsDelete, Option<String>),
}

impl Rpc for VitalsMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<VitalsAdd>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(VitalsMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<VitalsUpdate>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(VitalsMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<VitalsDelete>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(VitalsMethods::Delete(input, None))
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
            VitalsMethods::Add(input, _) => create_request(namespace, "Add", input),

            VitalsMethods::Update(input, _) => create_request(namespace, "Update", input),

            VitalsMethods::Delete(input, _) => create_request(namespace, "Delete", input),
        }
    }
}
