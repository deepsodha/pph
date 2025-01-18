use crate::app_state::AppState;
use crate::method::convention::ErrorData;
use anyhow::Result;
use auth::dto::authenticate_user::AuthenticatedUser;
use auth::dto::login_mobile::LoginMobile;
use biscuit_auth::KeyPair;
use common::dto::user_role::RolesRow;
use dotenv::var;
use log::{error, info};
use serde_json::Value;
use sqlx::Row;
use utils::biscuit::helper::{create_token, TokenData};
use utils::password_helper::verify_password;

pub async fn login_submit(input: LoginMobile, state: AppState) -> Result<Value, ErrorData> {
    info!("Starting login_submit function execution...");

    let mobile_number =
        var("FIRST_USER_MOBILE").expect("FIRST_USER_MOBILE is not set in .env file");
    let password = var("FIRST_USER_PASSWORD").expect("FIRST_USER_PASSWORD is not set in .env file");
    let first_user_first_name = var("FIRST_USER_FIRST_NAME").expect("first_name is not in env");
    let first_user_id = var("FIRST_USER_ID").expect("ID is not in env");

    if input.mobile_number == mobile_number && input.password == password {
        let token = create_token(
            &KeyPair::from(&state.private_key),
            TokenData {
                org_id: "",
                org_name: "",
                user_name: &first_user_first_name,
                user_roles: vec!["superadmin"],
                user_id: &first_user_id,
                service_location_id: "",
                service_location_name: "",
            },
        )?;
        serde_json::to_value(AuthenticatedUser {
            id: first_user_id.to_string(),
            token,
            user_name: first_user_first_name.to_string(),
            selected_language: "EN".to_string(),
            role: vec!["superadmin".to_string()],
            org_id: "".to_string(),
            org_name: "".to_string(),
            service_location_id: "".to_string(),
        })
        .map_err(ErrorData::from)
    } else {
        let query_result = sqlx::query(
            "SELECT id, mobile_number, password_hash,user_name,org_id,org_name,service_location_id FROM auth WHERE mobile_number = $1",
        )
        .bind(&input.mobile_number)
        .fetch_optional(&state.read_pool)
        .await?;

        if let Some(row) = query_result {
            info!("User data found in the database.");

            let stored_hash: &str = row.get("password_hash");
            let user_name: &str = row.get("user_name");
            let id: &str = row.get("id");
            let org_id: &str = row.get("org_id");
            let org_name: &str = row.get("org_name");
            let serivce_location_id: &str = row.get("service_location_id");

            let is_valid = verify_password(stored_hash, &input.password).unwrap_or(false);

            if is_valid {
                info!("Password validated successfully.");

                let query = "SELECT role FROM roles WHERE user_id = $1";
                let rows: Vec<RolesRow> = sqlx::query_as(query)
                    .bind(id)
                    .fetch_all(&state.read_pool)
                    .await?;

                let roles: Vec<String> = rows
                    .into_iter()
                    .map(|row: RolesRow| row.role.to_string())
                    .collect();

                let vec_of_refs: Vec<&str> = roles.iter().map(|s| s.as_str()).collect();

                if roles.is_empty() {
                    return Err(ErrorData {
                        message: String::from("PLEASE_ADD_ROLE"),
                        data: Value::Null,
                        code: -32600,
                    });
                }

                let token = create_token(
                    &KeyPair::from(&state.private_key),
                    TokenData {
                        org_id,
                        org_name,
                        user_name,
                        user_roles: vec_of_refs,
                        user_id: id,
                        service_location_id: serivce_location_id,
                        service_location_name: "",
                    },
                )?;

                serde_json::to_value(AuthenticatedUser {
                    id: id.to_string(),
                    token,
                    user_name: user_name.to_string(),
                    selected_language: "EN".to_string(),
                    role: roles.clone(),
                    org_id: org_id.to_string(),
                    org_name: org_name.to_string(),
                    service_location_id: serivce_location_id.to_string(),
                })
                .map_err(ErrorData::from)
            } else {
                error!("Password validation failed.");
                Err(ErrorData {
                    message: String::from("Username or Password doesn't match"),
                    data: Value::Null,
                    code: -32600,
                })
            }
        } else {
            error!("User data not found in the database.");
            Err(ErrorData {
                message: String::from("Username or Password doesn't match"),
                data: Value::Null,
                code: -32600,
            })
        }
    }
}
