mod config;
mod controllers;
mod middleware;
mod models;
mod repository;

use config::{database::DatabaseConn, settings::Settings};
use controllers::user_controllers::*;
use middleware::auth::auth;

use axum::{
    middleware::from_fn_with_state,
    routing::{get, post},
    Router,
};

use std::sync::Arc;

async fn app(repo_arch: Arc<DatabaseConn>) -> Router {
    Router::new()
        .route("/login", post(login_user))
        .route("/users", post(create_new_user))
        .route(
            "/users",
            get(find_user)
                .patch(update_user)
                .delete(delete_user)
                .route_layer(from_fn_with_state(repo_arch.clone(), auth)),
        )
        .with_state(repo_arch)
}

#[tokio::main]
async fn main() {
    let settings = Settings::from_env();

    let conn_db = DatabaseConn::connect(settings.db_url).await;

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app(Arc::new(conn_db)).await.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::models::user_model::NewUserTest;

    use ::axum_test::TestServer;
    use axum::http::StatusCode;

    async fn create_server() -> TestServer {
        let settings = Settings::from_env();
        let repo = DatabaseConn::connect(settings.db_url).await;
        TestServer::new(app(Arc::new(repo)).await.into_make_service()).unwrap()
    }

    #[tokio::test]
    async fn create_person_test() {
        let server = create_server().await;

        let new_user = NewUserTest {
            name: "João".to_string(),
            email: "kakashi".to_string(),
            password: "123".to_string(),
        };

        let response = server.post("/users").json(&new_user).await;
        println!("{:?}", response);
        assert_eq!(response.status_code(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn find_person_test() {
        let server = create_server().await;

        let response = server.get("/users/1").await;
        println!("{:?}", response.text());
        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn update_person_test() {
        let server = create_server().await;

        let people = NewUserTest {
            name: "João".to_string(),
            email: "kakashi".to_string(),
            password: "123".to_string(),
        };

        let response = server.patch("/users/1").json(&people).await;
        println!("{:?}", response.text());
        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn delete_person_test() {
        let server = create_server().await;

        let response = server.delete("/users/1").await;
        println!("{:?}", response);
        assert_eq!(response.status_code(), StatusCode::OK);
    }
}
