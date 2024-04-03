mod auth;
mod conf;
mod entities;
mod helpers;
mod migrator;
mod users;

use crate::conf::get_config;
use crate::migrator::Migrator;

use dotenvy::dotenv;
use env_logger::Env;
use sea_orm::prelude::*;
use sea_orm::Database;
use sea_orm_migration::prelude::*;

use actix_web::{middleware::Logger, web, App, HttpServer};

#[derive(Debug, Clone)]
struct AppState {
    db: DatabaseConnection,
    settings: conf::AppConfig,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let settings = get_config();

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        settings.pg.username,
        settings.pg.password,
        settings.pg.host,
        settings.pg.port,
        settings.pg.db_name
    );

    let db = Database::connect(&db_url).await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    let state = AppState {
        db,
        settings: settings.clone(),
    };

    println!(
        "🚀 Server started successfully at http://{}:{}/",
        settings.server.domain, settings.server.port
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(auth::auth_handlers::config)
            .configure(users::handlers::config)
            .wrap(Logger::default())
    })
    .bind((settings.server.domain, settings.server.port))?
    .run()
    .await
}
