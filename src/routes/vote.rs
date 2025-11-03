use crate::models::vote::{NewVote, Vote, VoteResponse};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Json, extract::State};
use serde_json::json;
use sqlx::{Error, PgPool};

pub async fn create_vote(
    State(pool): State<PgPool>,
    Json(new_vote): Json<NewVote>,
) -> Json<serde_json::Value> {
    let result = sqlx::query_as::<_, Vote>(
        "INSERT INTO votes (poll_id, option_id, voter_identifier, user_id)
         VALUES ($1, $2, $3, $4)
         RETURNING id, poll_id, option_id, voter_identifier, user_id, created_at",
    )
    .bind(new_vote.poll_id)
    .bind(new_vote.option_id)
    .bind(new_vote.voter_identifier)
    .bind(new_vote.user_id)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(vote) => Json(json!({
            "status": "success",
            "vote": vote
        })),
        Err(Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
            // 23505 = unique_violation in PostgreSQL
            Json(json!({
                "status": "error",
                "message": "You have already voted in this poll."
            }))
        }
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            Json(json!({
                "status": "error",
                "message": "Failed to record vote."
            }))
        }
    }
}

pub async fn get_votes_for_poll(
    State(pool): State<PgPool>,
    Path(poll_id): Path<i32>,
) -> Json<Vec<Vote>> {
    let result = sqlx::query_as::<_, Vote>(
        "SELECT * FROM votes WHERE poll_id = $1 ORDER BY created_at DESC",
    )
    .bind(poll_id)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    Json(result)
}

pub async fn get_vote_responses(
    State(pool): State<PgPool>,
    Path(poll_id): Path<i32>,
) -> Json<serde_json::Value> {
    let result = sqlx::query_as::<_, VoteResponse>(
        "SELECT o.id AS option_id,
       o.option_text,
       COUNT(v.id) AS vote_count
        FROM poll_options o
        LEFT JOIN votes v ON o.id = v.option_id
        WHERE o.poll_id = $1
        GROUP BY o.id, o.option_text
        ORDER BY vote_count DESC
        ",
    )
    .bind(poll_id)
    .fetch_all(&pool)
    .await;

    match result {
        Ok(vote) => {
            let total_votes: i64 = vote.iter().map(|v| v.vote_count).sum();
            Json(json!({
                "status": "success",
                "total_votes": total_votes,
                "vote": vote
            }))
        }
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            Json(json!({
                "status": "error",
                "message": "Failed to fetch vote results."
            }))
        }
    }
}
