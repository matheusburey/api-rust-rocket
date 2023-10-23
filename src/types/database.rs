use axum::extract::State;
use std::sync::Arc;

use crate::config::database::DatabaseConn;

pub type IArchDb = Arc<DatabaseConn>;

pub type AppState = State<Arc<DatabaseConn>>;
