use sqlx::PgPool;

use crate::{models::task_model::Task, types::task::NewTask};

pub struct TaskRepository {
    pool: PgPool,
}

impl TaskRepository {
    pub async fn new(pool: PgPool) -> Self {
        TaskRepository { pool }
    }

    pub async fn find_task(&self, id: i32) -> Result<Option<Task>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM tasks WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn get_all_tasks(&self) -> Result<Vec<Task>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM tasks")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn create_task(&self, task: NewTask) -> Result<Task, sqlx::Error> {
        sqlx::query_as("INSERT INTO tasks (title, description, start_at, end_at, priority, user_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *")
            .bind(task.title)
            .bind(task.description)
            .bind(task.start_at)
            .bind(task.end_at)
            .bind(task.priority)
            .bind(task.user_id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn update_task(&self, id: i32, task: NewTask) -> Result<Task, sqlx::Error> {
        sqlx::query_as(
            "UPDATE tasks SET title = $1, description = $2, start_at = $3, end_at = $4, priority = $5 WHERE id = $6 RETURNING *",
        )
        .bind(task.title)
        .bind(task.description)
        .bind(task.start_at)
        .bind(task.end_at)
        .bind(task.priority)
        .bind(task.user_id)
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete_task(&self, id: i32) -> Result<Task, sqlx::Error> {
        sqlx::query_as("DELETE FROM tasks WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }
}
