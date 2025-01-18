use log::{debug, info};
use organization::domain::organization_domain::OrganizationState;
use sqlx::{Error, Pool, Sqlite};

pub async fn upsert_organization(
    read_pool: Pool<Sqlite>,
    p: OrganizationState,
    version: i64,
    stream_id: String,
) -> std::result::Result<(), Error> {
    debug!("upsert_organization function called");

    let mut tx = read_pool.begin().await?;

    debug!("Executing SQL query for upsert_organization");

    sqlx::query(
        r#"
        INSERT INTO organization_table_state (id, org_id, stream_id, version, data, last_updated)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT(id)
        DO UPDATE SET org_id=$2, version=$4, data=$5, last_updated=$6
        "#,
    )
    .bind(p.id.clone())
    .bind(p.id.clone())
    .bind(stream_id.clone())
    .bind(version)
    .bind(serde_json::to_value(&p).unwrap())
    .bind(p.last_updated)
    .execute(&mut *tx)
    .await?;

    // let servicelocation_data = ServiceLocationState {
    //     id: Uuid::new_v4().as_simple().to_string(),
    //     org_id: p.id.clone(),
    //     org_name: p.name,
    //     created_by: p.created_by,
    //     updated_by: p.updated_by,
    //     created_at: p.created_at,
    //     last_updated: p.last_updated,
    //     address: p.address,
    //     is_deleted: p.is_deleted,
    //     service_location_name: p.
    // };

    // let servicelocation_json_value = serde_json::to_value(&servicelocation_data).unwrap();

    // sqlx::query(
    //     r#"
    //     INSERT INTO servicelocation_table_state (id, org_id, stream_id, version, data, last_updated)
    //     VALUES ($1, $2, $3, $4, $5, $6)
    //     ON CONFLICT(id)
    //     DO UPDATE SET org_id=$2, version=$4, data=$5, last_updated=$6
    //     "#,
    // )
    // .bind(servicelocation_data.id)
    // .bind(p.id.clone())
    // .bind(stream_id.clone())
    // .bind(version)
    // .bind(servicelocation_json_value)
    // .bind(p.last_updated)
    // .execute(&mut *tx)
    // .await?;

    debug!("Committing transaction for upsert_organization");

    tx.commit().await?;

    info!("upsert_organization successfully executed");

    Ok(())
}
