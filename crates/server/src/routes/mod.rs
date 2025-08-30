use actix_web::web;

pub mod v1;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/v1").configure(v1::config));
}
