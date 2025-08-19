use actix_web::web;

pub mod get_me;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_me::get_me);
}
