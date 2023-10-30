use chrono::{serde::ts_microseconds, DateTime, Utc};
use serde::Serialize;

#[derive(Clone, Serialize, sqlx::FromRow)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    #[serde(with = "ts_microseconds")]
    pub start_at: DateTime<Utc>,
    #[serde(with = "ts_microseconds")]
    pub end_at: DateTime<Utc>,
    pub priority: String,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
}
