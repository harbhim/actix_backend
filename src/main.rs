mod auth;
mod auth_schema;
mod entities;
mod handlers;
mod jwt;
mod migrator;
mod schema;

use crate::migrator::Migrator;
use dotenvy::dotenv;
use entities::{prelude::*, *};
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

    let db_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let secret = dotenvy::var("JWT_SECRET_KEY").unwrap();
    let server_domain = dotenvy::var("SERVER_DOMAIN").unwrap();
    let port: u16 = dotenvy::var("PORT").unwrap().parse().unwrap();

    let db = Database::connect(&db_url).await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    let state = AppState { db, secret };

    println!("🚀 Server started successfully at http://{server_domain}:{port}/");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(auth::config)
            .configure(handlers::config)
            .wrap(Logger::default())
    })
    .bind((server_domain, port))?
    .run()
    .await
}
