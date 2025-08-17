use actix_web::web;

pub mod login;
pub mod register;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(register::register).service(login::login);
}
