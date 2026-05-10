use sea_orm_migration::migrator::MigratorTrait;

#[tokio::main]
async fn main() {
    let url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "sqlite://./dev.db?mode=rwc".to_string());
    let db = sea_orm::Database::connect(&url)
        .await
        .expect("DB connect failed");
    brewski_lib::migration::Migrator::up(&db, None)
        .await
        .expect("migration failed");
    println!("Migrations applied to {url}");
}
