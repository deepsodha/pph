use crate::method::convention::{ErrorData, Request};
use crate::rpc::create_request;
use crate::rpc::Rpc;
use patient::dto::immunization::not_administered_types::{
    NotAdministeredAdd, NotAdministeredDelete, NotAdministeredUpdate,
};
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum NotaddministerMethods {
    Add(NotAdministeredAdd, Option<String>),
    Update(NotAdministeredUpdate, Option<String>),
    Delete(NotAdministeredDelete, Option<String>),
}

impl Rpc for NotaddministerMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        //Add as str
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<NotAdministeredAdd>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(NotaddministerMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<NotAdministeredUpdate>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(NotaddministerMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<NotAdministeredDelete>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(NotaddministerMethods::Delete(input, None))
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
            NotaddministerMethods::Add(input, _) => create_request(namespace, "Add", input),

            NotaddministerMethods::Update(input, _) => create_request(namespace, "Update", input),

            NotaddministerMethods::Delete(input, _) => create_request(namespace, "Delete", input),
        }
    }
}
