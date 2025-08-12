use std::sync::Arc;

use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    AppState,
    models::database::{Role, Session},
};

pub const SESSION_COOKIE_NAME: &str = "user_session";
pub const REDIS_SESSION_PREFIX: &str = "session";

#[derive(Debug, Serialize)]
pub struct SessionOut {
    pub session_id: Uuid,

    pub os: Option<String>,
    pub device: Option<String>,
    pub user_agent: Option<String>,
}

impl From<SessionWithRole> for SessionOut {
    fn from(value: SessionWithRole) -> Self {
        SessionOut {
            session_id: value.session.session_id,
            os: value.session.os,
            device: value.session.device,
            user_agent: value.session.user_agent,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionWithRole {
    #[serde(flatten)]
    pub session: Session,
    pub role: Role,
}

impl SessionWithRole {
    pub fn from_session_with_role(session: Session, role: Role) -> Self {
        SessionWithRole { session, role }
    }

    pub fn is_moderator(&self) -> bool {
        self.role.is_moderator()
    }

    pub fn is_admin(&self) -> bool {
        self.role.is_admin()
    }
}

pub struct OptionSession {
    pub item: Option<SessionWithRole>,
}

impl FromRequestParts<Arc<AppState>> for OptionSession {
    type Rejection = crate::error::Error;

    async fn from_request_parts(
        req: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&req.headers);

        match jar.get(SESSION_COOKIE_NAME) {
            Some(cookie) => {
                let session_key = [REDIS_SESSION_PREFIX, cookie.value()].join(":");

                let item = state
                    .redis
                    .get_json::<SessionWithRole>(&session_key)
                    .await?;

                Ok(OptionSession { item })
            }
            None => Ok(OptionSession { item: None }),
        }
    }
}
