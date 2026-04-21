use crate::{
    apis::api_error::ApiError,
    dto::users::{UpdateUsers, Users},
};
use actix_web::{delete, post, put, web, HttpResponse};
use biscuit_auth::Biscuit;
use sqlx::{types::uuid, Pool, Sqlite};
use utils::{biscuit::helper::authorize, password_helper::hash_password};

#[post("")]
pub async fn add(
    input: web::Json<Users>,
    biscuit: web::ReqData<Biscuit>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<HttpResponse, ApiError> {
    let password = hash_password(&input.password)
        .map_err(|e| ApiError::Other(anyhow::anyhow!("Password hashing failed: {}", e)))?;
    match authorize("superadmin", &biscuit) {
        Ok(_) => {
            let mut tr = pool.begin().await?;

            if input.role.is_empty() {
                return Err(ApiError::Validation("Role is required.".to_string()));
            }

            let role = serde_json::to_string(&input.role)?;
            let _ = sqlx::query(
                "INSERT INTO users (id,mobile_number,password,role,is_deleted)
            VALUES($1,$2,$3,$4,$5)",
            )
            .bind(uuid::Uuid::new_v4().as_simple().to_string())
            .bind(&input.mobile_number)
            .bind(password)
            .bind(&role)
            .bind(false)
            .execute(&mut *tr)
            .await?;

            tr.commit().await?;
            Ok(HttpResponse::Ok().body("successful"))
        }
        Err(e) => Err(ApiError::Unauthorized(format!("Error: {}", e))),
    }
}

#[put("/{id}")]
pub async fn update(
    path: web::Path<String>,
    input: web::Json<UpdateUsers>,
    biscuit: web::ReqData<Biscuit>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    // let password = hash_password(&input.password).unwrap();
    match authorize("superadmin", &biscuit) {
        Ok(_) => {
            let mut tr = pool.begin().await?;
            let role = serde_json::to_string(&input.role)?;

            let _ = sqlx::query(
                "UPDATE users
                SET mobile_number = $1 , role = $2, is_deleted=$3
                WHERE id = $4",
            )
            .bind(&input.mobile_number)
            .bind(role)
            .bind(false)
            .bind(&id)
            .execute(&mut *tr)
            .await?;

            tr.commit().await?;
            Ok(HttpResponse::Ok().body("Successful"))
        }
        Err(e) => Err(ApiError::Unauthorized(format!("Error: {}", e))),
    }
}

#[delete("/{id}")]
pub async fn remove(
    path: web::Path<String>,
    biscuit: web::ReqData<Biscuit>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    match authorize("superadmin", &biscuit) {
        Ok(_) => {
            let mut tr = pool.begin().await?;

            let _ = sqlx::query(
                "UPDATE users
                SET
                is_deleted=$1
                WHERE id = $2",
            )
            .bind(true)
            .bind(&id)
            .execute(&mut *tr)
            .await?;
            tr.commit().await?;
            Ok(HttpResponse::Ok().body("Successful"))
        }
        Err(e) => Err(ApiError::Unauthorized(format!("Error: {}", e))),
    }
}
