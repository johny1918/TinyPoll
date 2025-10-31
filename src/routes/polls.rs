
use crate::models::poll::{NewPoll, Poll};
use axum::{Json, extract::{ State, Path}};
use sqlx::PgPool;

pub async fn get_polls(State(pool): State<PgPool>) -> Json<Vec<Poll>> {
    let polls = sqlx::query_as!(
        Poll,
        r#"SELECT id, question, created_at FROM polls ORDER BY created_at DESC"#
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    Json(polls)
}

pub async fn get_poll(State(pool): State<PgPool>, Path(id): Path<i32>) -> Json<Option<Poll>> {
    let poll = sqlx::query_as!(
        Poll,
        r#"SELECT id, question, created_at FROM polls WHERE id = $1"#,
        id
    )
    .fetch_optional(&pool)
    .await.expect("Failed to fetch poll");

    Json(poll)
}

pub async fn create_poll(State(pool): State<PgPool>, Json(new_poll): Json<NewPoll>) -> Json<Poll> {
    let poll = sqlx::query_as::<_, Poll>(
        "INSERT INTO polls (question, created_at)
         VALUES ($1, NOW())
         RETURNING id, question, created_at",
    )
    .bind(new_poll.question)
    .fetch_one(&pool)
    .await
    .expect("Failed to insert poll");

    Json(poll)
}
