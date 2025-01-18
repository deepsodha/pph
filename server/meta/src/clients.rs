use actix_files::Files;
use actix_web::web;

use utils::path_helper::get_current_statics_path;

pub fn ui_config(cfg: &mut web::ServiceConfig) {
    let admin_path = get_current_statics_path("statics/admin").unwrap();
    let doc_path = get_current_statics_path("statics/clinic").unwrap();
    cfg.service(Files::new("/admin", admin_path).index_file("index.html"))
        .service(Files::new("/", doc_path).index_file("index.html"));
}
