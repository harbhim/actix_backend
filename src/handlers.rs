use actix_web::{delete, get, post, put, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use sea_orm::*;
use serde::Deserialize;
use uuid::Uuid;

use crate::jwt::JWTMiddleware;
use crate::AppState;

use crate::entities::prelude::*;
use crate::entities::users;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("api/users")
        .service(get_users)
        .service(create_user)
        .service(user_me)
        .service(update_user)
        .service(delete_user);

    conf.service(scope);
}

#[derive(Debug, Deserialize)]
struct Paginate {
    page: u64,
    size: u64,
}

#[get("/")]
async fn get_users(
    paginate: web::Query<Paginate>,
    data: web::Data<AppState>,
    _: JWTMiddleware,
) -> impl Responder {
    let paginator = Users::find()
        .order_by_desc(users::Column::CreatedAt)
        .paginate(&data.db, paginate.size);
    let num_pages = paginator.num_pages().await;

    let users = paginator
        .fetch_page(paginate.page - 1)
        .await
        .map(|p| (p, num_pages));
    HttpResponse::Ok().json(users.unwrap().0)
}

#[post("/")]
async fn create_user(
    body: web::Json<users::InsertModel>,
    data: web::Data<AppState>,
    _: JWTMiddleware,
) -> impl Responder {
    let obj = body.into_inner().into_active_model();
    let res = obj.save(&data.db).await.unwrap().try_into_model().unwrap();
    HttpResponse::Ok().json(res)
}

#[get("/me")]
async fn user_me(req: HttpRequest, data: web::Data<AppState>, _: JWTMiddleware) -> impl Responder {
    let extensions = req.extensions();
    let user_id = extensions.get::<Uuid>().unwrap();

    let user: Option<users::Model> = Users::find_by_id(*user_id).one(&data.db).await.unwrap();

    HttpResponse::Ok().json(user.unwrap().try_into_model().unwrap())
}

#[put("/{id}")]
async fn update_user(
    path: web::Path<Uuid>,
    body: web::Json<users::UpdateModel>,
    data: web::Data<AppState>,
    _: JWTMiddleware,
) -> impl Responder {
    let id = path.into_inner();

    let mut obj = body.into_inner().into_active_model();
    obj.id = Unchanged(id);
    let res = obj.save(&data.db).await.unwrap().try_into_model().unwrap();
    HttpResponse::Ok().json(res)
}

#[delete("/{id}")]
async fn delete_user(
    path: web::Path<Uuid>,
    data: web::Data<AppState>,
    _: JWTMiddleware,
) -> impl Responder {
    let id = path.into_inner();
    Users::delete_by_id(id).exec(&data.db).await.unwrap();
    HttpResponse::NoContent().finish()
}
