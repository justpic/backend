use actix_web::web;

pub mod create;

pub mod fetch_card;
pub mod fetch_list;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create::create)
        .service(fetch_card::fetch_card)
        .service(fetch_list::fetch_list);
}
