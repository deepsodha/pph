use crate::method::convention::{ErrorData, Request};
use crate::rpc::create_request;
use crate::rpc::Rpc;
use patient::dto::allergies::allergy_add::AllergyAdd;
use patient::dto::allergies::allergy_delete::AllergyDelete;
use patient::dto::allergies::allergy_update::AllergyUpdate;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum AllergyMethods {
    Add(AllergyAdd, Option<String>),
    Update(AllergyUpdate, Option<String>),
    Delete(AllergyDelete, Option<String>),
    Name,
}

impl Rpc for AllergyMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        //Add as str
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<AllergyAdd>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(AllergyMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<AllergyUpdate>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(AllergyMethods::Update(input, None))
                }
                "Name" => Ok(AllergyMethods::Name),
                "Delete" => {
                    let input = serde_json::from_value::<AllergyDelete>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(AllergyMethods::Delete(input, None))
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
            AllergyMethods::Add(input, _) => create_request(namespace, "Add", input),

            AllergyMethods::Update(input, _) => create_request(namespace, "Update", input),

            AllergyMethods::Delete(input, _) => create_request(namespace, "Delete", input),

            AllergyMethods::Name => create_request(namespace, "Name", Value::Null),
        }
    }
}
