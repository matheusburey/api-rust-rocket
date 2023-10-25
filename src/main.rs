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

    sqlx::migrate!("./migrations")
        .run(&conn_db.pool())
        .await
        .unwrap();

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app(Arc::new(conn_db)).await.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use crate::types::user::{IResLoginUser, NewUserTest};

    use super::*;

    use ::axum_test::TestServer;
    use axum::http::{HeaderName, HeaderValue, StatusCode};

    async fn create_server() -> TestServer {
        let settings = Settings::from_env();
        let repo = DatabaseConn::connect(settings.db_url).await;
        sqlx::migrate!("./migrations")
            .run(&repo.pool())
            .await
            .unwrap();

        TestServer::new(app(Arc::new(repo)).await.into_make_service()).unwrap()
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_suite() {
        create_person_test().await;

        let token = login_test().await;
        let bearer_token: &'static str = Box::leak(format!("Bearer {}", token).into_boxed_str());
        println!("logado {}", bearer_token);
        find_person_test(bearer_token).await;
        println!("find_person_test");
        update_person_test(bearer_token).await;
        println!("update_person_test");
        delete_person_test(bearer_token).await;
        println!("delete_person_test");
    }

    async fn create_person_test() {
        let server = create_server().await;
        let new_user = NewUserTest {
            name: "João".to_string(),
            email: "teste@gmail.com".to_string(),
            password: "1234".to_string(),
        };

        let response = server.post("/users").json(&new_user).await;
        assert_eq!(response.status_code(), StatusCode::CREATED);
    }

    async fn login_test() -> String {
        let server = create_server().await;
        let data_login = serde_json::json!({"email": "teste@gmail.com", "password": "1234"});
        let response = server.post("/login").json(&data_login).await;
        assert_eq!(response.status_code(), StatusCode::OK);

        let response_body: IResLoginUser = serde_json::from_str(&response.text()).unwrap();
        response_body.token
    }

    async fn find_person_test(token: &'static str) {
        let server = create_server().await;
        let response = server
            .get("/users")
            .add_header(
                HeaderName::from_static("authorization"),
                HeaderValue::from_static(token),
            )
            .await;
        println!("{:?}", response.text());
        assert_eq!(response.status_code(), StatusCode::OK);
    }

    async fn update_person_test(token: &'static str) {
        let server = create_server().await;
        let people = NewUserTest {
            name: "João".to_string(),
            email: "kakashi".to_string(),
            password: "1234".to_string(),
        };

        let response = server
            .patch("/users")
            .json(&people)
            .add_header(
                HeaderName::from_static("authorization"),
                HeaderValue::from_static(token),
            )
            .await;
        println!("{:?}", response.text());
        assert_eq!(response.status_code(), StatusCode::OK);
    }

    async fn delete_person_test(token: &'static str) {
        let server = create_server().await;
        let response = server
            .delete("/users")
            .add_header(
                HeaderName::from_static("authorization"),
                HeaderValue::from_static(token),
            )
            .await;
        println!("{:?}", response);
        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);
    }
}
