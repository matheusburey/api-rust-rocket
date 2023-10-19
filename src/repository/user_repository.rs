use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::models::user_model::{NewUser, User};

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub async fn connect(url: String) -> Self {
        UserRepository {
            pool: PgPoolOptions::new()
                .max_connections(5)
                .connect(&url)
                .await
                .unwrap(),
        }
    }

    pub async fn find_user(&self, id: i32) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn find_user_by_email(&self, id: String) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM users WHERE email = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn create_user(&self, user: NewUser) -> Result<User, sqlx::Error> {
        sqlx::query_as("INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING *")
            .bind(user.name)
            .bind(user.email)
            .bind(user.password)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn update_user(&self, id: i32, user: NewUser) -> Result<User, sqlx::Error> {
        sqlx::query_as(
            "UPDATE users SET name = $1, email = $2, password = $3 WHERE id = $4 RETURNING *",
        )
        .bind(user.name)
        .bind(user.email)
        .bind(user.password)
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete_user(&self, id: i32) -> Result<User, sqlx::Error> {
        sqlx::query_as("DELETE FROM users WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }
}
