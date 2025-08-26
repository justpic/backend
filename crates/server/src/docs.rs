use utoipa::OpenApi;

use crate::routes::v1;

#[derive(OpenApi)]
#[openapi(
	tags(
		(name = "auth"),
		(name = "users"),
		(name = "picks")
	),
	paths(
		v1::auth::login::login,
		v1::auth::register::register,
		v1::auth::logout::logout,

		v1::users::get_me::get_me,
		v1::users::get_me_sessions::get_me_sessions,
		v1::users::get_by_username::get_by_username,

		v1::picks::create::create,
		v1::picks::get_file::get_file,
	)
)]
pub struct ApiDoc;
