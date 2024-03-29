mod auth;
mod auth_schema;
mod handlers;
mod helpers;
mod jwt;
mod models;
mod schema;

use std::u16;

use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use actix_web::{middleware::Logger, web, App, HttpServer};

struct AppState {
    db: Pool<Postgres>,
    secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pg_url = dotenvy::var("DATABASE_URL").unwrap();

    let _pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&pg_url)
        .await;

    let pool = match _pool {
        Ok(_pool) => {
            println!("✅ Connection to the database is successful!");
            _pool
        }
        Err(e) => {
            println!("🔥 Failed to connect to the database: {:?}", e);
            std::process::exit(1);
        }
    };

    // HTTP Server
    let server_domain = dotenvy::var("SERVER_DOMAIN").unwrap();
    let port: u16 = dotenvy::var("PORT").unwrap().parse().unwrap();
    let secret = dotenvy::var("JWT_SECRET_KEY").unwrap();

    println!("🚀 Server started successfully at http://{server_domain}:{port}/");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                secret: secret.clone(),
            }))
            .configure(handlers::config)
            .configure(auth::config)
            .wrap(Logger::default())
    })
    .bind((server_domain, port))?
    .run()
    .await
}
