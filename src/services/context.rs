use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder, Scope};
use serde::{Deserialize, Serialize};
use sqlx;

#[derive(Serialize, Deserialize)]
struct Context {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub content: String,
}

pub fn get_scope() -> Scope {
    print!("set scope context");
    web::scope("/context")
        .service(fetch_all)
        // .service(fetch_one)
        .service(fetch_or_create)
        .service(update)
        .service(delete)
}

#[get("/")]
pub async fn fetch_all() -> impl Responder {
    println!("fetch_all");
    HttpResponse::Ok().body("fetch articles")
}

#[post("/")]
pub async fn fetch_or_create(req: HttpRequest) -> impl Responder {
    println!("req: {:?}", req);
    HttpResponse::Ok().body("create article")
}

#[put("/{id}")]
pub async fn update() -> impl Responder {
    HttpResponse::Ok().body("update article")
}

#[delete("/")]
pub async fn delete() -> impl Responder {
    HttpResponse::Ok().body("delete article")
}
