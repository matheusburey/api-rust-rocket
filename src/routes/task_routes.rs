use crate::middleware::auth::auth;
use crate::{controllers::task_controllers::*, types::database::IArchDb};

use axum::{middleware::from_fn_with_state, routing::get, Router};

pub async fn task_routes(repo_arch: IArchDb) -> Router {
    Router::new()
        .route("/", get(get_all_tasks).post(create_new_task))
        .route(
            "/:task_id",
            get(find_task).patch(update_task).delete(delete_task),
        )
        .route_layer(from_fn_with_state(repo_arch.clone(), auth))
        .with_state(repo_arch)
}
