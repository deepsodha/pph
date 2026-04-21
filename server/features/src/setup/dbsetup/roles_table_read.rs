use anyhow::Result;
use sqlx::{Pool, Sqlite};

pub async fn create_roles_table(read_pool: Pool<Sqlite>) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS roles (
            id VARCHAR PRIMARY KEY,
            user_id VARCHAR,
            role TEXT CHECK (role IN ('systemadmin', 'superadmin','patient','doctor','clinicalassistant','officestaff','biller','bussinessmanager'))
        );
        "#,
    )
    .execute(&read_pool)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_roles_user_id ON roles(user_id)")
        .execute(&read_pool)
        .await?;

    Ok(())
}
