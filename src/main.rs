mod handlers;

use std::u16;

use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use actix_web::{web, App, HttpServer};

struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Get PostgreSQL pool
    let pg_host = dotenvy::var("PG_HOST").unwrap();
    let pg_port = dotenvy::var("PG_PORT").unwrap();
    let pg_user = dotenvy::var("PG_USER").unwrap();
    let pg_password = dotenvy::var("PG_PASSWORD").unwrap();
    let pg_dbname = dotenvy::var("PG_DBNAME").unwrap();

    let pg_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        pg_user, pg_password, pg_host, pg_port, pg_dbname
    );

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

    println!("🚀 Server started successfully");

    // HTTP Server
    let server_domain = dotenvy::var("SERVER_DOMAIN").unwrap();
    let port: u16 = dotenvy::var("PORT").unwrap().parse().unwrap();

    HttpServer::new(move || App::new().app_data(web::Data::new(AppState { db: pool.clone() })))
        .bind((server_domain, port))?
        .run()
        .await
}
