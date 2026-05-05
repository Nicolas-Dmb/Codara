mod config;
mod services;
mod model;
mod analysis;
mod adapters;
mod persistence;


#[tokio::main]
async fn main(){
    let cfg = config::load_env::load_env();
    let pool = services::db::init_pool(&cfg.database_url)
        .await.expect("Failed to initialize database connection pool");
    services::listener::start_listener(pool).await;
}