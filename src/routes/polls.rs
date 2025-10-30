use axum::{Json, extract::State};
use sqlx::PgPool;
use crate::models::poll::{ Poll, NewPoll };

pub async fn test_poll_route() -> Json<String> {
    Json("Polls endpoint work!".to_string())
}
pub async fn get_polls(State(pool): State<PgPool>) -> Json<Vec<Poll>> {
    let polls = sqlx::query_as::<_, Poll>("SELECT * FROM polls")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    Json(polls)
}

pub async fn create_poll(
    State(pool): State<PgPool>,
    Json(new_poll): Json<NewPoll>,
) -> Json<Poll> {
    let poll = sqlx::query_as::<_, Poll>(
        "INSERT INTO polls (question, created_at)
         VALUES ($1, NOW())
         RETURNING id, question, created_at"
    )
        .bind(new_poll.question)
        .fetch_one(&pool)
        .await
        .expect("Failed to insert poll");

    Json(poll)
}