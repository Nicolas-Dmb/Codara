use sqlx::postgres::PgPoolOptions;

// Max number of connections could be set to 5 for now, but it can be changed later if needed
const MAX_CONNECTIONS: u32 = 5;

pub async fn init_db(db_url: &str) -> sqlx::Result<sqlx::PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect(db_url).await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    
    Ok(pool)
}