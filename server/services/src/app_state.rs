use biscuit_auth::PrivateKey;
use cosmo_store_sqlx_sqlite::event_store_sqlx_sqlite::EventStoreSQLXSqlite;
use sqlx::Sqlite;

#[derive(Clone)]
pub struct AppState {
    pub store: EventStoreSQLXSqlite,
    pub read_pool: sqlx::Pool<Sqlite>,
    pub private_key: PrivateKey,
    pub write_pool: sqlx::Pool<Sqlite>,
}
