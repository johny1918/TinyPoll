use crate::db;
mod polls;

pub use polls::test_poll_route;
pub use polls::get_polls;
use axum::{Router};
use sqlx::PgPool;
use axum::routing::{get, post};
use crate::routes::polls::create_poll;

pub async fn start_server() -> Result<(), sqlx::Error>{
    let pool = db::init_db().await?;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, server_paths(pool).await).await?;
    Ok(())
}

async fn server_paths(pool: PgPool) -> Router {
    let app: Router = Router::new()
        .route("/polls", get(test_poll_route))
        .route("/polls/all", get(get_polls))
        .route("/polls/create", post(create_poll))
        .with_state(pool);
    app
}