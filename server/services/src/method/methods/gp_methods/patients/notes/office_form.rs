use crate::method::convention::{ErrorData, Request};
use crate::rpc::create_request;
use crate::rpc::Rpc;
use patient::dto::notes::office_form::office_form_add::OfficeFormAdd;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum OfficeFormMethods {
    Add(OfficeFormAdd, Option<String>),
}

impl Rpc for OfficeFormMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        //Add as str
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<OfficeFormAdd>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(OfficeFormMethods::Add(input, None))
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
            OfficeFormMethods::Add(input, _) => create_request(namespace, "Add", input),
        }
    }
}
