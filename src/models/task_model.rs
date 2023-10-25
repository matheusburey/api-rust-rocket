use chrono::serde::ts_seconds_option;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    #[serde(with = "ts_seconds_option")]
    pub start_at: DateTime,
    #[serde(with = "ts_seconds_option")]
    pub end_at: DateTime,
    pub priority: String,
    pub user_id: i32,
    pub created_at: DateTime,
}

#[derive(Clone, Deserialize)]
pub struct NewTask {
    pub title: String,
    pub description: String,
    #[serde(with = "ts_seconds_option")]
    pub start_at: DateTime,
    #[serde(with = "ts_seconds_option")]
    pub end_at: DateTime,
    pub priority: String,
    pub user_id: i32,
}
