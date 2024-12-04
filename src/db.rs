use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenv::dotenv;

pub async fn establish_connection() -> Result<sqlx::PgPool, sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}
