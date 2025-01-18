use crate::method::convention::{ErrorData, Request};
use crate::rpc::create_request;
use crate::rpc::Rpc;
use appointment::dto::appointment_add::AppointmentAdd;
use appointment::dto::appointment_delete::AppointmentDelete;
use appointment::dto::appointment_update::AppointmentUpdate;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum AppointmentMethods {
    Add(AppointmentAdd, Option<String>),
    Update(AppointmentUpdate, Option<String>),
    Delete(AppointmentDelete, Option<String>),
}

impl Rpc for AppointmentMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        //Add as str
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Delete" => {
                    let input = serde_json::from_value::<AppointmentDelete>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(AppointmentMethods::Delete(input, None))
                }
                "Add" => {
                    let input = serde_json::from_value::<AppointmentAdd>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(AppointmentMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<AppointmentUpdate>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(AppointmentMethods::Update(input, None))
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
            AppointmentMethods::Delete(input, _) => create_request(namespace, "Delete", input),

            AppointmentMethods::Add(input, _) => create_request(namespace, "Add", input),

            AppointmentMethods::Update(input, _) => create_request(namespace, "Update", input),
        }
    }
}
