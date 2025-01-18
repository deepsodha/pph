mod server;
use anyhow::Result;
pub mod apis;
pub mod auth;
pub mod clients;
pub mod dto;
pub mod meili_queries;

#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv::dotenv().ok();
    server::server().await
}
