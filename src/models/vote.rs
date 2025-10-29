use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Vote {
    pub id: i32,
    pub poll_id: i32,
    pub option_id: i32,
    pub voter_identifier: String,
    pub user_id: Option<i32>,
    pub created_at: DateTime<Utc>,
}
