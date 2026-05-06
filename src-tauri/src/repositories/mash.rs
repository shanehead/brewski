use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};

use crate::entities::{mash_steps, mashes};
use crate::error::AppError;
use crate::models::{CreateMashStepInput, Mash, MashStep, UpdateMashInput, UpdateMashStepInput};

use super::new_id;

pub struct MashRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> MashRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    async fn fetch_mash(&self, mash_id: &str) -> Result<Mash, AppError> {
        let mash_row = mashes::Entity::find_by_id(mash_id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let step_rows = mash_steps::Entity::find()
            .filter(mash_steps::Column::MashId.eq(mash_id))
            .order_by_asc(mash_steps::Column::StepOrder)
            .all(self.db)
            .await?;

        let steps: Result<Vec<MashStep>, AppError> =
            step_rows.into_iter().map(MashStep::try_from).collect();

        let equip_adjust = mash_row.equip_adjust.unwrap_or(0) != 0;

        let mash = Mash {
            id: mash_row.id,
            recipe_id: mash_row.recipe_id,
            name: mash_row.name,
            grain_temp_c: mash_row.grain_temp_c,
            tun_temp_c: mash_row.tun_temp_c,
            sparge_temp_c: mash_row.sparge_temp_c,
            ph: mash_row.ph,
            tun_weight_kg: mash_row.tun_weight_kg,
            tun_specific_heat: mash_row.tun_specific_heat,
            equip_adjust,
            notes: mash_row.notes,
            steps: steps?,
        };

        Ok(mash)
    }

    pub async fn get_for_recipe(&self, recipe_id: &str) -> Result<Mash, AppError> {
        let mash_row = mashes::Entity::find()
            .filter(mashes::Column::RecipeId.eq(recipe_id))
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        self.fetch_mash(&mash_row.id).await
    }

    pub async fn upsert_for_recipe(
        &self,
        recipe_id: &str,
        input: UpdateMashInput,
    ) -> Result<Mash, AppError> {
        let existing = mashes::Entity::find()
            .filter(mashes::Column::RecipeId.eq(recipe_id))
            .one(self.db)
            .await?;

        let mash_id = if let Some(mash_row) = existing {
            let mut active: mashes::ActiveModel = mash_row.into();
            if let Some(v) = input.name {
                active.name = Set(v);
            }
            if let Some(v) = input.grain_temp_c {
                active.grain_temp_c = Set(v);
            }
            if let Some(v) = input.tun_temp_c {
                active.tun_temp_c = Set(Some(v));
            }
            if let Some(v) = input.sparge_temp_c {
                active.sparge_temp_c = Set(Some(v));
            }
            if let Some(v) = input.ph {
                active.ph = Set(Some(v));
            }
            if let Some(v) = input.notes {
                active.notes = Set(Some(v));
            }
            let updated = active.update(self.db).await?;
            updated.id
        } else {
            let id = new_id();
            mashes::ActiveModel {
                id: Set(id.clone()),
                recipe_id: Set(recipe_id.to_string()),
                name: Set(input.name.unwrap_or_else(|| "Mash".to_string())),
                grain_temp_c: Set(input.grain_temp_c.unwrap_or(20.0)),
                tun_temp_c: Set(input.tun_temp_c),
                sparge_temp_c: Set(input.sparge_temp_c),
                ph: Set(input.ph),
                tun_weight_kg: Set(None),
                tun_specific_heat: Set(None),
                equip_adjust: Set(Some(0i32)),
                notes: Set(input.notes),
            }
            .insert(self.db)
            .await?;
            id
        };

        self.fetch_mash(&mash_id).await
    }

