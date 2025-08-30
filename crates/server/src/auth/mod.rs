use actix_web::cookie::Cookie;
use extract::SESSION_COOKIE_NAME;
use justpic_database::models::sessions::{self, DbSession};

pub mod extract;

pub fn generate_session_cache_key(session_key: &str) -> String {
    [sessions::CACHE_PREFIX, session_key].join(":")
}

pub fn generate_session_cookie<'a>(session: &'a DbSession) -> actix_web::cookie::Cookie<'a> {
    Cookie::build(SESSION_COOKIE_NAME, &session.session_key)
        .path("/")
        .http_only(true)
        .expires(session.expires)
        .finish()
}
