use rust_decimal::prelude::FromPrimitive;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};

use crate::entities::{
    equipment_profiles, mash_steps, mashes, recipe_addition_fermentables, recipe_addition_hops,
    recipe_addition_miscs, recipe_addition_waters, recipe_addition_yeasts, recipes, styles,
};
use crate::error::AppError;
use crate::models::{
    CreateRecipeInput, EquipmentProfile, Mash, MashStep, Recipe, RecipeAdditionFermentable,
    RecipeAdditionHop, RecipeAdditionMisc, RecipeAdditionWater, RecipeAdditionYeast, RecipeSummary,
    Style, UpdateRecipeInput,
};

use super::{new_id, now_secs};

fn to_dec(v: f64) -> rust_decimal::Decimal {
    rust_decimal::Decimal::from_f64(v).unwrap_or_default()
}

fn from_dec(v: rust_decimal::Decimal) -> Result<f64, AppError> {
    use rust_decimal::prelude::ToPrimitive;
    v.to_f64()
        .ok_or_else(|| AppError::Conversion(format!("cannot convert {} to f64", v)))
}

pub struct RecipeRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> RecipeRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list(&self) -> Result<Vec<RecipeSummary>, AppError> {
        let results = recipes::Entity::find()
            .find_also_related(styles::Entity)
            .order_by_desc(recipes::Column::UpdatedAt)
            .all(self.db)
            .await?;

