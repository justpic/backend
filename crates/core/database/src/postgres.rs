use sqlx::{
    Connection, PgConnection, PgPool, Postgres, migrate::MigrateDatabase, postgres::PgPoolOptions,
};
use tracing::info;

pub async fn init_pool() -> sqlx::Result<PgPool> {
    info!("Initializing the Postgres connection pool...");
    let url = dotenvy::var("DATABASE_URL").expect(".env file does not contain 'DATABASE_URL'");

    PgPoolOptions::new().connect(&url).await
}

pub async fn apply_migrations() -> sqlx::Result<()> {
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
