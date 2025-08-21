use actix_web::{App, HttpServer, web};
use justpic_database::postgres;
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod auth;
mod docs;
mod error;
mod routes;

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: postgres::Pool,
    pub redis_pool: justpic_cache::Pool,
    pub s3: justpic_storage::S3Client,
}

impl AppState {
    pub fn new(
        pool: postgres::Pool,
        redis_pool: justpic_cache::Pool,
        s3: justpic_storage::S3Client,
    ) -> Self {
        info!("AppState initialized");
        AppState {
            pool,
            redis_pool,
            s3,
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(true)
        .with_level(true)
        .with_ansi(true)
        .init();
    info!("Tracer initialized");

    dotenvy::dotenv().ok();

    let pool = postgres::init_pool()
        .await
        .expect("Database connection failed");

    postgres::apply_migrations()
        .await
        .expect("An error occurred while running migrations");

    let redis = justpic_cache::init_pool().await;
    let s3_client = justpic_storage::setup().await;

    let state = AppState::new(pool, redis, s3_client);

    info!("Running justpic server...");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.pool.clone()))
            .app_data(web::Data::new(state.redis_pool.clone()))
            .app_data(web::Data::new(state.s3.clone()))
            .service(
                SwaggerUi::new("/docs/{_:.*}")
                    .url("/api-docs/openapi.json", docs::ApiDoc::openapi()),
            )
            .configure(routes::config)
    })
    .bind(dotenvy::var("HOST_URL").unwrap())?
    .run()
    .await
}
