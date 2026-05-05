use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

use crate::entities::recipe_addition_yeasts;
use crate::error::AppError;
use crate::models::{CreateYeastAdditionInput, RecipeAdditionYeast, UpdateYeastAdditionInput};

use super::{new_id, to_dec, to_dec_opt};

pub struct YeastRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> YeastRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list(&self, recipe_id: &str) -> Result<Vec<RecipeAdditionYeast>, AppError> {
        recipe_addition_yeasts::Entity::find()
            .filter(recipe_addition_yeasts::Column::RecipeId.eq(recipe_id))
            .all(self.db)
            .await?
            .into_iter()
            .map(RecipeAdditionYeast::try_from)
            .collect()
    }

    pub async fn create(
        &self,
        recipe_id: &str,
        input: CreateYeastAdditionInput,
    ) -> Result<RecipeAdditionYeast, AppError> {
        let id = new_id();
        recipe_addition_yeasts::ActiveModel {
            id: Set(id.clone()),
            recipe_id: Set(recipe_id.to_string()),
            yeast_id: Set(input.yeast_id),
            name: Set(input.name),
            r#type: Set(input.type_),
            form: Set(input.form),
            laboratory: Set(input.laboratory),
            product_id: Set(input.product_id),
            attenuation_pct: Set(to_dec_opt(input.attenuation_pct)),
            amount: Set(to_dec_opt(input.amount)),
            amount_is_weight: Set(input.amount_is_weight.map(|v| v as i32)),
            add_to_secondary: Set(input.add_to_secondary.map(|v| v as i32)),
            times_cultured: Set(input.times_cultured.map(|v| v as i32)),
        }
        .insert(self.db)
        .await?;

        recipe_addition_yeasts::Entity::find_by_id(&id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(RecipeAdditionYeast::try_from)
    }

    pub async fn update(
        &self,
        id: &str,
        input: UpdateYeastAdditionInput,
    ) -> Result<RecipeAdditionYeast, AppError> {
        let row = recipe_addition_yeasts::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut active: recipe_addition_yeasts::ActiveModel = row.into();

        if let Some(v) = input.attenuation_pct {
            active.attenuation_pct = Set(Some(to_dec(v)));
        }
        if let Some(v) = input.amount {
            active.amount = Set(Some(to_dec(v)));
        }
        if let Some(v) = input.amount_is_weight {
            active.amount_is_weight = Set(Some(v as i32));
        }
        if let Some(v) = input.add_to_secondary {
            active.add_to_secondary = Set(Some(v as i32));
        }
        if let Some(v) = input.times_cultured {
            active.times_cultured = Set(Some(v as i32));
        }

        active.update(self.db).await?;

        recipe_addition_yeasts::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(RecipeAdditionYeast::try_from)
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        recipe_addition_yeasts::Entity::delete_by_id(id)
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

    fn input() -> CreateYeastAdditionInput {
        CreateYeastAdditionInput {
            yeast_id: None,
            name: "US-05".into(),
            type_: "ale".into(),
            form: "dry".into(),
            laboratory: Some("Fermentis".into()),
            product_id: None,
            attenuation_pct: Some(77.0),
            amount: None,
            amount_is_weight: None,
            add_to_secondary: None,
            times_cultured: None,
        }
    }

    #[tokio::test]
    async fn test_create_and_list() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = YeastRepository::new(&db);
        repo.create(&recipe_id, input()).await.unwrap();
        let items = repo.list(&recipe_id).await.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "US-05");
        assert_eq!(items[0].attenuation_pct, Some(77.0));
    }

    #[tokio::test]
    async fn test_create_preserves_add_to_secondary() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = YeastRepository::new(&db);
        let mut i = input();
        i.add_to_secondary = Some(true);
        i.times_cultured = Some(2);
        let created = repo.create(&recipe_id, i).await.unwrap();
        assert!(created.add_to_secondary);
        assert_eq!(created.times_cultured, 2);
    }

    #[tokio::test]
    async fn test_update() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = YeastRepository::new(&db);
        let created = repo.create(&recipe_id, input()).await.unwrap();
        let updated = repo
            .update(&created.id, UpdateYeastAdditionInput {
                attenuation_pct: Some(80.0),
                amount: None,
                amount_is_weight: None,
                add_to_secondary: None,
                times_cultured: None,
            })
            .await
            .unwrap();
        assert_eq!(updated.attenuation_pct, Some(80.0));
    }

    #[tokio::test]
    async fn test_delete() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = YeastRepository::new(&db);
        let created = repo.create(&recipe_id, input()).await.unwrap();
        repo.delete(&created.id).await.unwrap();
        assert!(repo.list(&recipe_id).await.unwrap().is_empty());
    }
}
