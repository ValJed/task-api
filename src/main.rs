use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::{pool::PoolOptions, postgres::PgPoolOptions, Pool, Postgres};

mod services;

use services::context::get_scope as context_scope;
use services::task::get_scope as task_scope;

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = connect_db().await;

    run_migration(&pool).await;

    HttpServer::new(move || {
        let context = context_scope();
        let task = task_scope();

        println!("Starting server at http://");
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(context)
            .service(task)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

async fn connect_db() -> Pool<Postgres> {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("Error connecting to the database");

    pool
}

async fn run_migration(pool: &Pool<Postgres>) {
    sqlx::migrate!("db/migrations")
        .run(pool)
        .await
        .expect("Error while running migrations");
}
