use crate::method::convention::{ErrorData, Request};
use crate::rpc::create_request;
use crate::rpc::Rpc;
use serde_json::Value;
use staff::dto::staff_add::StaffAdd;
use staff::dto::staff_delete::StaffDelete;
use staff::dto::staff_update::StaffUpdate;

#[derive(PartialEq, Eq, Debug)]
pub enum StaffMethods {
    Add(StaffAdd, Option<String>),
    Update(StaffUpdate, Option<String>),
    Delete(StaffDelete, Option<String>),
}
impl Rpc for StaffMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<StaffAdd>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(StaffMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<StaffUpdate>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(StaffMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<StaffDelete>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(StaffMethods::Delete(input, None))
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
            StaffMethods::Add(input, _) => create_request(namespace, "Add", input),
            StaffMethods::Update(input, _) => create_request(namespace, "Update", input),
            StaffMethods::Delete(input, _) => create_request(namespace, "Delete", input),
        }
    }
}
