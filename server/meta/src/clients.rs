use actix_files::Files;
use actix_web::web;
use std::path::PathBuf;

use utils::path_helper::get_current_statics_path;

fn static_path(subdir: &str) -> PathBuf {
    get_current_statics_path(subdir).unwrap_or_else(|_| PathBuf::from(subdir))
}

pub fn ui_config(cfg: &mut web::ServiceConfig) {
    let admin_path = static_path("statics/admin");
    let doc_path = static_path("statics/clinic");
    cfg.service(Files::new("/admin", admin_path).index_file("index.html"))
        .service(Files::new("/", doc_path).index_file("index.html"));
}
