use crate::db;
mod polls;

use crate::routes::polls::{create_options, create_poll, get_all_options, get_poll, get_polls};
use axum::Router;
use axum::routing::{get, post};
use sqlx::PgPool;

pub async fn start_server() -> Result<(), sqlx::Error> {
    let pool = db::init_db().await?;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, server_paths(pool).await).await?;
    Ok(())
}

async fn server_paths(pool: PgPool) -> Router {
    let app: Router = Router::new()
        .route("/polls/{id}", get(get_poll))
        .route("/polls/all", get(get_polls))
        .route("/polls/create", post(create_poll))
        .route("/options/all", get(get_all_options))
        .route("/options/create", post(create_options))
        .with_state(pool);
    app
}
