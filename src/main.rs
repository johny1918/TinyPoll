mod models;
pub use routes::start_server;
mod db;
mod routes;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    start_server().await.expect("Failed to start server");
    Ok(())
}
