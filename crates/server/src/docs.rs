use utoipa::OpenApi;

use crate::routes;

#[derive(OpenApi)]
#[openapi(
	tags(
		(name = "auth"),
		(name = "users")
	),
	paths(
		routes::v1::auth::login::login,
		routes::v1::auth::register::register,

		routes::v1::users::get_me::get_me,
		routes::v1::users::get_by_username::get_by_username,
	)
)]
pub struct ApiDoc;
