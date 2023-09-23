use async_once::AsyncOnce;
use lazy_static::lazy_static;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use crate::shared::utils::load_configs::ConfigApp;
use crate::shared::errors::{CustomError, Result};

pub type DbPool = Pool<Postgres>;

lazy_static! {
    static ref DB_POOL: AsyncOnce<Result<DbPool>> = AsyncOnce::new(
        async { create_connection().await}
    );
}

pub async fn create_connection() -> Result<DbPool> {
    let config = ConfigApp::new();

    let db_url = config.db.db_url;
    let max_connection = config.db.max_connection;

    PgPoolOptions::new()
        .max_connections(max_connection as u32)
        .connect(&db_url)
        .await.map_err(CustomError::from)
}

pub async fn get_connection() -> Result<DbPool> {
    DB_POOL.get().await.clone()
}
