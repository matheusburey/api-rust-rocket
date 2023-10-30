use crate::repository::task_repository::TaskRepository;
use crate::types::database::AppState;
use crate::types::task::NewTask;

use axum::extract::{Json, Path, State};
use axum::{http::StatusCode, response::IntoResponse};

pub async fn find_task(State(db): AppState, Path(task_id): Path<i32>) -> impl IntoResponse {
    let task_repo: TaskRepository = TaskRepository::new(db.pool()).await;
    match task_repo.find_task(task_id).await {
        Ok(Some(task)) => Ok(Json(task)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_all_tasks(State(db): AppState) -> impl IntoResponse {
    let task_repo: TaskRepository = TaskRepository::new(db.pool()).await;
    match task_repo.get_all_tasks().await {
        Ok(task) => Ok(Json(task)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_new_task(
    State(db): AppState,
    Json(new_task): Json<NewTask>,
) -> impl IntoResponse {
    let task_repo: TaskRepository = TaskRepository::new(db.pool()).await;

    match task_repo.create_task(new_task).await {
        Ok(task) => Ok((StatusCode::CREATED, Json(task))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_task(
    State(db): AppState,
    Path(task_id): Path<i32>,
    Json(new_task): Json<NewTask>,
) -> impl IntoResponse {
    let task_repo: TaskRepository = TaskRepository::new(db.pool()).await;
    match task_repo.update_task(task_id, new_task).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_task(State(db): AppState, Path(task_id): Path<i32>) -> impl IntoResponse {
    let task_repo: TaskRepository = TaskRepository::new(db.pool()).await;
    match task_repo.delete_task(task_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
