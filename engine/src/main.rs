mod config;

fn main() {
    let cfg = config::load_env::load_env();
    println!("Database URL: {}", cfg.database_url);
    println!("VPS Port: {}", cfg.vps_port);
    println!("VPS IP: {}", cfg.vps_ip);
    println!("VPS User: {}", cfg.vps_user);
}
