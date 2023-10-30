use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct NewTask {
    pub title: String,
    pub description: String,
    pub user_id: i32,
    #[serde(with = "ts_milliseconds")]
    pub start_at: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub end_at: DateTime<Utc>,
    pub priority: String,
}
