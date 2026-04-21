use log::{error, info};
use sqlx::{Pool, Row, Sqlite};
use std::time::Duration;
use tokio::time;

/// Retention window in days. Events older than this are pruned, unless they
/// are the latest event for a given stream (which is always kept so the
/// aggregate can still be rebuilt from the most-recent snapshot).
const RETENTION_DAYS: i64 = 90;

/// How often to run the compaction loop.
const COMPACTION_INTERVAL_SECS: u64 = 24 * 60 * 60; // 24 hours

/// Spawn a background task that periodically compacts event log tables in the
/// write database. Events older than RETENTION_DAYS are deleted per stream,
/// but the highest-versioned event per stream is always retained so that
/// aggregates can still be reloaded from the most recent event.
pub fn spawn_compaction_task(write_pool: Pool<Sqlite>) {
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(COMPACTION_INTERVAL_SECS));
        loop {
            interval.tick().await;
            if let Err(e) = compact_event_log(&write_pool).await {
                error!("Event log compaction failed: {}", e);
            }
        }
    });
}

async fn compact_event_log(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    // Find all event tables created by cosmo_store (they follow the naming
    // convention "{app_name}_events").
    let tables: Vec<String> = sqlx::query(
        "SELECT name FROM sqlite_master WHERE type='table' AND name LIKE '%_events'",
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| row.get::<String, _>("name"))
    .collect();

    if tables.is_empty() {
        info!("Compaction: no event tables found, skipping");
        return Ok(());
    }

    let mut total_deleted: u64 = 0;

    for table in &tables {
        // Delete events older than RETENTION_DAYS, but keep the highest
        // version per stream so the aggregate can still be reconstructed.
        let query = format!(
            r#"
            DELETE FROM {table}
            WHERE created_at < datetime('now', '-{days} days')
              AND (stream_id, version) NOT IN (
                  SELECT stream_id, MAX(version)
                  FROM {table}
                  GROUP BY stream_id
              )
            "#,
            table = table,
            days = RETENTION_DAYS
        );

        let result = sqlx::query(&query).execute(pool).await?;
        let deleted = result.rows_affected();
        if deleted > 0 {
            info!(
                "Compaction: pruned {} old events from table '{}'",
                deleted, table
            );
        }
        total_deleted += deleted;
    }

    info!(
        "Compaction complete: {} events pruned across {} tables",
        total_deleted,
        tables.len()
    );
    Ok(())
}
