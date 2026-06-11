use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};

use crate::entities::{
    batches, equipment_profiles, mash_steps, mashes, recipe_addition_fermentables,
    recipe_addition_hops, recipe_addition_miscs, recipe_addition_waters, recipe_addition_yeasts,
    recipe_version_fermentables, recipe_version_hops, recipe_version_mash,
    recipe_version_mash_steps, recipe_version_miscs, recipe_version_water_adjustments,
    recipe_version_waters, recipe_version_yeasts, recipe_versions, recipe_water_adjustments,
    recipes, styles,
};
use crate::error::AppError;
use crate::models::{
    Mash, MashStep, Recipe, RecipeAdditionFermentable, RecipeAdditionHop, RecipeAdditionMisc,
    RecipeAdditionWater, RecipeAdditionYeast, RecipeVersionSummary, RecipeWaterAdjustment,
};
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

        let recipe_row = recipes::Entity::find_by_id(recipe_id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let branch_parent_id = recipe_row.branch_parent_id.clone();

        // If a branch parent is set, always create a new snapshot branching from it
        if let Some(bp_id) = branch_parent_id {
            recipes::ActiveModel {
                id: Set(recipe_id.to_string()),
                branch_parent_id: Set(None),
                ..Default::default()
            }
            .update(self.db)
            .await?;

            let next_number = recipe_versions::Entity::find()
                .filter(recipe_versions::Column::RecipeId.eq(recipe_id))
                .order_by_desc(recipe_versions::Column::VersionNumber)
                .one(self.db)
                .await?
                .map(|v| v.version_number + 1)
                .unwrap_or(1);

            return self
                .snapshot(recipe_id, &recipe, next_number, None, Some(bp_id))
                .await;
        }

        let last = recipe_versions::Entity::find()
            .filter(recipe_versions::Column::RecipeId.eq(recipe_id))
            .order_by_desc(recipe_versions::Column::CreatedAt)
            .one(self.db)
            .await?;

        if let Some(last_version) = last {
            if self.matches_current(&last_version.id, &recipe).await? {
                return RecipeVersionSummary::try_from(last_version);
            }
            let next_number = last_version.version_number + 1;
            let parent_id = Some(last_version.id.clone());
            self.snapshot(recipe_id, &recipe, next_number, None, parent_id)
                .await
        } else {
            self.snapshot(recipe_id, &recipe, 1, None, None).await
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
                || a.add_after_boil.unwrap_or(0) != b.add_after_boil as i32
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
        let mut vy_sorted = vy;
        vy_sorted.sort_by(|a, b| {
            a.name.cmp(&b.name).then(
                a.laboratory
                    .as_deref()
                    .unwrap_or("")
                    .cmp(b.laboratory.as_deref().unwrap_or("")),
            )
        });
        let mut recipe_yeasts_sorted = recipe.yeasts.clone();
        recipe_yeasts_sorted.sort_by(|a, b| {
            a.name.cmp(&b.name).then(
                a.laboratory
                    .as_deref()
                    .unwrap_or("")
                    .cmp(b.laboratory.as_deref().unwrap_or("")),
            )
        });
        for (a, b) in vy_sorted.iter().zip(recipe_yeasts_sorted.iter()) {
            if a.name != b.name
                || a.r#type != b.type_
                || a.form != b.form
                || a.laboratory != b.laboratory
                || a.product_id != b.product_id
                || a.attenuation_pct != b.attenuation_pct
                || a.amount != b.amount
                || a.amount_is_weight.unwrap_or(0) != b.amount_is_weight as i32
                || a.add_to_secondary.unwrap_or(0) != b.add_to_secondary as i32
                || a.times_cultured.unwrap_or(0) != b.times_cultured as i32
            {
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
            if a.name != b.name
                || a.amount != b.amount
                || a.time_min != b.time_min
                || a.r#type != b.type_
                || a.r#use != b.use_
                || a.unit != b.unit
            {
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
                    || vm.tun_temp_c != rm.tun_temp_c
                    || vm.sparge_temp_c != rm.sparge_temp_c
                    || vm.ph != rm.ph
                    || vm.notes != rm.notes
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
                    if a.step_temp_c != b.step_temp_c
                        || a.step_time_min != b.step_time_min as i32
                        || a.name != b.name
                        || a.r#type != b.type_
                        || a.infuse_amount_l != b.infuse_amount_l
                        || a.ramp_time_min != b.ramp_time_min.map(|v| v as i32)
                        || a.end_temp_c != b.end_temp_c
                    {
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
        name: Option<String>,
        parent_version_id: Option<String>,
    ) -> Result<RecipeVersionSummary, AppError> {
        let version_id = new_id();
        let now = now_secs() as i32;

        recipe_versions::ActiveModel {
            id: Set(version_id.clone()),
            recipe_id: Set(recipe_id.to_string()),
            version_number: Set(version_number),
            name: Set(name),
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
            parent_version_id: Set(parent_version_id),
            content_hash: Set(Some(crate::recipe_hash::recipe_content_hash(recipe)?)),
            hopstand_temp_c: Set(Some(recipe.hopstand_temp_c)),
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
                hopstand_temp_c: Set(h.hopstand_temp_c),
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
                unit: Set(m.unit.clone()),
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

    pub async fn branch_from(&self, recipe_id: &str, version_id: &str) -> Result<(), AppError> {
        let full = self.get_full(version_id).await?;

        // Delete all existing additions on the live recipe
        recipe_addition_fermentables::Entity::delete_many()
            .filter(recipe_addition_fermentables::Column::RecipeId.eq(recipe_id))
            .exec(self.db)
            .await?;
        recipe_addition_hops::Entity::delete_many()
            .filter(recipe_addition_hops::Column::RecipeId.eq(recipe_id))
            .exec(self.db)
            .await?;
        recipe_addition_yeasts::Entity::delete_many()
            .filter(recipe_addition_yeasts::Column::RecipeId.eq(recipe_id))
            .exec(self.db)
            .await?;
        recipe_addition_miscs::Entity::delete_many()
            .filter(recipe_addition_miscs::Column::RecipeId.eq(recipe_id))
            .exec(self.db)
            .await?;
        recipe_addition_waters::Entity::delete_many()
            .filter(recipe_addition_waters::Column::RecipeId.eq(recipe_id))
            .exec(self.db)
            .await?;
        recipe_water_adjustments::Entity::delete_many()
            .filter(recipe_water_adjustments::Column::RecipeId.eq(recipe_id))
            .exec(self.db)
            .await?;

        // Delete mash steps then mash
        if let Some(mash_row) = mashes::Entity::find()
            .filter(mashes::Column::RecipeId.eq(recipe_id))
            .one(self.db)
            .await?
        {
            mash_steps::Entity::delete_many()
                .filter(mash_steps::Column::MashId.eq(&mash_row.id))
                .exec(self.db)
                .await?;
            mashes::Entity::delete_by_id(&mash_row.id)
                .exec(self.db)
                .await?;
        }

        // Re-insert fermentables from version snapshot
        for f in &full.fermentables {
            recipe_addition_fermentables::ActiveModel {
                id: Set(new_id()),
                recipe_id: Set(recipe_id.to_string()),
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

        // Re-insert hops
        for h in &full.hops {
            recipe_addition_hops::ActiveModel {
                id: Set(new_id()),
                recipe_id: Set(recipe_id.to_string()),
                hop_id: Set(h.hop_id.clone()),
                name: Set(h.name.clone()),
                alpha_pct: Set(h.alpha_pct),
                form: Set(h.form.clone()),
                amount_kg: Set(h.amount_kg),
                r#use: Set(h.use_.clone()),
                time_min: Set(h.time_min),
                addition_order: Set(h.addition_order as i32),
                hopstand_temp_c: Set(h.hopstand_temp_c),
            }
            .insert(self.db)
            .await?;
        }

        // Re-insert yeasts
        for y in &full.yeasts {
            recipe_addition_yeasts::ActiveModel {
                id: Set(new_id()),
                recipe_id: Set(recipe_id.to_string()),
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

        // Re-insert miscs
        for m in &full.miscs {
            recipe_addition_miscs::ActiveModel {
                id: Set(new_id()),
                recipe_id: Set(recipe_id.to_string()),
                misc_id: Set(m.misc_id.clone()),
                name: Set(m.name.clone()),
                r#type: Set(m.type_.clone()),
                r#use: Set(m.use_.clone()),
                amount: Set(m.amount),
                amount_is_weight: Set(Some(m.amount_is_weight as i32)),
                unit: Set(m.unit.clone()),
                time_min: Set(m.time_min),
                addition_order: Set(m.addition_order as i32),
            }
            .insert(self.db)
            .await?;
        }

        // Re-insert waters
        for w in &full.waters {
            recipe_addition_waters::ActiveModel {
                id: Set(new_id()),
                recipe_id: Set(recipe_id.to_string()),
                water_id: Set(w.water_id.clone()),
                name: Set(w.name.clone()),
                amount_l: Set(w.amount_l),
            }
            .insert(self.db)
            .await?;
        }

        // Re-insert water adjustments
        for a in &full.water_adjustments {
            recipe_water_adjustments::ActiveModel {
                id: Set(new_id()),
                recipe_id: Set(recipe_id.to_string()),
                addition: Set(a.addition.to_string()),
                target: Set(a.target.to_string()),
                amount: Set(a.amount),
            }
            .insert(self.db)
            .await?;
        }

        // Re-insert mash if present
        if let Some(mash) = &full.mash {
            let mash_id = new_id();
            mashes::ActiveModel {
                id: Set(mash_id.clone()),
                recipe_id: Set(recipe_id.to_string()),
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

            for (i, step) in mash.steps.iter().enumerate() {
                mash_steps::ActiveModel {
                    id: Set(new_id()),
                    mash_id: Set(mash_id.clone()),
                    name: Set(step.name.clone()),
                    r#type: Set(step.type_.clone()),
                    infuse_amount_l: Set(step.infuse_amount_l),
                    step_temp_c: Set(step.step_temp_c),
                    step_time_min: Set(step.step_time_min as i32),
                    ramp_time_min: Set(step.ramp_time_min.map(|v| v as i32)),
                    end_temp_c: Set(step.end_temp_c),
                    step_order: Set(i as i32),
                }
                .insert(self.db)
                .await?;
            }
        }

        // Update recipe scalars and set branch_parent_id
        let now = now_secs() as i32;
        recipes::ActiveModel {
            id: Set(recipe_id.to_string()),
            r#type: Set(full.type_.clone()),
            brewer: Set(full.brewer.clone()),
            asst_brewer: Set(full.asst_brewer.clone()),
            batch_size_l: Set(full.batch_size_l),
            boil_size_l: Set(full.boil_size_l),
            boil_time_min: Set(full.boil_time_min),
            efficiency_pct: Set(full.efficiency_pct),
            style_id: Set(full.style_id.clone()),
            equipment_profile_id: Set(full.equipment_profile_id.clone()),
            notes: Set(full.notes.clone()),
            og: Set(full.og),
            fg: Set(full.fg),
            fermentation_stages: Set(Some(full.fermentation_stages as i32)),
            primary_age_days: Set(full.primary_age_days),
            primary_temp_c: Set(full.primary_temp_c),
            secondary_age_days: Set(full.secondary_age_days),
            secondary_temp_c: Set(full.secondary_temp_c),
            tertiary_age_days: Set(full.tertiary_age_days),
            tertiary_temp_c: Set(full.tertiary_temp_c),
            age_days: Set(full.age_days),
            age_temp_c: Set(full.age_temp_c),
            carbonation_vols: Set(full.carbonation_vols),
            forced_carbonation: Set(Some(full.forced_carbonation as i32)),
            priming_sugar_name: Set(full.priming_sugar_name.clone()),
            carbonation_temp_c: Set(full.carbonation_temp_c),
            priming_sugar_equiv: Set(full.priming_sugar_equiv),
            keg_priming_factor: Set(full.keg_priming_factor),
            mash_water_id: Set(full.mash_water_id.clone()),
            sparge_water_id: Set(full.sparge_water_id.clone()),
            branch_parent_id: Set(Some(version_id.to_string())),
            updated_at: Set(now),
            ..Default::default()
        }
        .update(self.db)
        .await?;

        Ok(())
    }

    pub async fn save_named(
        &self,
        recipe_id: &str,
        name: &str,
    ) -> Result<RecipeVersionSummary, AppError> {
        let recipe = RecipeRepository::new(self.db).get(recipe_id).await?;

        let recipe_row = recipes::Entity::find_by_id(recipe_id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let branch_parent_id = recipe_row.branch_parent_id.clone();

        let parent_id = if let Some(bp) = branch_parent_id {
            // Clear branch_parent_id now that we're consuming it
            recipes::ActiveModel {
                id: Set(recipe_id.to_string()),
                branch_parent_id: Set(None),
                ..Default::default()
            }
            .update(self.db)
            .await?;
            Some(bp)
        } else {
            recipe_versions::Entity::find()
                .filter(recipe_versions::Column::RecipeId.eq(recipe_id))
                .order_by_desc(recipe_versions::Column::CreatedAt)
                .one(self.db)
                .await?
                .map(|v| v.id)
        };

        let next_number = recipe_versions::Entity::find()
            .filter(recipe_versions::Column::RecipeId.eq(recipe_id))
            .all(self.db)
            .await?
            .len() as i32
            + 1;

        self.snapshot(
            recipe_id,
            &recipe,
            next_number,
            Some(name.to_string()),
            parent_id,
        )
        .await
    }

    pub async fn get_full(&self, version_id: &str) -> Result<Recipe, AppError> {
        let v = recipe_versions::Entity::find_by_id(version_id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let recipe_row = recipes::Entity::find_by_id(&v.recipe_id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let style = if let Some(sid) = &v.style_id {
            use crate::models::Style;
            styles::Entity::find_by_id(sid)
                .one(self.db)
                .await?
                .map(Style::try_from)
                .transpose()?
        } else {
            None
        };

        let equipment_profile = if let Some(eid) = &v.equipment_profile_id {
            use crate::models::EquipmentProfile;
            equipment_profiles::Entity::find_by_id(eid)
                .one(self.db)
                .await?
                .map(EquipmentProfile::try_from)
                .transpose()?
        } else {
            None
        };

        let fermentables = recipe_version_fermentables::Entity::find()
            .filter(recipe_version_fermentables::Column::RecipeVersionId.eq(version_id))
            .order_by_asc(recipe_version_fermentables::Column::AdditionOrder)
            .all(self.db)
            .await?
            .into_iter()
            .map(|m| RecipeAdditionFermentable {
                id: m.id,
                recipe_id: v.recipe_id.clone(),
                fermentable_id: m.fermentable_id,
                name: m.name,
                type_: m.r#type,
                yield_pct: m.yield_pct,
                color_lovibond: m.color_lovibond,
                amount_kg: m.amount_kg,
                add_after_boil: m.add_after_boil.unwrap_or(0) != 0,
                addition_order: m.addition_order as i64,
            })
            .collect();

        let hops = recipe_version_hops::Entity::find()
            .filter(recipe_version_hops::Column::RecipeVersionId.eq(version_id))
            .order_by_asc(recipe_version_hops::Column::AdditionOrder)
            .all(self.db)
            .await?
            .into_iter()
            .map(|m| RecipeAdditionHop {
                id: m.id,
                recipe_id: v.recipe_id.clone(),
                hop_id: m.hop_id,
                name: m.name,
                alpha_pct: m.alpha_pct,
                form: m.form,
                amount_kg: m.amount_kg,
                use_: m.r#use,
                time_min: m.time_min,
                addition_order: m.addition_order as i64,
                hopstand_temp_c: m.hopstand_temp_c,
            })
            .collect();

        let yeasts = recipe_version_yeasts::Entity::find()
            .filter(recipe_version_yeasts::Column::RecipeVersionId.eq(version_id))
            .all(self.db)
            .await?
            .into_iter()
            .map(|m| RecipeAdditionYeast {
                id: m.id,
                recipe_id: v.recipe_id.clone(),
                yeast_id: m.yeast_id,
                name: m.name,
                type_: m.r#type,
                form: m.form,
                laboratory: m.laboratory,
                product_id: m.product_id,
                attenuation_pct: m.attenuation_pct,
                amount: m.amount,
                amount_is_weight: m.amount_is_weight.unwrap_or(0) != 0,
                add_to_secondary: m.add_to_secondary.unwrap_or(0) != 0,
                times_cultured: m.times_cultured.unwrap_or(0) as i64,
            })
            .collect();

        let miscs = recipe_version_miscs::Entity::find()
            .filter(recipe_version_miscs::Column::RecipeVersionId.eq(version_id))
            .order_by_asc(recipe_version_miscs::Column::AdditionOrder)
            .all(self.db)
            .await?
            .into_iter()
            .map(|m| RecipeAdditionMisc {
                id: m.id,
                recipe_id: v.recipe_id.clone(),
                misc_id: m.misc_id,
                name: m.name,
                type_: m.r#type,
                use_: m.r#use,
                amount: m.amount,
                amount_is_weight: m.amount_is_weight.unwrap_or(0) != 0,
                unit: m.unit,
                time_min: m.time_min,
                addition_order: m.addition_order as i64,
            })
            .collect();

        let waters = recipe_version_waters::Entity::find()
            .filter(recipe_version_waters::Column::RecipeVersionId.eq(version_id))
            .all(self.db)
            .await?
            .into_iter()
            .map(|m| RecipeAdditionWater {
                id: m.id,
                recipe_id: v.recipe_id.clone(),
                water_id: m.water_id,
                name: m.name,
                amount_l: m.amount_l,
            })
            .collect();

        let water_adjustments = recipe_version_water_adjustments::Entity::find()
            .filter(recipe_version_water_adjustments::Column::RecipeVersionId.eq(version_id))
            .all(self.db)
            .await?
            .into_iter()
            .map(|m| -> Result<RecipeWaterAdjustment, AppError> {
                Ok(RecipeWaterAdjustment {
                    id: m.id,
                    recipe_id: v.recipe_id.clone(),
                    addition: m
                        .addition
                        .parse()
                        .map_err(|e| AppError::Internal(format!("{e}")))?,
                    target: m
                        .target
                        .parse()
                        .map_err(|e| AppError::Internal(format!("{e}")))?,
                    amount: m.amount,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mash = if let Some(vm) = recipe_version_mash::Entity::find()
            .filter(recipe_version_mash::Column::RecipeVersionId.eq(version_id))
            .one(self.db)
            .await?
        {
            let steps = recipe_version_mash_steps::Entity::find()
                .filter(recipe_version_mash_steps::Column::RecipeVersionMashId.eq(&vm.id))
                .order_by_asc(recipe_version_mash_steps::Column::StepOrder)
                .all(self.db)
                .await?
                .into_iter()
                .map(|s| MashStep {
                    id: s.id,
                    mash_id: vm.id.clone(),
                    name: s.name,
                    type_: s.r#type,
                    infuse_amount_l: s.infuse_amount_l,
                    step_temp_c: s.step_temp_c,
                    step_time_min: s.step_time_min as i64,
                    ramp_time_min: s.ramp_time_min.map(|v| v as i64),
                    end_temp_c: s.end_temp_c,
                    step_order: s.step_order as i64,
                })
                .collect();
            Some(Mash {
                id: vm.id,
                recipe_id: v.recipe_id.clone(),
                name: vm.name,
                grain_temp_c: vm.grain_temp_c,
                tun_temp_c: vm.tun_temp_c,
                sparge_temp_c: vm.sparge_temp_c,
                ph: vm.ph,
                notes: vm.notes,
                ratio_l_per_kg: vm.ratio_l_per_kg,
                tun_weight_kg: vm.tun_weight_kg,
                tun_specific_heat: vm.tun_specific_heat,
                equip_adjust: vm.equip_adjust.unwrap_or(0) != 0,
                steps,
            })
        } else {
            None
        };

        Ok(Recipe {
            id: v.recipe_id.clone(),
            name: recipe_row.name,
            type_: v.r#type,
            brewer: v.brewer,
            asst_brewer: v.asst_brewer,
            batch_size_l: v.batch_size_l,
            boil_size_l: v.boil_size_l,
            boil_time_min: v.boil_time_min,
            efficiency_pct: v.efficiency_pct,
            style_id: v.style_id,
            equipment_profile_id: v.equipment_profile_id,
            notes: v.notes,
            taste_notes: None,
            taste_rating: None,
            og: v.og,
            fg: v.fg,
            fermentation_stages: v.fermentation_stages.unwrap_or(1) as i64,
            primary_age_days: v.primary_age_days,
            primary_temp_c: v.primary_temp_c,
            secondary_age_days: v.secondary_age_days,
            secondary_temp_c: v.secondary_temp_c,
            tertiary_age_days: v.tertiary_age_days,
            tertiary_temp_c: v.tertiary_temp_c,
            age_days: v.age_days,
            age_temp_c: v.age_temp_c,
            carbonation_vols: v.carbonation_vols,
            forced_carbonation: v.forced_carbonation.unwrap_or(0) != 0,
            priming_sugar_name: v.priming_sugar_name,
            carbonation_temp_c: v.carbonation_temp_c,
            priming_sugar_equiv: v.priming_sugar_equiv,
            keg_priming_factor: v.keg_priming_factor,
            date: None,
            source: recipe_row
                .source
                .parse()
                .map_err(|e| AppError::Internal(format!("invalid source value: {e}")))?,
            mash_water_id: v.mash_water_id,
            sparge_water_id: v.sparge_water_id,
            hopstand_temp_c: v.hopstand_temp_c.unwrap_or(80.0),
            image_path: None,
            created_at: v.created_at as i64,
            updated_at: v.created_at as i64,
            style,
            equipment_profile,
            fermentables,
            hops,
            yeasts,
            miscs,
            waters,
            water_adjustments,
            mash,
        })
    }

    /// Returns the content hash for a version, recomputing (and persisting) it when
    /// missing or produced by an older projection.
    async fn version_hash(&self, v: &recipe_versions::Model) -> Result<String, AppError> {
        let current_prefix = format!("{}:", crate::recipe_hash::PROJECTION_VERSION);
        if let Some(h) = &v.content_hash {
            if h.starts_with(&current_prefix) {
                return Ok(h.clone());
            }
        }
        let snap = self.get_full(&v.id).await?;
        let h = crate::recipe_hash::recipe_content_hash(&snap)?;
        recipe_versions::ActiveModel {
            id: Set(v.id.clone()),
            content_hash: Set(Some(h.clone())),
            ..Default::default()
        }
        .update(self.db)
        .await?;
        Ok(h)
    }

    pub async fn status(
        &self,
        recipe_id: &str,
    ) -> Result<crate::models::RecipeVersionStatus, AppError> {
        let versions = recipe_versions::Entity::find()
            .filter(recipe_versions::Column::RecipeId.eq(recipe_id))
            .order_by_desc(recipe_versions::Column::VersionNumber)
            .all(self.db)
            .await?;

        let has_unversioned_changes = if let Some(latest) = versions.first() {
            let live = RecipeRepository::new(self.db).get(recipe_id).await?;
            let live_hash = crate::recipe_hash::recipe_content_hash(&live)?;
            live_hash != self.version_hash(latest).await?
        } else {
            false
        };

        Ok(crate::models::RecipeVersionStatus {
            version_count: versions.len() as i64,
            latest_version_id: versions.first().map(|v| v.id.clone()),
            has_unversioned_changes,
        })
    }

    pub async fn delete(&self, version_id: &str) -> Result<(), AppError> {
        // 1. Confirm version exists
        let version = recipe_versions::Entity::find_by_id(version_id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        // 2. Block if any batch references this version
        let batch_count = batches::Entity::find()
            .filter(batches::Column::RecipeVersionId.eq(version_id))
            .count(self.db)
            .await?;
        if batch_count > 0 {
            return Err(AppError::Conflict(
                "version is referenced by a batch".to_string(),
            ));
        }

        // 3. Re-parent child versions
        recipe_versions::Entity::update_many()
            .col_expr(
                recipe_versions::Column::ParentVersionId,
                sea_orm::sea_query::Expr::value(version.parent_version_id.clone()),
            )
            .filter(recipe_versions::Column::ParentVersionId.eq(version_id))
            .exec(self.db)
            .await?;

        // 4. Nullify recipes.branch_parent_id if it points here
        recipes::Entity::update_many()
            .col_expr(
                recipes::Column::BranchParentId,
                sea_orm::sea_query::Expr::value::<Option<String>>(None),
            )
            .filter(recipes::Column::BranchParentId.eq(version_id))
            .exec(self.db)
            .await?;

        // 5. Delete the version (cascades clean up all recipe_version_* tables)
        recipe_versions::Entity::delete_by_id(version_id)
            .exec(self.db)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::CreateFermentableAdditionInput;
    use crate::models::CreateRecipeInput;
    use crate::models::UpdateRecipeInput;
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

    #[tokio::test]
    async fn test_save_named_always_creates_new_version() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = RecipeVersionRepository::new(&db);

        // First brew creates v1
        let v1 = repo.create_or_reuse(&recipe_id).await.unwrap();

        // Manual save with same unchanged recipe still creates v2
        let v2 = repo.save_named(&recipe_id, "My checkpoint").await.unwrap();

        assert_ne!(v1.id, v2.id);
        assert_eq!(v2.version_number, 2);
        assert_eq!(v2.name.as_deref(), Some("My checkpoint"));
        assert_eq!(v2.parent_version_id.as_deref(), Some(v1.id.as_str()));
    }

    #[tokio::test]
    async fn test_create_or_reuse_respects_branch_parent() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = RecipeVersionRepository::new(&db);

        let v1 = repo.create_or_reuse(&recipe_id).await.unwrap();

        // Simulate branch_from_version having been called: set branch_parent_id = v1.id
        crate::entities::recipes::ActiveModel {
            id: Set(recipe_id.clone()),
            branch_parent_id: Set(Some(v1.id.clone())),
            ..Default::default()
        }
        .update(&db)
        .await
        .unwrap();

        // Add a fermentable so recipe is "changed" from v1 baseline
        FermentableRepository::new(&db)
            .create(
                &recipe_id,
                CreateFermentableAdditionInput {
                    fermentable_id: None,
                    name: "Crystal 60".into(),
                    type_: "grain".into(),
                    yield_pct: 70.0,
                    color_lovibond: 60.0,
                    amount_kg: 0.5,
                    add_after_boil: None,
                },
            )
            .await
            .unwrap();

        let v2 = repo.create_or_reuse(&recipe_id).await.unwrap();

        assert_ne!(v1.id, v2.id);
        assert_eq!(v2.parent_version_id.as_deref(), Some(v1.id.as_str()));

        // branch_parent_id should be cleared on the recipe
        let recipe_row = crate::entities::recipes::Entity::find_by_id(&recipe_id)
            .one(&db)
            .await
            .unwrap()
            .unwrap();
        assert!(recipe_row.branch_parent_id.is_none());
    }

    #[tokio::test]
    async fn test_get_full_returns_recipe_shaped_data() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;

        // Add a fermentable so there's something to round-trip
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

        let repo = RecipeVersionRepository::new(&db);
        let v = repo.create_or_reuse(&recipe_id).await.unwrap();
        let full = repo.get_full(&v.id).await.unwrap();

        assert_eq!(full.fermentables.len(), 1);
        assert_eq!(full.fermentables[0].name, "Pale Malt");
        assert_eq!(full.fermentables[0].amount_kg, 4.5);
    }

    #[tokio::test]
    async fn live_and_snapshot_hash_match_for_unchanged_recipe() {
        use crate::models::{CreateHopAdditionInput, UpdateRecipeInput};
        use crate::recipe_hash::recipe_content_hash;
        use crate::repositories::hop::HopRepository;
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;

        // Set a non-default recipe-level hopstand temp so the gap is exercised
        RecipeRepository::new(&db)
            .update(
                &recipe_id,
                UpdateRecipeInput {
                    hopstand_temp_c: Some(77.0),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        // Add a hop with a per-hop hopstand override so that field is also exercised
        HopRepository::new(&db)
            .create(
                &recipe_id,
                CreateHopAdditionInput {
                    hop_id: None,
                    name: "Citra".into(),
                    alpha_pct: 12.0,
                    amount_kg: 0.05,
                    form: None,
                    use_: "Hopstand".into(),
                    time_min: 0.0,
                    hopstand_temp_c: Some(75.5),
                },
            )
            .await
            .unwrap();

        let repo = RecipeVersionRepository::new(&db);
        let v = repo.save_named(&recipe_id, "v1").await.unwrap();
        let live = RecipeRepository::new(&db).get(&recipe_id).await.unwrap();
        let snap = repo.get_full(&v.id).await.unwrap();

        assert_eq!(
            recipe_content_hash(&live).unwrap(),
            recipe_content_hash(&snap).unwrap(),
            "an unmodified recipe must hash identically to its own snapshot"
        );
    }

    #[tokio::test]
    async fn status_zero_versions_is_not_dirty() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let st = RecipeVersionRepository::new(&db)
            .status(&recipe_id)
            .await
            .unwrap();
        assert_eq!(st.version_count, 0);
        assert!(st.latest_version_id.is_none());
        assert!(!st.has_unversioned_changes);
    }

    #[tokio::test]
    async fn status_clean_after_snapshot() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = RecipeVersionRepository::new(&db);
        repo.save_named(&recipe_id, "v1").await.unwrap();
        let st = repo.status(&recipe_id).await.unwrap();
        assert_eq!(st.version_count, 1);
        assert!(!st.has_unversioned_changes);
    }

    #[tokio::test]
    async fn status_dirty_after_edit() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;
        let repo = RecipeVersionRepository::new(&db);
        repo.save_named(&recipe_id, "v1").await.unwrap();

        // Edit a brew-identity field so the live recipe diverges from the snapshot.
        RecipeRepository::new(&db)
            .update(
                &recipe_id,
                UpdateRecipeInput {
                    batch_size_l: Some(19.0),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        let st = repo.status(&recipe_id).await.unwrap();
        assert!(
            st.has_unversioned_changes,
            "editing batch size must mark the recipe dirty"
        );
    }

    #[tokio::test]
    async fn test_branch_from_restores_version_data() {
        let db = setup_test_db().await;
        let recipe_id = make_recipe(&db).await;

        // Add a fermentable to create v1
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

        let repo = RecipeVersionRepository::new(&db);
        let v1 = repo.create_or_reuse(&recipe_id).await.unwrap();

        // Now change the live recipe (delete the fermentable, add different one)
        let fermentables = FermentableRepository::new(&db)
            .list(&recipe_id)
            .await
            .unwrap();
        FermentableRepository::new(&db)
            .delete(&fermentables[0].id)
            .await
            .unwrap();
        FermentableRepository::new(&db)
            .create(
                &recipe_id,
                CreateFermentableAdditionInput {
                    fermentable_id: None,
                    name: "Munich".into(),
                    type_: "grain".into(),
                    yield_pct: 80.0,
                    color_lovibond: 8.0,
                    amount_kg: 2.0,
                    add_after_boil: None,
                },
            )
            .await
            .unwrap();

        // Branch back to v1
        repo.branch_from(&recipe_id, &v1.id).await.unwrap();

        // Live recipe should now have Pale Malt again
        let live_fermentables = FermentableRepository::new(&db)
            .list(&recipe_id)
            .await
            .unwrap();
        assert_eq!(live_fermentables.len(), 1);
        assert_eq!(live_fermentables[0].name, "Pale Malt");

        // branch_parent_id should be set on the recipe
        let recipe_row = crate::entities::recipes::Entity::find_by_id(&recipe_id)
            .one(&db)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(recipe_row.branch_parent_id.as_deref(), Some(v1.id.as_str()));
    }
}
