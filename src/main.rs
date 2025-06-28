use actix_web::body::MessageBody;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    web, App, HttpServer,
};
use actix_web_lab::middleware::{from_fn, Next};
use dotenv::dotenv;

mod db;
mod services;
mod structs;

use services::context::get_scope as context_scope;
use services::task::get_scope as task_scope;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();
    let pool = db::connect_db().await;

    db::run_migration(&pool).await;

    HttpServer::new(move || {
        let context = context_scope();
        let task = task_scope();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(from_fn(authorize))
            .service(context)
            .service(task)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}

async fn authorize(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set");
    let token = req.headers().get("AUTHORIZATION");

    if token.is_none() || *token.unwrap() != api_key {
        return Err(actix_web::error::ErrorUnauthorized("Not authorized"));
    }

    next.call(req).await
}
