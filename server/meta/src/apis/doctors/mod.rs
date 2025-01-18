use actix_web::web;

use self::{
    command::{add, remove, update},
    query::{get_all, get_by_id},
};

pub mod command;
pub mod query;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(add)
        .service(update)
        .service(remove)
        .service(get_all)
        .service(get_by_id);
}

pub use config as doctors_config;
