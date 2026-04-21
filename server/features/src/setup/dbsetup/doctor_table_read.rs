use anyhow::Result;
use sqlx::{Pool, Sqlite};

pub async fn create_doctor_state_table(read_pool: Pool<Sqlite>) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS doctor_table_state (
            id TEXT PRIMARY KEY,
            org_id TEXT NOT NULL,
            stream_id TEXT NOT NULL,
            version INT NOT NULL,
            data JSON NOT NULL,
            last_updated TIMESTAMPTZ NOT NULL
        );
        "#,
    )
    .execute(&read_pool)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_doctor_org_id ON doctor_table_state(org_id)")
        .execute(&read_pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_doctor_last_updated ON doctor_table_state(last_updated)")
        .execute(&read_pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_doctor_stream_id ON doctor_table_state(stream_id)")
        .execute(&read_pool)
        .await?;

    Ok(())
}
