mod auth;
mod conf;
mod entities;
mod migrator;
mod users;

use crate::conf::get_config;
use crate::migrator::Migrator;
use crate::users::handlers;

use dotenvy::dotenv;
use env_logger::Env;
use sea_orm::prelude::*;
use sea_orm::Database;
use sea_orm_migration::prelude::*;

use actix_web::{middleware::Logger, web, App, HttpServer};

#[derive(Debug, Clone)]
struct AppState {
    db: DatabaseConnection,
    secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let settings = get_config();
    println!("{settings:#?}");
    let pg = settings.pg;
    let server_ = settings.server;
    let jwt_ = settings.jwt;

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        pg.username, pg.password, pg.host, pg.port, pg.db_name
    );

    let db = Database::connect(&db_url).await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    let state = AppState {
        db,
        secret: jwt_.secret_key,
    };

    println!(
        "🚀 Server started successfully at http://{}:{}/",
        server_.domain, server_.port
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(auth::auth_handlers::config)
            .configure(handlers::config)
            .wrap(Logger::default())
    })
    .bind((server_.domain, server_.port))?
    .run()
    .await
}
