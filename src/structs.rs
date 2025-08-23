#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct ContextDb {
    pub id: i32,
    pub name: String,
    pub active: i32,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Context {
    pub id: i32,
    pub name: String,
    pub active: bool,
}

impl From<ContextDb> for Context {
    fn from(raw: ContextDb) -> Self {
        Context {
            id: raw.id,
            name: raw.name,
            active: raw.active != 0,
        }
    }
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

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct TaskDb {
    pub id: i32,
    content: String,
    done: i32,
    creation_date: String,
    modification_date: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Task {
    pub id: i32,
    content: String,
    done: bool,
    creation_date: String,
    modification_date: String,
}

impl From<TaskDb> for Task {
    fn from(raw: TaskDb) -> Self {
        Task {
            id: raw.id,
            content: raw.content,
            done: raw.done != 0,
            creation_date: raw.creation_date,
            modification_date: raw.modification_date,
        }
    }
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
pub struct FullContext {
    pub id: i32,
    pub name: String,
    pub active: bool,
    pub tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct FullContextDb {
    pub id: i32,
    pub name: String,
    pub active: i32,
    pub tasks: Json<Vec<TaskDb>>,
}

impl From<FullContextDb> for FullContext {
    fn from(raw: FullContextDb) -> Self {
        FullContext {
            id: raw.id,
            name: raw.name,
            active: raw.active != 0,
            tasks: raw.tasks.0.into_iter().map(Task::from).collect(),
        }
    }
}
