use actix_web::web;

pub mod auth;
pub mod files;
pub mod picks;
pub mod users;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").configure(auth::config))
        .service(web::scope("/users").configure(users::config))
        .service(web::scope("/picks").configure(picks::config))
        .service(web::scope("/files").configure(files::config));
}
