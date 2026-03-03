use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;

const MAX_CONNECTIONS: u32 = 15;

pub struct Db {
    pool: PgPool,
}

impl Db {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

        let pool = PgPoolOptions::new()
            .max_connections(MAX_CONNECTIONS)
            .connect(&database_url)
            .await?;

        Ok(Db { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
