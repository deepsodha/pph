use crate::method::convention::{ErrorData, Request};
use crate::rpc::create_request;
use crate::rpc::Rpc;
use patient::dto::medication::medication_add::MedicationsAdd;
use patient::dto::medication::medication_delete::MedicationDelete;
use patient::dto::medication::medication_update::MedicationUpdate;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum MedicationsMethods {
    Add(MedicationsAdd, Option<String>),
    Update(MedicationUpdate, Option<String>),
    Delete(MedicationDelete, Option<String>),
}

impl Rpc for MedicationsMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        //Add as str
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<MedicationsAdd>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(MedicationsMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<MedicationUpdate>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(MedicationsMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<MedicationDelete>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(MedicationsMethods::Delete(input, None))
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
            MedicationsMethods::Add(input, _) => create_request(namespace, "Add", input),

            MedicationsMethods::Update(input, _) => create_request(namespace, "Update", input),
            MedicationsMethods::Delete(input, _) => create_request(namespace, "Delete", input),
        }
    }
}
