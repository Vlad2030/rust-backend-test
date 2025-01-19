use crate::core;

pub type DbPool = sqlx::PgPool;
pub type DbPoolState = ntex::web::types::State<sqlx::PgPool>;

pub async fn create_pool(database_url: &str) -> core::result::Result<DbPool> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .map_err(|_| core::error::Error::Database {
            error: database_url.into(),
        })
}
