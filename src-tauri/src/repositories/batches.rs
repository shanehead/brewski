use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};

use crate::entities::{batch_gravity_readings, batches, recipes};
use crate::error::AppError;
use crate::models::{
    Batch, BatchSummary, CreateBatchInput, CreateGravityReadingInput, GravityReading,
    UpdateBatchInput,
};
use crate::repositories::recipe_version::RecipeVersionRepository;

use super::{new_id, now_secs};

pub struct BatchRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> BatchRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, input: CreateBatchInput) -> Result<Batch, AppError> {
        let version = RecipeVersionRepository::new(self.db)
            .create_or_reuse(&input.recipe_id)
            .await?;

        let id = new_id();
        let now = now_secs() as i32;

        batches::ActiveModel {
            id: Set(id.clone()),
            recipe_id: Set(input.recipe_id),
            recipe_version_id: Set(version.id),
            name: Set(input.name),
            status: Set("planned".to_string()),
            brew_date: Set(None),
            fermenter_date: Set(None),
            conditioning_date: Set(None),
            packaging_date: Set(None),
            actual_pre_boil_volume_l: Set(None),
            actual_post_boil_volume_l: Set(None),
            actual_batch_size_l: Set(None),
            actual_pre_boil_gravity: Set(None),
            actual_og: Set(None),
            actual_fg: Set(None),
            notes: Set(None),
            rating: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        }
        .insert(self.db)
        .await?;

