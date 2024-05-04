use actix_web::HttpResponse;
use sqlx;

pub fn handle_err(err: sqlx::Error) -> HttpResponse {
    match err {
        sqlx::Error::RowNotFound => {
            return HttpResponse::NotFound().body("Not found");
        }
        _ => {
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    }
}
