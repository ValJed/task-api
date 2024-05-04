#[path = "../structs.rs"]
mod structs;

#[path = "../utils.rs"]
mod utils;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result, Scope};
use sqlx::{Pool, Postgres};
use structs::{Context, ContextRequest};
use utils::handle_err;

pub fn get_scope() -> Scope {
    web::scope("/context")
        .service(fetch_all)
        .service(use_or_create)
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
pub async fn use_or_create(
    pool: web::Data<Pool<Postgres>>,
    data: web::Json<ContextRequest>,
) -> impl Responder {
    if data.name.is_empty() {
        return HttpResponse::BadRequest().body("Name is required");
    };

    if data.simple_create.is_some() && data.simple_create.unwrap() {
        let context: Result<Context, sqlx::Error> =
            sqlx::query_as("INSERT INTO context (name) VALUES ($1) RETURNING *")
                .bind(data.name.clone())
                .fetch_one(pool.get_ref())
                .await;

        if context.is_err() {
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }

        return HttpResponse::Ok().json(context.unwrap());
    }

    let update_req = r#"
        UPDATE context
        SET active = true
        WHERE name = $1
        RETURNING *"#;

    let existing: Result<Option<Context>, sqlx::Error> = sqlx::query_as(update_req)
        .bind(data.name.clone())
        .fetch_optional(pool.get_ref())
        .await;

    if existing.is_err() {
        return HttpResponse::InternalServerError().body("Internal Server Error");
    }

    let _unset_active =
        sqlx::query("UPDATE context SET active = false WHERE active = true AND name != $1")
            .bind(data.name.clone())
            .execute(pool.get_ref())
            .await;

    let ctx = existing.unwrap();

    if ctx.is_some() {
        return HttpResponse::Ok().json(ctx.unwrap());
    }

    let context: Result<Context, sqlx::Error> =
        sqlx::query_as("INSERT INTO context (name, active) VALUES ($1, $2) RETURNING *")
            .bind(data.name.clone())
            .bind(true)
            .fetch_one(pool.get_ref())
            .await;

    if context.is_err() {
        return HttpResponse::InternalServerError().body("Internal Server Error");
    }

    HttpResponse::Ok().json(context.unwrap())
}

#[post("/clear/{id}")]
pub async fn clear(pool: web::Data<Pool<Postgres>>, id: web::Path<i32>) -> impl Responder {
    let deleted_tasks = sqlx::query("DELETE FROM task WHERE context_id = $1")
        .bind(*id)
        .execute(pool.get_ref())
        .await;

    println!("deleted_tasks: {:?}", deleted_tasks);
    if deleted_tasks.is_err() {
        return HttpResponse::InternalServerError().body("Internal Server Error");
    }

    HttpResponse::Ok().body("Context cleared")
}

#[delete("/{id}")]
pub async fn delete(pool: web::Data<Pool<Postgres>>, id: web::Path<i32>) -> impl Responder {
    let deleted_tasks = sqlx::query("DELETE FROM task WHERE context_id = $1")
        .bind(*id)
        .execute(pool.get_ref())
        .await;

    if deleted_tasks.is_err() {
        return HttpResponse::InternalServerError().body("Internal Server Error");
    }

    let deleted: Result<Context, sqlx::Error> =
        sqlx::query_as("DELETE FROM context WHERE id = $1 RETURNING * ")
            .bind(*id)
            .fetch_one(pool.get_ref())
            .await;

    println!("deleted: {:?}", deleted);

    match deleted {
        Ok(ctx) => {
            if ctx.active {
                let new_active: Result<Context, sqlx::Error> = sqlx::query_as(
                    "SELECT * FROM context WHERE active = false ORDER BY id LIMIT 1",
                )
                .fetch_one(pool.get_ref())
                .await;

                println!("new_active: {:?}", new_active);

                if new_active.is_ok() {
                    let _ = sqlx::query("UPDATE context SET active = true WHERE id = $1")
                        .bind(new_active.unwrap().id)
                        .execute(pool.get_ref())
                        .await;
                }
            }

            return HttpResponse::Ok().json(ctx);
        }
        Err(err) => match err {
            sqlx::Error::RowNotFound => {
                return HttpResponse::NotFound().body("Context not found");
            }
            _ => {
                return HttpResponse::InternalServerError().body("Internal Server Error");
            }
        },
    }
}

#[post("/{id}")]
pub async fn update(
    pool: web::Data<Pool<Postgres>>,
    data: web::Json<Context>,
    id: web::Path<i32>,
) -> impl Responder {
    println!("data: {:?}", data);
    println!("id: {:?}", id);
    let updated: Result<Context, sqlx::Error> =
        sqlx::query_as("UPDATE context SET name = $1, active = $2 WHERE id = $3 RETURNING *")
            .bind(data.name.clone())
            .bind(data.active)
            .bind(*id)
            .fetch_one(pool.get_ref())
            .await;

    println!("updated: {:?}", updated);

    match updated {
        Ok(ctx) => HttpResponse::Ok().json(ctx),
        Err(err) => handle_err(err),
    }
}
