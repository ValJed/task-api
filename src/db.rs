use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePool, Pool, Sqlite};
use std::path::Path;

pub async fn connect_db() -> Pool<Sqlite> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_path = db_url
        .strip_prefix("sqlite://")
        .expect("DATABASE_URL must start with 'sqlite://'");

    // Ensure the parent directory exists
    if let Some(parent) = Path::new(&db_path).parent() {
        match std::fs::create_dir_all(parent) {
            Ok(_) => {}
            Err(e) => panic!("Error creating data directory: {}", e),
        }
    }

    let db_exists = Sqlite::database_exists(&db_url)
        .await
        .expect("Error while checking if the database exists");

    if !db_exists {
        let created_res = Sqlite::create_database(&db_url).await;
        if created_res.is_err() {
            panic!("Error while creating the database");
        }
    }

    let pool = match SqlitePool::connect(&db_url).await {
        Ok(pool) => pool,
        Err(e) => panic!("Error connecting to the database: {}", e),
    };

    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect(&url)
    //     .await
    //     .expect("Error connecting to the database");

    pool
}

pub async fn run_migration(pool: &Pool<Sqlite>) {
    sqlx::migrate!("db/migrations")
        .run(pool)
        .await
        .expect("Error while running migrations");
}
