use sqlx::SqlitePool;
use crate::error::AppError;
use std::collections::HashMap;

pub async fn get_all(db: &SqlitePool) -> Result<HashMap<String, String>, AppError> {
    let rows: Vec<(String, String)> = sqlx::query_as("SELECT key, value FROM settings")
        .fetch_all(db).await?;
    Ok(rows.into_iter().collect())
}

pub async fn set(db: &SqlitePool, key: &str, value: &str) -> Result<(), AppError> {
    sqlx::query("INSERT INTO settings (key, value) VALUES (?, ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value")
        .bind(key).bind(value).execute(db).await?;
    Ok(())
}
