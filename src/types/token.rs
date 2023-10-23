use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ITokenClaims {
    pub sub: i32,
    pub iat: usize,
    pub exp: usize,
}
