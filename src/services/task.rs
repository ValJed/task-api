use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Scope};
use serde::{Deserialize, Serialize};
use sqlx;

#[derive(Serialize, Deserialize)]
struct Task {
    id: usize,
    name: String,
    done: bool,
    creation_date: String,
    modification_date: String,
}

pub fn get_scope() -> Scope {
    web::scope("/task")
        .service(fetch_all)
        .service(fetch_one)
        .service(create)
        .service(update)
        .service(delete)
}

#[get("/")]
pub async fn fetch_all() -> impl Responder {
    HttpResponse::Ok().body("fetch task")
}

#[get("/{id}")]
pub async fn fetch_one() -> impl Responder {
    HttpResponse::Ok().body("fetch article")
}

#[post("/")]
pub async fn create() -> impl Responder {
    HttpResponse::Ok().body("create article")
}

#[put("/")]
pub async fn update() -> impl Responder {
    HttpResponse::Ok().body("update article")
}

#[delete("/")]
pub async fn delete() -> impl Responder {
    HttpResponse::Ok().body("delete article")
}
