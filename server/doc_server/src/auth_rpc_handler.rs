use actix_web::web;
use actix_web::web::Bytes;
use actix_web::{Error, HttpResponse};
use log::info;
use serde_json::Value;
use services::apis::auth_apis::rpc_select::rpc_select;
use services::app_state::AppState;
use services::method::convention;
use services::method::methods::auth_methods::auth_app_methods::AuthAppMethods;
use services::rpc::Rpc;

/// The main handler for JSONRPC server.
pub async fn auth_rpc_handler(
    body: Bytes,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let req_json: convention::Request = match serde_json::from_slice(body.as_ref()) {
        Ok(ok) => ok,
        Err(_) => {
            info!(target : "rpc_handler", "in rpc handler error");
            let r = convention::Response {
                jsonrpc: String::from(convention::JSONRPC_VERSION),
                result: Value::Null,
                error: Some(convention::ErrorData::std(-32700)),
                id: Value::Null,
            };
            return Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(r.dump()));
        }
    };
    let mut result = convention::Response {
        id: req_json.id.clone(),
        ..convention::Response::default()
    };
    let methods = AuthAppMethods::from_name(req_json.method.as_str(), req_json.params);
    match methods {
        Ok(methods) => match rpc_select(&app_state, methods).await {
            Ok(ok) => result.result = ok,
            Err(e) => result.error = Some(e),
        },
        Err(e) => result.error = Some(e),
    }
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(result.dump()))
}
