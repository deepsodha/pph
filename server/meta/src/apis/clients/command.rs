use crate::{apis::api_error::ApiError, dto::clients::Clients};
use actix_web::{delete, post, put, web, HttpResponse};
use biscuit_auth::Biscuit;
use sqlx::{types::uuid, Pool, Sqlite};
use utils::biscuit::helper::authorize;

#[post("")]
pub async fn add(
    input: web::Json<Clients>,
    biscuit: web::ReqData<Biscuit>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<HttpResponse, ApiError> {
    match authorize("superadmin", &biscuit) {
        Ok(_) => {
            let mut tr = pool.begin().await?;

            let _ = sqlx::query(
                "INSERT INTO clients (id,name,address,mobile_number,email,gst_no,pan_no,is_deleted)
            VALUES($1,$2,$3,$4,$5,$6,$7,$8)",
            )
            .bind(uuid::Uuid::new_v4().as_simple().to_string())
            .bind(&input.name)
            .bind(&input.address)
            .bind(&input.mobile_number)
            .bind(&input.email)
            .bind(&input.gst_no)
            .bind(&input.pan_no)
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
    input: web::Json<Clients>,
    biscuit: web::ReqData<Biscuit>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    match authorize("superadmin", &biscuit) {
        Ok(_) => {
            let mut tr = pool.begin().await?;

            let _ = sqlx::query("UPDATE clients
                SET name = $1 , address = $2, mobile_number=$3, email=$4,gst_no=$5,pan_no=$6,is_deleted=$7
                WHERE id = $8")
                .bind(&input.name)
                .bind(&input.address)
                .bind(&input.mobile_number)
                .bind(&input.email)
                .bind(&input.gst_no)
                .bind(&input.pan_no)
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
                "UPDATE clients
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
