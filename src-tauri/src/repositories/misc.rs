use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};

use crate::entities::recipe_addition_miscs;
use crate::error::AppError;
use crate::models::{CreateMiscAdditionInput, RecipeAdditionMisc, UpdateMiscAdditionInput};

use super::new_id;

pub struct MiscRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> MiscRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list(&self, recipe_id: &str) -> Result<Vec<RecipeAdditionMisc>, AppError> {
        recipe_addition_miscs::Entity::find()
            .filter(recipe_addition_miscs::Column::RecipeId.eq(recipe_id))
            .order_by_asc(recipe_addition_miscs::Column::AdditionOrder)
            .all(self.db)
            .await?
            .into_iter()
            .map(RecipeAdditionMisc::try_from)
            .collect()
    }

    pub async fn create(
        &self,
        recipe_id: &str,
        input: CreateMiscAdditionInput,
    ) -> Result<RecipeAdditionMisc, AppError> {
        let order = recipe_addition_miscs::Entity::find()
            .filter(recipe_addition_miscs::Column::RecipeId.eq(recipe_id))
            .count(self.db)
            .await? as i32;

        let amount_is_weight = if ["g", "oz"].contains(&input.unit.as_str()) {
            1i32
        } else {
            0i32
        };

        let id = new_id();
        recipe_addition_miscs::ActiveModel {
            id: Set(id.clone()),
            recipe_id: Set(recipe_id.to_string()),
            misc_id: Set(input.misc_id),
            name: Set(input.name),
            r#type: Set(input.type_),
            r#use: Set(input.use_),
            amount: Set(input.amount),
            amount_is_weight: Set(Some(amount_is_weight)),
            unit: Set(input.unit),
            time_min: Set(input.time_min),
            addition_order: Set(order),
        }
        .insert(self.db)
        .await?;

        recipe_addition_miscs::Entity::find_by_id(&id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(RecipeAdditionMisc::try_from)
    }

    pub async fn update(
        &self,
        id: &str,
        input: UpdateMiscAdditionInput,
    ) -> Result<RecipeAdditionMisc, AppError> {
        let row = recipe_addition_miscs::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut active: recipe_addition_miscs::ActiveModel = row.into();

        if let Some(v) = input.amount {
            active.amount = Set(v);
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
        if let Some(v) = input.unit {
            let is_weight = if ["g", "oz"].contains(&v.as_str()) {
                1i32
            } else {
                0i32
            };
            active.unit = Set(v);
            active.amount_is_weight = Set(Some(is_weight));
        }

        active.update(self.db).await?;

        recipe_addition_miscs::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(RecipeAdditionMisc::try_from)
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        recipe_addition_miscs::Entity::delete_by_id(id)
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
            .create(CreateRecipeInput {
                name: "Test".into(),
                ..Default::default()
            })
            .await
            .unwrap()
            .id
    }

    fn input() -> CreateMiscAdditionInput {
        CreateMiscAdditionInput {
            misc_id: None,
            name: "Irish Moss".into(),
            type_: "fining".into(),
            use_: "Boil".into(),
            amount: 1.0,
            unit: "g".into(),
            time_min: 15.0,
            amount_is_weight: None,
        }
    }

    #[tokio::test]
    async fn test_create_and_list() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = MiscRepository::new(&db);
        repo.create(&recipe_id, input()).await.unwrap();
        let items = repo.list(&recipe_id).await.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Irish Moss");
        assert_eq!(items[0].addition_order, 0);
    }

    #[tokio::test]
    async fn test_update() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = MiscRepository::new(&db);
        let created = repo.create(&recipe_id, input()).await.unwrap();
        let updated = repo
            .update(
                &created.id,
                UpdateMiscAdditionInput {
                    amount: Some(2.0),
                    amount_is_weight: None,
                    use_: None,
                    time_min: None,
                    addition_order: None,
                    unit: None,
                },
            )
            .await
            .unwrap();
        assert_eq!(updated.amount, 2.0);
    }

    #[tokio::test]
    async fn test_delete() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = MiscRepository::new(&db);
        let created = repo.create(&recipe_id, input()).await.unwrap();
        repo.delete(&created.id).await.unwrap();
        assert!(repo.list(&recipe_id).await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_unit_roundtrips() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = MiscRepository::new(&db);
        let created = repo
            .create(
                &recipe_id,
                CreateMiscAdditionInput {
                    misc_id: None,
                    name: "Coriander".into(),
                    type_: "Spice".into(),
                    use_: "Boil".into(),
                    amount: 2.0,
                    unit: "tsp".into(),
                    time_min: 5.0,
                    amount_is_weight: None,
                },
            )
            .await
            .unwrap();
        assert_eq!(created.unit, "tsp");
        assert!(!created.amount_is_weight); // tsp is volume
    }

    #[tokio::test]
    async fn test_update_unit() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = MiscRepository::new(&db);
        let created = repo.create(&recipe_id, input()).await.unwrap();
        // created has unit="g" (weight)
        let updated = repo
            .update(
                &created.id,
                UpdateMiscAdditionInput {
                    unit: Some("oz".into()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(updated.unit, "oz");
        assert!(updated.amount_is_weight); // oz is weight
    }
}
