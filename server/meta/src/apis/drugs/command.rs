// use crate::dto::home_types::HomeInput;
use crate::{apis::api_error::ApiError, dto::drugs::Drugs};
use actix_web::{delete, post, put, web, HttpResponse};
use biscuit_auth::Biscuit;
use sqlx::{types::uuid, Pool, Sqlite};
use utils::biscuit::helper::authorize;

#[post("")]
pub async fn add(
    input: web::Json<Drugs>,
    biscuit: web::ReqData<Biscuit>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<HttpResponse, ApiError> {
    match authorize("superadmin", &biscuit) {
        Ok(_) => {
            let mut tr = pool.begin().await?;

            let _ = sqlx::query("INSERT INTO drugs (id,brand,generic,details,category,side_effects,drugsdose_info,precautions,manufacturer_name,medicines,contra_indications,diseases,interactions,contains,is_deleted)
            VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15)")
            .bind(uuid::Uuid::new_v4().as_simple().to_string())
            .bind(&input.brand)
            .bind(&input.generic)
            .bind(&input.details)
            .bind(&input.category)
            .bind(&input.side_effects)
            .bind(&input.drugsdose_info)
            .bind(&input.precautions)
            .bind(&input.manufacturer_name)
            .bind(&input.medicines)
            .bind(&input.contra_indications)
            .bind(&input.diseases)
            .bind(&input.interactions)
            .bind(&input.contains)
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
    input: web::Json<Drugs>,
    biscuit: web::ReqData<Biscuit>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    match authorize("superadmin", &biscuit) {
        Ok(_) => {
            let mut tr = pool.begin().await?;

            let _ = sqlx::query("UPDATE drugs
                SET brand = $1 , generic = $2, details=$3, category=$4,side_effects=$5,drugsdose_info=$6,precautions=$7, manufacturer_name=$8,medicines=$9,contra_indications=$10,diseases=$11,interactions=$12,contains=$13,is_deleted=$14
                WHERE id = $15")
                .bind(&input.brand)
                .bind(&input.generic)
                .bind(&input.details)
                .bind(&input.category)
                .bind(&input.side_effects)
                .bind(&input.drugsdose_info)
                .bind(&input.precautions)
                .bind(&input.manufacturer_name)
                .bind(&input.medicines)
                .bind(&input.contra_indications)
                .bind(&input.diseases)
                .bind(&input.interactions)
                .bind(&input.contains)
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
                "UPDATE drugs
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
