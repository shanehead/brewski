use serde::{Deserialize, Serialize};

use crate::entities;
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Style {
    pub id: String,
    pub name: String,
    pub category: String,
    pub category_number: String,
    pub style_letter: String,
    pub style_guide: String,
    pub type_: String,
    pub og_min: f64,
    pub og_max: f64,
    pub fg_min: f64,
    pub fg_max: f64,
    pub ibu_min: f64,
    pub ibu_max: f64,
    pub color_min_srm: f64,
    pub color_max_srm: f64,
    pub carb_min_vols: Option<f64>,
    pub carb_max_vols: Option<f64>,
    pub abv_min_pct: Option<f64>,
    pub abv_max_pct: Option<f64>,
    pub notes: Option<String>,
    pub profile: Option<String>,
    pub ingredients: Option<String>,
    pub examples: Option<String>,
}

impl TryFrom<entities::styles::Model> for Style {
    type Error = AppError;
    fn try_from(m: entities::styles::Model) -> Result<Self, AppError> {
        Ok(Self {
            id: m.id,
            name: m.name,
            category: m.category,
            category_number: m.category_number,
            style_letter: m.style_letter,
            style_guide: m.style_guide,
            type_: m.r#type,
            og_min: m.og_min,
            og_max: m.og_max,
            fg_min: m.fg_min,
            fg_max: m.fg_max,
            ibu_min: m.ibu_min,
            ibu_max: m.ibu_max,
            color_min_srm: m.color_min_srm,
            color_max_srm: m.color_max_srm,
            carb_min_vols: m.carb_min_vols,
            carb_max_vols: m.carb_max_vols,
            abv_min_pct: m.abv_min_pct,
            abv_max_pct: m.abv_max_pct,
            notes: m.notes,
            profile: m.profile,
            ingredients: m.ingredients,
            examples: m.examples,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EquipmentProfile {
    pub id: String,
    pub name: String,
    pub notes: Option<String>,
    pub boil_size_l: f64,
    pub batch_size_l: f64,
    pub calc_boil_volume: bool,
    pub tun_volume_l: Option<f64>,
    pub tun_weight_kg: Option<f64>,
    pub tun_specific_heat: Option<f64>,
    pub lauter_deadspace_l: f64,
    pub top_up_kettle_l: f64,
    pub trub_chiller_loss_l: f64,
    pub evap_rate_pct_hr: f64,
    pub boil_time_min: f64,
    pub top_up_water_l: f64,
    pub fermenter_loss_l: f64,
    pub hop_utilization_pct: f64,
    pub efficiency_pct: f64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl TryFrom<entities::equipment_profiles::Model> for EquipmentProfile {
    type Error = AppError;
    fn try_from(m: entities::equipment_profiles::Model) -> Result<Self, AppError> {
        Ok(Self {
            id: m.id,
            name: m.name,
            notes: m.notes,
            boil_size_l: m.boil_size_l,
            batch_size_l: m.batch_size_l,
            calc_boil_volume: m.calc_boil_volume != 0,
            tun_volume_l: m.tun_volume_l,
            tun_weight_kg: m.tun_weight_kg,
            tun_specific_heat: m.tun_specific_heat,
            lauter_deadspace_l: m.lauter_deadspace_l.unwrap_or(0.0),
            top_up_kettle_l: m.top_up_kettle_l.unwrap_or(0.0),
            trub_chiller_loss_l: m.trub_chiller_loss_l.unwrap_or(0.0),
            evap_rate_pct_hr: m.evap_rate_pct_hr.unwrap_or(10.0),
            boil_time_min: m.boil_time_min,
            top_up_water_l: m.top_up_water_l.unwrap_or(0.0),
            fermenter_loss_l: m.fermenter_loss_l.unwrap_or(0.0),
            hop_utilization_pct: m.hop_utilization_pct.unwrap_or(100.0),
            efficiency_pct: m.efficiency_pct,
            created_at: m.created_at as i64,
            updated_at: m.updated_at as i64,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Fermentable {
    pub id: String,
    pub name: String,
    pub type_: String,
    pub yield_pct: f64,
    pub color_lovibond: f64,
    pub origin: Option<String>,
    pub supplier: Option<String>,
    pub notes: Option<String>,
    pub add_after_boil: bool,
    pub coarse_fine_diff_pct: Option<f64>,
    pub moisture_pct: Option<f64>,
    pub diastatic_power_lintner: Option<f64>,
    pub protein_pct: Option<f64>,
    pub max_in_batch_pct: Option<f64>,
    pub recommend_mash: Option<bool>,
    pub ibu_gal_per_lb: Option<f64>,
}

impl TryFrom<entities::fermentables::Model> for Fermentable {
    type Error = AppError;
    fn try_from(m: entities::fermentables::Model) -> Result<Self, AppError> {
        Ok(Self {
            id: m.id,
            name: m.name,
            type_: m.r#type,
            yield_pct: m.yield_pct,
            color_lovibond: m.color_lovibond,
            origin: m.origin,
            supplier: m.supplier,
            notes: m.notes,
            add_after_boil: m.add_after_boil.unwrap_or(0) != 0,
            coarse_fine_diff_pct: m.coarse_fine_diff_pct,
            moisture_pct: m.moisture_pct,
            diastatic_power_lintner: m.diastatic_power_lintner,
            protein_pct: m.protein_pct,
            max_in_batch_pct: m.max_in_batch_pct,
            recommend_mash: m.recommend_mash.map(|v| v != 0),
            ibu_gal_per_lb: m.ibu_gal_per_lb,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hop {
    pub id: String,
    pub name: String,
    pub alpha_pct: f64,
    pub beta_pct: Option<f64>,
    pub form: String,
    pub type_: Option<String>,
    pub origin: Option<String>,
    pub year: Option<String>,
    pub notes: Option<String>,
    pub substitutes: Option<String>,
    pub hsi_pct: Option<f64>,
    pub humulene_pct: Option<f64>,
    pub caryophyllene_pct: Option<f64>,
    pub cohumulone_pct: Option<f64>,
    pub myrcene_pct: Option<f64>,
}

impl TryFrom<entities::hops::Model> for Hop {
    type Error = AppError;
    fn try_from(m: entities::hops::Model) -> Result<Self, AppError> {
        Ok(Self {
            id: m.id,
            name: m.name,
            alpha_pct: m.alpha_pct,
            beta_pct: m.beta_pct,
            form: m.form,
            type_: m.r#type,
            origin: m.origin,
            year: m.year,
            notes: m.notes,
            substitutes: m.substitutes,
            hsi_pct: m.hsi_pct,
            humulene_pct: m.humulene_pct,
            caryophyllene_pct: m.caryophyllene_pct,
            cohumulone_pct: m.cohumulone_pct,
            myrcene_pct: m.myrcene_pct,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Yeast {
    pub id: String,
    pub name: String,
    pub type_: String,
    pub form: String,
    pub laboratory: Option<String>,
    pub product_id: Option<String>,
    pub min_temperature_c: Option<f64>,
    pub max_temperature_c: Option<f64>,
    pub flocculation: Option<String>,
    pub attenuation_pct: Option<f64>,
    pub notes: Option<String>,
    pub best_for: Option<String>,
    pub max_reuse: Option<i64>,
    pub add_to_secondary: bool,
}

impl TryFrom<entities::yeasts::Model> for Yeast {
    type Error = AppError;
    fn try_from(m: entities::yeasts::Model) -> Result<Self, AppError> {
        Ok(Self {
            id: m.id,
            name: m.name,
            type_: m.r#type,
            form: m.form,
            laboratory: m.laboratory,
            product_id: m.product_id,
            min_temperature_c: m.min_temperature_c,
            max_temperature_c: m.max_temperature_c,
            flocculation: m.flocculation,
            attenuation_pct: m.attenuation_pct,
            notes: m.notes,
            best_for: m.best_for,
            max_reuse: m.max_reuse.map(|v| v as i64),
            add_to_secondary: m.add_to_secondary.unwrap_or(0) != 0,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Misc {
    pub id: String,
    pub name: String,
    pub type_: String,
    pub use_: String,
    pub time_min: f64,
    pub notes: Option<String>,
    pub use_for: Option<String>,
    pub amount_is_weight: bool,
}

impl TryFrom<entities::miscs::Model> for Misc {
    type Error = AppError;
    fn try_from(m: entities::miscs::Model) -> Result<Self, AppError> {
        Ok(Self {
            id: m.id,
            name: m.name,
            type_: m.r#type,
            use_: m.r#use,
            time_min: m.time_min,
            notes: m.notes,
            use_for: m.use_for,
            amount_is_weight: m.amount_is_weight.unwrap_or(0) != 0,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Water {
    pub id: String,
    pub name: String,
    pub calcium_ppm: f64,
    pub bicarbonate_ppm: f64,
    pub sulfate_ppm: f64,
    pub chloride_ppm: f64,
    pub sodium_ppm: f64,
    pub magnesium_ppm: f64,
    pub ph: Option<f64>,
    pub notes: Option<String>,
}

impl TryFrom<entities::waters::Model> for Water {
    type Error = AppError;
    fn try_from(m: entities::waters::Model) -> Result<Self, AppError> {
        Ok(Self {
            id: m.id,
            name: m.name,
            calcium_ppm: m.calcium_ppm,
            bicarbonate_ppm: m.bicarbonate_ppm,
            sulfate_ppm: m.sulfate_ppm,
            chloride_ppm: m.chloride_ppm,
            sodium_ppm: m.sodium_ppm,
            magnesium_ppm: m.magnesium_ppm,
            ph: m.ph,
            notes: m.notes,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecipeSummary {
    pub id: String,
    pub name: String,
    pub style_name: Option<String>,
    pub type_: String,
    pub batch_size_l: f64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub type_: String,
    pub brewer: Option<String>,
    pub asst_brewer: Option<String>,
    pub batch_size_l: f64,
    pub boil_size_l: f64,
    pub boil_time_min: f64,
    pub efficiency_pct: Option<f64>,
    pub style_id: Option<String>,
    pub equipment_profile_id: Option<String>,
    pub notes: Option<String>,
    pub taste_notes: Option<String>,
    pub taste_rating: Option<f64>,
    pub og: Option<f64>,
    pub fg: Option<f64>,
    pub fermentation_stages: i64,
    pub primary_age_days: Option<f64>,
    pub primary_temp_c: Option<f64>,
    pub secondary_age_days: Option<f64>,
    pub secondary_temp_c: Option<f64>,
    pub tertiary_age_days: Option<f64>,
    pub tertiary_temp_c: Option<f64>,
    pub age_days: Option<f64>,
    pub age_temp_c: Option<f64>,
    pub carbonation_vols: Option<f64>,
    pub forced_carbonation: bool,
    pub priming_sugar_name: Option<String>,
    pub carbonation_temp_c: Option<f64>,
    pub priming_sugar_equiv: Option<f64>,
    pub keg_priming_factor: Option<f64>,
    pub date: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub equipment_profile: Option<EquipmentProfile>,
    pub style: Option<Style>,
    pub fermentables: Vec<RecipeAdditionFermentable>,
    pub hops: Vec<RecipeAdditionHop>,
    pub yeasts: Vec<RecipeAdditionYeast>,
    pub miscs: Vec<RecipeAdditionMisc>,
    pub waters: Vec<RecipeAdditionWater>,
    pub mash: Option<Mash>,
}

// "Addition" (not "ingredient") — captures a timed process event (amount, order, boil time, etc.)
// rather than a bare ingredient reference. See CLAUDE.md § Domain terminology.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecipeAdditionFermentable {
    pub id: String,
    pub recipe_id: String,
    pub fermentable_id: Option<String>,
    pub name: String,
    pub type_: String,
    pub yield_pct: f64,
    pub color_lovibond: f64,
    pub amount_kg: f64,
    pub add_after_boil: bool,
    pub addition_order: i64,
}

impl TryFrom<entities::recipe_addition_fermentables::Model> for RecipeAdditionFermentable {
    type Error = AppError;
    fn try_from(m: entities::recipe_addition_fermentables::Model) -> Result<Self, AppError> {
        Ok(Self {
            id: m.id,
            recipe_id: m.recipe_id,
            fermentable_id: m.fermentable_id,
            name: m.name,
            type_: m.r#type,
            yield_pct: m.yield_pct,
            color_lovibond: m.color_lovibond,
            amount_kg: m.amount_kg,
            add_after_boil: m.add_after_boil.unwrap_or(0) != 0,
            addition_order: m.addition_order as i64,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecipeAdditionHop {
    pub id: String,
    pub recipe_id: String,
    pub hop_id: Option<String>,
    pub name: String,
    pub alpha_pct: f64,
    pub form: String,
    pub amount_kg: f64,
    pub use_: String,
    pub time_min: f64,
    pub addition_order: i64,
}

impl TryFrom<entities::recipe_addition_hops::Model> for RecipeAdditionHop {
    type Error = AppError;
    fn try_from(m: entities::recipe_addition_hops::Model) -> Result<Self, AppError> {
        Ok(Self {
            id: m.id,
            recipe_id: m.recipe_id,
            hop_id: m.hop_id,
            name: m.name,
            alpha_pct: m.alpha_pct,
            form: m.form,
            amount_kg: m.amount_kg,
            use_: m.r#use,
            time_min: m.time_min,
            addition_order: m.addition_order as i64,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecipeAdditionYeast {
    pub id: String,
    pub recipe_id: String,
    pub yeast_id: Option<String>,
    pub name: String,
    pub type_: String,
    pub form: String,
    pub laboratory: Option<String>,
    pub product_id: Option<String>,
    pub attenuation_pct: Option<f64>,
    pub amount: Option<f64>,
    pub amount_is_weight: bool,
    pub add_to_secondary: bool,
    pub times_cultured: i64,
}

impl TryFrom<entities::recipe_addition_yeasts::Model> for RecipeAdditionYeast {
    type Error = AppError;
    fn try_from(m: entities::recipe_addition_yeasts::Model) -> Result<Self, AppError> {
        Ok(Self {
            id: m.id,
            recipe_id: m.recipe_id,
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
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecipeAdditionMisc {
    pub id: String,
    pub recipe_id: String,
    pub misc_id: Option<String>,
    pub name: String,
    pub type_: String,
    pub use_: String,
    pub amount: f64,
    pub amount_is_weight: bool,
    pub time_min: f64,
    pub addition_order: i64,
}

impl TryFrom<entities::recipe_addition_miscs::Model> for RecipeAdditionMisc {
    type Error = AppError;
    fn try_from(m: entities::recipe_addition_miscs::Model) -> Result<Self, AppError> {
        Ok(Self {
            id: m.id,
            recipe_id: m.recipe_id,
            misc_id: m.misc_id,
            name: m.name,
            type_: m.r#type,
            use_: m.r#use,
            amount: m.amount,
            amount_is_weight: m.amount_is_weight.unwrap_or(0) != 0,
            time_min: m.time_min,
            addition_order: m.addition_order as i64,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecipeAdditionWater {
    pub id: String,
    pub recipe_id: String,
    pub water_id: Option<String>,
    pub name: String,
    pub amount_l: f64,
}

impl TryFrom<entities::recipe_addition_waters::Model> for RecipeAdditionWater {
    type Error = AppError;
    fn try_from(m: entities::recipe_addition_waters::Model) -> Result<Self, AppError> {
        Ok(Self {
            id: m.id,
            recipe_id: m.recipe_id,
            water_id: m.water_id,
            name: m.name,
            amount_l: m.amount_l,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mash {
    pub id: String,
    pub recipe_id: String,
    pub name: String,
    pub grain_temp_c: f64,
    pub tun_temp_c: Option<f64>,
    pub sparge_temp_c: Option<f64>,
    pub ph: Option<f64>,
    pub tun_weight_kg: Option<f64>,
    pub tun_specific_heat: Option<f64>,
    pub equip_adjust: bool,
    pub notes: Option<String>,
    pub steps: Vec<MashStep>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MashStep {
    pub id: String,
    pub mash_id: String,
    pub name: String,
    pub type_: String,
    pub infuse_amount_l: Option<f64>,
    pub step_temp_c: f64,
    pub step_time_min: i64,
    pub ramp_time_min: Option<i64>,
    pub end_temp_c: Option<f64>,
    pub step_order: i64,
}

impl TryFrom<entities::mash_steps::Model> for MashStep {
    type Error = AppError;
    fn try_from(m: entities::mash_steps::Model) -> Result<Self, AppError> {
        Ok(Self {
            id: m.id,
            mash_id: m.mash_id,
            name: m.name,
            type_: m.r#type,
            infuse_amount_l: m.infuse_amount_l,
            step_temp_c: m.step_temp_c,
            step_time_min: m.step_time_min as i64,
            ramp_time_min: m.ramp_time_min.map(|v| v as i64),
            end_temp_c: m.end_temp_c,
            step_order: m.step_order as i64,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecipeStats {
    pub og: f64,
    pub fg: f64,
    pub abv_pct: f64,
    pub ibu: f64,
    pub srm: f64,
    pub calories_per_355ml: f64,
    pub bu_gu_ratio: f64,
    pub pre_boil_gravity: f64,
    pub pre_boil_volume_l: f64,
    pub post_boil_volume_l: f64,
}

// --- Input types for create/update commands ---

#[derive(Debug, Deserialize, Default)]
pub struct CreateRecipeInput {
    pub name: String,
    pub type_: Option<String>,
    pub batch_size_l: Option<f64>,
    pub boil_size_l: Option<f64>,
    pub boil_time_min: Option<f64>,
    pub equipment_profile_id: Option<String>,
    pub source_id: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct UpdateRecipeInput {
    pub name: Option<String>,
    pub type_: Option<String>,
    pub brewer: Option<String>,
    pub asst_brewer: Option<String>,
    pub batch_size_l: Option<f64>,
    pub boil_size_l: Option<f64>,
    pub boil_time_min: Option<f64>,
    pub efficiency_pct: Option<f64>,
    pub style_id: Option<String>,
    pub equipment_profile_id: Option<String>,
    pub notes: Option<String>,
    pub taste_notes: Option<String>,
    pub taste_rating: Option<f64>,
    pub fermentation_stages: Option<i64>,
    pub primary_age_days: Option<f64>,
    pub primary_temp_c: Option<f64>,
    pub secondary_age_days: Option<f64>,
    pub secondary_temp_c: Option<f64>,
    pub tertiary_age_days: Option<f64>,
    pub tertiary_temp_c: Option<f64>,
    pub age_days: Option<f64>,
    pub age_temp_c: Option<f64>,
    pub carbonation_vols: Option<f64>,
    pub forced_carbonation: Option<bool>,
    pub priming_sugar_name: Option<String>,
    pub carbonation_temp_c: Option<f64>,
    pub priming_sugar_equiv: Option<f64>,
    pub keg_priming_factor: Option<f64>,
    pub date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFermentableAdditionInput {
    pub fermentable_id: Option<String>,
    pub name: String,
    pub type_: String,
    pub yield_pct: f64,
    pub color_lovibond: f64,
    pub amount_kg: f64,
    pub add_after_boil: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFermentableAdditionInput {
    pub amount_kg: Option<f64>,
    pub add_after_boil: Option<bool>,
    pub addition_order: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateHopAdditionInput {
    pub hop_id: Option<String>,
    pub name: String,
    pub alpha_pct: f64,
    pub form: Option<String>,
    pub amount_kg: f64,
    pub use_: String,
    pub time_min: f64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateHopAdditionInput {
    pub amount_kg: Option<f64>,
    pub use_: Option<String>,
    pub time_min: Option<f64>,
    pub addition_order: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateYeastAdditionInput {
    pub yeast_id: Option<String>,
    pub name: String,
    pub type_: String,
    pub form: String,
    pub laboratory: Option<String>,
    pub product_id: Option<String>,
    pub attenuation_pct: Option<f64>,
    pub amount: Option<f64>,
    pub amount_is_weight: Option<bool>,
    pub add_to_secondary: Option<bool>,
    pub times_cultured: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateYeastAdditionInput {
    pub attenuation_pct: Option<f64>,
    pub amount: Option<f64>,
    pub amount_is_weight: Option<bool>,
    pub add_to_secondary: Option<bool>,
    pub times_cultured: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMiscAdditionInput {
    pub misc_id: Option<String>,
    pub name: String,
    pub type_: String,
    pub use_: String,
    pub amount: f64,
    pub amount_is_weight: Option<bool>,
    pub time_min: f64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMiscAdditionInput {
    pub amount: Option<f64>,
    pub amount_is_weight: Option<bool>,
    pub use_: Option<String>,
    pub time_min: Option<f64>,
    pub addition_order: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateWaterAdditionInput {
    pub water_id: Option<String>,
    pub name: String,
    pub amount_l: f64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWaterAdditionInput {
    pub amount_l: Option<f64>,
}

#[derive(Debug, Deserialize, Default)]
pub struct UpdateMashInput {
    pub name: Option<String>,
    pub grain_temp_c: Option<f64>,
    pub tun_temp_c: Option<f64>,
    pub sparge_temp_c: Option<f64>,
    pub ph: Option<f64>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMashStepInput {
    pub name: String,
    pub type_: Option<String>,
    pub infuse_amount_l: Option<f64>,
    pub step_temp_c: f64,
    pub step_time_min: i64,
    pub ramp_time_min: Option<i64>,
    pub end_temp_c: Option<f64>,
}

#[derive(Debug, Deserialize, Default)]
pub struct UpdateMashStepInput {
    pub name: Option<String>,
    pub type_: Option<String>,
    pub infuse_amount_l: Option<f64>,
    pub step_temp_c: Option<f64>,
    pub step_time_min: Option<i64>,
    pub ramp_time_min: Option<i64>,
    pub end_temp_c: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEquipmentProfileInput {
    pub name: String,
    pub notes: Option<String>,
    pub boil_size_l: f64,
    pub batch_size_l: f64,
    pub boil_time_min: Option<f64>,
    pub evap_rate_pct_hr: Option<f64>,
    pub trub_chiller_loss_l: Option<f64>,
    pub fermenter_loss_l: Option<f64>,
    pub efficiency_pct: f64,
}

#[derive(Debug, Deserialize, Default)]
pub struct UpdateEquipmentProfileInput {
    pub name: Option<String>,
    pub notes: Option<String>,
    pub boil_size_l: Option<f64>,
    pub batch_size_l: Option<f64>,
    pub boil_time_min: Option<f64>,
    pub evap_rate_pct_hr: Option<f64>,
    pub trub_chiller_loss_l: Option<f64>,
    pub fermenter_loss_l: Option<f64>,
    pub efficiency_pct: Option<f64>,
}
