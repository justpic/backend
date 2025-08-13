use deadpool_redis::{Config, Runtime};
use sqlx::{Connection, PgConnection, Postgres, migrate::MigrateDatabase, postgres::PgPoolOptions};
use tracing::info;

pub async fn init_pool() -> sqlx::Result<sqlx::Pool<Postgres>> {
    info!("Initializing the DB connection pool");
    let url = dotenvy::var("DATABASE_URL").expect(".env file does not contain 'DATABASE_URL'");

    let pool = PgPoolOptions::new().connect(&url).await?;

    Ok(pool)
}

pub async fn run_migrations() -> sqlx::Result<()> {
    info!("Launching database migrations");
    let url = dotenvy::var("DATABASE_URL").expect(".env file does not contain 'DATABASE_URL'");
    let url = url.as_str();

    if !Postgres::database_exists(url).await? {
        info!("Database does not exist. Creating it!");
        Postgres::create_database(url).await?;
    }

    let mut conn = PgConnection::connect(url).await?;

    sqlx::migrate!()
        .run(&mut conn)
        .await
        .expect("An error occurred while applying migrations!");

    Ok(())
}

pub async fn init_redis_pool() -> deadpool_redis::Pool {
    info!("Initializing the Redis connection pool");
    let url = dotenvy::var("REDIS_URL").expect(".env file does not contain 'REDIS_URL'");

    Config::from_url(url)
        .builder()
        .expect("Error building Redis pool")
        .max_size(10000)
        .runtime(Runtime::Tokio1)
        .build()
        .expect("Redis connection failed")
}
