use rust_decimal::prelude::FromPrimitive;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};

use crate::entities::{
    recipe_addition_fermentables, recipe_addition_hops, recipe_addition_miscs,
    recipe_addition_waters, recipe_addition_yeasts,
};
use crate::error::AppError;
use crate::models::{
    CreateFermentableAdditionInput, CreateHopAdditionInput, CreateMiscAdditionInput,
    CreateWaterAdditionInput, CreateYeastAdditionInput, RecipeAdditionFermentable,
    RecipeAdditionHop, RecipeAdditionMisc, RecipeAdditionWater, RecipeAdditionYeast,
    UpdateFermentableAdditionInput, UpdateHopAdditionInput, UpdateMiscAdditionInput,
    UpdateWaterAdditionInput, UpdateYeastAdditionInput,
};

use super::new_id;

fn to_dec(v: f64) -> rust_decimal::Decimal {
    rust_decimal::Decimal::from_f64(v).unwrap_or_default()
}

fn to_dec_opt(v: Option<f64>) -> Option<rust_decimal::Decimal> {
    v.map(|x| rust_decimal::Decimal::from_f64(x).unwrap_or_default())
}

pub struct AdditionRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> AdditionRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    // ── Fermentables ────────────────────────────────────────────────────────

    pub async fn create_fermentable(
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

        let row = recipe_addition_fermentables::Entity::find_by_id(&id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        RecipeAdditionFermentable::try_from(row)
    }

    pub async fn update_fermentable(
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

        let updated = recipe_addition_fermentables::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        RecipeAdditionFermentable::try_from(updated)
    }

    pub async fn delete_fermentable(&self, id: &str) -> Result<(), AppError> {
        recipe_addition_fermentables::Entity::delete_by_id(id)
            .exec(self.db)
            .await?;
        Ok(())
    }

    // ── Hops ────────────────────────────────────────────────────────────────

    pub async fn create_hop(
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
            alpha_pct: Set(to_dec(input.alpha_pct)),
            form: Set(input.form.unwrap_or_else(|| "Pellet".to_string())),
            amount_kg: Set(to_dec(input.amount_kg)),
            r#use: Set(input.use_),
            time_min: Set(to_dec(input.time_min)),
            addition_order: Set(order),
        }
        .insert(self.db)
        .await?;

        let row = recipe_addition_hops::Entity::find_by_id(&id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        RecipeAdditionHop::try_from(row)
    }

    pub async fn update_hop(
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
            active.amount_kg = Set(to_dec(v));
        }
        if let Some(v) = input.use_ {
            active.r#use = Set(v);
        }
        if let Some(v) = input.time_min {
            active.time_min = Set(to_dec(v));
        }
        if let Some(v) = input.addition_order {
            active.addition_order = Set(v as i32);
        }

        active.update(self.db).await?;

        let updated = recipe_addition_hops::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        RecipeAdditionHop::try_from(updated)
    }

    pub async fn delete_hop(&self, id: &str) -> Result<(), AppError> {
        recipe_addition_hops::Entity::delete_by_id(id)
            .exec(self.db)
            .await?;
        Ok(())
    }

    // ── Yeasts ──────────────────────────────────────────────────────────────

    pub async fn create_yeast(
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
            add_to_secondary: Set(None),
            times_cultured: Set(None),
        }
        .insert(self.db)
        .await?;

        let row = recipe_addition_yeasts::Entity::find_by_id(&id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        RecipeAdditionYeast::try_from(row)
    }

    pub async fn update_yeast(
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

        let updated = recipe_addition_yeasts::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        RecipeAdditionYeast::try_from(updated)
    }

    pub async fn delete_yeast(&self, id: &str) -> Result<(), AppError> {
        recipe_addition_yeasts::Entity::delete_by_id(id)
            .exec(self.db)
            .await?;
        Ok(())
    }

    // ── Miscs ───────────────────────────────────────────────────────────────

    pub async fn create_misc(
        &self,
        recipe_id: &str,
        input: CreateMiscAdditionInput,
    ) -> Result<RecipeAdditionMisc, AppError> {
        let order = recipe_addition_miscs::Entity::find()
            .filter(recipe_addition_miscs::Column::RecipeId.eq(recipe_id))
            .count(self.db)
            .await? as i32;

        let id = new_id();
        recipe_addition_miscs::ActiveModel {
            id: Set(id.clone()),
            recipe_id: Set(recipe_id.to_string()),
            misc_id: Set(input.misc_id),
            name: Set(input.name),
            r#type: Set(input.type_),
            r#use: Set(input.use_),
            amount: Set(to_dec(input.amount)),
            amount_is_weight: Set(input.amount_is_weight.map(|v| v as i32)),
            time_min: Set(to_dec(input.time_min)),
            addition_order: Set(order),
        }
        .insert(self.db)
        .await?;

        let row = recipe_addition_miscs::Entity::find_by_id(&id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        RecipeAdditionMisc::try_from(row)
    }

    pub async fn update_misc(
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
            active.amount = Set(to_dec(v));
        }
        if let Some(v) = input.amount_is_weight {
            active.amount_is_weight = Set(Some(v as i32));
        }
        if let Some(v) = input.use_ {
            active.r#use = Set(v);
        }
        if let Some(v) = input.time_min {
            active.time_min = Set(to_dec(v));
        }
        if let Some(v) = input.addition_order {
            active.addition_order = Set(v as i32);
        }

        active.update(self.db).await?;

        let updated = recipe_addition_miscs::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        RecipeAdditionMisc::try_from(updated)
    }

    pub async fn delete_misc(&self, id: &str) -> Result<(), AppError> {
        recipe_addition_miscs::Entity::delete_by_id(id)
            .exec(self.db)
            .await?;
        Ok(())
    }

    // ── Waters ──────────────────────────────────────────────────────────────

    pub async fn create_water(
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

        let row = recipe_addition_waters::Entity::find_by_id(&id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        RecipeAdditionWater::try_from(row)
    }

    pub async fn update_water(
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

        let updated = recipe_addition_waters::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        RecipeAdditionWater::try_from(updated)
    }

    pub async fn delete_water(&self, id: &str) -> Result<(), AppError> {
        recipe_addition_waters::Entity::delete_by_id(id)
            .exec(self.db)
            .await?;
        Ok(())
    }
}
