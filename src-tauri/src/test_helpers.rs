use sea_orm::DatabaseConnection;
use sea_orm::SqlxSqliteConnector;
use sqlx::SqlitePool;

pub async fn setup_test_db() -> DatabaseConnection {
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("in-memory DB failed");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("migration failed");
    SqlxSqliteConnector::from_sqlx_sqlite_pool(pool)
}
