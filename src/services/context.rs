#[path = "../structs.rs"]
mod structs;

use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder, Result, Scope};
use sqlx::{postgres::types::PgLQueryLevel, Pool, Postgres};
use structs::{Context, ContextRequest, RequestError};

pub fn get_scope() -> Scope {
    web::scope("/context")
        .service(fetch_all)
        .service(fetch_or_create)
        .service(update)
        .service(delete)
}

#[get("")]
pub async fn fetch_all(pool: web::Data<Pool<Postgres>>) -> Result<impl Responder, RequestError> {
    let contexts: Result<Vec<Context>, sqlx::Error> = sqlx::query_as("SELECT * FROM context")
        .fetch_all(pool.get_ref())
        .await;

    if contexts.is_err() {
        return Err(RequestError::InternalServerError);
    }

    Ok(web::Json(contexts.unwrap()))
}

#[post("")]
pub async fn fetch_or_create(
    pool: web::Data<Pool<Postgres>>,
    data: web::Json<ContextRequest>,
) -> Result<impl Responder, RequestError> {
    if data.name.is_empty() {
        return Err(RequestError::BadRequest);
    };

    let existing: Result<Option<Context>, sqlx::Error> =
        sqlx::query_as("SELECT * FROM context WHERE name = $1")
            .bind(data.name.clone())
            .fetch_optional(pool.get_ref())
            .await;

    if existing.is_err() {
        return Err(RequestError::InternalServerError);
    }

    let ctx = existing.unwrap();

    println!("ctx: {:?}", ctx);
    if ctx.is_some() {
        return Ok(web::Json(ctx.unwrap()));
    }

    let context: Result<Context, sqlx::Error> =
        sqlx::query_as("INSERT INTO context (name) VALUES ($1)")
            .bind(data.name.clone())
            .fetch_one(pool.get_ref())
            .await;

    if context.is_err() {
        return Err(RequestError::InternalServerError);
    }

    Ok(web::Json(context.unwrap()))
}

#[put("/{id}")]
pub async fn update(pool: web::Data<Pool<Postgres>>, req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("update context")
}

#[delete("/")]
pub async fn delete(pool: web::Data<Pool<Postgres>>, req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("delete context")
}
