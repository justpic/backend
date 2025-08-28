use actix_web::web;

// Get
pub mod get_by_username;
pub mod get_me;
pub mod get_me_cards;
pub mod get_me_sessions;

// Update

// Delete
pub mod delete_me;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_me::get_me)
        .service(get_by_username::get_by_username)
        .service(get_me_sessions::get_me_sessions)
        .service(get_me_cards::get_me_cards)
        .service(delete_me::delete_me);
}
