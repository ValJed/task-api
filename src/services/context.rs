#[path = "../structs.rs"]
mod structs;

use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder, Scope};
use sqlx;
use structs::AppState;

pub fn get_scope() -> Scope {
    web::scope("/context")
        .service(fetch_all)
        .service(fetch_or_create)
        .service(update)
        .service(delete)
}

#[get("")]
pub async fn fetch_all(data: AppState) -> impl Responder {
    println!("data: {:?}", data);
    HttpResponse::Ok().body("fetch contexts")
}

#[post("")]
pub async fn fetch_or_create(data: AppState) -> impl Responder {
    HttpResponse::Ok().body("create context")
}

#[put("/{id}")]
pub async fn update(data: AppState) -> impl Responder {
    HttpResponse::Ok().body("update context")
}

#[delete("/")]
pub async fn delete(data: AppState) -> impl Responder {
    HttpResponse::Ok().body("delete context")
}
