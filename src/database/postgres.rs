use std::time::Duration;
use sqlx::{Connection, PgConnection, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use tracing::info;

use crate::database::error::DatabaseError;

pub type DbPool = Pool<Postgres>;

fn get_db_url() -> String {
	dotenvy::var("DATABASE_URL")
		.expect(".env file does not contain database url")
}

pub async fn init_pool() -> Result<DbPool, DatabaseError> {
	let url = get_db_url();
	let lifetime = Duration::from_secs(60 * 30);

	info!("Initializing database connection...");

	let pool = PgPoolOptions::new()
		.max_lifetime(Some(lifetime))
		.connect(&url)
		.await?;
	Ok(pool)
}

pub async fn run_migrations() -> Result<(), DatabaseError> {
	let url = get_db_url();
	info!("Running migrations...");

	let mut conn = PgConnection::connect(&url).await?;
	sqlx::migrate!()
		.run(&mut conn)
		.await.expect("Failed to run migrations");

	Ok(())
}