use crate::setup::prelude::*;
use anyhow::Result;
use sqlx::{Pool, Sqlite};
use utils;
use utils::env_helper::AppEnv;

pub async fn setup(app_environment: AppEnv, read_pool: Pool<Sqlite>) -> Result<()> {
    match app_environment {
        AppEnv::Development => dev_setup(read_pool.clone()).await?,
        AppEnv::Production => prod_setup(read_pool.clone())?,
        AppEnv::Staging => stag_setup(read_pool.clone()).await?,
        AppEnv::Test => test_setup(read_pool.clone()).await?,
    }
    Ok(())
}
