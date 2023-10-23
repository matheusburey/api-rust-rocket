use crate::middleware::auth::auth;
use crate::{controllers::user_controllers::*, types::database::IArchDb};

use axum::{
    middleware::from_fn_with_state,
    routing::{get, post},
    Router,
};

pub async fn user_routes(repo_arch: IArchDb) -> Router {
    Router::new()
        .route("/", post(create_new_user))
        .route(
            "/",
            get(find_user)
                .patch(update_user)
                .delete(delete_user)
                .route_layer(from_fn_with_state(repo_arch.clone(), auth)),
        )
        .with_state(repo_arch)
}
