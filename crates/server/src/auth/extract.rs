use actix_web::HttpRequest;
use justpic_database::models::{roles::Role, sessions::DbSession, users::DbUser};

use crate::error::{Error, Result};

pub const SESSION_COOKIE_NAME: &str = "user_session";

pub async fn get_maybe_session_from_request() -> Result<Option<DbSession>> {
    todo!()
}

/// Get User from request headers
///
/// With filter by role
pub async fn get_session_from_request(
    req: &HttpRequest,
    role_needed: Role,
    pool: &justpic_database::postgres::Pool,
    redis_pool: &justpic_database::redis::Pool,
) -> Result<DbSession> {
    let cookie = req.cookie(SESSION_COOKIE_NAME).ok_or(Error::Unauthorized)?;
    let session_key = cookie.value();

    let db_session = DbSession::get_from_cache(session_key, redis_pool)
        .await?
        .ok_or(Error::Unauthorized)?; // temp error

    match role_needed {
        Role::Moderator | Role::Admin => {
            let user_role = DbUser::get_by_id(db_session.user_id, pool)
                .await?
                .ok_or(Error::Unauthorized)?
                .role;

            if role_needed.is_admin() && !user_role.is_admin()
                || role_needed.is_moder() && !user_role.is_moder()
            {
                return Err(Error::Forbidden);
            }

            Ok(db_session)
        }
        Role::Regular => Ok(db_session),
    }
}
