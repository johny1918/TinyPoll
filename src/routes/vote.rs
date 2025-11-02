use crate::models::vote::{NewVote, Vote};
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