    pub async fn create_step(
        &self,
        mash_id: &str,
        input: CreateMashStepInput,
    ) -> Result<MashStep, AppError> {
        let count = mash_steps::Entity::find()
            .filter(mash_steps::Column::MashId.eq(mash_id))
            .count(self.db)
            .await? as i32;

        let id = new_id();
        mash_steps::ActiveModel {
            id: Set(id.clone()),
            mash_id: Set(mash_id.to_string()),
            name: Set(input.name),
            r#type: Set(input.type_.unwrap_or_else(|| "Infusion".to_string())),
            infuse_amount_l: Set(input.infuse_amount_l),
            step_temp_c: Set(input.step_temp_c),
            step_time_min: Set(input.step_time_min as i32),
            ramp_time_min: Set(input.ramp_time_min.map(|v| v as i32)),
            end_temp_c: Set(input.end_temp_c),
            step_order: Set(count),
        }
        .insert(self.db)
        .await?;

        let row = mash_steps::Entity::find_by_id(&id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        MashStep::try_from(row)
    }

    pub async fn update_step(
        &self,
        id: &str,
        input: UpdateMashStepInput,
    ) -> Result<MashStep, AppError> {
        let row = mash_steps::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut active: mash_steps::ActiveModel = row.into();

        if let Some(v) = input.name {
            active.name = Set(v);
        }
        if let Some(v) = input.type_ {
            active.r#type = Set(v);
        }
        if let Some(v) = input.infuse_amount_l {
            active.infuse_amount_l = Set(Some(v));
        }
        if let Some(v) = input.step_temp_c {
            active.step_temp_c = Set(v);
        }
        if let Some(v) = input.step_time_min {
            active.step_time_min = Set(v as i32);
        }
        if let Some(v) = input.ramp_time_min {
            active.ramp_time_min = Set(Some(v as i32));
        }
        if let Some(v) = input.end_temp_c {
            active.end_temp_c = Set(Some(v));
        }

        active.update(self.db).await?;

        let updated = mash_steps::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        MashStep::try_from(updated)
    }

    pub async fn delete_step(&self, id: &str) -> Result<(), AppError> {
        mash_steps::Entity::delete_by_id(id)
            .exec(self.db)
            .await?;
        Ok(())
    }

    pub async fn update_step_order(&self, ordered_ids: Vec<String>) -> Result<(), AppError> {
        for (i, step_id) in ordered_ids.iter().enumerate() {
            mash_steps::ActiveModel {
                id: Set(step_id.clone()),
                step_order: Set(i as i32),
                ..Default::default()
            }
            .update(self.db)
            .await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{CreateMashStepInput, CreateRecipeInput, UpdateMashInput, UpdateMashStepInput};
    use crate::repositories::recipe::RecipeRepository;
    use crate::test_helpers::setup_test_db;

    async fn create_recipe(db: &DatabaseConnection) -> String {
        RecipeRepository::new(db)
            .create(CreateRecipeInput {
                name: "Test Recipe".into(),
                type_: Some("all_grain".into()),
                batch_size_l: Some(23.0),
                boil_size_l: Some(27.0),
                boil_time_min: Some(60.0),
                equipment_profile_id: None,
                source_id: None,
            })
            .await
            .unwrap()
            .id
    }

    fn mash_input(name: &str) -> UpdateMashInput {
        UpdateMashInput {
            name: Some(name.into()),
            grain_temp_c: Some(20.0),
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_upsert_creates_mash() {
        let db = setup_test_db().await;
        let recipe_id = create_recipe(&db).await;
        let mash = MashRepository::new(&db)
            .upsert_for_recipe(&recipe_id, mash_input("Single Infusion"))
            .await
            .unwrap();
        assert_eq!(mash.name, "Single Infusion");
        assert_eq!(mash.recipe_id, recipe_id);
        assert!(mash.steps.is_empty());
    }

    #[tokio::test]
    async fn test_upsert_updates_existing_mash() {
        let db = setup_test_db().await;
        let recipe_id = create_recipe(&db).await;
        let repo = MashRepository::new(&db);
        repo.upsert_for_recipe(&recipe_id, mash_input("First")).await.unwrap();
        let updated = repo
            .upsert_for_recipe(
                &recipe_id,
                UpdateMashInput {
                    name: Some("Updated".into()),
                    grain_temp_c: Some(22.0),
                    sparge_temp_c: Some(76.0),
                    tun_temp_c: Some(20.0),
                    ph: Some(5.4),
                    notes: Some("test notes".into()),
                },
            )
            .await
            .unwrap();
        assert_eq!(updated.name, "Updated");
        assert_eq!(updated.sparge_temp_c, Some(76.0));
    }

    #[tokio::test]
    async fn test_get_for_recipe() {
        let db = setup_test_db().await;
        let recipe_id = create_recipe(&db).await;
        let repo = MashRepository::new(&db);
        repo.upsert_for_recipe(&recipe_id, mash_input("Mash")).await.unwrap();
        let fetched = repo.get_for_recipe(&recipe_id).await.unwrap();
        assert_eq!(fetched.name, "Mash");
    }

    #[tokio::test]
    async fn test_get_for_recipe_not_found() {
        let db = setup_test_db().await;
        let result = MashRepository::new(&db).get_for_recipe("nonexistent").await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[tokio::test]
    async fn test_create_step() {
        let db = setup_test_db().await;
        let recipe_id = create_recipe(&db).await;
        let repo = MashRepository::new(&db);
        let mash = repo.upsert_for_recipe(&recipe_id, mash_input("Mash")).await.unwrap();

        let step = repo
            .create_step(
                &mash.id,
                CreateMashStepInput {
                    name: "Mash In".into(),
                    type_: Some("Infusion".into()),
                    infuse_amount_l: Some(15.0),
                    step_temp_c: 68.0,
                    step_time_min: 60,
                    ramp_time_min: Some(5),
                    end_temp_c: Some(70.0),
                },
            )
            .await
            .unwrap();

        assert_eq!(step.name, "Mash In");
        assert_eq!(step.step_temp_c, 68.0);
        assert_eq!(step.step_time_min, 60);
        assert_eq!(step.ramp_time_min, Some(5));
        assert_eq!(step.end_temp_c, Some(70.0));
    }

    #[tokio::test]
    async fn test_step_appears_in_mash() {
        let db = setup_test_db().await;
        let recipe_id = create_recipe(&db).await;
        let repo = MashRepository::new(&db);
        let mash = repo.upsert_for_recipe(&recipe_id, mash_input("Mash")).await.unwrap();
        repo.create_step(
            &mash.id,
            CreateMashStepInput {
                name: "Mash In".into(),
                type_: None,
                infuse_amount_l: None,
                step_temp_c: 68.0,
                step_time_min: 60,
                ramp_time_min: None,
                end_temp_c: None,
            },
        )
        .await
        .unwrap();

        let fetched = repo.get_for_recipe(&recipe_id).await.unwrap();
        assert_eq!(fetched.steps.len(), 1);
        assert_eq!(fetched.steps[0].name, "Mash In");
    }

    #[tokio::test]
    async fn test_update_step() {
        let db = setup_test_db().await;
        let recipe_id = create_recipe(&db).await;
        let repo = MashRepository::new(&db);
        let mash = repo.upsert_for_recipe(&recipe_id, mash_input("Mash")).await.unwrap();
        let step = repo
            .create_step(
                &mash.id,
                CreateMashStepInput {
                    name: "Mash In".into(),
                    type_: None,
                    infuse_amount_l: None,
                    step_temp_c: 68.0,
                    step_time_min: 60,
                    ramp_time_min: None,
                    end_temp_c: None,
                },
            )
            .await
            .unwrap();

        let updated = repo
            .update_step(
                &step.id,
                UpdateMashStepInput {
                    name: Some("Mash In Updated".into()),
                    type_: Some("Temperature".into()),
                    infuse_amount_l: Some(14.0),
                    step_temp_c: Some(70.0),
                    step_time_min: Some(45),
                    ramp_time_min: Some(5),
                    end_temp_c: Some(72.0),
                },
            )
            .await
            .unwrap();

        assert_eq!(updated.name, "Mash In Updated");
        assert_eq!(updated.step_temp_c, 70.0);
        assert_eq!(updated.step_time_min, 45);
        assert_eq!(updated.ramp_time_min, Some(5));
    }

    #[tokio::test]
    async fn test_delete_step() {
        let db = setup_test_db().await;
        let recipe_id = create_recipe(&db).await;
        let repo = MashRepository::new(&db);
        let mash = repo.upsert_for_recipe(&recipe_id, mash_input("Mash")).await.unwrap();
        let step = repo
            .create_step(
                &mash.id,
                CreateMashStepInput {
                    name: "Step".into(),
                    type_: None,
                    infuse_amount_l: None,
                    step_temp_c: 68.0,
                    step_time_min: 60,
                    ramp_time_min: None,
                    end_temp_c: None,
                },
            )
            .await
            .unwrap();

        repo.delete_step(&step.id).await.unwrap();

        let fetched = repo.get_for_recipe(&recipe_id).await.unwrap();
        assert!(fetched.steps.is_empty());
    }

    #[tokio::test]
    async fn test_update_step_order() {
        let db = setup_test_db().await;
        let recipe_id = create_recipe(&db).await;
        let repo = MashRepository::new(&db);
        let mash = repo.upsert_for_recipe(&recipe_id, mash_input("Mash")).await.unwrap();

        let step1 = repo
            .create_step(
                &mash.id,
                CreateMashStepInput {
                    name: "Step 1".into(),
                    type_: None,
                    infuse_amount_l: None,
                    step_temp_c: 68.0,
                    step_time_min: 60,
                    ramp_time_min: None,
                    end_temp_c: None,
                },
            )
            .await
            .unwrap();

        let step2 = repo
            .create_step(
                &mash.id,
                CreateMashStepInput {
                    name: "Step 2".into(),
                    type_: None,
                    infuse_amount_l: None,
                    step_temp_c: 76.0,
                    step_time_min: 10,
                    ramp_time_min: None,
                    end_temp_c: None,
                },
            )
            .await
            .unwrap();

        repo.update_step_order(vec![step2.id.clone(), step1.id.clone()])
            .await
            .unwrap();

        let fetched = repo.get_for_recipe(&recipe_id).await.unwrap();
        assert_eq!(fetched.steps[0].name, "Step 2");
        assert_eq!(fetched.steps[1].name, "Step 1");
    }
}
