use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};

use crate::entities::recipe_addition_hops;
use crate::error::AppError;
use crate::models::{CreateHopAdditionInput, RecipeAdditionHop, UpdateHopAdditionInput};

use super::new_id;

pub struct HopRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> HopRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list(&self, recipe_id: &str) -> Result<Vec<RecipeAdditionHop>, AppError> {
        recipe_addition_hops::Entity::find()
            .filter(recipe_addition_hops::Column::RecipeId.eq(recipe_id))
            .order_by_asc(recipe_addition_hops::Column::AdditionOrder)
            .all(self.db)
            .await?
            .into_iter()
            .map(RecipeAdditionHop::try_from)
            .collect()
    }

    pub async fn create(
        &self,
        recipe_id: &str,
        input: CreateHopAdditionInput,
    ) -> Result<RecipeAdditionHop, AppError> {
        let order = recipe_addition_hops::Entity::find()
            .filter(recipe_addition_hops::Column::RecipeId.eq(recipe_id))
            .count(self.db)
            .await? as i32;

        let id = new_id();
        recipe_addition_hops::ActiveModel {
            id: Set(id.clone()),
            recipe_id: Set(recipe_id.to_string()),
            hop_id: Set(input.hop_id),
            name: Set(input.name),
            alpha_pct: Set(input.alpha_pct),
            form: Set(input.form.unwrap_or_else(|| "Pellet".to_string())),
            amount_kg: Set(input.amount_kg),
            r#use: Set(input.use_),
            time_min: Set(input.time_min),
            addition_order: Set(order),
        }
        .insert(self.db)
        .await?;

        recipe_addition_hops::Entity::find_by_id(&id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(RecipeAdditionHop::try_from)
    }

    pub async fn update(
        &self,
        id: &str,
        input: UpdateHopAdditionInput,
    ) -> Result<RecipeAdditionHop, AppError> {
        let row = recipe_addition_hops::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut active: recipe_addition_hops::ActiveModel = row.into();

        if let Some(v) = input.amount_kg {
            active.amount_kg = Set(v);
        }
        if let Some(v) = input.use_ {
            active.r#use = Set(v);
        }
        if let Some(v) = input.time_min {
            active.time_min = Set(v);
        }
        if let Some(v) = input.addition_order {
            active.addition_order = Set(v as i32);
        }

        active.update(self.db).await?;

        recipe_addition_hops::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(RecipeAdditionHop::try_from)
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        recipe_addition_hops::Entity::delete_by_id(id)
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

    fn input() -> CreateHopAdditionInput {
        CreateHopAdditionInput {
            hop_id: None,
            name: "Cascade".into(),
            alpha_pct: 5.5,
            form: None,
            amount_kg: 0.05,
            use_: "Boil".into(),
            time_min: 60.0,
        }
    }

    #[tokio::test]
    async fn test_create_and_list() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = HopRepository::new(&db);
        repo.create(&recipe_id, input()).await.unwrap();
        let items = repo.list(&recipe_id).await.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Cascade");
        assert_eq!(items[0].form, "Pellet");
        assert_eq!(items[0].addition_order, 0);
    }

    #[tokio::test]
    async fn test_update() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = HopRepository::new(&db);
        let created = repo.create(&recipe_id, input()).await.unwrap();
        let updated = repo
            .update(&created.id, UpdateHopAdditionInput {
                amount_kg: Some(0.1),
                use_: None,
                time_min: None,
                addition_order: None,
            })
            .await
            .unwrap();
        assert_eq!(updated.amount_kg, 0.1);
    }

    #[tokio::test]
    async fn test_delete() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = HopRepository::new(&db);
        let created = repo.create(&recipe_id, input()).await.unwrap();
        repo.delete(&created.id).await.unwrap();
        assert!(repo.list(&recipe_id).await.unwrap().is_empty());
    }
}
