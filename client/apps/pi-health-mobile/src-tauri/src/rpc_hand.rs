use biscuit_auth::Biscuit;
use biscuit_auth::PublicKey;
use dotenv::var;
use log::info;
use serde_json::Value;
use services::apis::gp_apis::select_rpc::{rpc_select, RpcParams};
use services::app_state::AppState;
use services::method::convention;
use services::method::methods::gp_methods::app_methods::AppMethods;
use tauri::{Error, State};
use utils::biscuit::get_details::get_organization_detail;
use utils::biscuit::get_details::get_service_location_details;
use utils::biscuit::get_details::get_user_detail; 
use utils::biscuit::get_details::get_roles;
use services::rpc::Rpc;

#[tauri::command]
pub async fn rpc_handler(
    body: String,
    token: String,
    app_state: State<'_, AppState>,
) -> Result<String, Error> {
    println!("inside rpc");
    println!("Token: {:?}", token);

    let public_key =
        PublicKey::from_bytes_hex(&var("PUBLIC_KEY").expect("Public is not set in .env file"))
            .expect("Failed to parse public key");
    println!("Public key: {:?}", public_key);

    let biscuit = Biscuit::from_base64(token, public_key).unwrap();
    println!("biscuit: {:?}", biscuit);

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
                                        return Ok(r.dump());
                                    }
                                };
                            let mut result = convention::Response {
                                id: req_json.id.clone(),
                                ..convention::Response::default()
                            };
                            let methods = AppMethods::from_name(req_json.method.as_str(), req_json.params);
                            let roles: Vec<String> = match get_roles(&biscuit) {
                                Ok(r) => r,
                                Err(error) => {
                                    return Err(Error::AssetNotFound(error.to_string()));
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
                                            feature_access: true,
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
                            Ok(result.dump())
                        }
                        Err(e) => Err(Error::AssetNotFound(e.to_string())),
                    }
                }
                Err(e) => Err(Error::AssetNotFound(e.to_string())),
            }
        }
        Err(e) => Err(Error::AssetNotFound(e.to_string())),
    }
}
