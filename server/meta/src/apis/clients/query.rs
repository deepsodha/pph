use actix_web::{get, web, HttpResponse};
use biscuit_auth::Biscuit;
use sqlx::{Pool, Sqlite};
use utils::biscuit::helper::authorize;

use crate::{apis::api_error::ApiError, dto::clients::GetClients};

#[get("")]
pub async fn get_all(
    biscuit: web::ReqData<Biscuit>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<HttpResponse, ApiError> {
    match authorize("superadmin", &biscuit) {
        Ok(_) => {
            let query = "SELECT id, name, address, mobile_number, email, gst_no, pan_no,is_deleted FROM clients WHERE is_deleted =$1";

            let rows = sqlx::query_as::<_, GetClients>(query)
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
            let query = "SELECT id, name, address, mobile_number, email, gst_no, pan_no,is_deleted FROM clients WHERE id=$1 AND is_deleted =$2";

            let rows = sqlx::query_as::<_, GetClients>(query)
                .bind(&id)
                .bind(false)
                .fetch_all(&**pool)
                .await?;

            Ok(HttpResponse::Ok().json(rows))
        }
        Err(e) => Err(ApiError::Unauthorized(format!("Error: {}", e))),
    }
}
