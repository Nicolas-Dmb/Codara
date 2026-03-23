mod config;
mod services;

#[tokio::main]
async fn main(){
    let cfg = config::load_env::load_env();
    services::db::init_db(&cfg.database_url)
        .await.expect("Failed to initialize database connection pool");
    println!("Database connection pool initialized successfully");
}
