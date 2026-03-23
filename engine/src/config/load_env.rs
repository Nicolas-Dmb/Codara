use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub vps_port: String,
    pub vps_ip: String,
    pub vps_user: String,
}

pub fn load_env() -> Config {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let vps_port = env::var("VPS_PORT").expect("VPS_PORT must be set");
    let vps_ip = env::var("VPS_IP").expect("VPS_IP must be set");
    let vps_user = env::var("VPS_USER").expect("VPS_USER must be set");

    return Config {
        database_url,
        vps_port,
        vps_ip,
        vps_user,
    }
}