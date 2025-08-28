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
		v1::auth::login::login,
		v1::auth::register::register,
		v1::auth::logout::logout,

		v1::users::get_me::get_me,
		v1::users::get_me_cards::get_me_cards,
		v1::users::get_me_sessions::get_me_sessions,
		v1::users::get_by_username::get_by_username,
		v1::users::delete_me::delete_me,

		v1::cards::create::create,
		v1::cards::get_by_id::get_by_id,

		v1::files::get_file::get_file,
	)
)]
pub struct ApiDoc;
