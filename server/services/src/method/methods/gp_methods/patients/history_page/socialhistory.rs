use crate::method::convention::{ErrorData, Request};
use crate::rpc::{create_request, Rpc};
use patient::dto::history::social_history::socialhistory_add::SocialHistoryAdd;
use patient::dto::history::social_history::socialhistory_delete::SocialHistoryDelete;
use patient::dto::history::social_history::socialhistory_update::SocialHistoryUpdate;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum SocialHistoryMethods {
    Add(SocialHistoryAdd),
    Update(SocialHistoryUpdate),
    Delete(SocialHistoryDelete),
}

impl Rpc for SocialHistoryMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<SocialHistoryAdd>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(SocialHistoryMethods::Add(input))
                }
                "Update" => {
                    let input = serde_json::from_value::<SocialHistoryUpdate>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(SocialHistoryMethods::Update(input))
                }
                "Delete" => {
                    let input = serde_json::from_value::<SocialHistoryDelete>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(SocialHistoryMethods::Delete(input))
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
            SocialHistoryMethods::Add(input) => create_request(namespace, "Add", input),

            SocialHistoryMethods::Update(input) => create_request(namespace, "Update", input),

            SocialHistoryMethods::Delete(input) => create_request(namespace, "Delete", input),
        }
    }
}
