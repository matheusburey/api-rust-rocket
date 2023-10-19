use dotenv::dotenv;
use std::env;

pub struct Settings {
    pub db_url: String,
}

impl Settings {
    pub fn from_env() -> Self {
        dotenv().ok();
        Self {
            db_url: env::var("DB_URL").expect("DB_URL"),
        }
    }
}
