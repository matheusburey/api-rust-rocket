use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, sqlx::FromRow)]
pub struct User {
    pub username: String,
    pub name: String,
    pub password: String,
}

#[derive(Clone, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub name: String,
    pub password: String,
}

#[derive(Clone, Serialize)]
pub struct NewUserTest {
    pub username: String,
    pub name: String,
    pub password: String,
}
