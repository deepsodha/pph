use prescription::domain::prescription_domain::PrescriptionState;
use sqlx::{Error, Pool, Sqlite};

use log::{debug, info};

pub async fn upsert_prescription(
    read_pool: Pool<Sqlite>,
    p: PrescriptionState,
    version: i64,
    stream_id: String,
) -> Result<(), Error> {
    debug!("upsert_prescription function called");

    let mut tx = read_pool.begin().await?;

    debug!("Executing SQL query for upsert_prescription");

    sqlx::query(
        r#"
        INSERT INTO prescription_table_state (id, org_id, stream_id, version, data, last_updated)
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

    debug!("Committing transaction for upsert_prescription");

    tx.commit().await?;

    info!("upsert_prescription successfully executed");

    Ok(())
}
