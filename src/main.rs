use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::{pool::PoolOptions, postgres::PgPoolOptions, Pool, Postgres};

mod services;
use services::{create_article, delete_article, fetch_articles, update_article};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = connect_db().await;

    run_migration(&pool).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(create_article)
            .service(update_article)
            .service(delete_article)
            .service(fetch_articles)
    })
    .bind(("127.0.0.1", 8080))?
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
