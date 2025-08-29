use actix_web::web;

// Get
pub mod fetch_self;
pub mod fetch_self_cards;
pub mod fetch_self_sessions;
pub mod fetch_user;

// Patch
pub mod change_username;

// Delete
pub mod delete_self;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(fetch_self::fetch_self)
        .service(fetch_user::fetch_user)
        .service(fetch_self_sessions::fetch_self_sessions)
        .service(fetch_self_cards::fetch_self_cards)
        .service(change_username::change_username)
        .service(delete_self::delete_me);
}
