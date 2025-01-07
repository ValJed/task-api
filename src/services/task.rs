#[path = "../sql.rs"]
mod sql;
#[path = "../structs.rs"]
mod structs;
#[path = "../utils.rs"]
mod utils;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Scope};
use chrono::Local;
use sqlx::{Pool, Postgres};
use structs::{
    Context, FullContext, IndexQuery, Task, TaskGetRequest, TaskPutRequest, TaskRequest,
};
use utils::handle_err;

pub fn get_scope() -> Scope {
    web::scope("/task")
        .service(fetch)
        .service(fetch_one)
        .service(create)
        .service(create_batch)
        .service(update)
        .service(delete)
        .service(delete_all)
        .service(toggle_done)
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
    let request = match active {
        true => &sql::LIST_TASKS_ACTIVE,
        false => &sql::LIST_TASKS,
    };

    let tasks_res: Result<Vec<FullContext>, sqlx::Error> =
        sqlx::query_as(request).fetch_all(pool.get_ref()).await;

    match tasks_res {
        Ok(tasks) => {
            // TODO: Test response when no active context
            return HttpResponse::Ok().json(tasks);
        }
        Err(err) => {
            return handle_err(err);
        }
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

    let date = Local::now();
    let task_res: Result<Task, sqlx::Error> =
        sqlx::query_as("INSERT INTO task (content, context_id, creation_date, modification_date) VALUES ($1, $2, $3, $3) RETURNING *")
            .bind(data.content.clone())
            .bind(context_id)
            .bind(date)
            .fetch_one(pool.get_ref())
            .await;

    match task_res {
        Ok(task) => return HttpResponse::Ok().json(task),
        Err(err) => return handle_err(err),
    }
}

#[post("/batch")]
pub async fn create_batch(
    pool: web::Data<Pool<Postgres>>,
    data: web::Json<Vec<TaskRequest>>,
) -> impl Responder {
    let date = Local::now().to_string();

    // Inserting multiple items at once has limits
    // that should not be exceeded in this case
    let tasks_str = data
        .iter()
        .map(|task| {
            format!(
                "('{}', {}, '{}', '{}')",
                task.content.clone(),
                task.context_id.clone().unwrap(),
                task.creation_date.clone().unwrap_or(date.clone()),
                task.modification_date.clone().unwrap_or(date.clone())
            )
        })
        .collect::<Vec<String>>()
        .join(", ");

    let request = format!(
        "INSERT INTO task (content, context_id, creation_date, modification_date) VALUES {}",
        tasks_str
    );

    let inserted = sqlx::query(&request).execute(pool.get_ref()).await;
    match inserted {
        Ok(_) => return HttpResponse::Ok().body("Tasks created"),
        Err(err) => return handle_err(err),
    }
}

#[put("/done/{id}")]
pub async fn toggle_done(
    pool: web::Data<Pool<Postgres>>,
    id: web::Path<String>,
    query: web::Query<IndexQuery>,
) -> impl Responder {
    let task_ids = get_id_from_indexes(&pool, id.to_string(), query.index).await;
    if task_ids.len() == 0 {
        return HttpResponse::NotFound().body("Task not found");
    }

    let mut results: Vec<Task> = vec![];
    let mut error: Option<sqlx::Error> = None;

    for id in task_ids {
        let res: Result<Task, sqlx::Error> =
            sqlx::query_as("UPDATE task SET done = NOT done WHERE id = $1 RETURNING *")
                .bind(&id)
                .fetch_one(pool.get_ref())
                .await;
        match res {
            Ok(task) => results.push(task),
            Err(err) => {
                error = Some(err);
                break;
            }
        }
    }

    match error {
        None => HttpResponse::Ok().json(results),
        Some(err) => handle_err(err),
    }
}

#[put("/{id}")]
pub async fn update(
    pool: web::Data<Pool<Postgres>>,
    data: web::Json<TaskPutRequest>,
    query: web::Query<IndexQuery>,
    id: web::Path<String>,
) -> impl Responder {
    if data.content.is_none() && data.done.is_none() {
        return HttpResponse::BadRequest().body("Content or done is required");
    }

    let task_ids = get_id_from_indexes(&pool, id.to_string(), query.index).await;
    if task_ids.len() == 0 {
        return HttpResponse::NotFound().body("Task not found");
    }
    if task_ids.len() > 1 {
        return HttpResponse::BadRequest().body("Only one ID allowed for update");
    }
    let task_id = task_ids[0];

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
        .bind(task_id)
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
pub async fn delete(
    pool: web::Data<Pool<Postgres>>,
    id: web::Path<String>,
    query: web::Query<IndexQuery>,
) -> impl Responder {
    let task_ids = get_id_from_indexes(&pool, id.to_string(), query.index).await;
    if task_ids.len() == 0 {
        return HttpResponse::NotFound().body("Task not found");
    }

    let mut results: Vec<Task> = vec![];
    let mut error: Option<sqlx::Error> = None;
    for id in task_ids {
        let res: Result<Task, sqlx::Error> =
            sqlx::query_as("DELETE from task WHERE id = $1 RETURNING *")
                .bind(&id)
                .fetch_one(pool.get_ref())
                .await;

        match res {
            Ok(task) => results.push(task),
            Err(err) => {
                error = Some(err);
                break;
            }
        }
    }

    match error {
        None => HttpResponse::Ok().json(results),
        Some(err) => handle_err(err),
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
        Err(err) => match err {
            sqlx::Error::RowNotFound => {
                return HttpResponse::Ok().body("All contexts deleted");
            }
            _ => return handle_err(err),
        },
    }
}

async fn get_id_from_indexes(
    pool: &web::Data<Pool<Postgres>>,
    indexes_ids: String,
    by_index: Option<bool>,
) -> Vec<i32> {
    let indexes: Vec<i32> = indexes_ids
        .split(',')
        .filter_map(|str| match str.parse() {
            Ok(num) => Some(num),
            Err(_) => None,
        })
        .collect();

    if by_index.is_none() || !by_index.unwrap() {
        return indexes;
    }
    let request = r#"
        SELECT task.* 
        FROM context 
        INNER JOIN task ON task.context_id = context.id 
        WHERE context.active = true ORDER BY task.id ASC;
    "#;
    let tasks: Result<Vec<Task>, sqlx::Error> =
        sqlx::query_as(request).fetch_all(pool.get_ref()).await;

    let ids = indexes
        .iter()
        .filter_map(|index| match &tasks {
            Ok(tasks) => match tasks.get(*index as usize - 1).cloned() {
                Some(task) => return Some(task.id),
                None => return None,
            },
            Err(_) => None,
        })
        .collect();

    ids
}
