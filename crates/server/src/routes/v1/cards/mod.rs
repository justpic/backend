use actix_web::web;

pub mod create;
pub mod fetch_card;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create::create).service(fetch_card::fetch);
}
