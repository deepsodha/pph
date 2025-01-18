use crate::method::convention::{ErrorData, Request};
use crate::rpc::{create_request, Rpc};
use patient::dto::history::familyhistory::{
    familyhistory_add::FamilyHistoryAdd, familyhistory_delete::FamilyHistoryDelete,
    familyhistory_update::FamilyHistoryUpdate,
};
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum FamilyHistoryMethods {
    Add(FamilyHistoryAdd),
    Update(FamilyHistoryUpdate),
    Delete(FamilyHistoryDelete),
}

impl Rpc for FamilyHistoryMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        //Add as str
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<FamilyHistoryAdd>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(FamilyHistoryMethods::Add(input))
                }
                "Update" => {
                    let input = serde_json::from_value::<FamilyHistoryUpdate>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(FamilyHistoryMethods::Update(input))
                }
                "Delete" => {
                    let input = serde_json::from_value::<FamilyHistoryDelete>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(FamilyHistoryMethods::Delete(input))
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
            FamilyHistoryMethods::Add(input) => create_request(namespace, "Add", input),

            FamilyHistoryMethods::Update(input) => create_request(namespace, "Update", input),

            FamilyHistoryMethods::Delete(input) => create_request(namespace, "Delete", input),
        }
    }
}
