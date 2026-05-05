use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};

use crate::entities::{mash_steps, mashes};
use crate::error::AppError;
use crate::models::{CreateMashStepInput, Mash, MashStep, UpdateMashInput, UpdateMashStepInput};

use super::{new_id, to_dec, to_dec_opt};

fn from_dec(v: rust_decimal::Decimal) -> Result<f64, AppError> {
    use rust_decimal::prelude::ToPrimitive;
    v.to_f64()
        .ok_or_else(|| AppError::Conversion(format!("cannot convert {} to f64", v)))
}

fn from_dec_opt(v: Option<rust_decimal::Decimal>) -> Result<Option<f64>, AppError> {
    match v {
        Some(dec) => Ok(Some(from_dec(dec)?)),
        None => Ok(None),
    }
}

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
            grain_temp_c: from_dec(mash_row.grain_temp_c)?,
            tun_temp_c: from_dec_opt(mash_row.tun_temp_c)?,
            sparge_temp_c: from_dec_opt(mash_row.sparge_temp_c)?,
            ph: from_dec_opt(mash_row.ph)?,
            tun_weight_kg: from_dec_opt(mash_row.tun_weight_kg)?,
            tun_specific_heat: from_dec_opt(mash_row.tun_specific_heat)?,
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
                active.grain_temp_c = Set(to_dec(v));
            }
            if let Some(v) = input.tun_temp_c {
                active.tun_temp_c = Set(Some(to_dec(v)));
            }
            if let Some(v) = input.sparge_temp_c {
                active.sparge_temp_c = Set(Some(to_dec(v)));
            }
            if let Some(v) = input.ph {
                active.ph = Set(Some(to_dec(v)));
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
                grain_temp_c: Set(to_dec(input.grain_temp_c.unwrap_or(20.0))),
                tun_temp_c: Set(to_dec_opt(input.tun_temp_c)),
                sparge_temp_c: Set(to_dec_opt(input.sparge_temp_c)),
                ph: Set(to_dec_opt(input.ph)),
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
            infuse_amount_l: Set(to_dec_opt(input.infuse_amount_l)),
            step_temp_c: Set(to_dec(input.step_temp_c)),
            step_time_min: Set(input.step_time_min as i32),
            ramp_time_min: Set(input.ramp_time_min.map(|v| v as i32)),
            end_temp_c: Set(to_dec_opt(input.end_temp_c)),
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
            active.infuse_amount_l = Set(Some(to_dec(v)));
        }
        if let Some(v) = input.step_temp_c {
            active.step_temp_c = Set(to_dec(v));
        }
        if let Some(v) = input.step_time_min {
            active.step_time_min = Set(v as i32);
        }
        if let Some(v) = input.ramp_time_min {
            active.ramp_time_min = Set(Some(v as i32));
        }
        if let Some(v) = input.end_temp_c {
            active.end_temp_c = Set(Some(to_dec(v)));
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
