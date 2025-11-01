use crate::models::poll::{NewPoll, Poll};
use crate::models::poll_option::{PollOption, NewPollOption};
use axum::{
    Json,
    extract::{Path, State},
};
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
    .await
    .expect("Failed to fetch poll");

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

pub async fn get_all_options(State(pool): State<PgPool>) -> Json<Vec<PollOption>> {
    let rows = sqlx::query_as::<_, PollOption>("SELECT * FROM poll_options ORDER BY id DESC")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();
    Json(rows)
}

pub async fn create_options(
    State(pool): State<PgPool>,
    Json(po): Json<NewPollOption>,
) -> Json<PollOption> {
    let option = sqlx::query_as::<_, PollOption>(
        "INSERT INTO poll_options
        (poll_id, option_text)
        VALUES ($1, $2)
        RETURNING *",
    )
        .bind(po.poll_id)
        .bind(po.option_text)
        .fetch_one(&pool)
        .await
        .expect("Fail to insert option");

    Json(option)
}

pub async fn get_all_options_by_poll_id(State(pool): State<PgPool>, Path(id): Path<i32>) -> Json<Vec<PollOption>> {
    let rows = sqlx::query_as::<_, PollOption>(
        "SELECT * FROM poll_options WHERE poll_id = $1 ORDER BY id DESC",
    ).bind(id)
        .fetch_all(&pool)
        .await.unwrap_or_default();
    Json(rows)
}