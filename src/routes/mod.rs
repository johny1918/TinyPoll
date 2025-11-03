use crate::db;
mod polls;
mod vote;

use crate::routes::polls::{
    create_options, create_poll, get_all_options, get_all_options_by_poll_id, get_poll, get_polls,
};
use crate::routes::vote::{create_vote, get_vote_responses, get_votes_for_poll};
use axum::Router;
use axum::routing::{get, post};
use sqlx::PgPool;
use axum::routing::get_service;
use tower_http::services::{ServeDir, ServeFile};

pub async fn start_server() -> Result<(), sqlx::Error> {
    let pool = db::init_db().await?;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, server_paths(pool).await).await?;
    Ok(())
}

async fn server_paths(pool: PgPool) -> Router {

    let static_dir = ServeDir::new("static");
    let index_file = ServeFile::new("static/index.html");

    let app: Router = Router::new()
        .route("/polls/{id}", get(get_poll))
        .route("/polls/all", get(get_polls))
        .route("/polls/create", post(create_poll))
        .route("/options/all", get(get_all_options))
        .route("/options/create", post(create_options))
        .route("/options/{id}", get(get_all_options_by_poll_id))
        .route("/votes/create", post(create_vote))
        .route("/votes/{poll_id}", get(get_votes_for_poll))
        .route("/polls/{id}/results", get(get_vote_responses))
        .nest_service("/static", static_dir)
        .fallback_service(index_file)
        .with_state(pool);
    app
}
