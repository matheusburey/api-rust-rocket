use dotenv::dotenv;
use std::env;

pub struct Settings {
    pub db_url: String,
    pub jwt_secret: String,
}

impl Settings {
    pub fn from_env() -> Self {
        dotenv().ok();
        Self {
            db_url: env::var("DB_URL").expect("DB_URL"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET"),
        }
    }
}
