use serde_json::json;

use actix_web::{get, post, web, HttpResponse, Responder};

use crate::{
    helpers::pass_hash,
    models::User,
    schema::{CreateUserSchema, FilterOptions},
    AppState,
};

#[get("/users/")]
async fn get_users(params: web::Query<FilterOptions>, data: web::Data<AppState>) -> impl Responder {
    let limit = params.limit.unwrap_or(10);
    let offset = (params.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        User,
        "SELECT * FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        limit as i64,
        offset as i64
    )
    .fetch_all(&data.db)
    .await;

    let response = json!({"status": "success", "data": query_result.unwrap()});
    HttpResponse::Ok().json(response)
}

#[post("/users/")]
async fn create_user(
    body: web::Json<CreateUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    // let now = Utc::now();

    let password = body.password.as_bytes();
    let hash = pass_hash(&password);

    let query_result = sqlx::query_as!(
        User,
        "INSERT INTO users (first_name, last_name, email, password) VALUES ($1, $2, $3, $4) RETURNING *",
        body.first_name,
        body.last_name,
        body.email,
        hash,
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(user) => {
            let feedback_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "user": user
            })});

            return HttpResponse::Ok().json(feedback_response);
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "fail","message": "Feedback with that title already exists"}));
            }

            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api").service(get_users).service(create_user);

    conf.service(scope);
}
