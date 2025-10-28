use sqlx::{Pool, Postgres, postgres::PgPoolOptions, migrate::Migrator};
use dotenvy::dotenv;
use std::env;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn init_db() -> Result<Pool<Postgres>, sqlx::Error> {
    // Load environment variables from .env
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    let pool = PgPoolOptions::new()
        .connect(&db_url)
        .await?;

    // Run all pending migrations
    MIGRATOR.run(&pool).await?;

    println!("✅ Migrations applied successfully!");

    Ok(pool.clone())
}

