use sqlx::SqlitePool;

pub async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePool::connect(":memory:").await.expect("in-memory DB failed");
    sqlx::migrate!("src/db/migrations")
        .run(&pool)
        .await
        .expect("migration failed");
    pool
}
