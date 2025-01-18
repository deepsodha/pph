use crate::method::convention::{ErrorData, Request};
use crate::rpc::create_request;
use crate::rpc::Rpc;
use serde_json::Value;
use servicelocation::dto::select_servicelocation::SelectServiceLocation;
use servicelocation::dto::servicelocation_add::ServiceLocationAdd;
use servicelocation::dto::servicelocation_delete::ServiceLocationDelete;
use servicelocation::dto::servicelocation_update::ServiceLocationUpdate;

#[derive(PartialEq, Eq, Debug)]
pub enum ServiceLocationMethods {
    Add(ServiceLocationAdd, Option<String>),
    Update(ServiceLocationUpdate, Option<String>),
    Delete(ServiceLocationDelete, Option<String>),
    Select(SelectServiceLocation, Option<String>),
    GetAll,
}
impl Rpc for ServiceLocationMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<ServiceLocationAdd>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(ServiceLocationMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<ServiceLocationUpdate>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(ServiceLocationMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<ServiceLocationDelete>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(ServiceLocationMethods::Delete(input, None))
                }
                "Select" => {
                    let input = serde_json::from_value::<SelectServiceLocation>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(ServiceLocationMethods::Select(input, None))
                }
                "GetAll" => Ok(ServiceLocationMethods::GetAll),

                _ => Err(ErrorData::std(-32601)),
            }
        } else {
            Err(ErrorData::std(-32601))
        };

        res
    }

    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        match self {
            ServiceLocationMethods::Add(input, _) => create_request(namespace, "Add", input),
            ServiceLocationMethods::Update(input, _) => create_request(namespace, "Update", input),
            ServiceLocationMethods::Delete(input, _) => create_request(namespace, "Delete", input),
            ServiceLocationMethods::Select(input, _) => create_request(namespace, "Select", input),
            ServiceLocationMethods::GetAll => create_request(namespace, "GetAll", Value::Null),
        }
    }
}
