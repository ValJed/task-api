use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::{FromRow, Pool, Postgres};

#[derive(Debug)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Context {
    pub id: i32,
    pub name: String,
    pub active: bool,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct ContextName {
    pub name: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct ContextTaskCount {
    pub id: i32,
    pub name: String,
    pub active: bool,
    pub task_count: i64,
}

#[derive(Deserialize, Debug)]
pub struct ContextRequest {
    pub name: String,
    pub active: Option<bool>,
    pub simple_create: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct GetContextQuery {
    pub count: Option<bool>,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Task {
    pub id: i32,
    content: String,
    done: bool,
    creation_date: String,
    modification_date: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TaskRequest {
    pub content: String,
    pub context_id: Option<i32>,
    pub creation_date: Option<String>,
    pub modification_date: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TaskPutRequest {
    pub content: Option<String>,
    pub done: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct TaskGetRequest {
    pub active: Option<bool>,
    pub context_id: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct IndexQuery {
    pub index: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct FullContextTask {
    pub id: i32,
    pub name: String,
    pub active: bool,
    pub tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct FullContext {
    pub id: i32,
    pub name: String,
    pub active: bool,
    pub tasks: Json<Vec<Task>>,
}
