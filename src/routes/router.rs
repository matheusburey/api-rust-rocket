use crate::types::database::IArchDb;

use axum::Router;

use super::{login_routes, user_routes, task_routes};

pub async fn app(repo_arch: IArchDb) -> Router {
    Router::new()
        .nest("/login", login_routes::login_routes(repo_arch.clone()).await)
        .nest("/users", user_routes::user_routes(repo_arch.clone()).await)
        .nest("/tasks", task_routes::task_routes(repo_arch).await)
}
