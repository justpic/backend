use actix_web::web;

pub mod create;
pub mod get_by_id;
pub mod get_file;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create::create)
        .service(get_by_id::get_by_id)
        .service(get_file::get_file);
}
