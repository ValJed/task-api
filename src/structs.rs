use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};

#[derive(Debug)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Context {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContextRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Task {
    id: i32,
    name: String,
    done: bool,
    creation_date: String,
    modification_date: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TaskRequest {
    pub name: String,
}
