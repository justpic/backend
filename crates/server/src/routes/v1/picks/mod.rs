use actix_web::web;

pub mod create;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create::create);
}
