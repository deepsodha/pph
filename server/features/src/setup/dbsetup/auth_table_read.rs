use anyhow::Result;
use sqlx::{Pool, Sqlite};

pub async fn create_auth_table(read_pool: Pool<Sqlite>) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS auth (
            id VARCHAR PRIMARY KEY,
            user_name VARCHAR,
            password_hash VARCHAR NOT NULL,
            mobile_number VARCHAR(20) NOT NULL,
            org_id VARCHAR,
            org_name VARCHAR,
            service_location_id VARCHAR,
            created_at VARCHAR NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .execute(&read_pool)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_auth_user_name ON auth(user_name)")
        .execute(&read_pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_auth_mobile_number ON auth(mobile_number)")
        .execute(&read_pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_auth_org_id ON auth(org_id)")
        .execute(&read_pool)
        .await?;

    Ok(())
}
