use actix_web::cookie::{time::Duration as ActixWebDuration, Cookie};
use actix_web::{get, post, web, HttpResponse, Responder};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use sea_orm::*;

use super::auth_schema::LoginSchema;
use super::jwt::{generate_token, JWTMiddleware};

use crate::entities::prelude::*;
use crate::entities::users;
use crate::AppState;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("api/auth")
        .service(user_login)
        .service(user_logout);

    conf.service(scope);
}

pub async fn authenticate(
    email: &str,
    password: &[u8],
    db: &DatabaseConnection,
) -> Option<users::Model> {
    let record: Option<users::Model> = Users::find()
        .filter(users::Column::Email.eq(email))
        .one(db)
        .await
        .unwrap();

    match record {
        Some(_user) => {
            let parsed_hash = PasswordHash::new(&_user.password).unwrap();

            if Argon2::default()
                .verify_password(password, &parsed_hash)
                .is_ok()
            {
                Some(_user)
            } else {
                None
            }
        }
        None => None,
    }
}

#[post("/login")]
async fn user_login(body: web::Json<LoginSchema>, data: web::Data<AppState>) -> impl Responder {
    let db = &data.db;
    let record = authenticate(&body.email, &body.password.as_bytes(), db).await;

    match record {
        Some(user) => {
            // Generate JWT with claims and set cookie
            let token = generate_token(user.id, &data.settings.jwt).await;
            let cookie = Cookie::build("__tk_sess", token.to_owned())
                .path("/")
                .max_age(ActixWebDuration::new(
                    (&data.settings.jwt.access_token_lifetime_hours * 60.0 * 60.0) as i64,
                    0,
                ))
                .http_only(true)
                .finish();
            let response = serde_json::json!({"access_token": token});
            HttpResponse::Ok().cookie(cookie).json(response)
        }
        None => {
            let response = serde_json::json!({"error": "Unauthorized"});
            HttpResponse::Forbidden().json(serde_json::json!(response))
        }
    }
}

#[get("/logout")]
async fn user_logout(_: JWTMiddleware) -> impl Responder {
    let cookie = Cookie::build("__tk_sess", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({"status": "success"}))
}
