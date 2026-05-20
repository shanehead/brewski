use crate::entities::{fermentables, hops, miscs, waters, yeasts};
use crate::error::AppError;
use crate::models::{
    CreateFermentableInput, CreateHopInput, CreateMiscInput, CreateWaterInput, CreateYeastInput,
    Fermentable, Hop, Misc, UpdateFermentableInput, UpdateHopInput, UpdateMiscInput,
    UpdateWaterInput, UpdateYeastInput, Water, Yeast,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use super::new_id;

pub struct IngredientRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> IngredientRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_hop(&self, input: CreateHopInput) -> Result<Hop, AppError> {
        let id = new_id();
        hops::ActiveModel {
            id: Set(id.clone()),
            name: Set(input.name),
            alpha_pct: Set(input.alpha_pct),
            beta_pct: Set(input.beta_pct),
            form: Set(input.form),
            r#type: Set(input.type_),
            origin: Set(input.origin),
            year: Set(input.year),
            notes: Set(input.notes),
            substitutes: Set(input.substitutes),
            hsi_pct: Set(input.hsi_pct),
            humulene_pct: Set(input.humulene_pct),
            caryophyllene_pct: Set(input.caryophyllene_pct),
            cohumulone_pct: Set(input.cohumulone_pct),
            myrcene_pct: Set(input.myrcene_pct),
            source: Set("user".to_string()),
            forked_from_id: Set(input.forked_from_id),
        }
        .insert(self.db)
        .await?;

        hops::Entity::find_by_id(&id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(Hop::try_from)
    }

    pub async fn update_hop(&self, id: &str, input: UpdateHopInput) -> Result<Hop, AppError> {
        let row = hops::Entity::find_by_id(id)
            .filter(hops::Column::Source.eq("user"))
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut active: hops::ActiveModel = row.into();
        if let Some(v) = input.name {
            active.name = Set(v);
        }
        if let Some(v) = input.alpha_pct {
            active.alpha_pct = Set(v);
        }
        if let Some(v) = input.beta_pct {
            active.beta_pct = Set(Some(v));
        }
        if let Some(v) = input.form {
            active.form = Set(v);
        }
        if let Some(v) = input.type_ {
            active.r#type = Set(Some(v));
        }
        if let Some(v) = input.origin {
            active.origin = Set(Some(v));
        }
        if let Some(v) = input.year {
            active.year = Set(Some(v));
        }
        if let Some(v) = input.notes {
            active.notes = Set(Some(v));
        }
        if let Some(v) = input.substitutes {
            active.substitutes = Set(Some(v));
        }
        if let Some(v) = input.hsi_pct {
            active.hsi_pct = Set(Some(v));
        }
        if let Some(v) = input.humulene_pct {
            active.humulene_pct = Set(Some(v));
        }
        if let Some(v) = input.caryophyllene_pct {
            active.caryophyllene_pct = Set(Some(v));
        }
        if let Some(v) = input.cohumulone_pct {
            active.cohumulone_pct = Set(Some(v));
        }
        if let Some(v) = input.myrcene_pct {
            active.myrcene_pct = Set(Some(v));
        }
        active.update(self.db).await?;

        hops::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(Hop::try_from)
    }

    pub async fn delete_hop(&self, id: &str) -> Result<(), AppError> {
        let result = hops::Entity::delete_many()
            .filter(hops::Column::Id.eq(id))
            .filter(hops::Column::Source.eq("user"))
            .exec(self.db)
            .await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound);
        }
        Ok(())
    }

    pub async fn create_fermentable(
        &self,
        input: CreateFermentableInput,
    ) -> Result<Fermentable, AppError> {
        let id = new_id();
        fermentables::ActiveModel {
            id: Set(id.clone()),
            name: Set(input.name),
            r#type: Set(input.type_),
            yield_pct: Set(input.yield_pct),
            color_lovibond: Set(input.color_lovibond),
            origin: Set(input.origin),
            supplier: Set(input.supplier),
            notes: Set(input.notes),
            add_after_boil: Set(Some(input.add_after_boil as i32)),
            coarse_fine_diff_pct: Set(input.coarse_fine_diff_pct),
            moisture_pct: Set(input.moisture_pct),
            diastatic_power_lintner: Set(input.diastatic_power_lintner),
            protein_pct: Set(input.protein_pct),
            max_in_batch_pct: Set(input.max_in_batch_pct),
            recommend_mash: Set(input.recommend_mash.map(|b| b as i32)),
            ibu_gal_per_lb: Set(input.ibu_gal_per_lb),
            source: Set("user".to_string()),
            forked_from_id: Set(input.forked_from_id),
        }
        .insert(self.db)
        .await?;

        fermentables::Entity::find_by_id(&id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(Fermentable::try_from)
    }

    pub async fn update_fermentable(
        &self,
        id: &str,
        input: UpdateFermentableInput,
    ) -> Result<Fermentable, AppError> {
        let row = fermentables::Entity::find_by_id(id)
            .filter(fermentables::Column::Source.eq("user"))
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut active: fermentables::ActiveModel = row.into();
        if let Some(v) = input.name {
            active.name = Set(v);
        }
        if let Some(v) = input.type_ {
            active.r#type = Set(v);
        }
        if let Some(v) = input.yield_pct {
            active.yield_pct = Set(v);
        }
        if let Some(v) = input.color_lovibond {
            active.color_lovibond = Set(v);
        }
        if let Some(v) = input.origin {
            active.origin = Set(Some(v));
        }
        if let Some(v) = input.supplier {
            active.supplier = Set(Some(v));
        }
        if let Some(v) = input.notes {
            active.notes = Set(Some(v));
        }
        if let Some(v) = input.add_after_boil {
            active.add_after_boil = Set(Some(v as i32));
        }
        if let Some(v) = input.coarse_fine_diff_pct {
            active.coarse_fine_diff_pct = Set(Some(v));
        }
        if let Some(v) = input.moisture_pct {
            active.moisture_pct = Set(Some(v));
        }
        if let Some(v) = input.diastatic_power_lintner {
            active.diastatic_power_lintner = Set(Some(v));
        }
        if let Some(v) = input.protein_pct {
            active.protein_pct = Set(Some(v));
        }
        if let Some(v) = input.max_in_batch_pct {
            active.max_in_batch_pct = Set(Some(v));
        }
        if let Some(v) = input.recommend_mash {
            active.recommend_mash = Set(Some(v as i32));
        }
        if let Some(v) = input.ibu_gal_per_lb {
            active.ibu_gal_per_lb = Set(Some(v));
        }
        active.update(self.db).await?;

        fermentables::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(Fermentable::try_from)
    }

    pub async fn delete_fermentable(&self, id: &str) -> Result<(), AppError> {
        let result = fermentables::Entity::delete_many()
            .filter(fermentables::Column::Id.eq(id))
            .filter(fermentables::Column::Source.eq("user"))
            .exec(self.db)
            .await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound);
        }
        Ok(())
    }

    pub async fn create_yeast(&self, input: CreateYeastInput) -> Result<Yeast, AppError> {
        let id = new_id();
        yeasts::ActiveModel {
            id: Set(id.clone()),
            name: Set(input.name),
            r#type: Set(input.type_),
            form: Set(input.form),
            laboratory: Set(input.laboratory),
            product_id: Set(input.product_id),
            min_temperature_c: Set(input.min_temperature_c),
            max_temperature_c: Set(input.max_temperature_c),
            flocculation: Set(input.flocculation),
            attenuation_pct: Set(input.attenuation_pct),
            min_attenuation_pct: Set(input.min_attenuation_pct),
            max_attenuation_pct: Set(input.max_attenuation_pct),
            alcohol_tolerance: Set(input.alcohol_tolerance),
            flavor_profile: Set(input.flavor_profile),
            styles: Set(input.styles),
            substitutes: Set(input.substitutes),
            species: Set(input.species),
            pof_positive: Set(input.pof_positive.map(|b| b as i32)),
            sta1_positive: Set(input.sta1_positive.map(|b| b as i32)),
            notes: Set(input.notes),
            best_for: Set(input.best_for),
            max_reuse: Set(input.max_reuse.map(|v| v as i32)),
            add_to_secondary: Set(Some(input.add_to_secondary as i32)),
            source: Set("user".to_string()),
            forked_from_id: Set(input.forked_from_id),
        }
        .insert(self.db)
        .await?;

        yeasts::Entity::find_by_id(&id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(Yeast::try_from)
    }

    pub async fn update_yeast(&self, id: &str, input: UpdateYeastInput) -> Result<Yeast, AppError> {
        let row = yeasts::Entity::find_by_id(id)
            .filter(yeasts::Column::Source.eq("user"))
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut active: yeasts::ActiveModel = row.into();
        if let Some(v) = input.name {
            active.name = Set(v);
        }
        if let Some(v) = input.type_ {
            active.r#type = Set(v);
        }
        if let Some(v) = input.form {
            active.form = Set(v);
        }
        if let Some(v) = input.laboratory {
            active.laboratory = Set(Some(v));
        }
        if let Some(v) = input.product_id {
            active.product_id = Set(Some(v));
        }
        if let Some(v) = input.min_temperature_c {
            active.min_temperature_c = Set(Some(v));
        }
        if let Some(v) = input.max_temperature_c {
            active.max_temperature_c = Set(Some(v));
        }
        if let Some(v) = input.flocculation {
            active.flocculation = Set(Some(v));
        }
        if let Some(v) = input.attenuation_pct {
            active.attenuation_pct = Set(Some(v));
        }
        if let Some(v) = input.min_attenuation_pct {
            active.min_attenuation_pct = Set(Some(v));
        }
        if let Some(v) = input.max_attenuation_pct {
            active.max_attenuation_pct = Set(Some(v));
        }
        if let Some(v) = input.alcohol_tolerance {
            active.alcohol_tolerance = Set(Some(v));
        }
        if let Some(v) = input.flavor_profile {
            active.flavor_profile = Set(Some(v));
        }
        if let Some(v) = input.styles {
            active.styles = Set(Some(v));
        }
        if let Some(v) = input.substitutes {
            active.substitutes = Set(Some(v));
        }
        if let Some(v) = input.species {
            active.species = Set(Some(v));
        }
        if let Some(v) = input.pof_positive {
            active.pof_positive = Set(Some(v as i32));
        }
        if let Some(v) = input.sta1_positive {
            active.sta1_positive = Set(Some(v as i32));
        }
        if let Some(v) = input.notes {
            active.notes = Set(Some(v));
        }
        if let Some(v) = input.best_for {
            active.best_for = Set(Some(v));
        }
        if let Some(v) = input.max_reuse {
            active.max_reuse = Set(Some(v as i32));
        }
        if let Some(v) = input.add_to_secondary {
            active.add_to_secondary = Set(Some(v as i32));
        }
        active.update(self.db).await?;

        yeasts::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(Yeast::try_from)
    }

    pub async fn delete_yeast(&self, id: &str) -> Result<(), AppError> {
        let result = yeasts::Entity::delete_many()
            .filter(yeasts::Column::Id.eq(id))
            .filter(yeasts::Column::Source.eq("user"))
            .exec(self.db)
            .await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound);
        }
        Ok(())
    }

    pub async fn create_misc(&self, input: CreateMiscInput) -> Result<Misc, AppError> {
        let id = new_id();
        miscs::ActiveModel {
            id: Set(id.clone()),
            name: Set(input.name),
            r#type: Set(input.type_),
            r#use: Set(input.use_),
            time_min: Set(input.time_min),
            notes: Set(input.notes),
            use_for: Set(input.use_for),
            amount_is_weight: Set(Some(input.amount_is_weight as i32)),
            source: Set("user".to_string()),
            forked_from_id: Set(input.forked_from_id),
        }
        .insert(self.db)
        .await?;

        miscs::Entity::find_by_id(&id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(Misc::try_from)
    }

    pub async fn update_misc(&self, id: &str, input: UpdateMiscInput) -> Result<Misc, AppError> {
        let row = miscs::Entity::find_by_id(id)
            .filter(miscs::Column::Source.eq("user"))
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut active: miscs::ActiveModel = row.into();
        if let Some(v) = input.name {
            active.name = Set(v);
        }
        if let Some(v) = input.type_ {
            active.r#type = Set(v);
        }
        if let Some(v) = input.use_ {
            active.r#use = Set(v);
        }
        if let Some(v) = input.time_min {
            active.time_min = Set(v);
        }
        if let Some(v) = input.notes {
            active.notes = Set(Some(v));
        }
        if let Some(v) = input.use_for {
            active.use_for = Set(Some(v));
        }
        if let Some(v) = input.amount_is_weight {
            active.amount_is_weight = Set(Some(v as i32));
        }
        active.update(self.db).await?;

        miscs::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(Misc::try_from)
    }

    pub async fn delete_misc(&self, id: &str) -> Result<(), AppError> {
        let result = miscs::Entity::delete_many()
            .filter(miscs::Column::Id.eq(id))
            .filter(miscs::Column::Source.eq("user"))
            .exec(self.db)
            .await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound);
        }
        Ok(())
    }

    pub async fn create_water(&self, input: CreateWaterInput) -> Result<Water, AppError> {
        let id = new_id();
        waters::ActiveModel {
            id: Set(id.clone()),
            name: Set(input.name),
            calcium_ppm: Set(input.calcium_ppm),
            bicarbonate_ppm: Set(input.bicarbonate_ppm),
            sulfate_ppm: Set(input.sulfate_ppm),
            chloride_ppm: Set(input.chloride_ppm),
            sodium_ppm: Set(input.sodium_ppm),
            magnesium_ppm: Set(input.magnesium_ppm),
            ph: Set(input.ph),
            notes: Set(input.notes),
            source: Set("user".to_string()),
            forked_from_id: Set(input.forked_from_id),
        }
        .insert(self.db)
        .await?;

        waters::Entity::find_by_id(&id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(Water::try_from)
    }

    pub async fn update_water(&self, id: &str, input: UpdateWaterInput) -> Result<Water, AppError> {
        let row = waters::Entity::find_by_id(id)
            .filter(waters::Column::Source.eq("user"))
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut active: waters::ActiveModel = row.into();
        if let Some(v) = input.name {
            active.name = Set(v);
        }
        if let Some(v) = input.calcium_ppm {
            active.calcium_ppm = Set(v);
        }
        if let Some(v) = input.bicarbonate_ppm {
            active.bicarbonate_ppm = Set(v);
        }
        if let Some(v) = input.sulfate_ppm {
            active.sulfate_ppm = Set(v);
        }
        if let Some(v) = input.chloride_ppm {
            active.chloride_ppm = Set(v);
        }
        if let Some(v) = input.sodium_ppm {
            active.sodium_ppm = Set(v);
        }
        if let Some(v) = input.magnesium_ppm {
            active.magnesium_ppm = Set(v);
        }
        if let Some(v) = input.ph {
            active.ph = Set(Some(v));
        }
        if let Some(v) = input.notes {
            active.notes = Set(Some(v));
        }
        active.update(self.db).await?;

        waters::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)
            .and_then(Water::try_from)
    }

    pub async fn delete_water(&self, id: &str) -> Result<(), AppError> {
        let result = waters::Entity::delete_many()
            .filter(waters::Column::Id.eq(id))
            .filter(waters::Column::Source.eq("user"))
            .exec(self.db)
            .await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::setup_test_db;

    // ── Hop ──────────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_hop_create_sets_source_user() {
        let db = setup_test_db().await;
        let repo = IngredientRepository::new(&db);
        let hop = repo
            .create_hop(CreateHopInput {
                name: "My Hop".into(),
                alpha_pct: 10.0,
                form: "pellet".into(),
                forked_from_id: None,
                beta_pct: None,
                type_: None,
                origin: None,
                year: None,
                notes: None,
                substitutes: None,
                hsi_pct: None,
                humulene_pct: None,
                caryophyllene_pct: None,
                cohumulone_pct: None,
                myrcene_pct: None,
            })
            .await
            .unwrap();
        assert_eq!(hop.source, "user");
        assert_eq!(hop.name, "My Hop");
        assert!(hop.forked_from_id.is_none());
    }

    #[tokio::test]
    async fn test_hop_create_fork_sets_forked_from_id() {
        let db = setup_test_db().await;
        let repo = IngredientRepository::new(&db);
        // Insert a seeded hop to fork from
        hops::ActiveModel {
            id: Set("bm-hop-test".to_string()),
            name: Set("Test Hop".to_string()),
            alpha_pct: Set(13.0),
            form: Set("pellet".to_string()),
            source: Set("seeded".to_string()),
            ..Default::default()
        }
        .insert(&db)
        .await
        .unwrap();
        let hop = repo
            .create_hop(CreateHopInput {
                name: "Test Hop (Custom)".into(),
                alpha_pct: 12.0,
                form: "pellet".into(),
                forked_from_id: Some("bm-hop-test".into()),
                beta_pct: None,
                type_: None,
                origin: None,
                year: None,
                notes: None,
                substitutes: None,
                hsi_pct: None,
                humulene_pct: None,
                caryophyllene_pct: None,
                cohumulone_pct: None,
                myrcene_pct: None,
            })
            .await
            .unwrap();
        assert_eq!(hop.source, "user");
        assert_eq!(hop.forked_from_id, Some("bm-hop-test".into()));
    }

    #[tokio::test]
    async fn test_hop_update_user_row() {
        let db = setup_test_db().await;
        let repo = IngredientRepository::new(&db);
        let created = repo
            .create_hop(CreateHopInput {
                name: "My Hop".into(),
                alpha_pct: 10.0,
                form: "pellet".into(),
                forked_from_id: None,
                beta_pct: None,
                type_: None,
                origin: None,
                year: None,
                notes: None,
                substitutes: None,
                hsi_pct: None,
                humulene_pct: None,
                caryophyllene_pct: None,
                cohumulone_pct: None,
                myrcene_pct: None,
            })
            .await
            .unwrap();
        let updated = repo
            .update_hop(
                &created.id,
                UpdateHopInput {
                    name: Some("My Hop v2".into()),
                    alpha_pct: Some(11.0),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(updated.name, "My Hop v2");
        assert_eq!(updated.alpha_pct, 11.0);
    }

    #[tokio::test]
    async fn test_hop_update_seeded_row_fails() {
        let db = setup_test_db().await;
        let repo = IngredientRepository::new(&db);
        let seeded = hops::Entity::find()
            .filter(hops::Column::Source.eq("seeded"))
            .one(&db)
            .await
            .unwrap()
            .unwrap();
        let result = repo
            .update_hop(
                &seeded.id,
                UpdateHopInput {
                    name: Some("Hacked".into()),
                    ..Default::default()
                },
            )
            .await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[tokio::test]
    async fn test_hop_delete_user_row() {
        let db = setup_test_db().await;
        let repo = IngredientRepository::new(&db);
        let created = repo
            .create_hop(CreateHopInput {
                name: "Temp Hop".into(),
                alpha_pct: 5.0,
                form: "pellet".into(),
                forked_from_id: None,
                beta_pct: None,
                type_: None,
                origin: None,
                year: None,
                notes: None,
                substitutes: None,
                hsi_pct: None,
                humulene_pct: None,
                caryophyllene_pct: None,
                cohumulone_pct: None,
                myrcene_pct: None,
            })
            .await
            .unwrap();
        repo.delete_hop(&created.id).await.unwrap();
        let result = hops::Entity::find_by_id(&created.id)
            .one(&db)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_hop_delete_seeded_row_fails() {
        let db = setup_test_db().await;
        let repo = IngredientRepository::new(&db);
        let seeded = hops::Entity::find()
            .filter(hops::Column::Source.eq("seeded"))
            .one(&db)
            .await
            .unwrap()
            .unwrap();
        let result = repo.delete_hop(&seeded.id).await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    // ── Fermentable ──────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_fermentable_create_sets_source_user() {
        let db = setup_test_db().await;
        let repo = IngredientRepository::new(&db);
        let f = repo
            .create_fermentable(CreateFermentableInput {
                name: "My Malt".into(),
                type_: "Grain".into(),
                yield_pct: 75.0,
                color_lovibond: 3.5,
                add_after_boil: false,
                forked_from_id: None,
                origin: None,
                supplier: None,
                notes: None,
                coarse_fine_diff_pct: None,
                moisture_pct: None,
                diastatic_power_lintner: None,
                protein_pct: None,
                max_in_batch_pct: None,
                recommend_mash: None,
                ibu_gal_per_lb: None,
            })
            .await
            .unwrap();
        assert_eq!(f.source, "user");
    }

    #[tokio::test]
    async fn test_fermentable_delete_seeded_row_fails() {
        let db = setup_test_db().await;
        let repo = IngredientRepository::new(&db);
        let seeded = fermentables::Entity::find()
            .filter(fermentables::Column::Source.eq("seeded"))
            .one(&db)
            .await
            .unwrap()
            .unwrap();
        let result = repo.delete_fermentable(&seeded.id).await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    // ── Yeast ────────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_yeast_create_sets_source_user() {
        let db = setup_test_db().await;
        let repo = IngredientRepository::new(&db);
        let y = repo
            .create_yeast(CreateYeastInput {
                name: "My Yeast".into(),
                type_: "ale".into(),
                form: "dry".into(),
                add_to_secondary: false,
                forked_from_id: None,
                laboratory: None,
                product_id: None,
                min_temperature_c: None,
                max_temperature_c: None,
                flocculation: None,
                attenuation_pct: None,
                min_attenuation_pct: None,
                max_attenuation_pct: None,
                alcohol_tolerance: None,
                flavor_profile: None,
                styles: None,
                substitutes: None,
                species: None,
                pof_positive: None,
                sta1_positive: None,
                notes: None,
                best_for: None,
                max_reuse: None,
            })
            .await
            .unwrap();
        assert_eq!(y.source, "user");
    }
}
