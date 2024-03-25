#[path = "../structs.rs"]
mod structs;

use actix_web::{delete, get, post, web, HttpResponse, Responder, Result, Scope};
use sqlx::{Pool, Postgres};
use structs::{Context, ContextRequest};

pub fn get_scope() -> Scope {
    web::scope("/context")
        .service(fetch_all)
        .service(fetch_or_create)
        .service(delete)
    // .service(update)
}

#[get("")]
pub async fn fetch_all(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let contexts_res: Result<Vec<Context>, sqlx::Error> = sqlx::query_as("SELECT * FROM context")
        .fetch_all(pool.get_ref())
        .await;

    match contexts_res {
        Ok(contexts) => HttpResponse::Ok().json(contexts),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[post("")]
pub async fn fetch_or_create(
    pool: web::Data<Pool<Postgres>>,
    data: web::Json<ContextRequest>,
) -> impl Responder {
    if data.name.is_empty() {
        return HttpResponse::BadRequest().body("Name is required");
    };

    let existing: Result<Option<Context>, sqlx::Error> =
        sqlx::query_as("SELECT * FROM context WHERE name = $1")
            .bind(data.name.clone())
            .fetch_optional(pool.get_ref())
            .await;

    if existing.is_err() {
        return HttpResponse::InternalServerError().body("Internal Server Error");
    }

    let ctx = existing.unwrap();

    if ctx.is_some() {
        return HttpResponse::Ok().json(ctx.unwrap());
    }

    let context: Result<Context, sqlx::Error> =
        sqlx::query_as("INSERT INTO context (name) VALUES ($1) RETURNING *")
            .bind(data.name.clone())
            .fetch_one(pool.get_ref())
            .await;

    if context.is_err() {
        return HttpResponse::InternalServerError().body("Internal Server Error");
    }

    HttpResponse::Ok().json(context.unwrap())
}

#[delete("/{id}")]
pub async fn delete(pool: web::Data<Pool<Postgres>>, id: web::Path<i32>) -> impl Responder {
    let deleted: Result<Context, sqlx::Error> =
        sqlx::query_as("DELETE FROM context WHERE id = $1 RETURNING * ")
            .bind(*id)
            .fetch_one(pool.get_ref())
            .await;

    match deleted {
        Ok(ctx) => {
            return HttpResponse::Ok().json(ctx);
        }
        Err(err) => {
            // TODO: Handle multiple error types
            println!("err: {:?}", err);
            return HttpResponse::NotFound().body("Not Found");
        }
    }
}

// Not needed for now
// #[put("/{id}")]
// pub async fn update(pool: web::Data<Pool<Postgres>>, req: HttpRequest) -> impl Responder {
//     HttpResponse::Ok().body("update context")
// }
