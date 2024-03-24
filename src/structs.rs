use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Debug)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

#[derive(Serialize, Deserialize)]
pub struct Context {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    id: usize,
    name: String,
    done: bool,
    creation_date: String,
    modification_date: String,
}
