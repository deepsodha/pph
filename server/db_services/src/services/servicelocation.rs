use log::{debug, info};
use servicelocation::domain::service_location_domain::ServiceLocationState;
use sqlx::{Error, Pool, Sqlite};

pub async fn upsert_servicelocation(
    read_pool: Pool<Sqlite>,
    p: ServiceLocationState,
    version: i64,
    stream_id: String,
) -> std::result::Result<(), Error> {
    debug!("upsert_servicelocation function called");

    let mut tx = read_pool.begin().await?;

    debug!("Executing SQL query for upsert_servicelocation");

    sqlx::query(
        r#"
        INSERT INTO servicelocation_table_state (id, org_id, stream_id, version, data, last_updated)
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

    debug!("Committing transaction for upsert_servicelocation");

    tx.commit().await?;

    info!("upsert_servicelocation successfully executed");

    Ok(())
}
