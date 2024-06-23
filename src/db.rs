use sqlx::{migrate::MigrateDatabase, postgres::PgPoolOptions, Pool, Postgres};

pub async fn connect_db() -> Pool<Postgres> {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_exists = Postgres::database_exists(&url)
        .await
        .expect("Error while checking if the database exists");

    if !db_exists {
        let created_res = Postgres::create_database(&url).await;
        if created_res.is_err() {
            panic!("Error while creating the database");
        }
    }

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
