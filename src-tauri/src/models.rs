use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Style {
    pub id: String,
    pub name: String,
    pub category: String,
    pub category_number: String,
    pub style_letter: String,
    pub style_guide: String,
    #[sqlx(rename = "type")]
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

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
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

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Fermentable {
    pub id: String,
    pub name: String,
    #[sqlx(rename = "type")]
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

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Hop {
    pub id: String,
    pub name: String,
    pub alpha_pct: f64,
    pub beta_pct: Option<f64>,
    pub form: String,
    #[sqlx(rename = "type")]
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

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Yeast {
    pub id: String,
    pub name: String,
    #[sqlx(rename = "type")]
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

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Misc {
    pub id: String,
    pub name: String,
    #[sqlx(rename = "type")]
    pub type_: String,
    #[sqlx(rename = "use")]
    pub use_: String,
    pub time_min: f64,
    pub notes: Option<String>,
    pub use_for: Option<String>,
    pub amount_is_weight: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
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

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct RecipeSummary {
    pub id: String,
    pub name: String,
    pub style_name: Option<String>,
    #[sqlx(rename = "type")]
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

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct RecipeRow {
    pub id: String,
    pub name: String,
    #[sqlx(rename = "type")]
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
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct RecipeAdditionFermentable {
    pub id: String,
    pub recipe_id: String,
    pub fermentable_id: Option<String>,
    pub name: String,
    #[sqlx(rename = "type")]
    pub type_: String,
    pub yield_pct: f64,
    pub color_lovibond: f64,
    pub amount_kg: f64,
    pub add_after_boil: bool,
    pub addition_order: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct RecipeAdditionHop {
    pub id: String,
    pub recipe_id: String,
    pub hop_id: Option<String>,
    pub name: String,
    pub alpha_pct: f64,
    pub form: String,
    pub amount_kg: f64,
    #[sqlx(rename = "use")]
    pub use_: String,
    pub time_min: f64,
    pub addition_order: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct RecipeAdditionYeast {
    pub id: String,
    pub recipe_id: String,
    pub yeast_id: Option<String>,
    pub name: String,
    #[sqlx(rename = "type")]
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

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct RecipeAdditionMisc {
    pub id: String,
    pub recipe_id: String,
    pub misc_id: Option<String>,
    pub name: String,
    #[sqlx(rename = "type")]
    pub type_: String,
    #[sqlx(rename = "use")]
    pub use_: String,
    pub amount: f64,
    pub amount_is_weight: bool,
    pub time_min: f64,
    pub addition_order: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct RecipeAdditionWater {
    pub id: String,
    pub recipe_id: String,
    pub water_id: Option<String>,
    pub name: String,
    pub amount_l: f64,
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

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct MashRow {
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
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct MashStep {
    pub id: String,
    pub mash_id: String,
    pub name: String,
    #[sqlx(rename = "type")]
    pub type_: String,
    pub infuse_amount_l: Option<f64>,
    pub step_temp_c: f64,
    pub step_time_min: i64,
    pub ramp_time_min: Option<i64>,
    pub end_temp_c: Option<f64>,
    pub step_order: i64,
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

#[derive(Debug, Deserialize)]
pub struct CreateRecipeInput {
    pub name: String,
    pub type_: Option<String>,
    pub batch_size_l: Option<f64>,
    pub boil_size_l: Option<f64>,
    pub boil_time_min: Option<f64>,
    pub equipment_profile_id: Option<String>,
    pub source_id: Option<String>,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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
