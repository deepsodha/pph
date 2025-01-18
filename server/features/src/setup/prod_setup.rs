// use crate::setup::prelude::first_user_setup::first_user_setup;
use anyhow::{Ok, Result};
use log::info;
use sqlx::{Pool, Sqlite};

pub fn prod_setup(read_pool: Pool<Sqlite>) -> Result<()> {
    info!(target: "Setup", "Setting up dev Env with pool {:?}",read_pool);
    //first_user_setup(read_pool.clone()).await?;
    //TODO: By Pass FU Check and Crirst User if Not There
    //TODO: Set up Events it is triggered from ENV file ( some variable to reset data for dev only)
    //TODO: Set up Read Database if Events are new depends on ENV file ( as above )
    Ok(())
}
