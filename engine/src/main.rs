mod config;
mod services;
mod model;
mod analysis;
mod adapters;
mod persistence;
use tracing::{info, error};


#[tokio::main]
async fn main(){
    tracing_subscriber::fmt::init();
    let cfg = config::load_env::load_env();
    let pool = match services::db::init_pool(&cfg.database_url)
        .await {
            Ok(pool) => pool,
            Err(e) => {
                error!("Failed to initialize database connection pool: {}", e);
                std::process::exit(1);
            }
        };
    info!("Database connection pool initialized successfully, starting listener...");
    services::listener::start_listener(pool).await;
}