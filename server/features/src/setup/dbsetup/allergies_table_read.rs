use anyhow::Result;
use sqlx::{Pool, Sqlite};

pub async fn create_allergies_state_table(read_pool: Pool<Sqlite>) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS allergies_table_state (
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

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_allergies_org_id ON allergies_table_state(org_id)")
        .execute(&read_pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_allergies_last_updated ON allergies_table_state(last_updated)")
        .execute(&read_pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_allergies_stream_id ON allergies_table_state(stream_id)")
        .execute(&read_pool)
        .await?;

    Ok(())
}
