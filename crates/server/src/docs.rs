use utoipa::OpenApi;

use crate::routes;

#[derive(OpenApi)]
#[openapi(
	tags(
		(name = "auth")
	),
	paths(
		routes::v1::auth::register::register
	)
)]
pub struct ApiDoc;
