use actix_web::{delete, get, post, put, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use sea_orm::*;
use uuid::Uuid;

use crate::auth::jwt::JWTMiddleware;
use crate::entities::prelude::*;
use crate::entities::users;
use crate::helpers::{get_paginated_result, Paginate};
use crate::AppState;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("api/users")
        .service(get_users)
        .service(create_user)
        .service(user_me)
        .service(update_user)
        .service(delete_user);

    conf.service(scope);
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

    let res = get_paginated_result(paginator, paginate.page).await;
    HttpResponse::Ok().json(res)
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
