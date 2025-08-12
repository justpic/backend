use sqlx::postgres::PgPoolOptions;
use sqlx::{Connection, PgConnection, Pool, Postgres};
use tracing::info;

use crate::error::Error;

pub type DbPool = Pool<Postgres>;

fn get_db_url() -> String {
    dotenvy::var("DATABASE_URL").expect(".env file does not contain database url")
}

pub async fn init_pool() -> Result<DbPool, Error> {
    let url = get_db_url();

    info!("Initializing database connection...");

    let pool = PgPoolOptions::new().connect(&url).await?;
    Ok(pool)
}

pub async fn run_migrations() -> Result<(), Error> {
    let url = get_db_url();
    info!("Running migrations...");

    let mut conn = PgConnection::connect(&url).await?;
    sqlx::migrate!()
        .run(&mut conn)
        .await
        .expect("Failed to run migrations");

    Ok(())
}
