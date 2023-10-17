use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::models::user_model::{NewUser, User};

pub struct PostgresRepository {
    pool: PgPool,
}

impl PostgresRepository {
    pub async fn connect(url: String) -> Self {
        PostgresRepository {
            pool: PgPoolOptions::new()
                .max_connections(5)
                .connect(&url)
                .await
                .unwrap(),
        }
    }

    pub async fn find_person(&self, id: i32) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn get_all_persons(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM users")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn create_person(&self, person: NewUser) -> Result<User, sqlx::Error> {
        sqlx::query_as(
            "INSERT INTO users (name, username, password) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(person.name)
        .bind(person.username)
        .bind(person.password)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update_person(&self, id: i32, person: NewUser) -> Result<User, sqlx::Error> {
        sqlx::query_as(
            "UPDATE users SET name = $1, username = $2, password = $3 WHERE id = $4 RETURNING *",
        )
        .bind(person.name)
        .bind(person.username)
        .bind(person.password)
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete_person(&self, id: i32) -> Result<User, sqlx::Error> {
        sqlx::query_as("DELETE FROM users WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }
}
