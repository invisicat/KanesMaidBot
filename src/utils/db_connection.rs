use dotenv::dotenv;
use std::env;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};


async fn init_pool(database_url: &str) -> Pool<Postgres> {
   PgPoolOptions::new()
       .max_connections(10)
       .connect(database_url).await?
}

pub async fn establish_connection() -> Pool<Postgres> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    init_pool(&database_url).await.expect("Failed to create pool")
}
