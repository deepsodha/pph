use actix_web::{get, web, HttpResponse};
use biscuit_auth::Biscuit;
use sqlx::{Pool, Sqlite};
use utils::biscuit::helper::authorize;

use crate::{apis::api_error::ApiError, dto::users::GetUsers};

#[get("")]
pub async fn get_all(
    biscuit: web::ReqData<Biscuit>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<HttpResponse, ApiError> {
    match authorize("superadmin", &biscuit) {
        Ok(_) => {
            let query = "SELECT id, mobile_number, role, is_deleted FROM users WHERE is_deleted=$1";

            let rows = sqlx::query_as::<_, GetUsers>(query)
                .bind(false)
                .fetch_all(&**pool)
                .await?;

            Ok(HttpResponse::Ok().json(rows))
        }
        Err(e) => Err(ApiError::Unauthorized(format!("Error: {}", e))),
    }
}

#[get("/{id}")]
pub async fn get_by_id(
    path: web::Path<String>,
    biscuit: web::ReqData<Biscuit>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    match authorize("superadmin", &biscuit) {
        Ok(_) => {
            let query = "SELECT id, mobile_number, role, is_deleted FROM users WHERE id=$1 AND is_deleted = $2";

            let rows = sqlx::query_as::<_, GetUsers>(query)
                .bind(&id)
                .bind(false)
                .fetch_all(&**pool)
                .await?;

            Ok(HttpResponse::Ok().json(rows))
        }
        Err(e) => Err(ApiError::Unauthorized(format!("Error: {}", e))),
    }
}
