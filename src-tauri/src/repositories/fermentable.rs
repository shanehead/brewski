use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};

use crate::entities::recipe_addition_fermentables;
use crate::error::AppError;
use crate::models::{
    CreateFermentableAdditionInput, RecipeAdditionFermentable, UpdateFermentableAdditionInput,
};

use super::{new_id, to_dec};

pub struct FermentableRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> FermentableRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list(&self, recipe_id: &str) -> Result<Vec<RecipeAdditionFermentable>, AppError> {
        recipe_addition_fermentables::Entity::find()
            .filter(recipe_addition_fermentables::Column::RecipeId.eq(recipe_id))
            .order_by_asc(recipe_addition_fermentables::Column::AdditionOrder)
            .all(self.db)
            .await?
            .into_iter()
            .map(RecipeAdditionFermentable::try_from)
            .collect()
    }

    pub async fn create(
        &self,
        recipe_id: &str,
        input: CreateFermentableAdditionInput,
    ) -> Result<RecipeAdditionFermentable, AppError> {
        let order = recipe_addition_fermentables::Entity::find()
            .filter(recipe_addition_fermentables::Column::RecipeId.eq(recipe_id))
            .count(self.db)
            .await? as i32;

        let id = new_id();
        recipe_addition_fermentables::ActiveModel {
            id: Set(id.clone()),
            recipe_id: Set(recipe_id.to_string()),
            fermentable_id: Set(input.fermentable_id),
            name: Set(input.name),
            r#type: Set(input.type_),
            yield_pct: Set(to_dec(input.yield_pct)),
            color_lovibond: Set(to_dec(input.color_lovibond)),
            amount_kg: Set(to_dec(input.amount_kg)),
            add_after_boil: Set(input.add_after_boil.map(|v| v as i32)),
            addition_order: Set(order),
        }
        .insert(self.db)
        .await?;

        recipe_addition_fermentables::Entity::find_by_id(&id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(RecipeAdditionFermentable::try_from)
    }

    pub async fn update(
        &self,
        id: &str,
        input: UpdateFermentableAdditionInput,
    ) -> Result<RecipeAdditionFermentable, AppError> {
        let row = recipe_addition_fermentables::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut active: recipe_addition_fermentables::ActiveModel = row.into();

        if let Some(v) = input.amount_kg {
            active.amount_kg = Set(to_dec(v));
        }
        if let Some(v) = input.add_after_boil {
            active.add_after_boil = Set(Some(v as i32));
        }
        if let Some(v) = input.addition_order {
            active.addition_order = Set(v as i32);
        }

        active.update(self.db).await?;

        recipe_addition_fermentables::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(RecipeAdditionFermentable::try_from)
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        recipe_addition_fermentables::Entity::delete_by_id(id)
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

    fn input() -> CreateFermentableAdditionInput {
        CreateFermentableAdditionInput {
            fermentable_id: None,
            name: "Pale Malt".into(),
            type_: "grain".into(),
            yield_pct: 78.0,
            color_lovibond: 1.8,
            amount_kg: 4.5,
            add_after_boil: None,
        }
    }

    #[tokio::test]
    async fn test_create_and_list() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = FermentableRepository::new(&db);
        repo.create(&recipe_id, input()).await.unwrap();
        let items = repo.list(&recipe_id).await.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Pale Malt");
        assert_eq!(items[0].addition_order, 0);
    }

    #[tokio::test]
    async fn test_list_order() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = FermentableRepository::new(&db);
        repo.create(&recipe_id, input()).await.unwrap();
        let mut second = input();
        second.name = "Crystal 60".into();
        repo.create(&recipe_id, second).await.unwrap();
        let items = repo.list(&recipe_id).await.unwrap();
        assert_eq!(items[0].addition_order, 0);
        assert_eq!(items[1].addition_order, 1);
    }

    #[tokio::test]
    async fn test_update() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = FermentableRepository::new(&db);
        let created = repo.create(&recipe_id, input()).await.unwrap();
        let updated = repo
            .update(&created.id, UpdateFermentableAdditionInput {
                amount_kg: Some(5.0),
                add_after_boil: None,
                addition_order: None,
            })
            .await
            .unwrap();
        assert_eq!(updated.amount_kg, 5.0);
    }

    #[tokio::test]
    async fn test_delete() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = FermentableRepository::new(&db);
        let created = repo.create(&recipe_id, input()).await.unwrap();
        repo.delete(&created.id).await.unwrap();
        assert!(repo.list(&recipe_id).await.unwrap().is_empty());
    }
}
