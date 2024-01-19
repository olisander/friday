use sqlx::{migrate::MigrateDatabase, SqlitePool};

pub async fn create_database_if_not_exists(database_url: &str) {
    if !sqlx::Sqlite::database_exists(database_url)
        .await
        .expect("Failed to check if database exists")
    {
        sqlx::Sqlite::create_database(database_url)
            .await
            .expect("Failed to create database");
    }
}

pub async fn apply_migrations(pool: &SqlitePool) {
    sqlx::migrate!()
        .run(pool)
        .await
        .expect("Failed to apply migrations");
}

pub async fn connection_pool(database_url: &str) -> SqlitePool {
    SqlitePool::connect(database_url)
        .await
        .expect("Failed to connect to database")
}
