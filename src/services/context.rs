use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder, Scope};
use sqlx;

pub fn get_scope() -> Scope {
    web::scope("/context")
        .service(fetch_all)
        .service(fetch_or_create)
        .service(update)
        .service(delete)
}

#[get("")]
pub async fn fetch_all(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("fetch contexts")
}

#[post("")]
pub async fn fetch_or_create(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("create context")
}

#[put("/{id}")]
pub async fn update() -> impl Responder {
    HttpResponse::Ok().body("update context")
}

#[delete("/")]
pub async fn delete() -> impl Responder {
    HttpResponse::Ok().body("delete context")
}
