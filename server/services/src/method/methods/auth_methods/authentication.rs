use crate::{
    method::convention::{ErrorData, Request},
    rpc::{create_request, Rpc},
};
use auth::dto::login_mobile::LoginMobile;
use serde_json::Value;

#[derive(PartialEq, Eq, Debug)]
pub enum AuthMethods {
    LoginMobile(LoginMobile),
}

impl Rpc for AuthMethods {
    fn from_name(str: &str, data: Vec<Value>) -> Result<Self, ErrorData>
    where
        Self: Sized,
    {
        //Add as str
        let names = str.split("::").collect::<Vec<_>>();
        let res = if let Some((first, _elements)) = names.split_first() {
            match *first {
                "LoginMobile" => {
                    let login_mobile = serde_json::from_value::<LoginMobile>(data[0].clone())
                        .map_err(|_| ErrorData::std(-32601))?;
                    Ok(AuthMethods::LoginMobile(login_mobile))
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
            AuthMethods::LoginMobile(login_mobile) => {
                create_request(namespace, "LoginMobile", login_mobile)
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn to_rpc() {
//         const AUTH: &'static str = "Auth";
//         let loginmobile = AuthMethods::LoginMobile.to_rpc(&AUTH,_);
//         assert_eq!(loginmobile.method, "Auth::LoginMobile");
//     }

//     #[test]
//     fn from_name() {
//         let add = AuthMethods::from_name("LoginMobile", vec![Value::Null]);
//         assert!(add.is_ok());
//         assert_eq!(add.unwrap(), AuthMethods::LoginMobile(_,_));
//     }
// }
