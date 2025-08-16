use crate::DbResult;
use sqlx::{Connection, PgConnection, Postgres, migrate::MigrateDatabase, postgres::PgPoolOptions};
use tracing::info;

/// ### Postgres Connection Pool
pub type Pool = sqlx::PgPool;

/// ### Initialize the database connection pool
pub async fn init_pool() -> DbResult<Pool> {
    info!("Initializing the Postgres connection pool...");
    let url = dotenvy::var("DATABASE_URL").expect(".env file does not contain 'DATABASE_URL'");

    let pool = PgPoolOptions::new().connect(&url).await?;
    Ok(pool)
}

/// ### Apply migrations to the database
/// Migrations are taken from the `/migrations` folder in the root directory
pub async fn apply_migrations() -> DbResult<()> {
    info!("Launching database migrations...");
    let url = dotenvy::var("DATABASE_URL").expect(".env file does not contain 'DATABASE_URL'");
    let url = url.as_str();

    if !Postgres::database_exists(url).await? {
        info!("Database does not exist. Creating it!");
        Postgres::create_database(url).await?;
    }

    let mut conn = PgConnection::connect(url).await?;

    // TEMPORARY HARDCODED
    sqlx::migrate!("../../../migrations")
        .run(&mut conn)
        .await
        .expect("An error occurred while applying migrations!");

    Ok(())
}
