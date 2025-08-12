use crate::AppState;
use crate::error::Error;
use crate::models::api::{SelfUserOut, SessionOut, UserOut};
use crate::models::database::Session;

pub async fn get_user(state: &AppState, session: Session) -> Result<SelfUserOut, Error> {
    let uid = session.user_id;
    state
        .db
        .users
        .get_by_id::<SelfUserOut>(&uid)
        .await?
        .ok_or(Error::NotFound)
}

pub async fn get_user_sessions(
    state: &AppState,
    session: Session,
) -> Result<Vec<SessionOut>, Error> {
    let uid = session.user_id;
    state.db.sessions.get_by_user_id::<SessionOut>(&uid).await
}

pub async fn get_user_by_username(state: &AppState, username: String) -> Result<UserOut, Error> {
    state
        .db
        .users
        .get_by_username(&username)
        .await?
        .ok_or(Error::NotFound)
}
