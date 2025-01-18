use anyhow::Result;
use sqlx::{Pool, Sqlite};
pub async fn test_setup(_read_pool: Pool<Sqlite>) -> Result<()> {
    //TODO: By Pass FU Check and Create First User if Not There
    //TODO: Set up test database for integration tests
    todo!()
}
