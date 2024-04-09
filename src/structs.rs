use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::{FromRow, Pool, Postgres};

#[derive(Debug)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Context {
    pub id: i32,
    pub name: String,
    pub active: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContextRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Task {
    id: i32,
    content: String,
    done: bool,
    creation_date: Option<NaiveDateTime>,
    modification_date: Option<NaiveDateTime>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TaskRequest {
    pub content: String,
    pub context_id: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct TaskGetRequest {
    pub active: Option<bool>,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct TaskGetResponse {
    pub id: i32,
    pub name: String,
    pub tasks: Json<Vec<Task>>,
}
