use crate::method::convention::{ErrorData, Request};
use crate::rpc::create_request;
use crate::rpc::Rpc;
use doctor::dto::doctor_add::DoctorAdd;
use doctor::dto::doctor_delete::DoctorDelete;
use doctor::dto::doctor_update::DoctorUpdate;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum DoctorMethods {
    Add(DoctorAdd, Option<String>),
    Update(DoctorUpdate, Option<String>),
    Delete(DoctorDelete, Option<String>),
}

impl Rpc for DoctorMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        //Add as str
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Delete" => {
                    let input = serde_json::from_value::<DoctorDelete>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(DoctorMethods::Delete(input, None))
                }
                "Add" => {
                    let input = serde_json::from_value::<DoctorAdd>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(DoctorMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<DoctorUpdate>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(DoctorMethods::Update(input, None))
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
            DoctorMethods::Delete(input, _) => create_request(namespace, "Delete", input),

            DoctorMethods::Add(input, _) => create_request(namespace, "Add", input),

            DoctorMethods::Update(input, _) => create_request(namespace, "Update", input),
        }
    }
}
