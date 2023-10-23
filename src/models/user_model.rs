use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Clone, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Clone, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Serialize)]
pub struct NewUserTest {
    pub email: String,
    pub name: String,
    pub password: String,
}
