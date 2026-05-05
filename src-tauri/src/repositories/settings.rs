use sea_orm::{DatabaseConnection, EntityTrait, Set};
use sea_orm::sea_query::OnConflict;
use std::collections::HashMap;
use crate::entities::settings;
use crate::error::AppError;

pub struct SettingsRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> SettingsRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn get_all(&self) -> Result<HashMap<String, String>, AppError> {
        let rows = settings::Entity::find().all(self.db).await?;
        Ok(rows.into_iter().map(|r| (r.key, r.value)).collect())
    }

    pub async fn set(&self, key: &str, value: &str) -> Result<(), AppError> {
        settings::Entity::insert(settings::ActiveModel {
            key: Set(key.to_owned()),
            value: Set(value.to_owned()),
        })
        .on_conflict(
            OnConflict::column(settings::Column::Key)
                .update_column(settings::Column::Value)
                .to_owned(),
        )
        .exec(self.db)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::setup_test_db;

    #[tokio::test]
    async fn test_get_all_returns_defaults() {
        let db = setup_test_db().await;
        let repo = SettingsRepository::new(&db);
        let settings = repo.get_all().await.unwrap();
        assert_eq!(settings.get("theme").map(String::as_str), Some("midnight"));
        assert_eq!(settings.get("units").map(String::as_str), Some("metric"));
    }

    #[tokio::test]
    async fn test_set_upserts() {
        let db = setup_test_db().await;
        let repo = SettingsRepository::new(&db);
        repo.set("theme", "light").await.unwrap();
        let settings = repo.get_all().await.unwrap();
        assert_eq!(settings.get("theme").map(String::as_str), Some("light"));
    }
}
