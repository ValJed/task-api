use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod db;
mod services;
mod structs;

use services::context::get_scope as context_scope;
use services::task::get_scope as task_scope;
use structs::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = db::connect_db().await;

    db::run_migration(&pool).await;

    HttpServer::new(move || {
        let context = context_scope();
        let task = task_scope();

        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(context)
            .service(task)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
