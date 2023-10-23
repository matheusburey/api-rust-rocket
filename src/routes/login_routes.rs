use crate::{controllers::user_controllers::login_user, types::database::IArchDb};

use axum::{routing::post, Router};

pub async fn login_routes(repo_arch: IArchDb) -> Router {
    Router::new()
        .route("/", post(login_user))
        .with_state(repo_arch)
}
