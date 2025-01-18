use anyhow::{bail, Result};
use serde::Deserialize;
use services::method::convention::{Request as JSONRpcReq, Response};
use services::method::methods::gp_methods::app_methods::AppMethods;
use services::rpc::Rpc;

pub const URL: &str = "http://localhost:5000/api";

pub async fn process(url: &str, req: &JSONRpcReq) -> Result<Response> {
    let client = reqwest::Client::new();
    let r = client.post(url).json(req).send().await;
    match r {
        Ok(r) => {
            if r.status().is_success() {
                let resp: Response = r.json().await.unwrap();
                Ok(resp)
            } else {
                bail!(r.text().await.unwrap())
            }
        }
        Err(e) => bail!(format!("{:?}", e)),
    }
}

pub fn json_rpc_to_result<T: for<'de> Deserialize<'de> + Clone>(
    input: &Response,
) -> Result<T, String> {
    match &input.error {
        Some(e) => Err(format!("{:?}", e)),
        None => serde_json::from_value::<T>(input.result.clone())
            .map_err(|_| "Serialization error".to_string()),
    }
}

pub async fn services_helper<T: for<'de> Deserialize<'de> + Clone>(
    method: AppMethods,
    url: &str,
) -> Result<T, String> {
    let req = method.to_rpc("AppMethods").unwrap();
    let res: Response = process(url, &req).await.unwrap();
    json_rpc_to_result::<T>(&res)
}
