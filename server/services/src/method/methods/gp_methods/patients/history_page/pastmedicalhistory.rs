use crate::method::convention::{ErrorData, Request};
use crate::rpc::{create_request, Rpc};
use patient::dto::history::pastmedical_history::pastmedical_add::PastMedicalHistoryAdd;
use patient::dto::history::pastmedical_history::pastmedical_delete::PastMedicalHistoryDelete;
use patient::dto::history::pastmedical_history::pastmedical_update::PastMedicalHistoryUpdate;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum PastMedicalHistoryMethods {
    Add(PastMedicalHistoryAdd),
    Update(PastMedicalHistoryUpdate),
    Delete(PastMedicalHistoryDelete),
}

impl Rpc for PastMedicalHistoryMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        //Add as str
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<PastMedicalHistoryAdd>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(PastMedicalHistoryMethods::Add(input))
                }
                "Update" => {
                    let input = serde_json::from_value::<PastMedicalHistoryUpdate>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(PastMedicalHistoryMethods::Update(input))
                }
                "Delete" => {
                    let input = serde_json::from_value::<PastMedicalHistoryDelete>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(PastMedicalHistoryMethods::Delete(input))
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
            PastMedicalHistoryMethods::Add(input) => create_request(namespace, "Add", input),

            PastMedicalHistoryMethods::Update(input) => create_request(namespace, "Update", input),

            PastMedicalHistoryMethods::Delete(input) => create_request(namespace, "Delete", input),
        }
    }
}
