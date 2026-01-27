// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    pi_health_mobile_lib::run().await?;
    Ok(())
}
