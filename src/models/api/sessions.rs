use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::database::{Role, Session};

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionOut {
    pub session_id: Uuid,

    pub os: Option<String>,
    pub device: Option<String>,
    pub user_agent: Option<String>,
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
