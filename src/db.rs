use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn connect_db() -> Pool<Postgres> {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("Error connecting to the database");

    pool
}

pub async fn run_migration(pool: &Pool<Postgres>) {
    sqlx::migrate!("db/migrations")
        .run(pool)
        .await
        .expect("Error while running migrations");
}
