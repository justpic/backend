use actix_web::{App, HttpServer};

pub mod database;
pub mod redis;

pub mod error;

// Api modules
pub mod auth;
pub mod users;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    justpic::init_tracing();
    dotenvy::dotenv().ok();

    let pool = database::init_pool()
        .await
        .expect("Database connection failed");

    database::run_migrations()
        .await
        .expect("An error occurred while running migrations");

    let redis_pool = database::init_redis_pool().await;

    let state = justpic::setup(pool, redis_pool);

    HttpServer::new(move || App::new().configure(|cfg| justpic::config(cfg, state.clone())))
        .bind(dotenvy::var("HOST_URL").unwrap())?
        .run()
        .await
}
