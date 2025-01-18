use crate::method::convention::{ErrorData, Request};
use crate::rpc::create_request;
use crate::rpc::Rpc;
use serde_json::Value;
use user::dto::user_add::UserAdd;
use user::dto::user_delete::UserDelete;
use user::dto::user_password::UserPassword;
use user::dto::user_update::UserUpdate;

#[derive(PartialEq, Eq, Debug)]
pub enum UserMethods {
    Add(UserAdd, Option<String>),
    Update(UserUpdate, Option<String>),
    Delete(UserDelete, Option<String>),
    Password(UserPassword, Option<String>),
    ChangePassword(UserPassword, Option<String>),
}
impl Rpc for UserMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "Add" => {
                    let input = serde_json::from_value::<UserAdd>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(UserMethods::Add(input, None))
                }
                "Update" => {
                    let input = serde_json::from_value::<UserUpdate>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(UserMethods::Update(input, None))
                }
                "Delete" => {
                    let input = serde_json::from_value::<UserDelete>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(UserMethods::Delete(input, None))
                }
                "Password" => {
                    let input = serde_json::from_value::<UserPassword>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(UserMethods::Password(input, None))
                }
                "ChangePassword" => {
                    let input = serde_json::from_value::<UserPassword>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(UserMethods::ChangePassword(input, None))
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
            UserMethods::Add(input, _) => create_request(namespace, "Add", input),
            UserMethods::Update(input, _) => create_request(namespace, "Update", input),
            UserMethods::Delete(input, _) => create_request(namespace, "Delete", input),
            UserMethods::Password(input, _) => create_request(namespace, "Password", input),
            UserMethods::ChangePassword(input, _) => {
                create_request(namespace, "ChangePassword", input)
            }
        }
    }
}
