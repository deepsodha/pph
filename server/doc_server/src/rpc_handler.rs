use actix_web::error::ErrorUnauthorized;
use actix_web::web::{self, Bytes};
use actix_web::{Error, HttpResponse};
use biscuit_auth::Biscuit;
use log::info;
use serde_json::Value;
use services::apis::doc_apis::select_rpc::{rpc_select, RpcParams};
use services::app_state::AppState;
use services::method::convention;
use services::method::methods::gp_methods::app_methods::AppMethods;
use services::rpc::Rpc;
use utils::biscuit::get_details::{
    get_organization_detail, get_roles, get_service_location_details, get_user_detail,
};

pub async fn rpc_handler(
    body: Bytes,
    app_state: web::Data<AppState>,
    biscuit: web::ReqData<Biscuit>,
) -> Result<HttpResponse, Error> {
    let org_id = get_organization_detail(&biscuit);

    match org_id {
        Ok((id, org_name)) => {
            let service_location_id = get_service_location_details(&biscuit);
            match service_location_id {
                Ok((service_location_id, _service_location_name)) => {
                    let user = get_user_detail(&biscuit);
                    match user {
                        Ok((user_id, user_name)) => {
                            let req_json: convention::Request =
                                match serde_json::from_slice(body.as_ref()) {
                                    Ok(ok) => ok,
                                    Err(_) => {
                                        info!(target: "rpc_handler", "in rpc handler error");
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
                            let methods =
                                AppMethods::from_name(req_json.method.as_str(), req_json.params);
                            let roles: Vec<String> = match get_roles(&biscuit) {
                                Ok(r) => r,
                                Err(error) => {
                                    return Err(ErrorUnauthorized(error.to_string()));
                                }
                            };
                            match methods {
                                Ok(methods) => {
                                    match rpc_select(
                                        &app_state,
                                        RpcParams {
                                            methods,
                                            org_id: id.clone(),
                                            org_name,
                                            user_id,
                                            user_name,
                                            service_location_id,
                                            roles,
                                            feature_access: true, // TODO : will come from TOKEN
                                        },
                                    )
                                    .await
                                    {
                                        Ok(ok) => result.result = ok,
                                        Err(e) => result.error = Some(e),
                                    }
                                }
                                Err(e) => result.error = Some(e),
                            }
                            Ok(HttpResponse::Ok()
                                .content_type("application/json")
                                .body(result.dump()))
                        }
                        Err(error) => {
                            Ok(HttpResponse::InternalServerError().body(error.to_string()))
                        }
                    }
                }
                Err(error) => Ok(HttpResponse::InternalServerError().body(error.to_string())),
            }
        }
        Err(error) => Ok(HttpResponse::InternalServerError().body(error.to_string())),
    }
}
