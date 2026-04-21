use anyhow::Result;
use sqlx::{Pool, Sqlite};

pub async fn create_pastsurgicalhistory_state_table(read_pool: Pool<Sqlite>) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS pastsurgicalhistory_table_state (
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

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_pastsurgicalhistory_org_id ON pastsurgicalhistory_table_state(org_id)")
        .execute(&read_pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_pastsurgicalhistory_last_updated ON pastsurgicalhistory_table_state(last_updated)")
        .execute(&read_pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_pastsurgicalhistory_stream_id ON pastsurgicalhistory_table_state(stream_id)")
        .execute(&read_pool)
        .await?;

    Ok(())
}