        self.get(&id).await
    }

    pub async fn list(&self) -> Result<Vec<BatchSummary>, AppError> {
        let rows = batches::Entity::find()
            .find_also_related(recipes::Entity)
            .order_by_desc(batches::Column::BrewDate)
            .all(self.db)
            .await?;

        rows.into_iter()
            .map(|(b, r)| self.to_summary(b, r))
            .collect()
    }

    pub async fn list_for_recipe(&self, recipe_id: &str) -> Result<Vec<BatchSummary>, AppError> {
        let rows = batches::Entity::find()
            .filter(batches::Column::RecipeId.eq(recipe_id))
            .find_also_related(recipes::Entity)
            .order_by_desc(batches::Column::BrewDate)
            .all(self.db)
            .await?;

        rows.into_iter()
            .map(|(b, r)| self.to_summary(b, r))
            .collect()
    }

    pub async fn get(&self, id: &str) -> Result<Batch, AppError> {
        let (batch, recipe) = batches::Entity::find_by_id(id)
            .find_also_related(recipes::Entity)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let gravity_readings = batch_gravity_readings::Entity::find()
            .filter(batch_gravity_readings::Column::BatchId.eq(id))
            .order_by_asc(batch_gravity_readings::Column::RecordedAt)
            .all(self.db)
            .await?
            .into_iter()
            .map(GravityReading::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let recipe_name = recipe.map(|r| r.name).unwrap_or_default();

        let full_recipe = RecipeVersionRepository::new(self.db)
            .get_full(&batch.recipe_version_id)
            .await?;
        let stats = crate::brewing::calculate_stats(&full_recipe);

        Ok(Batch {
            id: batch.id,
            recipe_id: batch.recipe_id,
            recipe_name,
            recipe_version_id: batch.recipe_version_id,
            name: batch.name,
            status: batch.status,
            brew_date: batch.brew_date.map(|v| v as i64),
            fermenter_date: batch.fermenter_date.map(|v| v as i64),
            conditioning_date: batch.conditioning_date.map(|v| v as i64),
            packaging_date: batch.packaging_date.map(|v| v as i64),
            actual_pre_boil_volume_l: batch.actual_pre_boil_volume_l,
            actual_post_boil_volume_l: batch.actual_post_boil_volume_l,
            actual_batch_size_l: batch.actual_batch_size_l,
            actual_pre_boil_gravity: batch.actual_pre_boil_gravity,
            actual_og: batch.actual_og,
            actual_fg: batch.actual_fg,
            notes: batch.notes,
            rating: batch.rating.map(|v| v as i64),
            planned_og: Some(stats.og),
            planned_fg: Some(stats.fg),
            planned_pre_boil_gravity: Some(stats.pre_boil_gravity),
            planned_post_boil_volume_l: Some(stats.post_boil_volume_l),
            planned_batch_size_l: Some(full_recipe.batch_size_l),
            gravity_readings,
            created_at: batch.created_at as i64,
            updated_at: batch.updated_at as i64,
        })
    }

    pub async fn update(&self, id: &str, input: UpdateBatchInput) -> Result<Batch, AppError> {
        let row = batches::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut active: batches::ActiveModel = row.into();
        let now = now_secs() as i32;
        active.updated_at = Set(now);

        if let Some(v) = input.name {
            active.name = Set(Some(v));
        }
        if let Some(v) = input.status {
            active.status = Set(v);
        }
        if let Some(v) = input.brew_date {
            active.brew_date = Set(Some(v as i32));
        }
        if let Some(v) = input.fermenter_date {
            active.fermenter_date = Set(Some(v as i32));
        }
        if let Some(v) = input.packaging_date {
            active.packaging_date = Set(Some(v as i32));
        }
        if let Some(v) = input.conditioning_date {
            active.conditioning_date = Set(Some(v as i32));
        }
        if let Some(v) = input.notes {
            active.notes = Set(Some(v));
        }
        if let Some(v) = input.actual_pre_boil_volume_l {
            active.actual_pre_boil_volume_l = Set(Some(v));
        }
        if let Some(v) = input.actual_post_boil_volume_l {
            active.actual_post_boil_volume_l = Set(Some(v));
        }
        if let Some(v) = input.actual_batch_size_l {
            active.actual_batch_size_l = Set(Some(v));
        }
        if let Some(v) = input.actual_pre_boil_gravity {
            active.actual_pre_boil_gravity = Set(Some(v));
        }
        if let Some(v) = input.actual_og {
            active.actual_og = Set(Some(v));
        }
        if let Some(v) = input.actual_fg {
            active.actual_fg = Set(Some(v));
        }
        if let Some(v) = input.rating {
            active.rating = Set(Some(v as i32));
        }

        active.update(self.db).await?;
        self.get(id).await
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        batches::Entity::delete_by_id(id).exec(self.db).await?;
        Ok(())
    }

    pub async fn add_gravity_reading(
        &self,
        batch_id: &str,
        input: CreateGravityReadingInput,
    ) -> Result<GravityReading, AppError> {
        let id = new_id();
        batch_gravity_readings::ActiveModel {
            id: Set(id.clone()),
            batch_id: Set(batch_id.to_string()),
            recorded_at: Set(input.recorded_at as i32),
            gravity: Set(input.gravity),
            temp_c: Set(input.temp_c),
            notes: Set(input.notes),
        }
        .insert(self.db)
        .await?;

        batch_gravity_readings::Entity::find_by_id(&id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(GravityReading::try_from)
    }

    pub async fn delete_gravity_reading(&self, id: &str) -> Result<(), AppError> {
        batch_gravity_readings::Entity::delete_by_id(id)
            .exec(self.db)
            .await?;
        Ok(())
    }

    fn to_summary(
        &self,
        b: batches::Model,
        r: Option<recipes::Model>,
    ) -> Result<BatchSummary, AppError> {
        Ok(BatchSummary {
            id: b.id,
            recipe_id: b.recipe_id,
            recipe_name: r.map(|r| r.name).unwrap_or_default(),
            recipe_version_id: b.recipe_version_id,
            name: b.name,
            status: b.status,
            brew_date: b.brew_date.map(|v| v as i64),
            actual_og: b.actual_og,
            actual_fg: b.actual_fg,
            rating: b.rating.map(|v| v as i64),
            created_at: b.created_at as i64,
            updated_at: b.updated_at as i64,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::CreateRecipeInput;
    use crate::repositories::recipe::RecipeRepository;
    use crate::repositories::recipe_version::RecipeVersionRepository;
    use crate::test_helpers::setup_test_db;

    async fn setup(db: &DatabaseConnection) -> (String, String) {
        let recipe_id = RecipeRepository::new(db)
            .create(CreateRecipeInput {
                name: "Test IPA".into(),
                ..Default::default()
            })
            .await
            .unwrap()
            .id;
        let version_id = RecipeVersionRepository::new(db)
            .create_or_reuse(&recipe_id)
            .await
            .unwrap()
            .id;
        (recipe_id, version_id)
    }

    #[tokio::test]
    async fn test_create_and_get() {
        let db = setup_test_db().await;
        let (recipe_id, _) = setup(&db).await;
        let repo = BatchRepository::new(&db);
        let batch = repo
            .create(CreateBatchInput {
                recipe_id: recipe_id.clone(),
                name: Some("Batch 1".into()),
            })
            .await
            .unwrap();
        assert_eq!(batch.recipe_id, recipe_id);
        assert_eq!(batch.status, "planned");
        assert_eq!(batch.name, Some("Batch 1".into()));

        let fetched = repo.get(&batch.id).await.unwrap();
        assert_eq!(fetched.id, batch.id);
    }

    #[tokio::test]
    async fn test_list_and_list_for_recipe() {
        let db = setup_test_db().await;
        let (recipe_id, _) = setup(&db).await;
        let repo = BatchRepository::new(&db);
        repo.create(CreateBatchInput {
            recipe_id: recipe_id.clone(),
            name: None,
        })
        .await
        .unwrap();
        repo.create(CreateBatchInput {
            recipe_id: recipe_id.clone(),
            name: None,
        })
        .await
        .unwrap();
        assert_eq!(repo.list().await.unwrap().len(), 2);
        assert_eq!(repo.list_for_recipe(&recipe_id).await.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_update() {
        let db = setup_test_db().await;
        let (recipe_id, _) = setup(&db).await;
        let repo = BatchRepository::new(&db);
        let batch = repo
            .create(CreateBatchInput {
                recipe_id,
                name: None,
            })
            .await
            .unwrap();
        let updated = repo
            .update(
                &batch.id,
                UpdateBatchInput {
                    status: Some("brewing".into()),
                    actual_og: Some(1.058),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(updated.status, "brewing");
        assert_eq!(updated.actual_og, Some(1.058));
    }

    #[tokio::test]
    async fn test_update_conditioning_date_and_notes() {
        let db = setup_test_db().await;
        let (recipe_id, _) = setup(&db).await;
        let repo = BatchRepository::new(&db);
        let batch = repo
            .create(CreateBatchInput {
                recipe_id,
                name: None,
            })
            .await
            .unwrap();
        let updated = repo
            .update(
                &batch.id,
                UpdateBatchInput {
                    status: Some("conditioning".into()),
                    conditioning_date: Some(1_700_000_000),
                    notes: Some("Dry hop day 3".into()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(updated.status, "conditioning");
        assert_eq!(updated.conditioning_date, Some(1_700_000_000));
        assert_eq!(updated.notes, Some("Dry hop day 3".into()));
    }

    #[tokio::test]
    async fn test_delete_cascades_readings() {
        let db = setup_test_db().await;
        let (recipe_id, _) = setup(&db).await;
        let repo = BatchRepository::new(&db);
        let batch = repo
            .create(CreateBatchInput {
                recipe_id,
                name: None,
            })
            .await
            .unwrap();
        repo.add_gravity_reading(
            &batch.id,
            CreateGravityReadingInput {
                recorded_at: 1000000,
                gravity: 1.058,
                temp_c: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        repo.delete(&batch.id).await.unwrap();
        assert!(repo.get(&batch.id).await.is_err());
    }

    #[tokio::test]
    async fn test_get_includes_planned_targets() {
        let db = setup_test_db().await;
        let (recipe_id, _) = setup(&db).await;
        let repo = BatchRepository::new(&db);
        let batch = repo
            .create(CreateBatchInput {
                recipe_id,
                name: None,
            })
            .await
            .unwrap();
        let fetched = repo.get(&batch.id).await.unwrap();
        // Empty recipe has OG close to 1.0 (no fermentables)
        assert!(fetched.planned_og.is_some());
        assert!(fetched.planned_batch_size_l.is_some());
    }

    #[tokio::test]
    async fn test_gravity_readings() {
        let db = setup_test_db().await;
        let (recipe_id, _) = setup(&db).await;
        let repo = BatchRepository::new(&db);
        let batch = repo
            .create(CreateBatchInput {
                recipe_id,
                name: None,
            })
            .await
            .unwrap();
        let reading = repo
            .add_gravity_reading(
                &batch.id,
                CreateGravityReadingInput {
                    recorded_at: 1000000,
                    gravity: 1.058,
                    temp_c: Some(20.0),
                    notes: Some("brew day".into()),
                },
            )
            .await
            .unwrap();
        assert_eq!(reading.gravity, 1.058);

        let fetched = repo.get(&batch.id).await.unwrap();
        assert_eq!(fetched.gravity_readings.len(), 1);

        repo.delete_gravity_reading(&reading.id).await.unwrap();
        let fetched2 = repo.get(&batch.id).await.unwrap();
        assert!(fetched2.gravity_readings.is_empty());
    }
}
