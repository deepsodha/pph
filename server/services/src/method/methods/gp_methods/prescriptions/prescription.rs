use crate::method::convention::{ErrorData, Request};
use crate::rpc::create_request;
use crate::rpc::Rpc;
use prescription::dto::prescription_add::PrescriptionAdd;
use prescription::dto::prescription_delete::PrescriptionDelete;
use prescription::dto::prescription_update::PrescriptionUpdate;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum PrescriptionMethods {
    Add(PrescriptionAdd, Option<String>),
    Update(PrescriptionUpdate, Option<String>),
    Delete(PrescriptionDelete, Option<String>),
}
impl Rpc for PrescriptionMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<PrescriptionAdd>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(PrescriptionMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<PrescriptionUpdate>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(PrescriptionMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<PrescriptionDelete>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(PrescriptionMethods::Delete(input, None))
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
            PrescriptionMethods::Add(input, _) => create_request(namespace, "Add", input),
            PrescriptionMethods::Update(input, _) => create_request(namespace, "Update", input),
            PrescriptionMethods::Delete(input, _) => create_request(namespace, "Delete", input),
        }
    }
}
