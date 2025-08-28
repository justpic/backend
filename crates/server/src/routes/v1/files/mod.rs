use actix_web::web;

pub mod get_file;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_file::get_file);
}
