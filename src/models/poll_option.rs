use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PollOption {
    pub id: i32,
    pub poll_id: i32,
    pub option_text: String,
    pub created_at: DateTime<Utc>,
}
