use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "sqlite://./dev.db".to_string());
    let opts = SqliteConnectOptions::from_str(&url)
        .expect("invalid DB URL")
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(opts)
        .await
        .expect("DB connect failed");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("migration failed");
    println!("Migrations applied to {url}");
}
