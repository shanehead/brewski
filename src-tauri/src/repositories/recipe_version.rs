use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};

use crate::entities::{
    recipe_version_fermentables, recipe_version_hops, recipe_version_mash,
    recipe_version_mash_steps, recipe_version_miscs, recipe_version_water_adjustments,
    recipe_version_waters, recipe_version_yeasts, recipe_versions,
};
use crate::error::AppError;
use crate::models::RecipeVersionSummary;
use crate::repositories::recipe::RecipeRepository;

use super::{new_id, now_secs};

pub struct RecipeVersionRepository<'a> {
    pub db: &'a DatabaseConnection,
}

impl<'a> RecipeVersionRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list_for_recipe(
        &self,
        recipe_id: &str,
    ) -> Result<Vec<RecipeVersionSummary>, AppError> {
        recipe_versions::Entity::find()
            .filter(recipe_versions::Column::RecipeId.eq(recipe_id))
            .order_by_desc(recipe_versions::Column::VersionNumber)
            .all(self.db)
            .await?
            .into_iter()
            .map(RecipeVersionSummary::try_from)
            .collect()
    }

    pub async fn create_or_reuse(&self, recipe_id: &str) -> Result<RecipeVersionSummary, AppError> {
        let recipe = RecipeRepository::new(self.db).get(recipe_id).await?;

        let last = recipe_versions::Entity::find()
            .filter(recipe_versions::Column::RecipeId.eq(recipe_id))
            .order_by_desc(recipe_versions::Column::VersionNumber)
            .one(self.db)
            .await?;

        if let Some(last_version) = last {
            if self.matches_current(&last_version.id, &recipe).await? {
                return RecipeVersionSummary::try_from(last_version);
            }
            let next_number = last_version.version_number + 1;
            self.snapshot(recipe_id, &recipe, next_number).await
        } else {
            self.snapshot(recipe_id, &recipe, 1).await
        }
    }

    async fn matches_current(
        &self,
        version_id: &str,
        recipe: &crate::models::Recipe,
    ) -> Result<bool, AppError> {
        let v = recipe_versions::Entity::find_by_id(version_id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        // Compare recipe scalars
        if v.r#type != recipe.type_
            || v.batch_size_l != recipe.batch_size_l
            || v.boil_size_l != recipe.boil_size_l
            || v.boil_time_min != recipe.boil_time_min
            || v.efficiency_pct != recipe.efficiency_pct
            || v.style_id != recipe.style_id
            || v.mash_water_id != recipe.mash_water_id
            || v.sparge_water_id != recipe.sparge_water_id
            || v.notes != recipe.notes
            || v.og != recipe.og
            || v.fg != recipe.fg
            || v.primary_age_days != recipe.primary_age_days
            || v.primary_temp_c != recipe.primary_temp_c
            || v.secondary_age_days != recipe.secondary_age_days
            || v.secondary_temp_c != recipe.secondary_temp_c
            || v.carbonation_vols != recipe.carbonation_vols
            || v.brewer != recipe.brewer
        {
            return Ok(false);
        }

        // Compare fermentables
        let vf = recipe_version_fermentables::Entity::find()
            .filter(recipe_version_fermentables::Column::RecipeVersionId.eq(version_id))
            .order_by_asc(recipe_version_fermentables::Column::AdditionOrder)
            .all(self.db)
            .await?;
        if vf.len() != recipe.fermentables.len() {
            return Ok(false);
        }
        for (a, b) in vf.iter().zip(recipe.fermentables.iter()) {
            if a.name != b.name
                || a.r#type != b.type_
                || a.yield_pct != b.yield_pct
                || a.color_lovibond != b.color_lovibond
                || a.amount_kg != b.amount_kg
                || a.fermentable_id != b.fermentable_id
            {
                return Ok(false);
            }
        }

        // Compare hops
        let vh = recipe_version_hops::Entity::find()
            .filter(recipe_version_hops::Column::RecipeVersionId.eq(version_id))
            .order_by_asc(recipe_version_hops::Column::AdditionOrder)
            .all(self.db)
            .await?;
        if vh.len() != recipe.hops.len() {
            return Ok(false);
        }
        for (a, b) in vh.iter().zip(recipe.hops.iter()) {
            if a.name != b.name
                || a.alpha_pct != b.alpha_pct
                || a.amount_kg != b.amount_kg
                || a.r#use != b.use_
                || a.time_min != b.time_min
                || a.form != b.form
            {
                return Ok(false);
            }
        }

        // Compare yeasts
        let vy = recipe_version_yeasts::Entity::find()
            .filter(recipe_version_yeasts::Column::RecipeVersionId.eq(version_id))
            .all(self.db)
            .await?;
        if vy.len() != recipe.yeasts.len() {
            return Ok(false);
        }
        for (a, b) in vy.iter().zip(recipe.yeasts.iter()) {
            if a.name != b.name || a.r#type != b.type_ || a.form != b.form {
                return Ok(false);
            }
        }

        // Compare miscs
        let vm = recipe_version_miscs::Entity::find()
            .filter(recipe_version_miscs::Column::RecipeVersionId.eq(version_id))
            .order_by_asc(recipe_version_miscs::Column::AdditionOrder)
            .all(self.db)
            .await?;
        if vm.len() != recipe.miscs.len() {
            return Ok(false);
        }
        for (a, b) in vm.iter().zip(recipe.miscs.iter()) {
            if a.name != b.name || a.amount != b.amount || a.time_min != b.time_min {
                return Ok(false);
            }
        }

        // Compare waters
        let vw = recipe_version_waters::Entity::find()
            .filter(recipe_version_waters::Column::RecipeVersionId.eq(version_id))
            .all(self.db)
            .await?;
        if vw.len() != recipe.waters.len() {
            return Ok(false);
        }
        for (a, b) in vw.iter().zip(recipe.waters.iter()) {
            if a.name != b.name || a.amount_l != b.amount_l {
                return Ok(false);
            }
        }

        // Compare water adjustments
        let va = recipe_version_water_adjustments::Entity::find()
            .filter(recipe_version_water_adjustments::Column::RecipeVersionId.eq(version_id))
            .all(self.db)
            .await?;
        if va.len() != recipe.water_adjustments.len() {
            return Ok(false);
        }
        for (a, b) in va.iter().zip(recipe.water_adjustments.iter()) {
            if a.addition != b.addition.to_string()
                || a.target != b.target.to_string()
                || a.amount != b.amount
            {
                return Ok(false);
            }
        }

        // Compare mash
        let vmash = recipe_version_mash::Entity::find()
            .filter(recipe_version_mash::Column::RecipeVersionId.eq(version_id))
            .one(self.db)
            .await?;
        match (&vmash, &recipe.mash) {
            (None, None) => {}
            (Some(_), None) | (None, Some(_)) => return Ok(false),
            (Some(vm), Some(rm)) => {
                if vm.name != rm.name
                    || vm.grain_temp_c != rm.grain_temp_c
                    || vm.ratio_l_per_kg != rm.ratio_l_per_kg
                {
                    return Ok(false);
                }
                let vsteps = recipe_version_mash_steps::Entity::find()
                    .filter(recipe_version_mash_steps::Column::RecipeVersionMashId.eq(&vm.id))
                    .order_by_asc(recipe_version_mash_steps::Column::StepOrder)
                    .all(self.db)
                    .await?;
                if vsteps.len() != rm.steps.len() {
                    return Ok(false);
                }
                for (a, b) in vsteps.iter().zip(rm.steps.iter()) {
                    if a.step_temp_c != b.step_temp_c || a.step_time_min != b.step_time_min as i32 {
                        return Ok(false);
                    }
                }
            }
        }

        Ok(true)
    }

    async fn snapshot(
        &self,
        recipe_id: &str,
        recipe: &crate::models::Recipe,
        version_number: i32,
    ) -> Result<RecipeVersionSummary, AppError> {
        let version_id = new_id();
        let now = now_secs() as i32;

        recipe_versions::ActiveModel {
            id: Set(version_id.clone()),
            recipe_id: Set(recipe_id.to_string()),
            version_number: Set(version_number),
            name: Set(None),
            r#type: Set(recipe.type_.clone()),
            brewer: Set(recipe.brewer.clone()),
            asst_brewer: Set(recipe.asst_brewer.clone()),
            batch_size_l: Set(recipe.batch_size_l),
            boil_size_l: Set(recipe.boil_size_l),
            boil_time_min: Set(recipe.boil_time_min),
            efficiency_pct: Set(recipe.efficiency_pct),
            style_id: Set(recipe.style_id.clone()),
            equipment_profile_id: Set(recipe.equipment_profile_id.clone()),
            mash_water_id: Set(recipe.mash_water_id.clone()),
            sparge_water_id: Set(recipe.sparge_water_id.clone()),
            notes: Set(recipe.notes.clone()),
            og: Set(recipe.og),
            fg: Set(recipe.fg),
            fermentation_stages: Set(Some(recipe.fermentation_stages as i32)),
            primary_age_days: Set(recipe.primary_age_days),
            primary_temp_c: Set(recipe.primary_temp_c),
            secondary_age_days: Set(recipe.secondary_age_days),
            secondary_temp_c: Set(recipe.secondary_temp_c),
            tertiary_age_days: Set(recipe.tertiary_age_days),
            tertiary_temp_c: Set(recipe.tertiary_temp_c),
            age_days: Set(recipe.age_days),
            age_temp_c: Set(recipe.age_temp_c),
            carbonation_vols: Set(recipe.carbonation_vols),
            forced_carbonation: Set(Some(if recipe.forced_carbonation { 1 } else { 0 })),
            priming_sugar_name: Set(recipe.priming_sugar_name.clone()),
            carbonation_temp_c: Set(recipe.carbonation_temp_c),
            priming_sugar_equiv: Set(recipe.priming_sugar_equiv),
            keg_priming_factor: Set(recipe.keg_priming_factor),
            created_at: Set(now),
        }
        .insert(self.db)
        .await?;

        // Snapshot fermentables
        for f in &recipe.fermentables {
            recipe_version_fermentables::ActiveModel {
                id: Set(new_id()),
                recipe_version_id: Set(version_id.clone()),
                fermentable_id: Set(f.fermentable_id.clone()),
                name: Set(f.name.clone()),
                r#type: Set(f.type_.clone()),
                yield_pct: Set(f.yield_pct),
                color_lovibond: Set(f.color_lovibond),
                amount_kg: Set(f.amount_kg),
                add_after_boil: Set(Some(f.add_after_boil as i32)),
                addition_order: Set(f.addition_order as i32),
            }
            .insert(self.db)
            .await?;
        }

        // Snapshot hops
        for h in &recipe.hops {
            recipe_version_hops::ActiveModel {
                id: Set(new_id()),
                recipe_version_id: Set(version_id.clone()),
                hop_id: Set(h.hop_id.clone()),
                name: Set(h.name.clone()),
                alpha_pct: Set(h.alpha_pct),
                form: Set(h.form.clone()),
                amount_kg: Set(h.amount_kg),
                r#use: Set(h.use_.clone()),
                time_min: Set(h.time_min),
                addition_order: Set(h.addition_order as i32),
            }
            .insert(self.db)
            .await?;
        }

        // Snapshot yeasts
        for y in &recipe.yeasts {
            recipe_version_yeasts::ActiveModel {
                id: Set(new_id()),
                recipe_version_id: Set(version_id.clone()),
                yeast_id: Set(y.yeast_id.clone()),
                name: Set(y.name.clone()),
                r#type: Set(y.type_.clone()),
                form: Set(y.form.clone()),
                laboratory: Set(y.laboratory.clone()),
                product_id: Set(y.product_id.clone()),
                attenuation_pct: Set(y.attenuation_pct),
                amount: Set(y.amount),
                amount_is_weight: Set(Some(y.amount_is_weight as i32)),
                add_to_secondary: Set(Some(y.add_to_secondary as i32)),
                times_cultured: Set(Some(y.times_cultured as i32)),
            }
            .insert(self.db)
            .await?;
        }

        // Snapshot miscs
        for m in &recipe.miscs {
            recipe_version_miscs::ActiveModel {
                id: Set(new_id()),
                recipe_version_id: Set(version_id.clone()),
                misc_id: Set(m.misc_id.clone()),
                name: Set(m.name.clone()),
                r#type: Set(m.type_.clone()),
                r#use: Set(m.use_.clone()),
                amount: Set(m.amount),
                amount_is_weight: Set(Some(m.amount_is_weight as i32)),
                time_min: Set(m.time_min),
                addition_order: Set(m.addition_order as i32),
            }
            .insert(self.db)
            .await?;
        }

        // Snapshot waters
        for w in &recipe.waters {
            recipe_version_waters::ActiveModel {
                id: Set(new_id()),
                recipe_version_id: Set(version_id.clone()),
                water_id: Set(w.water_id.clone()),
                name: Set(w.name.clone()),
                amount_l: Set(w.amount_l),
            }
            .insert(self.db)
            .await?;
        }

        // Snapshot water adjustments
        for a in &recipe.water_adjustments {
            recipe_version_water_adjustments::ActiveModel {
                id: Set(new_id()),
                recipe_version_id: Set(version_id.clone()),
                addition: Set(a.addition.to_string()),
                target: Set(a.target.to_string()),
                amount: Set(a.amount),
            }
            .insert(self.db)
            .await?;
        }

        // Snapshot mash
        if let Some(mash) = &recipe.mash {
            let mash_snap_id = new_id();
            recipe_version_mash::ActiveModel {
                id: Set(mash_snap_id.clone()),
                recipe_version_id: Set(version_id.clone()),
                name: Set(mash.name.clone()),
                grain_temp_c: Set(mash.grain_temp_c),
                tun_temp_c: Set(mash.tun_temp_c),
                sparge_temp_c: Set(mash.sparge_temp_c),
                ph: Set(mash.ph),
                notes: Set(mash.notes.clone()),
                ratio_l_per_kg: Set(mash.ratio_l_per_kg),
                tun_weight_kg: Set(mash.tun_weight_kg),
                tun_specific_heat: Set(mash.tun_specific_heat),
                equip_adjust: Set(Some(mash.equip_adjust as i32)),
            }
            .insert(self.db)
            .await?;

            for step in &mash.steps {
                recipe_version_mash_steps::ActiveModel {
                    id: Set(new_id()),
                    recipe_version_mash_id: Set(mash_snap_id.clone()),
                    name: Set(step.name.clone()),
                    r#type: Set(step.type_.clone()),
                    infuse_amount_l: Set(step.infuse_amount_l),
                    step_temp_c: Set(step.step_temp_c),
                    step_time_min: Set(step.step_time_min as i32),
                    ramp_time_min: Set(step.ramp_time_min.map(|v| v as i32)),
                    end_temp_c: Set(step.end_temp_c),
                    step_order: Set(step.step_order as i32),
                }
                .insert(self.db)
                .await?;
            }
        }

        recipe_versions::Entity::find_by_id(&version_id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(RecipeVersionSummary::try_from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::CreateFermentableAdditionInput;
    use crate::models::CreateRecipeInput;
    use crate::repositories::fermentable::FermentableRepository;
    use crate::repositories::recipe::RecipeRepository;
    use crate::test_helpers::setup_test_db;

    async fn make_recipe(db: &DatabaseConnection) -> String {
        RecipeRepository::new(db)
            .create(CreateRecipeInput {
                name: "Test IPA".into(),
                batch_size_l: Some(23.0),
                boil_size_l: Some(27.0),
                boil_time_min: Some(60.0),
                ..Default::default()
            })
            .await
            .unwrap()
            .id
    }

    #[tokio::test]
    async fn test_first_brew_creates_version_1() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = RecipeVersionRepository::new(&db);
        let v = repo.create_or_reuse(&recipe_id).await.unwrap();
        assert_eq!(v.version_number, 1);
    }

    #[tokio::test]
    async fn test_unchanged_recipe_reuses_version() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = RecipeVersionRepository::new(&db);
        let v1 = repo.create_or_reuse(&recipe_id).await.unwrap();
        let v2 = repo.create_or_reuse(&recipe_id).await.unwrap();
        assert_eq!(v1.id, v2.id);
        assert_eq!(v2.version_number, 1);
    }

    #[tokio::test]
    async fn test_changed_recipe_creates_new_version() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = RecipeVersionRepository::new(&db);
        let v1 = repo.create_or_reuse(&recipe_id).await.unwrap();

        // Add a fermentable to change the recipe
        FermentableRepository::new(&db)
            .create(
                &recipe_id,
                CreateFermentableAdditionInput {
                    fermentable_id: None,
                    name: "Pale Malt".into(),
                    type_: "grain".into(),
                    yield_pct: 78.0,
                    color_lovibond: 1.8,
                    amount_kg: 4.5,
                    add_after_boil: None,
                },
            )
            .await
            .unwrap();

        let v2 = repo.create_or_reuse(&recipe_id).await.unwrap();
        assert_ne!(v1.id, v2.id);
        assert_eq!(v2.version_number, 2);
    }

    #[tokio::test]
    async fn test_list_for_recipe() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = RecipeVersionRepository::new(&db);
        repo.create_or_reuse(&recipe_id).await.unwrap();
        let versions = repo.list_for_recipe(&recipe_id).await.unwrap();
        assert_eq!(versions.len(), 1);
    }
}
