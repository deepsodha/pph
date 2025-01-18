use crate::method::convention::{ErrorData, Request};
use crate::rpc::{create_request, Rpc};
use patient::dto::history::pastsurgical_history::pastsurgicalhistory_add::PastSurgicalHistoryAdd;
use patient::dto::history::pastsurgical_history::pastsurgicalhistory_delete::PastSurgicalHistoryDelete;
use patient::dto::history::pastsurgical_history::pastsurgicalhistory_update::PastSurgicalHistoryUpdate;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum PastSurgicalHistoryMethods {
    Add(PastSurgicalHistoryAdd),
    Update(PastSurgicalHistoryUpdate),
    Delete(PastSurgicalHistoryDelete),
}

impl Rpc for PastSurgicalHistoryMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        //Add as str
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<PastSurgicalHistoryAdd>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(PastSurgicalHistoryMethods::Add(input))
                }
                "Update" => {
                    let input =
                        serde_json::from_value::<PastSurgicalHistoryUpdate>(data[0].clone())
                            .map_err(|_| ErrorData::std(-32601))?;
                    Ok(PastSurgicalHistoryMethods::Update(input))
                }
                "Delete" => {
                    let input =
                        serde_json::from_value::<PastSurgicalHistoryDelete>(data[0].clone())
                            .map_err(|_| ErrorData::std(-32601))?;
                    Ok(PastSurgicalHistoryMethods::Delete(input))
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
            PastSurgicalHistoryMethods::Add(input) => create_request(namespace, "Add", input),

            PastSurgicalHistoryMethods::Update(input) => create_request(namespace, "Update", input),

            PastSurgicalHistoryMethods::Delete(input) => create_request(namespace, "Delete", input),
        }
    }
}
