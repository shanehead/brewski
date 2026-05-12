use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use crate::brewing::water::IonContribution;
use crate::entities::{recipe_water_adjustments, recipes, waters};
use crate::error::AppError;
use crate::models::{
    CalculatedWaterProfile, CreateWaterAdjustmentInput, Recipe, RecipeWaterAdjustment,
    UpdateWaterAdjustmentInput, WaterProfile,
};

use super::new_id;

pub struct WaterChemistryRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> WaterChemistryRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn set_recipe_water_sources(
        &self,
        recipe_id: &str,
        mash_water_id: Option<String>,
        sparge_water_id: Option<String>,
    ) -> Result<Recipe, AppError> {
        let recipe_row = recipes::Entity::find_by_id(recipe_id.to_string())
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut active: recipes::ActiveModel = recipe_row.into();
        active.mash_water_id = Set(mash_water_id);
        active.sparge_water_id = Set(sparge_water_id);
        active.update(self.db).await?;

        // Fetch and return the full recipe
        super::recipe::RecipeRepository::new(self.db)
            .get(recipe_id)
            .await
    }

    pub async fn create_water_adjustment(
        &self,
        recipe_id: &str,
        input: CreateWaterAdjustmentInput,
    ) -> Result<RecipeWaterAdjustment, AppError> {
        let id = new_id();
        recipe_water_adjustments::ActiveModel {
            id: Set(id.clone()),
            recipe_id: Set(recipe_id.to_string()),
            addition: Set(input.addition.to_string()),
            target: Set(input.target.to_string()),
            amount: Set(input.amount),
        }
        .insert(self.db)
        .await?;

        recipe_water_adjustments::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(RecipeWaterAdjustment::try_from)
    }

    pub async fn update_water_adjustment(
        &self,
        id: &str,
        input: UpdateWaterAdjustmentInput,
    ) -> Result<RecipeWaterAdjustment, AppError> {
        let row = recipe_water_adjustments::Entity::find_by_id(id.to_string())
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut active: recipe_water_adjustments::ActiveModel = row.into();

        if let Some(v) = input.addition {
            active.addition = Set(v.to_string());
        }
        if let Some(v) = input.target {
            active.target = Set(v.to_string());
        }
        if let Some(v) = input.amount {
            active.amount = Set(v);
        }

        active.update(self.db).await?;

        recipe_water_adjustments::Entity::find_by_id(id.to_string())
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(RecipeWaterAdjustment::try_from)
    }

    pub async fn delete_water_adjustment(&self, id: &str) -> Result<(), AppError> {
        recipe_water_adjustments::Entity::delete_by_id(id.to_string())
            .exec(self.db)
            .await?;
        Ok(())
    }

    pub async fn list_adjustments(
        &self,
        recipe_id: &str,
    ) -> Result<Vec<RecipeWaterAdjustment>, AppError> {
        let adjustments = recipe_water_adjustments::Entity::find()
            .filter(recipe_water_adjustments::Column::RecipeId.eq(recipe_id))
            .all(self.db)
            .await?;

        adjustments
            .into_iter()
            .map(RecipeWaterAdjustment::try_from)
            .collect()
    }

    pub async fn calculate_water_profile(
        &self,
        recipe_id: &str,
    ) -> Result<CalculatedWaterProfile, AppError> {
        let recipe = super::recipe::RecipeRepository::new(self.db)
            .get(recipe_id)
            .await?;

        // Get mash and sparge water profiles
        let mash_water = if let Some(mash_water_id) = &recipe.mash_water_id {
            waters::Entity::find_by_id(mash_water_id.clone())
                .one(self.db)
                .await?
                .ok_or(AppError::NotFound)?
        } else {
            return Ok(CalculatedWaterProfile {
                mash: WaterProfile::default(),
                sparge: WaterProfile::default(),
                combined: WaterProfile::default(),
            });
        };

        let sparge_water = if let Some(sparge_water_id) = &recipe.sparge_water_id {
            waters::Entity::find_by_id(sparge_water_id.clone())
                .one(self.db)
                .await?
                .ok_or(AppError::NotFound)?
        } else {
            mash_water.clone()
        };

        // Calculate mash volume
        let total_grain_kg: f64 = recipe.fermentables.iter().map(|f| f.amount_kg).sum();
        let mash_volume_l = if let Some(mash) = &recipe.mash {
            if let Some(ratio) = mash.ratio_l_per_kg {
                ratio * total_grain_kg
            } else {
                0.0
            }
        } else {
            0.0
        };

        // Calculate sparge volume
        let boil_evaporation = if let Some(equipment) = &recipe.equipment_profile {
            (equipment.evap_rate_pct_hr / 100.0) * recipe.boil_time_min / 60.0 * recipe.boil_size_l
        } else {
            (10.0 / 100.0) * recipe.boil_time_min / 60.0 * recipe.boil_size_l
        };
        let sparge_volume_l = recipe.batch_size_l + boil_evaporation - mash_volume_l;

        // Get adjustments
        let adjustments = recipe_water_adjustments::Entity::find()
            .filter(recipe_water_adjustments::Column::RecipeId.eq(recipe_id))
            .all(self.db)
            .await?;

        // Calculate adjusted profiles
        let mash_profile = self.apply_adjustments(&mash_water, &adjustments, "mash", mash_volume_l);

        let sparge_profile =
            self.apply_adjustments(&sparge_water, &adjustments, "sparge", sparge_volume_l);

        // Combined profile (weighted average)
        let total_volume = mash_volume_l + sparge_volume_l;
        let combined = if total_volume > 0.0 {
            WaterProfile {
                calcium_ppm: (mash_profile.calcium_ppm * mash_volume_l
                    + sparge_profile.calcium_ppm * sparge_volume_l)
                    / total_volume,
                magnesium_ppm: (mash_profile.magnesium_ppm * mash_volume_l
                    + sparge_profile.magnesium_ppm * sparge_volume_l)
                    / total_volume,
                sodium_ppm: (mash_profile.sodium_ppm * mash_volume_l
                    + sparge_profile.sodium_ppm * sparge_volume_l)
                    / total_volume,
                chloride_ppm: (mash_profile.chloride_ppm * mash_volume_l
                    + sparge_profile.chloride_ppm * sparge_volume_l)
                    / total_volume,
                sulfate_ppm: (mash_profile.sulfate_ppm * mash_volume_l
                    + sparge_profile.sulfate_ppm * sparge_volume_l)
                    / total_volume,
                bicarbonate_ppm: (mash_profile.bicarbonate_ppm * mash_volume_l
                    + sparge_profile.bicarbonate_ppm * sparge_volume_l)
                    / total_volume,
                cl_so4_ratio: 0.0, // Will be calculated below
            }
        } else {
            WaterProfile::default()
        };

        let combined = WaterProfile {
            cl_so4_ratio: if combined.sulfate_ppm > 0.0 {
                combined.chloride_ppm / combined.sulfate_ppm
            } else {
                0.0
            },
            ..combined
        };

        Ok(CalculatedWaterProfile {
            mash: mash_profile,
            sparge: sparge_profile,
            combined,
        })
    }

    fn apply_adjustments(
        &self,
        base_water: &waters::Model,
        adjustments: &[recipe_water_adjustments::Model],
        target: &str,
        volume_l: f64,
    ) -> WaterProfile {
        let mut profile = WaterProfile {
            calcium_ppm: base_water.calcium_ppm,
            magnesium_ppm: base_water.magnesium_ppm,
            sodium_ppm: base_water.sodium_ppm,
            chloride_ppm: base_water.chloride_ppm,
            sulfate_ppm: base_water.sulfate_ppm,
            bicarbonate_ppm: base_water.bicarbonate_ppm,
            cl_so4_ratio: 0.0,
        };

        if volume_l <= 0.0 {
            return profile;
        }

        for adjustment in adjustments {
            if adjustment.target != target {
                continue;
            }

            if let Some(ion_contrib) = IonContribution::for_addition(&adjustment.addition) {
                let amount_f64 = adjustment.amount;

                let ppm_increase = (ion_contrib.calcium_ppm * amount_f64) / volume_l;
                profile.calcium_ppm += ppm_increase;

                let ppm_increase = (ion_contrib.magnesium_ppm * amount_f64) / volume_l;
                profile.magnesium_ppm += ppm_increase;

                let ppm_increase = (ion_contrib.sodium_ppm * amount_f64) / volume_l;
                profile.sodium_ppm += ppm_increase;

                let ppm_increase = (ion_contrib.chloride_ppm * amount_f64) / volume_l;
                profile.chloride_ppm += ppm_increase;

                let ppm_increase = (ion_contrib.sulfate_ppm * amount_f64) / volume_l;
                profile.sulfate_ppm += ppm_increase;

                let ppm_increase = (ion_contrib.bicarbonate_ppm * amount_f64) / volume_l;
                profile.bicarbonate_ppm += ppm_increase;
            }
        }

        profile.cl_so4_ratio = if profile.sulfate_ppm > 0.0 {
            profile.chloride_ppm / profile.sulfate_ppm
        } else {
            0.0
        };

        profile
    }
}
