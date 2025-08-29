use utoipa::OpenApi;

use crate::routes::v1;

#[derive(OpenApi)]
#[openapi(
	tags(
		(name = "auth"),
		(name = "users"),
		(name = "cards"),
		(name = "files")
	),
	paths(
		// Auth endpoints
		v1::auth::login::login,
		v1::auth::register::register,
		v1::auth::logout::logout,
		
		// User model endpoints
		v1::users::fetch_self::fetch,
		v1::users::fetch_self_cards::fetch,
		v1::users::fetch_self_sessions::fetch,
		v1::users::fetch_user::fetch,

		v1::users::delete_self::delete_me,

		// Card model endpoints
		v1::cards::create::create,
		v1::cards::fetch_card::fetch,

		// Files and Storage endpoints
		v1::files::get_file::get_file,
	)
)]
pub struct ApiDoc;
