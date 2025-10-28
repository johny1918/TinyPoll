
mod config;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error>{
    config::init_db().await?;
    Ok(())
}
