use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PG {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub db_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub domain: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JWT {
    pub secret_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub pg: PG,
    pub server: Server,
    pub jwt: JWT
}

pub fn get_config() -> AppConfig {
    let config_ = Config::builder()
        .add_source(::config::Environment::default().separator("__"))
        .build()
        .unwrap();
    let config: AppConfig = config_.try_deserialize().unwrap();

    config
}
