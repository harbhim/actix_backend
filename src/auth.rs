use actix_web::{post, web, HttpResponse, Responder};

use crate::auth_schema::LoginSchema;
use crate::jwt::generate_token;
use crate::models::User;
use crate::AppState;

#[post("/login")]
async fn user_login(body: web::Json<LoginSchema>, data: web::Data<AppState>) -> impl Responder {
    let record = User::authenticate(&body.email, &body.password.as_bytes(), data).await;

    match record {
        Some(user) => {
            // Generate JWT with claims
            let token = generate_token(user.id).await;
            let response = serde_json::json!({"status": "success", "access_token": token});
            HttpResponse::Ok().json(response)
        }
        None => {
            let response = serde_json::json!({"status": "error", "data": "Unauthorized"});
            HttpResponse::Forbidden().json(serde_json::json!(response))
        }
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("api/auth").service(user_login);

    conf.service(scope);
}
