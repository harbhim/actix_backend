use chrono::Utc;
use uuid::Uuid;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

use crate::{
    helpers::pass_hash,
    models::User,
    schema::{CreateUserSchema, FilterOptions, UpdateUserSchema},
    AppState,
};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("api/users")
        .service(get_users)
        .service(create_user)
        .service(update_user)
        .service(delete_user);

    conf.service(scope);
}

#[get("/")]
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

    let response = serde_json::json!({"status": "success", "data": query_result.unwrap()});
    HttpResponse::Ok().json(response)
}

#[post("/")]
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
            let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "user": user
            })});

            return HttpResponse::Ok().json(user_response);
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

#[put("/{id}")]
async fn update_user(
    path: web::Path<Uuid>,
    body: web::Json<UpdateUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let id = path.into_inner();

    let query_result = sqlx::query_as!(User, "SELECT * FROM users WHERE id=$1", id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let msg = format!("User with id {} not found", id);
        return HttpResponse::NotFound().json(serde_json::json!({"error": msg}));
    }

    let now = Utc::now();
    let user = query_result.unwrap();
    let query_result = sqlx::query_as!(
        User,
        "UPDATE users SET first_name = $1, last_name = $2, email = $3, updated_at = $4 WHERE id = $5 RETURNING *", 
        body.first_name.to_owned().unwrap_or(user.first_name.unwrap()), 
        body.last_name.to_owned().unwrap_or(user.last_name.unwrap()), 
        body.email.to_owned(), 
        now, 
        id 
    ) 
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(data) => {
            HttpResponse::Ok().json(serde_json::json!({"data": data}))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": format!("{:?}", e)}))
        }
    }
}

#[delete("/{id}")]
async fn delete_user(
    path: web::Path<Uuid>,
    data: web::Data<AppState>
) -> impl Responder {
    let id = path.into_inner();

    let rows_affected = sqlx::query_as!(
        User,
        "DELETE FROM users WHERE id = $1;",
        id
    )
    .execute(&data.db)
    .await
    .unwrap()
    .rows_affected(); 

    if rows_affected == 0 {
        let message = format!("User with ID: {} not found", id);
        return HttpResponse::NotFound().json(serde_json::json!({"status": "fail","message": message}));
    }
    HttpResponse::NoContent().finish()
}