        results
            .into_iter()
            .map(|(r, s)| {
                Ok(RecipeSummary {
                    id: r.id,
                    name: r.name,
                    style_name: s.map(|st| st.name),
                    type_: r.r#type,
                    batch_size_l: from_dec(r.batch_size_l)?,
                    created_at: r.created_at as i64,
                    updated_at: r.updated_at as i64,
                })
            })
            .collect()
    }

    pub async fn get(&self, id: &str) -> Result<Recipe, AppError> {
        let recipe_row = recipes::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        // Load all related additions
        let fermentable_rows = recipe_addition_fermentables::Entity::find()
            .filter(recipe_addition_fermentables::Column::RecipeId.eq(id))
            .order_by_asc(recipe_addition_fermentables::Column::AdditionOrder)
            .all(self.db)
            .await?;

        let hop_rows = recipe_addition_hops::Entity::find()
            .filter(recipe_addition_hops::Column::RecipeId.eq(id))
            .order_by_asc(recipe_addition_hops::Column::AdditionOrder)
            .all(self.db)
            .await?;

        let yeast_rows = recipe_addition_yeasts::Entity::find()
            .filter(recipe_addition_yeasts::Column::RecipeId.eq(id))
            .all(self.db)
            .await?;

        let misc_rows = recipe_addition_miscs::Entity::find()
            .filter(recipe_addition_miscs::Column::RecipeId.eq(id))
            .order_by_asc(recipe_addition_miscs::Column::AdditionOrder)
            .all(self.db)
            .await?;

        let water_rows = recipe_addition_waters::Entity::find()
            .filter(recipe_addition_waters::Column::RecipeId.eq(id))
            .all(self.db)
            .await?;

        // Load optional mash
        let mash = if let Some(mash_row) = mashes::Entity::find()
            .filter(mashes::Column::RecipeId.eq(id))
            .one(self.db)
            .await?
        {
            let step_rows = mash_steps::Entity::find()
                .filter(mash_steps::Column::MashId.eq(&mash_row.id))
                .order_by_asc(mash_steps::Column::StepOrder)
                .all(self.db)
                .await?;

            let steps: Result<Vec<MashStep>, AppError> =
                step_rows.into_iter().map(MashStep::try_from).collect();

            Some(Mash {
                id: mash_row.id,
                recipe_id: mash_row.recipe_id,
                name: mash_row.name,
                grain_temp_c: from_dec(mash_row.grain_temp_c)?,
                tun_temp_c: {
                    use rust_decimal::prelude::ToPrimitive;
                    mash_row.tun_temp_c.and_then(|v| v.to_f64())
                },
                sparge_temp_c: {
                    use rust_decimal::prelude::ToPrimitive;
                    mash_row.sparge_temp_c.and_then(|v| v.to_f64())
                },
                ph: {
                    use rust_decimal::prelude::ToPrimitive;
                    mash_row.ph.and_then(|v| v.to_f64())
                },
                tun_weight_kg: {
                    use rust_decimal::prelude::ToPrimitive;
                    mash_row.tun_weight_kg.and_then(|v| v.to_f64())
                },
                tun_specific_heat: {
                    use rust_decimal::prelude::ToPrimitive;
                    mash_row.tun_specific_heat.and_then(|v| v.to_f64())
                },
                equip_adjust: mash_row.equip_adjust.unwrap_or(0) != 0,
                notes: mash_row.notes,
                steps: steps?,
            })
        } else {
            None
        };

        // Load optional equipment profile
        let equipment_profile = if let Some(ep_id) = &recipe_row.equipment_profile_id {
            equipment_profiles::Entity::find_by_id(ep_id.as_str())
                .one(self.db)
                .await?
                .map(EquipmentProfile::try_from)
                .transpose()?
        } else {
            None
        };

        // Load optional style
        let style = if let Some(style_id) = &recipe_row.style_id {
            styles::Entity::find_by_id(style_id.as_str())
                .one(self.db)
                .await?
                .map(Style::try_from)
                .transpose()?
        } else {
            None
        };

        // Convert additions
        let fermentables: Result<Vec<RecipeAdditionFermentable>, AppError> =
            fermentable_rows
                .into_iter()
                .map(RecipeAdditionFermentable::try_from)
                .collect();
        let hops: Result<Vec<RecipeAdditionHop>, AppError> =
            hop_rows.into_iter().map(RecipeAdditionHop::try_from).collect();
        let yeasts: Result<Vec<RecipeAdditionYeast>, AppError> =
            yeast_rows.into_iter().map(RecipeAdditionYeast::try_from).collect();
        let miscs: Result<Vec<RecipeAdditionMisc>, AppError> =
            misc_rows.into_iter().map(RecipeAdditionMisc::try_from).collect();
        let waters: Result<Vec<RecipeAdditionWater>, AppError> =
            water_rows.into_iter().map(RecipeAdditionWater::try_from).collect();

        use rust_decimal::prelude::ToPrimitive;

        Ok(Recipe {
            id: recipe_row.id,
            name: recipe_row.name,
            type_: recipe_row.r#type,
            brewer: recipe_row.brewer,
            asst_brewer: recipe_row.asst_brewer,
            batch_size_l: from_dec(recipe_row.batch_size_l)?,
            boil_size_l: from_dec(recipe_row.boil_size_l)?,
            boil_time_min: from_dec(recipe_row.boil_time_min)?,
            efficiency_pct: recipe_row.efficiency_pct.and_then(|v| v.to_f64()),
            style_id: recipe_row.style_id,
            equipment_profile_id: recipe_row.equipment_profile_id,
            notes: recipe_row.notes,
            taste_notes: recipe_row.taste_notes,
            taste_rating: recipe_row.taste_rating.and_then(|v| v.to_f64()),
            og: recipe_row.og.and_then(|v| v.to_f64()),
            fg: recipe_row.fg.and_then(|v| v.to_f64()),
            fermentation_stages: recipe_row.fermentation_stages.unwrap_or(1) as i64,
            primary_age_days: recipe_row.primary_age_days.and_then(|v| v.to_f64()),
            primary_temp_c: recipe_row.primary_temp_c.and_then(|v| v.to_f64()),
            secondary_age_days: recipe_row.secondary_age_days.and_then(|v| v.to_f64()),
            secondary_temp_c: recipe_row.secondary_temp_c.and_then(|v| v.to_f64()),
            tertiary_age_days: recipe_row.tertiary_age_days.and_then(|v| v.to_f64()),
            tertiary_temp_c: recipe_row.tertiary_temp_c.and_then(|v| v.to_f64()),
            age_days: recipe_row.age_days.and_then(|v| v.to_f64()),
            age_temp_c: recipe_row.age_temp_c.and_then(|v| v.to_f64()),
            carbonation_vols: recipe_row.carbonation_vols.and_then(|v| v.to_f64()),
            forced_carbonation: recipe_row.forced_carbonation.unwrap_or(0) != 0,
            priming_sugar_name: recipe_row.priming_sugar_name,
            carbonation_temp_c: recipe_row.carbonation_temp_c.and_then(|v| v.to_f64()),
            priming_sugar_equiv: recipe_row.priming_sugar_equiv.and_then(|v| v.to_f64()),
            keg_priming_factor: recipe_row.keg_priming_factor.and_then(|v| v.to_f64()),
            date: recipe_row.date,
            created_at: recipe_row.created_at as i64,
            updated_at: recipe_row.updated_at as i64,
            equipment_profile,
            style,
            fermentables: fermentables?,
            hops: hops?,
            yeasts: yeasts?,
            miscs: miscs?,
            waters: waters?,
            mash,
        })
    }

    pub async fn create(&self, input: CreateRecipeInput) -> Result<Recipe, AppError> {
        let id = new_id();
        let now = now_secs() as i32;

        let (batch_size, boil_size, boil_time, ep_id) =
            if let Some(ref src_id) = input.source_id {
                let src = self.get(src_id).await?;
                (
                    src.batch_size_l,
                    src.boil_size_l,
                    src.boil_time_min,
                    src.equipment_profile_id,
                )
            } else {
                (
                    input.batch_size_l.unwrap_or(23.0),
                    input.boil_size_l.unwrap_or(27.0),
                    input.boil_time_min.unwrap_or(60.0),
                    input.equipment_profile_id,
                )
            };

        recipes::ActiveModel {
            id: Set(id.clone()),
            name: Set(input.name),
            r#type: Set(input.type_.unwrap_or_else(|| "all_grain".to_owned())),
            batch_size_l: Set(to_dec(batch_size)),
            boil_size_l: Set(to_dec(boil_size)),
            boil_time_min: Set(to_dec(boil_time)),
            equipment_profile_id: Set(ep_id),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        }
        .insert(self.db)
        .await?;

        if let Some(src_id) = input.source_id {
            self.copy_additions(&src_id, &id).await?;
        }

        self.get(&id).await
    }

    async fn copy_additions(&self, src_id: &str, dst_id: &str) -> Result<(), AppError> {
        // Copy fermentables
        let fermentables = recipe_addition_fermentables::Entity::find()
            .filter(recipe_addition_fermentables::Column::RecipeId.eq(src_id))
            .order_by_asc(recipe_addition_fermentables::Column::AdditionOrder)
            .all(self.db)
            .await?;

        for f in fermentables {
            recipe_addition_fermentables::ActiveModel {
                id: Set(new_id()),
                recipe_id: Set(dst_id.to_owned()),
                fermentable_id: Set(f.fermentable_id),
                name: Set(f.name),
                r#type: Set(f.r#type),
                yield_pct: Set(f.yield_pct),
                color_lovibond: Set(f.color_lovibond),
                amount_kg: Set(f.amount_kg),
                add_after_boil: Set(f.add_after_boil),
                addition_order: Set(f.addition_order),
            }
            .insert(self.db)
            .await?;
        }

        // Copy hops
        let hops = recipe_addition_hops::Entity::find()
            .filter(recipe_addition_hops::Column::RecipeId.eq(src_id))
            .order_by_asc(recipe_addition_hops::Column::AdditionOrder)
            .all(self.db)
            .await?;

        for h in hops {
            recipe_addition_hops::ActiveModel {
                id: Set(new_id()),
                recipe_id: Set(dst_id.to_owned()),
                hop_id: Set(h.hop_id),
                name: Set(h.name),
                alpha_pct: Set(h.alpha_pct),
                form: Set(h.form),
                amount_kg: Set(h.amount_kg),
                r#use: Set(h.r#use),
                time_min: Set(h.time_min),
                addition_order: Set(h.addition_order),
            }
            .insert(self.db)
            .await?;
        }

        // Copy yeasts
        let yeasts = recipe_addition_yeasts::Entity::find()
            .filter(recipe_addition_yeasts::Column::RecipeId.eq(src_id))
            .all(self.db)
            .await?;

        for y in yeasts {
            recipe_addition_yeasts::ActiveModel {
                id: Set(new_id()),
                recipe_id: Set(dst_id.to_owned()),
                yeast_id: Set(y.yeast_id),
                name: Set(y.name),
                r#type: Set(y.r#type),
                form: Set(y.form),
                laboratory: Set(y.laboratory),
                product_id: Set(y.product_id),
                attenuation_pct: Set(y.attenuation_pct),
                amount: Set(y.amount),
                amount_is_weight: Set(y.amount_is_weight),
                add_to_secondary: Set(y.add_to_secondary),
                times_cultured: Set(y.times_cultured),
            }
            .insert(self.db)
            .await?;
        }

        // Copy miscs
        let miscs = recipe_addition_miscs::Entity::find()
            .filter(recipe_addition_miscs::Column::RecipeId.eq(src_id))
            .order_by_asc(recipe_addition_miscs::Column::AdditionOrder)
            .all(self.db)
            .await?;

        for m in miscs {
            recipe_addition_miscs::ActiveModel {
                id: Set(new_id()),
                recipe_id: Set(dst_id.to_owned()),
                misc_id: Set(m.misc_id),
                name: Set(m.name),
                r#type: Set(m.r#type),
                r#use: Set(m.r#use),
                amount: Set(m.amount),
                amount_is_weight: Set(m.amount_is_weight),
                time_min: Set(m.time_min),
                addition_order: Set(m.addition_order),
            }
            .insert(self.db)
            .await?;
        }

        // Copy waters
        let waters = recipe_addition_waters::Entity::find()
            .filter(recipe_addition_waters::Column::RecipeId.eq(src_id))
            .all(self.db)
            .await?;

        for w in waters {
            recipe_addition_waters::ActiveModel {
                id: Set(new_id()),
                recipe_id: Set(dst_id.to_owned()),
                water_id: Set(w.water_id),
                name: Set(w.name),
                amount_l: Set(w.amount_l),
            }
            .insert(self.db)
            .await?;
        }

        Ok(())
    }

    pub async fn update(&self, id: &str, input: UpdateRecipeInput) -> Result<Recipe, AppError> {
        let current = recipes::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut active: recipes::ActiveModel = current.into();

        if let Some(v) = input.name {
            active.name = Set(v);
        }
        if let Some(v) = input.type_ {
            active.r#type = Set(v);
        }
        if let Some(v) = input.brewer {
            active.brewer = Set(Some(v));
        }
        if let Some(v) = input.asst_brewer {
            active.asst_brewer = Set(Some(v));
        }
        if let Some(v) = input.batch_size_l {
            active.batch_size_l = Set(to_dec(v));
        }
        if let Some(v) = input.boil_size_l {
            active.boil_size_l = Set(to_dec(v));
        }
        if let Some(v) = input.boil_time_min {
            active.boil_time_min = Set(to_dec(v));
        }
        if let Some(v) = input.efficiency_pct {
            active.efficiency_pct = Set(Some(to_dec(v)));
        }
        if let Some(v) = input.style_id {
            active.style_id = Set(Some(v));
        }
        if let Some(v) = input.equipment_profile_id {
            active.equipment_profile_id = Set(Some(v));
        }
        if let Some(v) = input.notes {
            active.notes = Set(Some(v));
        }
        if let Some(v) = input.taste_notes {
            active.taste_notes = Set(Some(v));
        }
        if let Some(v) = input.taste_rating {
            active.taste_rating = Set(Some(to_dec(v)));
        }
        if let Some(v) = input.fermentation_stages {
            active.fermentation_stages = Set(Some(v as i32));
        }
        if let Some(v) = input.primary_age_days {
            active.primary_age_days = Set(Some(to_dec(v)));
        }
        if let Some(v) = input.primary_temp_c {
            active.primary_temp_c = Set(Some(to_dec(v)));
        }
        if let Some(v) = input.secondary_age_days {
            active.secondary_age_days = Set(Some(to_dec(v)));
        }
        if let Some(v) = input.secondary_temp_c {
            active.secondary_temp_c = Set(Some(to_dec(v)));
        }
        if let Some(v) = input.tertiary_age_days {
            active.tertiary_age_days = Set(Some(to_dec(v)));
        }
        if let Some(v) = input.tertiary_temp_c {
            active.tertiary_temp_c = Set(Some(to_dec(v)));
        }
        if let Some(v) = input.age_days {
            active.age_days = Set(Some(to_dec(v)));
        }
        if let Some(v) = input.age_temp_c {
            active.age_temp_c = Set(Some(to_dec(v)));
        }
        if let Some(v) = input.carbonation_vols {
            active.carbonation_vols = Set(Some(to_dec(v)));
        }
        if let Some(v) = input.forced_carbonation {
            active.forced_carbonation = Set(Some(if v { 1 } else { 0 }));
        }
        if let Some(v) = input.priming_sugar_name {
            active.priming_sugar_name = Set(Some(v));
        }
        if let Some(v) = input.carbonation_temp_c {
            active.carbonation_temp_c = Set(Some(to_dec(v)));
        }
        if let Some(v) = input.date {
            active.date = Set(Some(v));
        }

        active.updated_at = Set(now_secs() as i32);
        active.update(self.db).await?;

        self.get(id).await
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        recipes::Entity::delete_by_id(id).exec(self.db).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test_helpers::setup_test_db;

    fn basic_input() -> CreateRecipeInput {
        CreateRecipeInput {
            name: "Test Recipe".into(),
            type_: Some("all_grain".into()),
            batch_size_l: Some(23.0),
            boil_size_l: Some(27.0),
            boil_time_min: Some(60.0),
            equipment_profile_id: None,
            source_id: None,
        }
    }

    #[tokio::test]
    async fn test_create_and_list() {
        let db = setup_test_db().await;
        let repo = RecipeRepository::new(&db);
        repo.create(basic_input()).await.unwrap();
        let all = repo.list().await.unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].name, "Test Recipe");
    }

    #[tokio::test]
    async fn test_get_returns_full_recipe() {
        let db = setup_test_db().await;
        let repo = RecipeRepository::new(&db);
        let created = repo.create(basic_input()).await.unwrap();
        let fetched = repo.get(&created.id).await.unwrap();
        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.batch_size_l, 23.0);
        assert!(fetched.fermentables.is_empty());
    }

    #[tokio::test]
    async fn test_update_name() {
        let db = setup_test_db().await;
        let repo = RecipeRepository::new(&db);
        let created = repo.create(basic_input()).await.unwrap();
        let updated = repo
            .update(
                &created.id,
                UpdateRecipeInput {
                    name: Some("Updated Name".into()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(updated.name, "Updated Name");
    }

    #[tokio::test]
    async fn test_delete() {
        let db = setup_test_db().await;
        let repo = RecipeRepository::new(&db);
        let created = repo.create(basic_input()).await.unwrap();
        repo.delete(&created.id).await.unwrap();
        assert!(repo.list().await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_duplicate_via_source_id() {
        let db = setup_test_db().await;
        let repo = RecipeRepository::new(&db);
        let original = repo.create(basic_input()).await.unwrap();
        let dupe = repo
            .create(CreateRecipeInput {
                name: "Copy".into(),
                source_id: Some(original.id.clone()),
                ..Default::default()
            })
            .await
            .unwrap();
        assert_ne!(dupe.id, original.id);
        assert_eq!(dupe.batch_size_l, original.batch_size_l);
    }
}
