use crate::method::convention::{ErrorData, Request};
use crate::rpc::create_request;
use crate::rpc::Rpc;
use patient::dto::immunization::administer_types::{
    AdministerAdd, AdministerDelete, AdministerUpdate,
};
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum AddministerMethods {
    Add(AdministerAdd, Option<String>),
    Update(AdministerUpdate, Option<String>),
    Delete(AdministerDelete, Option<String>),
}

impl Rpc for AddministerMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        //Add as str
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<AdministerAdd>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(AddministerMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<AdministerUpdate>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(AddministerMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<AdministerDelete>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(AddministerMethods::Delete(input, None))
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
            AddministerMethods::Add(input, _) => create_request(namespace, "Add", input),

            AddministerMethods::Update(input, _) => create_request(namespace, "Update", input),

            AddministerMethods::Delete(input, _) => create_request(namespace, "Delete", input),
        }
    }
}
