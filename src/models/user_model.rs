use serde::Serialize;

#[derive(Clone, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    #[serde(skip_serializing)]
    pub password: String,
}
