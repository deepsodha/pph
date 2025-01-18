use common::dto::user_role::UserRole;
use log::{debug, info};
use sqlx::{Error, Pool, Sqlite};
use staff::domain::staff_domain::StaffState;
use uuid::Uuid;

pub async fn upsert_staff(
    read_pool: Pool<Sqlite>,
    p: StaffState,
    version: i64,
    stream_id: String,
) -> std::result::Result<(), Error> {
    debug!("upsert_staff function called");

    let mut tx = read_pool.begin().await?;

    debug!("Executing SQL query for upsert_staff");

    sqlx::query(
        r#"
        INSERT INTO staff_table_state(id, org_id, stream_id, version, data, last_updated)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT(id)
        DO UPDATE SET org_id=$2, version=$4, data=$5, last_updated=$6
        "#,
    )
    .bind(p.id.clone())
    .bind(p.org_id.clone())
    .bind(stream_id.clone())
    .bind(version)
    .bind(serde_json::to_value(&p).unwrap())
    .bind(p.last_updated)
    .execute(&mut *tx)
    .await?;

    debug!("Committing transaction for upsert_staff");

    sqlx::query(
        r#"
        INSERT INTO roles(id, user_id, role)
        VALUES ($1, $2, $3)
        ON CONFLICT(id)
        DO UPDATE SET user_id=$2, role=$3
        "#,
    )
    .bind(Uuid::new_v4().as_simple().to_string())
    .bind(p.user.id.clone())
    .bind(UserRole::OfficeStaff)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    info!("upsert_staff successfully executed");

    Ok(())
}
