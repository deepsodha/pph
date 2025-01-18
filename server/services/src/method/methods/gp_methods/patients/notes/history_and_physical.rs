use crate::method::convention::{ErrorData, Request};
use crate::rpc::create_request;
use crate::rpc::Rpc;
use patient::dto::notes::history_and_physical::history_and_physical_add::HistoryAndPhysicalAdd;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum HistoryAndPhysicalMethods {
    Add(HistoryAndPhysicalAdd, Option<String>),
}

impl Rpc for HistoryAndPhysicalMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        //Add as str
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<HistoryAndPhysicalAdd>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(HistoryAndPhysicalMethods::Add(input, None))
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
            HistoryAndPhysicalMethods::Add(input, _) => create_request(namespace, "Add", input),
        }
    }
}
