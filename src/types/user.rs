use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Clone, Serialize)]
pub struct NewUserTest {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct IResLoginUser {
    pub status: String,
    pub token: String,
}
