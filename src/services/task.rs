#[path = "../structs.rs"]
mod structs;

#[path = "../utils.rs"]
mod utils;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Scope};
use sqlx::{Pool, Postgres};
use structs::{Context, FullContext, Task, TaskGetRequest, TaskPutRequest, TaskRequest};
use utils::handle_err;

pub fn get_scope() -> Scope {
    web::scope("/task")
        .service(fetch)
        .service(fetch_one)
        .service(create)
        .service(update)
        .service(delete)
        .service(delete_all)
}

#[get("")]
pub async fn fetch(
    pool: web::Data<Pool<Postgres>>,
    query: web::Query<TaskGetRequest>,
) -> impl Responder {
    if query.context_id.is_some() {
        let context_id = query.context_id.unwrap();
        let tasks_res: Result<Vec<Task>, sqlx::Error> =
            sqlx::query_as("SELECT * FROM task WHERE context_id = $1 ORDER BY id ASC")
                .bind(context_id)
                .fetch_all(pool.get_ref())
                .await;

        match tasks_res {
            Ok(tasks) => return HttpResponse::Ok().json(tasks),
            Err(err) => return handle_err(err),
        }
    }

    let active = query.active.unwrap_or(false);
    println!("active: {:?}", active);
    let partial_req = match active {
        true => "WHERE context.active = true",
        false => "",
    };
    println!("partial_req: {:?}", partial_req);
    let request = format!(
        r#"
        SELECT 
          context.id,
          context.name,
          context.active,
          json_agg(json_build_object('id', task.id, 'content', task.content, 'done', task.done)) AS tasks
        FROM context
        INNER JOIN task
        ON task.context_id = context.id
        {}
        GROUP BY context.id;
        "#,
        partial_req
    );

    let tasks_res: Result<Vec<FullContext>, sqlx::Error> =
        sqlx::query_as(&request).fetch_all(pool.get_ref()).await;
    println!("tasks_res: {:?}", tasks_res);

    match tasks_res {
        Ok(tasks) => {
            println!("tasks: {:?}", tasks);
            // TODO: Test response when no active context
            if active && tasks.len() == 1 {
                return HttpResponse::Ok().json(&tasks[0]);
            } else {
                return HttpResponse::Ok().json(tasks);
            }
        }
        Err(err) => return handle_err(err),
    }
}

#[get("/{id}")]
pub async fn fetch_one(pool: web::Data<Pool<Postgres>>, id: web::Path<i32>) -> impl Responder {
    let task_res: Result<Task, sqlx::Error> = sqlx::query_as("SELECT * FROM task WHERE id = $1")
        .bind(*id)
        .fetch_one(pool.get_ref())
        .await;

    match task_res {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(err) => handle_err(err),
    }
}

// TODO: Verify context exist when creating from context ID
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
        Err(err) => return handle_err(err),
    }
}

#[put("/{id}")]
pub async fn update(
    pool: web::Data<Pool<Postgres>>,
    data: web::Json<TaskPutRequest>,
    id: web::Path<i32>,
) -> impl Responder {
    if data.content.is_none() && data.done.is_none() {
        return HttpResponse::BadRequest().body("Content or done is required");
    }

    let set_content = if data.content.is_some() {
        let content = data.content.clone().unwrap();
        format!("content = '{}'", content)
    } else {
        String::new()
    };
    let set_done = if data.done.is_some() {
        let done = data.done.unwrap();
        let done_str = if done { "true" } else { "false" };
        format!("done = {}", done_str)
    } else {
        String::new()
    };

    let request = format!(
        r#"
        UPDATE task
        SET 
        {}
        {}
        WHERE id = $1
        RETURNING *;
        "#,
        set_content, set_done
    );

    let task_res: Result<Task, sqlx::Error> = sqlx::query_as(&request)
        .bind(*id)
        .fetch_one(pool.get_ref())
        .await;

    match task_res {
        Ok(task) => {
            return HttpResponse::Ok().json(task);
        }
        Err(err) => return handle_err(err),
    }
}

#[delete("/{id}")]
pub async fn delete(pool: web::Data<Pool<Postgres>>, id: web::Path<i32>) -> impl Responder {
    let deleted: Result<Task, sqlx::Error> =
        sqlx::query_as("DELETE from task WHERE id = $1 RETURNING *")
            .bind(*id)
            .fetch_one(pool.get_ref())
            .await;

    match deleted {
        Ok(task) => {
            return HttpResponse::Ok().json(task);
        }
        Err(err) => return handle_err(err),
    }
}

#[delete("")]
pub async fn delete_all(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let deleted: Result<(), sqlx::Error> = sqlx::query_as("DELETE from task")
        .fetch_one(pool.get_ref())
        .await;

    match deleted {
        Ok(_) => {
            return HttpResponse::Ok().body("All tasks deleted");
        }
        Err(err) => return handle_err(err),
    }
}
