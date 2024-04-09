#[path = "../structs.rs"]
mod structs;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Scope};
use sqlx::{Pool, Postgres};
use structs::{Context, Task, TaskGetRequest, TaskGetResponse, TaskRequest};

pub fn get_scope() -> Scope {
    web::scope("/task")
        .service(fetch)
        .service(fetch_one)
        .service(create)
        .service(update)
        .service(delete)
}

#[get("")]
pub async fn fetch(
    pool: web::Data<Pool<Postgres>>,
    query: web::Query<TaskGetRequest>,
) -> impl Responder {
    let active = query.active.unwrap_or(false);

    let partial_req = match active {
        true => "WHERE context.active = true",
        false => "",
    };
    let request = format!(
        r#"
        SELECT 
          context.id,
          context.name,
          json_agg(json_build_object('id', task.id, 'content', task.content, 'done', task.done)) AS tasks
        FROM context
        INNER JOIN task
        ON task.context_id = context.id
        {}
        GROUP BY context.id;
        "#,
        partial_req
    );

    let tasks_res: Result<Vec<TaskGetResponse>, sqlx::Error> =
        sqlx::query_as(&request).fetch_all(pool.get_ref()).await;

    match tasks_res {
        Ok(tasks) => {
            // TODO: Test response when no active context
            if active && tasks.len() == 1 {
                return HttpResponse::Ok().json(&tasks[0]);
            } else {
                return HttpResponse::Ok().json(tasks);
            }
        }
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

#[post("")]
pub async fn create(
    pool: web::Data<Pool<Postgres>>,
    data: web::Json<TaskRequest>,
) -> impl Responder {
    if data.content.is_empty() {
        return HttpResponse::BadRequest().body("Name is required");
    }

    let context_id: i32;

    if data.context_id.is_some() {
        context_id = data.context_id.unwrap();
    } else {
        let active_context: Result<Option<Context>, sqlx::Error> =
            sqlx::query_as("SELECT * from context WHERE context.active = true")
                .fetch_optional(pool.get_ref())
                .await;

        if active_context.is_err() {
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }

        let active = active_context.unwrap();

        if active.is_none() {
            return HttpResponse::BadRequest().body("A context must exist before to create a task");
        }

        println!("active: {:?}", active);
        context_id = active.unwrap().id;
    }

    let task_res: Result<Task, sqlx::Error> =
        sqlx::query_as("INSERT INTO task (content, context_id) VALUES ($1, $2) RETURNING *")
            .bind(data.content.clone())
            .bind(context_id)
            .fetch_one(pool.get_ref())
            .await;

    match task_res {
        Ok(task) => return HttpResponse::Ok().json(task),
        Err(err) => {
            println!("err: {:?}", err);
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
