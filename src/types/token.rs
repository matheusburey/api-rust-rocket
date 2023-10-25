use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ITokenClaims {
    pub sub: i32,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Clone, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}
