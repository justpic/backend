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
    cfg.service(fetch_self::fetch)
        .service(fetch_user::fetch)
        .service(fetch_self_sessions::fetch)
        .service(fetch_self_cards::fetch)
        .service(delete_self::delete_me);
}
