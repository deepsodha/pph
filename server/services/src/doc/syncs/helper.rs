use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Sqlite};

use super::DataTable;

pub async fn fetch_states<T>(
    table_name: &str,
    input: &DateTime<Utc>,
    org_id: &str,
    state: Pool<Sqlite>,
) -> Result<Vec<T>>
where
    T: serde::de::DeserializeOwned,
{
    let mut tx = state.begin().await?;

    let query = format!(
        "SELECT * FROM {} WHERE last_updated > ? AND org_id = ? AND (data->>'is_deleted') = $3",
        table_name
    );

    let rows = sqlx::query_as::<_, DataTable>(&query)
        .bind(input)
        .bind(org_id)
        .bind(false)
        .fetch_all(&mut *tx)
        .await?;

    let mut states = Vec::new();
    for row in rows {
        let state: T = serde_json::from_value(row.data)?;
        states.push(state);
    }
    tx.commit().await?;

    Ok(states)
}
