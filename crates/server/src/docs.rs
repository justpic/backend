use utoipa::OpenApi;

use crate::routes;

#[derive(OpenApi)]
#[openapi(
	tags(
		(name = "auth"),
		(name = "users"),
		(name = "picks")
	),
	paths(
		routes::v1::auth::login::login,
		routes::v1::auth::register::register,
		routes::v1::auth::logout::logout,

		routes::v1::users::get_me::get_me,
		routes::v1::users::get_me_sessions::get_me_sessions,
		routes::v1::users::get_by_username::get_by_username,

		routes::v1::picks::create::create,
	)
)]
pub struct ApiDoc;
