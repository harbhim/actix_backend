use actix_web::cookie::{time::Duration as ActixWebDuration, Cookie};
use actix_web::{get, post, web, HttpResponse, Responder};

use crate::auth_schema::LoginSchema;
use crate::jwt::{generate_token, JWTMiddleware};
use crate::models::User;
use crate::AppState;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("api/auth")
        .service(user_login)
        .service(user_logout);

    conf.service(scope);
}

#[post("/login")]
async fn user_login(body: web::Json<LoginSchema>, data: web::Data<AppState>) -> impl Responder {
    let db = &data.db;
    let record = User::authenticate(&body.email, &body.password.as_bytes(), db).await;

    match record {
        Some(user) => {
            // Generate JWT with claims and set cookie
            let secret = &data.secret;
            let token = generate_token(user.id, secret).await;
            let cookie = Cookie::build("__tk_sess", token.to_owned())
                .path("/")
                .max_age(ActixWebDuration::new(60 * 60, 0))
                .http_only(true)
                .finish();
            let response = serde_json::json!({"status": "success", "access_token": token});
            HttpResponse::Ok().cookie(cookie).json(response)
        }
        None => {
            let response = serde_json::json!({"status": "error", "data": "Unauthorized"});
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
