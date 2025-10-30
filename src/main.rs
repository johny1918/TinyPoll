
mod models;
pub use routes::start_server;
mod routes;
mod db;

#[tokio::main]
async fn main() -> std::io::Result<()>{

    let _ = start_server()
        .await
        .expect("Failed to start server");
    Ok(())
}
