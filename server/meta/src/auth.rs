use crate::dto::login::AuthLogin;
use actix_web::{web, HttpResponse, Responder};
use auth::dto::authenticate_user::AuthenticatedUser;
use biscuit_auth::{KeyPair, PrivateKey};
use dotenv::var;
use log::{error, info};
use serde_json::Value;
use services::method::convention::ErrorData;
use sqlx::Row;
use sqlx::{Pool, Sqlite};
use utils::{
    biscuit::helper::{create_token, TokenData},
    password_helper::verify_password,
};

pub async fn login(input: web::Json<AuthLogin>, pool: web::Data<Pool<Sqlite>>) -> impl Responder {
    let private_key =
        PrivateKey::from_bytes_hex(&var("PRIVATE_KEY").expect("Private is not set in .env file"))
            .expect("Failed to parse private key");
    let mobile_number =
        var("FIRST_USER_MOBILE").expect("FIRST_USER_MOBILE is not set in .env file");
    let password = var("FIRST_USER_PASSWORD").expect("FIRST_USER_PASSWORD is not set in .env file");
    let first_user_first_name =
        var("FIRST_USER_FIRST_NAME").expect("FIRST_USER_FIRST_NAME is not set in .env file");
    let first_user_id = var("FIRST_USER_ID").expect("FIRST_USER_ID is not set in .env file");

    if input.mobile_number == mobile_number && input.password == password {
        let token = match create_token(
            &KeyPair::from(&private_key),
            TokenData {
                org_id: "",
                org_name: "",
                user_name: &first_user_first_name,
                user_roles: vec!["superadmin"],
                user_id: &first_user_id,
                service_location_id: "",
                service_location_name: "",
            },
        ) {
            Ok(token) => token,
            Err(err) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Token creation error: {}", err));
            }
        };

        HttpResponse::Ok().json(AuthenticatedUser {
            id: first_user_id.to_string(),
            token,
            user_name: first_user_first_name.to_string(),
            selected_language: "EN".to_string(),
            role: vec!["superadmin".to_string()],
            org_id: "".to_string(),
            org_name: "".to_string(),
            service_location_id: "".to_string(),
        })
    } else {
        let query_result = sqlx::query(
            "SELECT id, mobile_number, password,role FROM users WHERE mobile_number = $1",
        )
        .bind(&input.mobile_number)
        .fetch_optional(&**pool)
        .await;

        let query_result = match query_result {
            Ok(result) => result,
            Err(err) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Database error: {}", err));
            }
        };

        if let Some(row) = query_result {
            info!("User data found in the database.");

            let stored_hash: &str = row.get("password");
            let id: &str = row.get("id");
            let role_json: String = row.get("role");

            let roles: Vec<String> = serde_json::from_str(&role_json).unwrap_or_else(|err| {
                error!("Failed to parse role JSON: {}", err);
                vec![]
            });

            let roles_refs: Vec<&str> = roles.iter().map(|r| r.as_str()).collect();

            let is_valid = verify_password(stored_hash, &input.password).unwrap_or(false);

            if is_valid {
                info!("Password validated successfully.");

                let token = create_token(
                    &KeyPair::from(&private_key),
                    TokenData {
                        org_id: "",
                        org_name: "",
                        user_name: "",
                        user_roles: roles_refs,
                        user_id: &first_user_id,
                        service_location_id: "",
                        service_location_name: "",
                    },
                );

                match token {
                    Ok(token) => {
                        let authenticated_user_data = serde_json::to_value(AuthenticatedUser {
                            id: id.to_string(),
                            token,
                            user_name: first_user_first_name.to_string(),
                            selected_language: "EN".to_string(),
                            role: roles.clone(),
                            org_id: "".to_string(),
                            org_name: "".to_string(),
                            service_location_id: "".to_string(),
                        })
                        .map_err(|err| {
                            HttpResponse::InternalServerError()
                                .body(format!("Token creation error: {}", err))
                        });

                        match authenticated_user_data {
                            Ok(auth_data) => HttpResponse::Ok().json(auth_data),
                            Err(err) => HttpResponse::InternalServerError()
                                .body(format!("Token creation error: {:?}", err)),
                        }
                    }
                    Err(err) => HttpResponse::InternalServerError()
                        .body(format!("Token creation error: {}", err)),
                }
            } else {
                error!("Password validation failed.");
                let error_data = ErrorData {
                    message: String::from("Username or Password doesn't match"),
                    data: Value::Null,
                    code: -32600,
                };
                HttpResponse::Unauthorized().json(error_data)
            }
        } else {
            error!("User data not found in the database.");
            let error_data = ErrorData {
                message: String::from("User Not Found"),
                data: Value::Null,
                code: -32600,
            };
            HttpResponse::NotFound().json(error_data)
        }
    }
}
