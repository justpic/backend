use crate::AppState;
use crate::error::Error;
use crate::models::api::SelfUserOut;
use crate::models::database::Session;

pub async fn get_user(
	state: &AppState,
	session: Session,
) -> Result<SelfUserOut, Error> {
	let uid = session.user_id;
	state.db.users.get_by_id::<SelfUserOut>(&uid).await?.ok_or(Error::NotFound)
}