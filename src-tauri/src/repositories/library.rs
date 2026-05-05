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
