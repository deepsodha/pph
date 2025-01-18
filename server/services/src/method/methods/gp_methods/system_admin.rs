use crate::method::convention::{ErrorData, Request};
use crate::rpc::create_request;
use crate::rpc::Rpc;
use serde_json::Value;
use system_admin::dto::system_admin_add::SystemAdminAdd;
use system_admin::dto::system_admin_delete::SystemAdminDelete;
use system_admin::dto::system_admin_update::SystemAdminUpdate;

#[derive(PartialEq, Eq, Debug)]
pub enum SystemAdminMethods {
    Add(SystemAdminAdd, Option<String>),
    Update(SystemAdminUpdate, Option<String>),
    Delete(SystemAdminDelete, Option<String>),
    GetAll,
}

impl Rpc for SystemAdminMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        //Add as str
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<SystemAdminAdd>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(SystemAdminMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<SystemAdminUpdate>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(SystemAdminMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<SystemAdminDelete>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(SystemAdminMethods::Delete(input, None))
                }
                "GetAll" => Ok(SystemAdminMethods::GetAll),

                _ => Err(ErrorData::std(-32601)),
            }
        } else {
            Err(ErrorData::std(-32601))
        };

        res
    }

    fn to_rpc(&self, namespace: &str) -> Result<Request, ErrorData> {
        match self {
            SystemAdminMethods::Add(input, _) => create_request(namespace, "Add", input),
            SystemAdminMethods::Update(input, _) => create_request(namespace, "Update", input),
            SystemAdminMethods::Delete(input, _) => create_request(namespace, "Delete", input),
            SystemAdminMethods::GetAll => create_request(namespace, "GetAll", Value::Null),
        }
    }
}
