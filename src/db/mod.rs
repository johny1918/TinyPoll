use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;

pub async fn init_db() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    let pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to database");
    println!("Connected to database");
    Ok(pool)
}
