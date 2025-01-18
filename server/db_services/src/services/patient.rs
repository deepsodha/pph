use log::{debug, info};
use patient::domain::patient_domain::PatientState;
use sqlx::{Error, Pool, Sqlite};

pub async fn upsert_patient(
    read_pool: Pool<Sqlite>,
    p: PatientState,
    version: i64,
    stream_id: String,
) -> std::result::Result<(), Error> {
    // Logging that the function has been called
    debug!("upsert_patient function called");

    let mut tx = read_pool.begin().await?;

    debug!("Executing SQL query for upsert_patient");

    sqlx::query(
        r#"
        INSERT INTO patient_table_state (id, org_id, stream_id, version, data, last_updated)
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

    // Logging transaction commit
    debug!("Committing transaction for upsert_patient");

    tx.commit().await?;

    // Logging successful execution
    info!("upsert_patient successfully executed");

    Ok(())
}
