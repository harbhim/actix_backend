use ::config::Config;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use tokio_postgres::NoTls;

mod config {
    use serde::Deserialize;

    #[derive(Debug, Default, Deserialize)]
    pub struct ExampleConfig {
        pub server_addr: String,
        pub pg: deadpool_postgres::Config,
    }
}

use config::ExampleConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let _config = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: ExampleConfig = _config.try_deserialize().unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || App::new().app_data(web::Data::new(pool.clone())))
        .bind(config.server_addr.clone())?
        .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}
