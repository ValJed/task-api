use actix_web::{web, App, HttpServer};
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
        // let data = AppState { db: pool.clone() };

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(context)
            .service(task)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
