use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Poll {
    pub id: i32,
    pub question: String,
    pub created_by: Option<i32>,
    pub created_at: DateTime<Utc>,
}
