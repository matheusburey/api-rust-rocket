mod controllers;
mod models;
mod repository;

use controllers::user_controllers::*;
use repository::user_repository::PostgresRepository;

use axum::{routing::get, Router};
use dotenv::dotenv;
use std::{env, sync::Arc};

async fn app(repo_arch: Arc<PostgresRepository>) -> Router {
    Router::new()
        .route("/users", get(get_all_persons).post(create_person))
        .route(
            "/users/:id",
            get(find_person).patch(update_person).delete(delete_person),
        )
        .with_state(repo_arch)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let repo = PostgresRepository::connect(env::var("DB_URL").expect("DB_URL")).await;
    let repo_arch = Arc::new(repo);

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app(repo_arch).await.into_make_service())
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
        dotenv().ok();
        let repo = PostgresRepository::connect(env::var("DB_URL_TEST").expect("DB_URL_TEST")).await;
        let repo_arch = Arc::new(repo);
        TestServer::new(app(repo_arch).await.into_make_service()).unwrap()
    }

    #[tokio::test]
    async fn get_all_persons_test() {
        let server = create_server().await;

        let response = server.get("/users").await;
        println!("{:?}", response.text());
        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_person_test() {
        let server = create_server().await;

        let new_user = NewUserTest {
            name: "João".to_string(),
            username: "kakashi".to_string(),
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
            username: "kakashi".to_string(),
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
