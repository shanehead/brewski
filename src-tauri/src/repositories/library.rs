use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder};
use crate::entities::{fermentables, hops, miscs, styles, waters, yeasts};
use crate::error::AppError;
use crate::models::{Fermentable, Hop, Misc, Style, Water, Yeast};

pub struct LibraryRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> LibraryRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn list_styles(&self) -> Result<Vec<Style>, AppError> {
        styles::Entity::find()
            .order_by_asc(styles::Column::Category)
            .order_by_asc(styles::Column::Name)
            .all(self.db).await?
            .into_iter().map(Style::try_from).collect()
    }

    pub async fn get_style(&self, id: &str) -> Result<Style, AppError> {
        styles::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(Style::try_from)
    }

    pub async fn list_fermentables(&self) -> Result<Vec<Fermentable>, AppError> {
        fermentables::Entity::find()
            .order_by_asc(fermentables::Column::Name)
            .all(self.db).await?
            .into_iter().map(Fermentable::try_from).collect()
    }

    pub async fn list_hops(&self) -> Result<Vec<Hop>, AppError> {
        hops::Entity::find()
            .order_by_asc(hops::Column::Name)
            .all(self.db).await?
            .into_iter().map(Hop::try_from).collect()
    }

    pub async fn list_yeasts(&self) -> Result<Vec<Yeast>, AppError> {
        yeasts::Entity::find()
            .order_by_asc(yeasts::Column::Name)
            .all(self.db).await?
            .into_iter().map(Yeast::try_from).collect()
    }

    pub async fn list_miscs(&self) -> Result<Vec<Misc>, AppError> {
        miscs::Entity::find()
            .order_by_asc(miscs::Column::Name)
            .all(self.db).await?
            .into_iter().map(Misc::try_from).collect()
    }

    pub async fn list_waters(&self) -> Result<Vec<Water>, AppError> {
        waters::Entity::find()
            .order_by_asc(waters::Column::Name)
            .all(self.db).await?
            .into_iter().map(Water::try_from).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::setup_test_db;

    #[tokio::test]
    async fn test_list_styles() {
        let db = setup_test_db().await;
        let result = LibraryRepository::new(&db).list_styles().await.unwrap();
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn test_list_fermentables() {
        let db = setup_test_db().await;
        let result = LibraryRepository::new(&db).list_fermentables().await.unwrap();
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn test_list_hops() {
        let db = setup_test_db().await;
        let result = LibraryRepository::new(&db).list_hops().await.unwrap();
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn test_list_yeasts() {
        let db = setup_test_db().await;
        let result = LibraryRepository::new(&db).list_yeasts().await.unwrap();
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn test_list_miscs() {
        let db = setup_test_db().await;
        // miscs are not seeded; just verify the query succeeds
        LibraryRepository::new(&db).list_miscs().await.unwrap();
    }

    #[tokio::test]
    async fn test_list_waters() {
        let db = setup_test_db().await;
        // waters are not seeded; just verify the query succeeds
        LibraryRepository::new(&db).list_waters().await.unwrap();
    }

    #[tokio::test]
    async fn test_get_style_not_found() {
        let db = setup_test_db().await;
        let result = LibraryRepository::new(&db).get_style("nonexistent").await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }
}
