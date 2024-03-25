#[path = "../structs.rs"]
mod structs;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Scope};
use sqlx::{Pool, Postgres};
use structs::{Task, TaskRequest};

pub fn get_scope() -> Scope {
    web::scope("/task")
        .service(fetch_all)
        .service(fetch_one)
        .service(create)
        .service(update)
        .service(delete)
}

// #[get("")]
// pub async fn fetch_all(pool: web::Data<Pool<Postgres>>) -> impl Responder {
//     let tasks: Result<Vec<Task>, sqlx::Error> = sqlx::query_as("SELECT * FROM task")
//         .fetch_all(pool.get_ref())
//         .await;
//
//     match tasks {
//         Ok(tasks) => HttpResponse::Ok().json(tasks),
//         Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
//     }
// }

#[get("")]
pub async fn fetch_all(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let tasks_res: Result<Vec<Task>, sqlx::Error> = sqlx::query_as("SELECT * FROM task")
        .fetch_all(pool.get_ref())
        .await;

    match tasks_res {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[get("/{id}")]
pub async fn fetch_one(pool: web::Data<Pool<Postgres>>, id: web::Path<i32>) -> impl Responder {
    let task_res: Result<Task, sqlx::Error> =
        sqlx::query_as("SELECT * FROM task WHERE id = $1 RETURNING *")
            .bind(*id)
            .fetch_one(pool.get_ref())
            .await;

    match task_res {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(_) => {
            // TODO: Manager errors (server / not found)
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    }
}

#[post("/")]
pub async fn create(
    pool: web::Data<Pool<Postgres>>,
    data: web::Json<TaskRequest>,
) -> impl Responder {
    if data.name.is_empty() {
        return HttpResponse::BadRequest().body("Name is required");
    }

    let task_res: Result<Task, sqlx::Error> =
        sqlx::query_as("INSERT INTO task (name) VALUES ($1) RETURNING *")
            .bind(data.name.clone())
            .fetch_one(pool.get_ref())
            .await;

    match task_res {
        Ok(task) => return HttpResponse::Ok().json(task),
        Err(_) => {
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    }
}

#[put("/")]
pub async fn update(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    HttpResponse::Ok().body("update article")
}

#[delete("/")]
pub async fn delete(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    HttpResponse::Ok().body("delete article")
}
