use sqlx::SqlitePool;
use crate::error::AppError;
use crate::models::{Style, Fermentable, Hop, Yeast, Misc, Water};

pub async fn list_styles(db: &SqlitePool) -> Result<Vec<Style>, AppError> {
    Ok(sqlx::query_as::<_, Style>("SELECT * FROM styles ORDER BY category, name")
        .fetch_all(db).await?)
}

pub async fn list_fermentables(db: &SqlitePool) -> Result<Vec<Fermentable>, AppError> {
    Ok(sqlx::query_as::<_, Fermentable>("SELECT * FROM fermentables ORDER BY name")
        .fetch_all(db).await?)
}

pub async fn list_hops(db: &SqlitePool) -> Result<Vec<Hop>, AppError> {
    Ok(sqlx::query_as::<_, Hop>("SELECT * FROM hops ORDER BY name")
        .fetch_all(db).await?)
}

pub async fn list_yeasts(db: &SqlitePool) -> Result<Vec<Yeast>, AppError> {
    Ok(sqlx::query_as::<_, Yeast>("SELECT * FROM yeasts ORDER BY name")
        .fetch_all(db).await?)
}

pub async fn list_miscs(db: &SqlitePool) -> Result<Vec<Misc>, AppError> {
    Ok(sqlx::query_as::<_, Misc>("SELECT * FROM miscs ORDER BY name")
        .fetch_all(db).await?)
}

pub async fn list_waters(db: &SqlitePool) -> Result<Vec<Water>, AppError> {
    Ok(sqlx::query_as::<_, Water>("SELECT * FROM waters ORDER BY name")
        .fetch_all(db).await?)
}
