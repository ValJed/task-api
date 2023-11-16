use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx;

#[derive(Serialize, Deserialize)]
struct Article {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub content: String,
}

#[post("/articles")]
pub async fn create_article() -> impl Responder {
    HttpResponse::Ok().body("create article")
}

#[put("/articles")]
pub async fn update_article() -> impl Responder {
    HttpResponse::Ok().body("update article")
}

#[delete("/articles")]
pub async fn delete_article() -> impl Responder {
    HttpResponse::Ok().body("delete article")
}

#[get("/{id}")]
pub async fn fetch_article() -> impl Responder {
    HttpResponse::Ok().body("fetch article")
}

#[get("/")]
pub async fn fetch_articles() -> impl Responder {
    HttpResponse::Ok().body("fetch articles")
}
