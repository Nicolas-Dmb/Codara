use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
}

pub fn load_env() -> Config {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    return Config {
        database_url,
    }
}