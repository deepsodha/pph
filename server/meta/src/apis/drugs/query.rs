use actix_web::{get, web, HttpResponse};
use biscuit_auth::Biscuit;
use sqlx::{Pool, Sqlite};
use utils::biscuit::helper::authorize;

use crate::{apis::api_error::ApiError, dto::drugs::GetDrugs};

#[get("")]
pub async fn get_all(
    biscuit: web::ReqData<Biscuit>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<HttpResponse, ApiError> {
    match authorize("superadmin", &biscuit) {
        Ok(_) => {
            let query = "SELECT id, brand, generic, details, category, side_effects, drugsdose_info, precautions, manufacturer_name, medicines, contra_indications, diseases, interactions, contains, is_deleted FROM drugs WHERE is_deleted = $1";

            let rows = sqlx::query_as::<_, GetDrugs>(query)
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
            let query = "SELECT id, brand, generic, details, category, side_effects, drugsdose_info, precautions, manufacturer_name, medicines, contra_indications, diseases, interactions, contains, is_deleted FROM drugs WHERE id=$1 AND is_deleted = $2";

            let rows = sqlx::query_as::<_, GetDrugs>(query)
                .bind(&id)
                .bind(false)
                .fetch_all(&**pool)
                .await?;

            Ok(HttpResponse::Ok().json(rows))
        }
        Err(e) => Err(ApiError::Unauthorized(format!("Error: {}", e))),
    }
}
