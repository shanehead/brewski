use sea_orm::DatabaseConnection;
use sea_orm_migration::MigratorTrait;
use crate::migration::Migrator;

pub async fn setup_test_db() -> DatabaseConnection {
    let db = sea_orm::Database::connect("sqlite::memory:")
        .await
        .expect("in-memory DB failed");
    Migrator::up(&db, None).await.expect("migration failed");
    db
}
