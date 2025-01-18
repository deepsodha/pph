use crate::method::convention::{ErrorData, Request};
use crate::rpc::create_request;
use crate::rpc::Rpc;
use patient::dto::immunization::order_types::{OrderAdd, OrderDelete, OrderUpdate};
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum OrderMethods {
    Add(OrderAdd, Option<String>),
    Update(OrderUpdate, Option<String>),
    Delete(OrderDelete, Option<String>),
}

impl Rpc for OrderMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        //Add as str
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<OrderAdd>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(OrderMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<OrderUpdate>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(OrderMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<OrderDelete>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(OrderMethods::Delete(input, None))
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
            OrderMethods::Add(input, _) => create_request(namespace, "Add", input),

            OrderMethods::Update(input, _) => create_request(namespace, "Update", input),

            OrderMethods::Delete(input, _) => create_request(namespace, "Delete", input),
        }
    }
}
