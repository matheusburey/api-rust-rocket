use sqlx::{postgres::PgPoolOptions, PgPool};

pub struct DatabaseConn {
    pool: PgPool,
}

impl DatabaseConn {
    pub async fn connect(url: String) -> Self {
        DatabaseConn {
            pool: PgPoolOptions::new()
                .max_connections(5)
                .connect(&url)
                .await
                .unwrap(),
        }
    }

    pub fn pool(&self) -> PgPool {
        self.pool.clone()
    }
}
