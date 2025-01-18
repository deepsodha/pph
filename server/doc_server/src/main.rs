mod clients;
mod health_check;
mod rpc_handler;
mod server;
use anyhow::Result;
pub use rpc_handler::rpc_handler;
mod auth_rpc_handler;
pub use auth_rpc_handler::auth_rpc_handler;
extern crate validator;
pub mod medicine_prediction;
pub mod phone_code;

#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv::dotenv().ok();
    server::server().await
}
