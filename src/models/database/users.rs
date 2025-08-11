use time::UtcDateTime;
use uuid::Uuid;

pub struct User {
	id: Uuid,
	created: UtcDateTime,

	username: String,
	email: String,
	password_hash: String,
	
	role: Role,

	avatar_url: Option<String>,

	email_confirmed: bool,
	nsfw_allowed: bool
}

pub enum Role {
	Regular,
	Moderator,
	Admin
}