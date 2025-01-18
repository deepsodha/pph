use log::{debug, info};
use sqlx::{Error, Pool, Sqlite};
use system_admin::domain::systemadmin_domain::SystemAdminState;
use uuid::Uuid;

pub async fn upsert_systemadmin(
    read_pool: Pool<Sqlite>,
    p: SystemAdminState,
    org_id: String,
    org_name: String,
    version: i64,
    stream_id: String,
) -> std::result::Result<(), Error> {
    debug!("upsert_systemadmin function called");

    let mut tx = read_pool.begin().await?;

    debug!("Executing SQL query for upsert_systemadmin");

    sqlx::query(
        r#"
        INSERT INTO systemadmin_table_state(id, org_id, stream_id, version, data, last_updated)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT(id)
        DO UPDATE SET org_id=$2, version=$4, data=$5, last_updated=$6
        "#,
    )
    .bind(p.id.clone())
    .bind(org_id.to_string()) // Bind an empty string for org_id as it's not provided in the function arguments
    .bind(stream_id.clone())
    .bind(version)
    .bind(serde_json::to_value(&p).unwrap())
    .bind(p.last_updated)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO auth(id, user_name, password_hash, mobile_number, org_id,org_name,service_location_id,created_at)
        VALUES ($1, $2, $3, $4, $5,$6,$7,$8)
        ON CONFLICT(id)
        DO UPDATE SET user_name=$2, password_hash=$3, mobile_number=$4, org_id=$5,org_name=$6,service_location_id=$7, created_at=$8
        "#,
    )
    .bind(p.id.clone())
    .bind(p.user.first_name)
    .bind(p.password)
    .bind(p.phone.number)
    .bind(p.org_id)
    .bind(org_name)
    .bind("".to_string())
    .bind(p.last_updated)
    .execute(&mut *tx)
    .await?;

    for role in &p.roles {
        sqlx::query(
            r#"
            INSERT INTO roles(id, user_id, role)
            VALUES ($1, $2, $3)
            ON CONFLICT(id)
            DO UPDATE SET user_id=$2, role=$3
            "#,
        )
        .bind(Uuid::new_v4().as_simple().to_string())
        .bind(p.id.clone())
        .bind(role)
        .execute(&mut *tx)
        .await?;
    }

    debug!("Committing transaction for upsert_systemadmin");

    tx.commit().await?;

    info!("upsert_systemadmin successfully executed");

    Ok(())
}
