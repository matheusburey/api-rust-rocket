use todo_list_rs::{
    config::{database::DatabaseConn, settings::Settings},
    routes::router::app,
};

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
