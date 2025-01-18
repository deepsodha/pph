use anyhow::Result;
use sqlx::{Pool, Sqlite};
pub async fn stag_setup(_read_pool: Pool<Sqlite>) -> Result<()> {
    //TODO: By Pass FU Check and Create First User if Not There
    //TODO: Set up Events from generated commands
    //TODO: Set up Read Database from events
    todo!()
}
