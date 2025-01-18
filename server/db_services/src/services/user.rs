use common::dto::org_name::OrgName;
use log::{debug, info};
use sqlx::{Error, Pool, Sqlite};
use user::domain::user_domain::UserState;

pub async fn upsert_user(
    read_pool: Pool<Sqlite>,
    p: UserState,
    version: i64,
    stream_id: String,
) -> std::result::Result<(), Error> {
    debug!("upsert_user function called");

    let mut tx = read_pool.begin().await?;

    let org_name = sqlx::query_as::<_, OrgName>(
        "SELECT data->>'name' AS org_name from organization_table_state WHERE id = $1 LIMIT 1",
    )
    .bind(p.org_id.clone())
    .fetch_optional(&read_pool)
    .await?;

    let org_name_value = org_name.map(|org| org.org_name);

    sqlx::query(
        r#"
        INSERT INTO user_table_state(id, org_id, stream_id, version, data, last_updated)
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
    .bind(org_name_value)
    .bind("".to_string())
    .bind(p.last_updated)
    .execute(&mut *tx)
    .await?;

    debug!("Committing transaction for upsert_user");

    tx.commit().await?;

    info!("upsert_user successfully executed");

    Ok(())
}
