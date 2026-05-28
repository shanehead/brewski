use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};

use crate::entities::{recipes, styles};
use crate::error::AppError;
use crate::models::{
    CreateFermentableAdditionInput, CreateHopAdditionInput, CreateMashStepInput,
    CreateMiscAdditionInput, CreateRecipeInput, CreateWaterAdditionInput,
    CreateWaterAdjustmentInput, CreateYeastAdditionInput, Recipe, RecipeSummary, UpdateMashInput,
    UpdateRecipeInput,
};

use super::equipment::EquipmentRepository;
use super::fermentable::FermentableRepository;
use super::hop::HopRepository;
use super::library::LibraryRepository;
use super::mash::MashRepository;
use super::misc::MiscRepository;
use super::water::WaterRepository;
use super::water_chemistry::WaterChemistryRepository;
use super::yeast::YeastRepository;
use super::{new_id, now_secs};

pub struct RecipeRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> RecipeRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list(&self) -> Result<Vec<RecipeSummary>, AppError> {
        let results = recipes::Entity::find()
            .filter(recipes::Column::Source.eq("user"))
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
                    batch_size_l: r.batch_size_l,
                    source: r
                        .source
                        .parse()
                        .map_err(|e| AppError::Internal(format!("invalid source value: {e}")))?,
                    created_at: r.created_at as i64,
                    updated_at: r.updated_at as i64,
                    image_path: r.image_path,
                })
            })
            .collect()
    }

    pub async fn list_baseline(&self) -> Result<Vec<RecipeSummary>, AppError> {
        let results = recipes::Entity::find()
            .filter(recipes::Column::Source.eq("seeded"))
            .find_also_related(styles::Entity)
            .order_by_asc(recipes::Column::Name)
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
                    batch_size_l: r.batch_size_l,
                    source: r
                        .source
                        .parse()
                        .map_err(|e| AppError::Internal(format!("invalid source value: {e}")))?,
                    created_at: r.created_at as i64,
                    updated_at: r.updated_at as i64,
                    image_path: r.image_path,
                })
            })
            .collect()
    }

    pub async fn get(&self, id: &str) -> Result<Recipe, AppError> {
        let recipe_row = recipes::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let fermentables = FermentableRepository::new(self.db).list(id).await?;
        let hops = HopRepository::new(self.db).list(id).await?;
        let yeasts = YeastRepository::new(self.db).list(id).await?;
        let miscs = MiscRepository::new(self.db).list(id).await?;
        let waters = WaterRepository::new(self.db).list(id).await?;
        let water_adjustments = WaterChemistryRepository::new(self.db)
            .list_adjustments(id)
            .await?;

        let mash = match MashRepository::new(self.db).get_for_recipe(id).await {
            Ok(mash) => Some(mash),
            Err(AppError::NotFound) => None,
            Err(e) => return Err(e),
        };

        let equipment_profile = if let Some(ep_id) = &recipe_row.equipment_profile_id {
            Some(EquipmentRepository::new(self.db).get(ep_id).await?)
        } else {
            None
        };

        let style = if let Some(style_id) = &recipe_row.style_id {
            Some(LibraryRepository::new(self.db).get_style(style_id).await?)
        } else {
            None
        };

        Ok(Recipe {
            id: recipe_row.id,
            name: recipe_row.name,
            type_: recipe_row.r#type,
            brewer: recipe_row.brewer,
            asst_brewer: recipe_row.asst_brewer,
            batch_size_l: recipe_row.batch_size_l,
            boil_size_l: recipe_row.boil_size_l,
            boil_time_min: recipe_row.boil_time_min,
            efficiency_pct: recipe_row.efficiency_pct,
            style_id: recipe_row.style_id,
            equipment_profile_id: recipe_row.equipment_profile_id,
            notes: recipe_row.notes,
            taste_notes: recipe_row.taste_notes,
            taste_rating: recipe_row.taste_rating,
            og: recipe_row.og,
            fg: recipe_row.fg,
            fermentation_stages: recipe_row.fermentation_stages.unwrap_or(1) as i64,
            primary_age_days: recipe_row.primary_age_days,
            primary_temp_c: recipe_row.primary_temp_c,
            secondary_age_days: recipe_row.secondary_age_days,
            secondary_temp_c: recipe_row.secondary_temp_c,
            tertiary_age_days: recipe_row.tertiary_age_days,
            tertiary_temp_c: recipe_row.tertiary_temp_c,
            age_days: recipe_row.age_days,
            age_temp_c: recipe_row.age_temp_c,
            carbonation_vols: recipe_row.carbonation_vols,
            forced_carbonation: recipe_row.forced_carbonation.unwrap_or(0) != 0,
            priming_sugar_name: recipe_row.priming_sugar_name,
            carbonation_temp_c: recipe_row.carbonation_temp_c,
            priming_sugar_equiv: recipe_row.priming_sugar_equiv,
            keg_priming_factor: recipe_row.keg_priming_factor,
            date: recipe_row.date,
            source: recipe_row
                .source
                .parse()
                .map_err(|e| AppError::Internal(format!("invalid source value: {e}")))?,
            created_at: recipe_row.created_at as i64,
            updated_at: recipe_row.updated_at as i64,
            equipment_profile,
            style,
            fermentables,
            hops,
            yeasts,
            miscs,
            waters,
            water_adjustments,
            mash_water_id: recipe_row.mash_water_id,
            sparge_water_id: recipe_row.sparge_water_id,
            hopstand_temp_c: recipe_row.hopstand_temp_c,
            image_path: recipe_row.image_path,
            mash,
        })
    }

    pub async fn create(&self, input: CreateRecipeInput) -> Result<Recipe, AppError> {
        let id = new_id();
        let now = now_secs() as i32;

        let (batch_size, boil_size, boil_time, ep_id, mash_water_id, sparge_water_id) =
            if let Some(ref src_id) = input.source_id {
                let src = self.get(src_id).await?;
                (
                    src.batch_size_l,
                    src.boil_size_l,
                    src.boil_time_min,
                    src.equipment_profile_id,
                    src.mash_water_id,
                    src.sparge_water_id,
                )
            } else {
                (
                    input.batch_size_l.unwrap_or(23.0),
                    input.boil_size_l.unwrap_or(27.0),
                    input.boil_time_min.unwrap_or(60.0),
                    input.equipment_profile_id,
                    None,
                    None,
                )
            };

        recipes::ActiveModel {
            id: Set(id.clone()),
            name: Set(input.name),
            r#type: Set(input.type_.unwrap_or_else(|| "all_grain".to_owned())),
            batch_size_l: Set(batch_size),
            boil_size_l: Set(boil_size),
            boil_time_min: Set(boil_time),
            equipment_profile_id: Set(ep_id),
            mash_water_id: Set(mash_water_id),
            sparge_water_id: Set(sparge_water_id),
            hopstand_temp_c: Set(input.hopstand_temp_c),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        }
        .insert(self.db)
        .await?;

        if let Some(src_id) = input.source_id {
            self.copy_additions(&src_id, &id, 1.0).await?;
        }

        self.get(&id).await
    }

    /// Copy every addition from `src_id` onto `dst_id`, multiplying each amount
    /// by `ratio`. Use `ratio = 1.0` for a plain duplicate; pass a scaling
    /// factor to resize the recipe. Additions are copied rather than referenced
    /// so that edits to the source recipe don't affect the destination.
    async fn copy_additions(&self, src_id: &str, dst_id: &str, ratio: f64) -> Result<(), AppError> {
        let fermentable_repo = FermentableRepository::new(self.db);
        for f in fermentable_repo.list(src_id).await? {
            fermentable_repo
                .create(
                    dst_id,
                    CreateFermentableAdditionInput {
                        fermentable_id: f.fermentable_id,
                        name: f.name,
                        type_: f.type_,
                        yield_pct: f.yield_pct,
                        color_lovibond: f.color_lovibond,
                        amount_kg: f.amount_kg * ratio,
                        add_after_boil: Some(f.add_after_boil),
                    },
                )
                .await?;
        }

        let hop_repo = HopRepository::new(self.db);
        for h in hop_repo.list(src_id).await? {
            hop_repo
                .create(
                    dst_id,
                    CreateHopAdditionInput {
                        hop_id: h.hop_id,
                        name: h.name,
                        alpha_pct: h.alpha_pct,
                        form: Some(h.form),
                        amount_kg: h.amount_kg * ratio,
                        use_: h.use_,
                        time_min: h.time_min,
                        hopstand_temp_c: h.hopstand_temp_c,
                    },
                )
                .await?;
        }

        let yeast_repo = YeastRepository::new(self.db);
        for y in yeast_repo.list(src_id).await? {
            yeast_repo
                .create(
                    dst_id,
                    CreateYeastAdditionInput {
                        yeast_id: y.yeast_id,
                        name: y.name,
                        type_: y.type_,
                        form: y.form,
                        laboratory: y.laboratory,
                        product_id: y.product_id,
                        attenuation_pct: y.attenuation_pct,
                        amount: y.amount.map(|a| a * ratio),
                        amount_is_weight: Some(y.amount_is_weight),
                        add_to_secondary: Some(y.add_to_secondary),
                        times_cultured: Some(y.times_cultured),
                    },
                )
                .await?;
        }

        let misc_repo = MiscRepository::new(self.db);
        for m in misc_repo.list(src_id).await? {
            misc_repo
                .create(
                    dst_id,
                    CreateMiscAdditionInput {
                        misc_id: m.misc_id,
                        name: m.name,
                        type_: m.type_,
                        use_: m.use_,
                        amount: m.amount * ratio,
                        amount_is_weight: Some(m.amount_is_weight),
                        time_min: m.time_min,
                    },
                )
                .await?;
        }

        let water_repo = WaterRepository::new(self.db);
        for w in water_repo.list(src_id).await? {
            water_repo
                .create(
                    dst_id,
                    CreateWaterAdditionInput {
                        water_id: w.water_id,
                        name: w.name,
                        amount_l: w.amount_l * ratio,
                    },
                )
                .await?;
        }

        let water_chem_repo = WaterChemistryRepository::new(self.db);
        for a in water_chem_repo.list_adjustments(src_id).await? {
            water_chem_repo
                .create_water_adjustment(
                    dst_id,
                    CreateWaterAdjustmentInput {
                        addition: a
                            .addition
                            .to_string()
                            .parse()
                            .map_err(|e| AppError::Internal(format!("{}", e)))?,
                        target: a
                            .target
                            .to_string()
                            .parse()
                            .map_err(|e| AppError::Internal(format!("{}", e)))?,
                        amount: a.amount * ratio,
                    },
                )
                .await?;
        }

        Ok(())
    }

    pub async fn scale(&self, recipe_id: &str, new_batch_size_l: f64) -> Result<Recipe, AppError> {
        let src = self.get(recipe_id).await?;
        if new_batch_size_l <= 0.0 {
            return Err(AppError::Internal(
                "target batch size must be positive".into(),
            ));
        }
        if src.batch_size_l == 0.0 {
            return Err(AppError::Internal("source batch_size_l is zero".into()));
        }
        let ratio = new_batch_size_l / src.batch_size_l;
        let id = new_id();
        let now = now_secs() as i32;

        recipes::ActiveModel {
            id: Set(id.clone()),
            name: Set(format!("{} (scaled)", src.name)),
            r#type: Set(src.type_),
            brewer: Set(src.brewer),
            asst_brewer: Set(src.asst_brewer),
            batch_size_l: Set(new_batch_size_l),
            boil_size_l: Set(src.boil_size_l * ratio),
            boil_time_min: Set(src.boil_time_min),
            efficiency_pct: Set(src.efficiency_pct),
            style_id: Set(src.style_id),
            equipment_profile_id: Set(src.equipment_profile_id),
            notes: Set(src.notes),
            taste_notes: Set(src.taste_notes),
            taste_rating: Set(src.taste_rating),
            fermentation_stages: Set(Some(src.fermentation_stages as i32)),
            primary_age_days: Set(src.primary_age_days),
            primary_temp_c: Set(src.primary_temp_c),
            secondary_age_days: Set(src.secondary_age_days),
            secondary_temp_c: Set(src.secondary_temp_c),
            tertiary_age_days: Set(src.tertiary_age_days),
            tertiary_temp_c: Set(src.tertiary_temp_c),
            age_days: Set(src.age_days),
            age_temp_c: Set(src.age_temp_c),
            carbonation_vols: Set(src.carbonation_vols),
            forced_carbonation: Set(Some(if src.forced_carbonation { 1 } else { 0 })),
            priming_sugar_name: Set(src.priming_sugar_name),
            carbonation_temp_c: Set(src.carbonation_temp_c),
            priming_sugar_equiv: Set(src.priming_sugar_equiv),
            keg_priming_factor: Set(src.keg_priming_factor),
            date: Set(src.date),
            hopstand_temp_c: Set(src.hopstand_temp_c),
            mash_water_id: Set(src.mash_water_id),
            sparge_water_id: Set(src.sparge_water_id),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        }
        .insert(self.db)
        .await?;

        self.copy_additions(recipe_id, &id, ratio).await?;

        if let Some(mash) = src.mash {
            let mash_repo = MashRepository::new(self.db);
            let new_mash = mash_repo
                .upsert_for_recipe(
                    &id,
                    UpdateMashInput {
                        name: Some(mash.name),
                        grain_temp_c: Some(mash.grain_temp_c),
                        tun_temp_c: mash.tun_temp_c,
                        sparge_temp_c: mash.sparge_temp_c,
                        ph: mash.ph,
                        ratio_l_per_kg: mash.ratio_l_per_kg,
                        notes: mash.notes,
                    },
                )
                .await?;
            for step in mash.steps {
                mash_repo
                    .create_step(
                        &new_mash.id,
                        CreateMashStepInput {
                            name: step.name,
                            type_: Some(step.type_),
                            step_temp_c: step.step_temp_c,
                            step_time_min: step.step_time_min,
                            infuse_amount_l: step.infuse_amount_l.map(|v| v * ratio),
                            ramp_time_min: step.ramp_time_min,
                            end_temp_c: step.end_temp_c,
                        },
                    )
                    .await?;
            }
        }

        self.get(&id).await
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
            active.batch_size_l = Set(v);
        }
        if let Some(v) = input.boil_size_l {
            active.boil_size_l = Set(v);
        }
        if let Some(v) = input.boil_time_min {
            active.boil_time_min = Set(v);
        }
        if let Some(v) = input.efficiency_pct {
            active.efficiency_pct = Set(Some(v));
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
            active.taste_rating = Set(Some(v));
        }
        if let Some(v) = input.fermentation_stages {
            active.fermentation_stages = Set(Some(v as i32));
        }
        if let Some(v) = input.primary_age_days {
            active.primary_age_days = Set(Some(v));
        }
        if let Some(v) = input.primary_temp_c {
            active.primary_temp_c = Set(Some(v));
        }
        if let Some(v) = input.secondary_age_days {
            active.secondary_age_days = Set(Some(v));
        }
        if let Some(v) = input.secondary_temp_c {
            active.secondary_temp_c = Set(Some(v));
        }
        if let Some(v) = input.tertiary_age_days {
            active.tertiary_age_days = Set(Some(v));
        }
        if let Some(v) = input.tertiary_temp_c {
            active.tertiary_temp_c = Set(Some(v));
        }
        if let Some(v) = input.age_days {
            active.age_days = Set(Some(v));
        }
        if let Some(v) = input.age_temp_c {
            active.age_temp_c = Set(Some(v));
        }
        if let Some(v) = input.carbonation_vols {
            active.carbonation_vols = Set(Some(v));
        }
        if let Some(v) = input.forced_carbonation {
            active.forced_carbonation = Set(Some(if v { 1 } else { 0 }));
        }
        if let Some(v) = input.priming_sugar_name {
            active.priming_sugar_name = Set(Some(v));
        }
        if let Some(v) = input.carbonation_temp_c {
            active.carbonation_temp_c = Set(Some(v));
        }
        if let Some(v) = input.priming_sugar_equiv {
            active.priming_sugar_equiv = Set(Some(v));
        }
        if let Some(v) = input.keg_priming_factor {
            active.keg_priming_factor = Set(Some(v));
        }
        if let Some(v) = input.date {
            active.date = Set(Some(v));
        }
        if let Some(v) = input.hopstand_temp_c {
            active.hopstand_temp_c = Set(Some(v));
        }

        active.updated_at = Set(now_secs() as i32);
        active.update(self.db).await?;

        self.get(id).await
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        recipes::Entity::delete_by_id(id).exec(self.db).await?;
        Ok(())
    }

    pub async fn set_image_path(
        &self,
        id: &str,
        filename: Option<&str>,
    ) -> Result<Recipe, AppError> {
        use sea_orm::ActiveValue::Set;
        let model = recipes::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;
        let mut active: recipes::ActiveModel = model.into();
        active.image_path = Set(filename.map(|s| s.to_owned()));
        active.update(self.db).await?;
        self.get(id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::recipes;
    use crate::models::{
        CreateFermentableAdditionInput, CreateHopAdditionInput, RecipeSummarySource,
    };
    use crate::repositories::fermentable::FermentableRepository;
    use crate::repositories::hop::HopRepository;
    use crate::test_helpers::setup_test_db;

    fn basic_input() -> CreateRecipeInput {
        CreateRecipeInput {
            name: "Test Recipe".into(),
            type_: Some("all_grain".into()),
            batch_size_l: Some(23.0),
            boil_size_l: Some(27.0),
            boil_time_min: Some(60.0),
            ..Default::default()
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
    async fn test_get_not_found() {
        let db = setup_test_db().await;
        let result = RecipeRepository::new(&db).get("nonexistent").await;
        assert!(matches!(result, Err(crate::error::AppError::NotFound)));
    }

    #[tokio::test]
    async fn test_update_not_found() {
        let db = setup_test_db().await;
        let result = RecipeRepository::new(&db)
            .update("nonexistent", UpdateRecipeInput::default())
            .await;
        assert!(matches!(result, Err(crate::error::AppError::NotFound)));
    }

    #[tokio::test]
    async fn test_update_many_fields() {
        let db = setup_test_db().await;
        let repo = RecipeRepository::new(&db);
        let created = repo.create(basic_input()).await.unwrap();

        let updated = repo
            .update(
                &created.id,
                UpdateRecipeInput {
                    type_: Some("extract".into()),
                    brewer: Some("Shane".into()),
                    asst_brewer: Some("Bob".into()),
                    batch_size_l: Some(19.0),
                    boil_size_l: Some(25.0),
                    boil_time_min: Some(90.0),
                    efficiency_pct: Some(70.0),
                    notes: Some("test notes".into()),
                    taste_notes: Some("tasty".into()),
                    taste_rating: Some(8.5),
                    fermentation_stages: Some(2),
                    primary_age_days: Some(7.0),
                    primary_temp_c: Some(18.0),
                    secondary_age_days: Some(14.0),
                    secondary_temp_c: Some(16.0),
                    tertiary_age_days: Some(7.0),
                    tertiary_temp_c: Some(15.0),
                    age_days: Some(30.0),
                    age_temp_c: Some(12.0),
                    carbonation_vols: Some(2.5),
                    forced_carbonation: Some(true),
                    priming_sugar_name: Some("Corn Sugar".into()),
                    carbonation_temp_c: Some(20.0),
                    priming_sugar_equiv: Some(1.0),
                    keg_priming_factor: Some(0.5),
                    date: Some("2026-05-05".into()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        assert_eq!(updated.type_, "extract");
        assert_eq!(updated.brewer, Some("Shane".into()));
        assert_eq!(updated.asst_brewer, Some("Bob".into()));
        assert_eq!(updated.batch_size_l, 19.0);
        assert_eq!(updated.efficiency_pct, Some(70.0));
        assert_eq!(updated.fermentation_stages, 2);
        assert_eq!(updated.forced_carbonation, true);
        assert_eq!(updated.priming_sugar_name, Some("Corn Sugar".into()));
        assert_eq!(updated.priming_sugar_equiv, Some(1.0));
        assert_eq!(updated.keg_priming_factor, Some(0.5));
        assert_eq!(updated.date, Some("2026-05-05".into()));
    }

    #[tokio::test]
    async fn test_duplicate_via_source_id() {
        let db = setup_test_db().await;
        let repo = RecipeRepository::new(&db);
        let original = repo.create(basic_input()).await.unwrap();

        FermentableRepository::new(&db)
            .create(
                &original.id,
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

        HopRepository::new(&db)
            .create(
                &original.id,
                CreateHopAdditionInput {
                    hop_id: None,
                    name: "Cascade".into(),
                    alpha_pct: 5.5,
                    form: None,
                    amount_kg: 0.05,
                    use_: "Boil".into(),
                    time_min: 60.0,
                    hopstand_temp_c: None,
                },
            )
            .await
            .unwrap();

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
        assert_eq!(dupe.fermentables.len(), 1);
        assert_eq!(dupe.fermentables[0].name, "Pale Malt");
        assert_eq!(dupe.hops.len(), 1);
        assert_eq!(dupe.hops[0].name, "Cascade");
    }

    #[tokio::test]
    async fn test_list_excludes_seeded_recipes() {
        let db = setup_test_db().await;
        let repo = RecipeRepository::new(&db);

        // Create a user recipe (default source = 'user')
        repo.create(basic_input()).await.unwrap();

        // Directly insert a seeded recipe
        recipes::ActiveModel {
            id: Set("bm-test-seeded".to_string()),
            name: Set("Seeded Recipe".to_string()),
            r#type: Set("All Grain".to_string()),
            batch_size_l: Set(19.0),
            boil_size_l: Set(23.0),
            boil_time_min: Set(60.0),
            source: Set("seeded".to_string()),
            created_at: Set(0),
            updated_at: Set(0),
            ..Default::default()
        }
        .insert(&db)
        .await
        .unwrap();

        let list = repo.list().await.unwrap();
        assert!(list.iter().all(|r| r.source == RecipeSummarySource::User));
        assert_eq!(list.len(), 1);
    }

    #[tokio::test]
    async fn test_list_baseline_returns_only_seeded() {
        let db = setup_test_db().await;
        let repo = RecipeRepository::new(&db);

        repo.create(basic_input()).await.unwrap();

        recipes::ActiveModel {
            id: Set("bm-test-seeded".to_string()),
            name: Set("Seeded Recipe".to_string()),
            r#type: Set("All Grain".to_string()),
            batch_size_l: Set(19.0),
            boil_size_l: Set(23.0),
            boil_time_min: Set(60.0),
            source: Set("seeded".to_string()),
            created_at: Set(0),
            updated_at: Set(0),
            ..Default::default()
        }
        .insert(&db)
        .await
        .unwrap();

        let baselines = repo.list_baseline().await.unwrap();
        // All returned recipes must have source = 'seeded'
        assert!(baselines
            .iter()
            .all(|r| r.source == RecipeSummarySource::Seeded));
        // Our explicitly inserted seeded recipe must be present
        assert!(baselines.iter().any(|r| r.name == "Seeded Recipe"));
        // The user recipe must not appear
        assert!(!baselines.iter().any(|r| r.name == "Test Recipe"));
    }

    #[tokio::test]
    async fn test_scale_creates_new_recipe() {
        let db = setup_test_db().await;
        let repo = RecipeRepository::new(&db);
        let original = repo
            .create(CreateRecipeInput {
                name: "My IPA".into(),
                batch_size_l: Some(23.0),
                boil_size_l: Some(27.0),
                ..Default::default()
            })
            .await
            .unwrap();

        let scaled = repo.scale(&original.id, 46.0).await.unwrap();

        assert_ne!(scaled.id, original.id);
        assert_eq!(scaled.name, "My IPA (scaled)");
        assert_eq!(scaled.batch_size_l, 46.0);
        assert!((scaled.boil_size_l - 54.0).abs() < 0.001);
        // original unchanged
        let still_original = repo.get(&original.id).await.unwrap();
        assert_eq!(still_original.batch_size_l, 23.0);
    }

    #[tokio::test]
    async fn test_scale_ingredients() {
        let db = setup_test_db().await;
        let repo = RecipeRepository::new(&db);
        let original = repo
            .create(CreateRecipeInput {
                name: "My IPA".into(),
                batch_size_l: Some(23.0),
                boil_size_l: Some(27.0),
                ..Default::default()
            })
            .await
            .unwrap();

        FermentableRepository::new(&db)
            .create(
                &original.id,
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

        HopRepository::new(&db)
            .create(
                &original.id,
                CreateHopAdditionInput {
                    hop_id: None,
                    name: "Cascade".into(),
                    alpha_pct: 5.5,
                    form: None,
                    amount_kg: 0.05,
                    use_: "Boil".into(),
                    time_min: 60.0,
                    hopstand_temp_c: None,
                },
            )
            .await
            .unwrap();

        let scaled = repo.scale(&original.id, 46.0).await.unwrap();

        assert_eq!(scaled.fermentables.len(), 1);
        assert!((scaled.fermentables[0].amount_kg - 9.0).abs() < 0.001);
        assert_eq!(scaled.hops.len(), 1);
        assert!((scaled.hops[0].amount_kg - 0.1).abs() < 0.0001);
    }

    #[tokio::test]
    async fn test_scale_mash_steps() {
        use crate::models::{CreateMashStepInput, UpdateMashInput};
        use crate::repositories::mash::MashRepository;

        let db = setup_test_db().await;
        let repo = RecipeRepository::new(&db);
        let original = repo
            .create(CreateRecipeInput {
                name: "My IPA".into(),
                batch_size_l: Some(23.0),
                boil_size_l: Some(27.0),
                ..Default::default()
            })
            .await
            .unwrap();

        let mash_repo = MashRepository::new(&db);
        let mash = mash_repo
            .upsert_for_recipe(
                &original.id,
                UpdateMashInput {
                    name: Some("Single Infusion".into()),
                    grain_temp_c: Some(20.0),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        mash_repo
            .create_step(
                &mash.id,
                CreateMashStepInput {
                    name: "Mash".into(),
                    type_: Some("Infusion".into()),
                    step_temp_c: 67.0,
                    step_time_min: 60,
                    infuse_amount_l: Some(15.0),
                    ramp_time_min: None,
                    end_temp_c: None,
                },
            )
            .await
            .unwrap();

        let scaled = repo.scale(&original.id, 46.0).await.unwrap();

        let scaled_mash = scaled.mash.expect("scaled recipe should have mash");
        assert_eq!(scaled_mash.steps.len(), 1);
        let infuse = scaled_mash.steps[0]
            .infuse_amount_l
            .expect("infuse should be set");
        assert!((infuse - 30.0).abs() < 0.001);
    }

    #[tokio::test]
    async fn test_scale_rejects_zero_batch_size() {
        let db = setup_test_db().await;
        let repo = RecipeRepository::new(&db);
        let original = repo
            .create(CreateRecipeInput {
                name: "My IPA".into(),
                batch_size_l: Some(23.0),
                ..Default::default()
            })
            .await
            .unwrap();

        let result = repo.scale(&original.id, 0.0).await;
        assert!(result.is_err());

        let result2 = repo.scale(&original.id, -5.0).await;
        assert!(result2.is_err());
    }

    async fn seed_one_of_each(db: &sea_orm::DatabaseConnection, recipe_id: &str) {
        use crate::models::{
            CreateMiscAdditionInput, CreateWaterAdditionInput, CreateWaterAdjustmentInput,
            CreateWaterAdjustmentInputAddition, CreateWaterAdjustmentInputTarget,
            CreateYeastAdditionInput,
        };
        use crate::repositories::misc::MiscRepository;
        use crate::repositories::water::WaterRepository;
        use crate::repositories::water_chemistry::WaterChemistryRepository;
        use crate::repositories::yeast::YeastRepository;

        FermentableRepository::new(db)
            .create(
                recipe_id,
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
        HopRepository::new(db)
            .create(
                recipe_id,
                CreateHopAdditionInput {
                    hop_id: None,
                    name: "Cascade".into(),
                    alpha_pct: 5.5,
                    form: None,
                    amount_kg: 0.05,
                    use_: "Boil".into(),
                    time_min: 60.0,
                    hopstand_temp_c: None,
                },
            )
            .await
            .unwrap();
        YeastRepository::new(db)
            .create(
                recipe_id,
                CreateYeastAdditionInput {
                    yeast_id: None,
                    name: "US-05".into(),
                    type_: "ale".into(),
                    form: "dry".into(),
                    laboratory: None,
                    product_id: None,
                    attenuation_pct: Some(77.0),
                    amount: Some(1.0),
                    amount_is_weight: Some(true),
                    add_to_secondary: None,
                    times_cultured: None,
                },
            )
            .await
            .unwrap();
        MiscRepository::new(db)
            .create(
                recipe_id,
                CreateMiscAdditionInput {
                    misc_id: None,
                    name: "Irish Moss".into(),
                    type_: "fining".into(),
                    use_: "boil".into(),
                    amount: 0.005,
                    amount_is_weight: Some(true),
                    time_min: 15.0,
                },
            )
            .await
            .unwrap();
        WaterRepository::new(db)
            .create(
                recipe_id,
                CreateWaterAdditionInput {
                    water_id: None,
                    name: "Mash Water".into(),
                    amount_l: 10.0,
                },
            )
            .await
            .unwrap();
        WaterChemistryRepository::new(db)
            .create_water_adjustment(
                recipe_id,
                CreateWaterAdjustmentInput {
                    addition: CreateWaterAdjustmentInputAddition::Gypsum,
                    target: CreateWaterAdjustmentInputTarget::Mash,
                    amount: 5.0,
                },
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_scale_scales_all_addition_types() {
        let db = setup_test_db().await;
        let repo = RecipeRepository::new(&db);
        let original = repo
            .create(CreateRecipeInput {
                name: "Full Recipe".into(),
                batch_size_l: Some(23.0),
                ..Default::default()
            })
            .await
            .unwrap();
        seed_one_of_each(&db, &original.id).await;

        let scaled = repo.scale(&original.id, 46.0).await.unwrap(); // ratio 2.0

        assert!((scaled.fermentables[0].amount_kg - 9.0).abs() < 1e-9);
        assert!((scaled.hops[0].amount_kg - 0.1).abs() < 1e-9);
        assert!((scaled.yeasts[0].amount.unwrap() - 2.0).abs() < 1e-9);
        assert!((scaled.miscs[0].amount - 0.01).abs() < 1e-9);
        assert!((scaled.waters[0].amount_l - 20.0).abs() < 1e-9);
        assert!((scaled.water_adjustments[0].amount - 10.0).abs() < 1e-9);
        assert_eq!(scaled.water_adjustments[0].addition.to_string(), "gypsum");
        assert_eq!(scaled.water_adjustments[0].target.to_string(), "mash");
    }

    #[tokio::test]
    async fn test_duplicate_copies_all_addition_types_unchanged() {
        let db = setup_test_db().await;
        let repo = RecipeRepository::new(&db);
        let original = repo
            .create(CreateRecipeInput {
                name: "Full Recipe".into(),
                batch_size_l: Some(23.0),
                ..Default::default()
            })
            .await
            .unwrap();
        seed_one_of_each(&db, &original.id).await;

        let dupe = repo
            .create(CreateRecipeInput {
                name: "Copy".into(),
                source_id: Some(original.id.clone()),
                ..Default::default()
            })
            .await
            .unwrap();

        assert_eq!(dupe.fermentables.len(), 1);
        assert!((dupe.fermentables[0].amount_kg - 4.5).abs() < 1e-9);
        assert_eq!(dupe.hops.len(), 1);
        assert!((dupe.hops[0].amount_kg - 0.05).abs() < 1e-9);
        assert_eq!(dupe.yeasts.len(), 1);
        assert!((dupe.yeasts[0].amount.unwrap() - 1.0).abs() < 1e-9);
        assert_eq!(dupe.miscs.len(), 1);
        assert!((dupe.miscs[0].amount - 0.005).abs() < 1e-9);
        assert_eq!(dupe.waters.len(), 1);
        assert!((dupe.waters[0].amount_l - 10.0).abs() < 1e-9);
        assert_eq!(dupe.water_adjustments.len(), 1);
        assert!((dupe.water_adjustments[0].amount - 5.0).abs() < 1e-9);
        assert_eq!(dupe.water_adjustments[0].addition.to_string(), "gypsum");
        assert_eq!(dupe.water_adjustments[0].target.to_string(), "mash");
    }
}
