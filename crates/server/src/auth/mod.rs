use justpic_database::models::sessions;

pub mod extract;

pub fn generate_session_cache_key(session_key: &str) -> String {
    [sessions::CACHE_PREFIX, session_key].join(":")
}
