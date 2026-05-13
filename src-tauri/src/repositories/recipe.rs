use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryOrder, Set};

use crate::entities::{recipes, styles};
use crate::error::AppError;
use crate::models::{
    CreateFermentableAdditionInput, CreateHopAdditionInput, CreateMiscAdditionInput,
    CreateRecipeInput, CreateWaterAdditionInput, CreateWaterAdjustmentInput,
    CreateYeastAdditionInput, Recipe, RecipeSummary, UpdateRecipeInput,
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
            self.copy_additions(&src_id, &id).await?;
        }

        self.get(&id).await
    }

    async fn copy_additions(&self, src_id: &str, dst_id: &str) -> Result<(), AppError> {
        // Additions are copied rather than referenced so that edits to the
        // source recipe don't affect the duplicate.

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
                        amount_kg: f.amount_kg,
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
                        amount_kg: h.amount_kg,
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
                        amount: y.amount,
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
                        amount: m.amount,
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
                        amount_l: w.amount_l,
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
                        amount: a.amount,
                    },
                )
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{CreateFermentableAdditionInput, CreateHopAdditionInput};
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
}
