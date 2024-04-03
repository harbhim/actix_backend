use config::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PG {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub db_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub domain: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JWT {
    pub secret_key: String,
    pub access_token_lifetime_hours: f32,
    pub refresh_token_lifetime_hours: f32,
    pub algorithm: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub pg: PG,
    pub server: Server,
    pub jwt: JWT,
}

pub fn get_config() -> AppConfig {
    let config_ = Config::builder()
        .add_source(::config::Environment::default().separator("__"))
        .build()
        .unwrap();
    let config: AppConfig = config_.try_deserialize().unwrap();

    config
}
