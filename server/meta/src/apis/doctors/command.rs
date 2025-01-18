// use crate::dto::home_types::HomeInput;
use crate::{apis::api_error::ApiError, dto::doctors::Doctors};
use actix_web::{delete, post, put, web, HttpResponse};
use biscuit_auth::Biscuit;
use sqlx::{types::uuid, Pool, Sqlite};
use utils::biscuit::helper::authorize;

#[post("")]
pub async fn add(
    input: web::Json<Doctors>,
    biscuit: web::ReqData<Biscuit>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<HttpResponse, ApiError> {
    match authorize("superadmin", &biscuit) {
        Ok(_) => {
            let mut tr = pool.begin().await?;

            let _ = sqlx::query("INSERT INTO doctors (id,doctor_name,speciality,experience,hospital_address,city,pincode,is_deleted)
            VALUES($1,$2,$3,$4,$5,$6,$7,$8)")
            .bind(uuid::Uuid::new_v4().as_simple().to_string())
            .bind(&input.doctor_name)
            .bind(&input.speciality)
            .bind(&input.experience)
            .bind(&input.hospital_address)
            .bind(&input.city)
            .bind(&input.pincode)
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
    input: web::Json<Doctors>,
    biscuit: web::ReqData<Biscuit>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    match authorize("superadmin", &biscuit) {
        Ok(_) => {
            let mut tr = pool.begin().await?;

            let _ = sqlx::query("UPDATE doctors
                SET doctor_name = $1 , speciality = $2, experience=$3, hospital_address=$4,city=$5,pincode=$6,is_deleted=$7
                WHERE id = $8")
                .bind(&input.doctor_name)
                .bind(&input.speciality)
                .bind(&input.experience)
                .bind(&input.hospital_address)
                .bind(&input.city)
                .bind(&input.pincode)
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
                "UPDATE doctors
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
