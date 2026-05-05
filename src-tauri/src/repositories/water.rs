use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

use crate::entities::recipe_addition_waters;
use crate::error::AppError;
use crate::models::{CreateWaterAdditionInput, RecipeAdditionWater, UpdateWaterAdditionInput};

use super::{new_id, to_dec};

pub struct WaterRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> WaterRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list(&self, recipe_id: &str) -> Result<Vec<RecipeAdditionWater>, AppError> {
        recipe_addition_waters::Entity::find()
            .filter(recipe_addition_waters::Column::RecipeId.eq(recipe_id))
            .all(self.db)
            .await?
            .into_iter()
            .map(RecipeAdditionWater::try_from)
            .collect()
    }

    pub async fn create(
        &self,
        recipe_id: &str,
        input: CreateWaterAdditionInput,
    ) -> Result<RecipeAdditionWater, AppError> {
        let id = new_id();
        recipe_addition_waters::ActiveModel {
            id: Set(id.clone()),
            recipe_id: Set(recipe_id.to_string()),
            water_id: Set(input.water_id),
            name: Set(input.name),
            amount_l: Set(to_dec(input.amount_l)),
        }
        .insert(self.db)
        .await?;

        recipe_addition_waters::Entity::find_by_id(&id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(RecipeAdditionWater::try_from)
    }

    pub async fn update(
        &self,
        id: &str,
        input: UpdateWaterAdditionInput,
    ) -> Result<RecipeAdditionWater, AppError> {
        let row = recipe_addition_waters::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut active: recipe_addition_waters::ActiveModel = row.into();

        if let Some(v) = input.amount_l {
            active.amount_l = Set(to_dec(v));
        }

        active.update(self.db).await?;

        recipe_addition_waters::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(RecipeAdditionWater::try_from)
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        recipe_addition_waters::Entity::delete_by_id(id)
            .exec(self.db)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::CreateRecipeInput;
    use crate::repositories::recipe::RecipeRepository;
    use crate::test_helpers::setup_test_db;

    async fn make_recipe(db: &DatabaseConnection) -> String {
        RecipeRepository::new(db)
            .create(CreateRecipeInput { name: "Test".into(), ..Default::default() })
            .await
            .unwrap()
            .id
    }

    fn input() -> CreateWaterAdditionInput {
        CreateWaterAdditionInput {
            water_id: None,
            name: "RO Water".into(),
            amount_l: 25.0,
        }
    }

    #[tokio::test]
    async fn test_create_and_list() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = WaterRepository::new(&db);
        repo.create(&recipe_id, input()).await.unwrap();
        let items = repo.list(&recipe_id).await.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "RO Water");
        assert_eq!(items[0].amount_l, 25.0);
    }

    #[tokio::test]
    async fn test_update() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = WaterRepository::new(&db);
        let created = repo.create(&recipe_id, input()).await.unwrap();
        let updated = repo
            .update(&created.id, UpdateWaterAdditionInput { amount_l: Some(20.0) })
            .await
            .unwrap();
        assert_eq!(updated.amount_l, 20.0);
    }

    #[tokio::test]
    async fn test_delete() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = WaterRepository::new(&db);
        let created = repo.create(&recipe_id, input()).await.unwrap();
        repo.delete(&created.id).await.unwrap();
        assert!(repo.list(&recipe_id).await.unwrap().is_empty());
    }
}
