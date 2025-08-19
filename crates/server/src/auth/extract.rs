use actix_web::HttpRequest;
use justpic_database::models::{roles::Role, sessions::DbSession, users::DbUser};

use crate::error::{Error, Result};

pub const SESSION_COOKIE_NAME: &str = "user_session";

pub async fn get_maybe_session_from_request(
    req: &HttpRequest,
    redis_pool: &justpic_database::redis::Pool,
) -> Result<Option<DbSession>> {
    let session = match req.cookie(SESSION_COOKIE_NAME) {
        Some(cookie) => {
            let session_key = cookie.value();
            DbSession::get_from_cache(session_key, redis_pool).await?
        }
        None => None,
    };

    Ok(session)
}

pub async fn throw_err_if_client_has_active_session(
    req: &HttpRequest,
    redis_pool: &justpic_database::redis::Pool,
) -> Result<()> {
    let session_exist = get_maybe_session_from_request(req, redis_pool)
        .await?
        .is_some();

    if session_exist {
        return Err(Error::AlreadyExists);
    }

    Ok(())
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
    let db_session = get_maybe_session_from_request(req, redis_pool)
        .await?
        .ok_or(Error::Unauthorized)?; // temp error

    match role_needed {
        Role::Moderator | Role::Admin => {
            let user_role = DbUser::get_by_id(&db_session.user_id, pool)
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
