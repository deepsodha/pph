use log::info;
use patient::domain::notes::note_domain::NoteState;
use sqlx::{Error, Pool, Sqlite};

pub async fn upsert_notes(
    read_pool: Pool<Sqlite>,
    p: NoteState,
    version: i64,
    stream_id: String,
) -> std::result::Result<(), Error> {
    info!("Upserting notes state");

    let mut tx = read_pool.begin().await?;

    sqlx::query(
        r#"
        INSERT INTO note_table_state (id, org_id, stream_id, version, data, last_updated) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT(id) DO UPDATE SET org_id=$2, version=$4, data=$5, last_updated=$6
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

    tx.commit().await?;

    info!("Note state upserted successfully");

    Ok(())
}
