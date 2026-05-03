use sqlx::postgres::PgPoolOptions;
use crate::model::ServiceError;

const MAX_CONNECTIONS: u32 = 5;

pub async fn init_pool(db_url: &str) -> Result<sqlx::PgPool, ServiceError> {
    let pool = PgPoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect(db_url)
        .await
        .map_err(|_| ServiceError::DatabaseInitializationFailed)?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|_| ServiceError::MigrationFailed("Failed to run migrations".into()))?;

    Ok(pool)
}