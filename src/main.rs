mod config;
mod controllers;
mod middleware;
mod models;
mod repository;
mod routes;
mod types;

use config::{database::DatabaseConn, settings::Settings};
use routes::router::app;

use std::sync::Arc;

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
