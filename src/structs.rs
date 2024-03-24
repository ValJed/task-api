use std::fmt::Display;

use actix_web::{http::header::ContentType, http::StatusCode, HttpResponse, ResponseError};
use derive_more::{Display, Error};
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

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Task {
    id: usize,
    name: String,
    done: bool,
    creation_date: String,
    modification_date: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContextRequest {
    pub name: String,
}

#[derive(Debug, Display, Error)]
pub enum RequestError {
    NotFound,
    BadRequest,
    InternalServerError,
    Forbidden,
}

impl ResponseError for RequestError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            RequestError::NotFound => StatusCode::NOT_FOUND,
            RequestError::BadRequest => StatusCode::BAD_REQUEST,
            RequestError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            RequestError::Forbidden => StatusCode::FORBIDDEN,
        }
    }
}
