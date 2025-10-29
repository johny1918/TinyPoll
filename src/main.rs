use axum::http::StatusCode;
use axum::Router;
use axum::routing::get;

mod config;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error>{
    config::init_db().await?;

    //Start a server
    let app: Router = Router::new()
        .route("/health", get(health));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app.into_make_service()).await?;
    println!("Server is running on 127.0.0.1:8080");
    Ok(())
}

async fn health() -> StatusCode {
    StatusCode::OK
}