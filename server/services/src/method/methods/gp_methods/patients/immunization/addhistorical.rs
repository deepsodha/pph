use crate::method::convention::{ErrorData, Request};
use crate::rpc::create_request;
use crate::rpc::Rpc;
use patient::dto::immunization::add_historical_types::{
    HistoricalAdd, HistoricalDelete, HistoricalUpdate,
};
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum AddhisitoricalMethods {
    Add(HistoricalAdd, Option<String>),
    Update(HistoricalUpdate, Option<String>),
    Delete(HistoricalDelete, Option<String>),
}

impl Rpc for AddhisitoricalMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        //Add as str
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<HistoricalAdd>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(AddhisitoricalMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<HistoricalUpdate>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(AddhisitoricalMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<HistoricalDelete>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(AddhisitoricalMethods::Delete(input, None))
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
            AddhisitoricalMethods::Add(input, _) => create_request(namespace, "Add", input),

            AddhisitoricalMethods::Update(input, _) => create_request(namespace, "Update", input),

            AddhisitoricalMethods::Delete(input, _) => create_request(namespace, "Delete", input),
        }
    }
}
