use crate::entities;
use crate::error::AppError;
pub use crate::models_gen::*;

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
            batch_volume_target: m.batch_volume_target,
            mash_tun_loss_l: m.mash_tun_loss_l,
            hlt_deadspace_l: m.hlt_deadspace_l,
            cooling_shrinkage_pct: m.cooling_shrinkage_pct,
            calc_mash_efficiency: m.calc_mash_efficiency != 0,
            mash_efficiency_pct: m.mash_efficiency_pct,
            calc_aroma_hop_utilization: m.calc_aroma_hop_utilization != 0,
            aroma_hop_utilization_pct: m.aroma_hop_utilization_pct,
            whirlpool_time_min: m.whirlpool_time_min,
            altitude_adjustment: m.altitude_adjustment != 0,
            boil_temp_f: m.boil_temp_f,
            sparge_method: m.sparge_method,
            mash_volume_min_l: m.mash_volume_min_l,
            mash_volume_max_l: m.mash_volume_max_l,
            sparge_volume_min_l: m.sparge_volume_min_l,
            sparge_volume_max_l: m.sparge_volume_max_l,
            calc_strike_water_temp: m.calc_strike_water_temp != 0,
            created_at: m.created_at as i64,
            updated_at: m.updated_at as i64,
        })
    }
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
            min_attenuation_pct: m.min_attenuation_pct,
            max_attenuation_pct: m.max_attenuation_pct,
            alcohol_tolerance: m.alcohol_tolerance,
            flavor_profile: m.flavor_profile,
            styles: m.styles,
            substitutes: m.substitutes,
            species: m.species,
            pof_positive: m.pof_positive.map(|v| v != 0),
            sta1_positive: m.sta1_positive.map(|v| v != 0),
        })
    }
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
            hopstand_temp_c: m.hopstand_temp_c,
        })
    }
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

impl TryFrom<entities::recipe_water_adjustments::Model> for RecipeWaterAdjustment {
    type Error = AppError;
    fn try_from(m: entities::recipe_water_adjustments::Model) -> Result<Self, AppError> {
        Ok(Self {
            id: m.id,
            recipe_id: m.recipe_id,
            addition: m
                .addition
                .parse()
                .map_err(|e| AppError::Internal(format!("{}", e)))?,
            target: m
                .target
                .parse()
                .map_err(|e| AppError::Internal(format!("{}", e)))?,
            amount: m.amount,
        })
    }
}

impl Default for WaterProfile {
    fn default() -> Self {
        Self {
            calcium_ppm: 0.0,
            magnesium_ppm: 0.0,
            sodium_ppm: 0.0,
            chloride_ppm: 0.0,
            sulfate_ppm: 0.0,
            bicarbonate_ppm: 0.0,
            cl_so4_ratio: 0.0,
        }
    }
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

impl Default for CreateRecipeInput {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            batch_size_l: None,
            boil_size_l: None,
            boil_time_min: None,
            equipment_profile_id: None,
            source_id: None,
            style_id: None,
            type_: None,
            hopstand_temp_c: None,
        }
    }
}

impl TryFrom<entities::batch_gravity_readings::Model> for GravityReading {
    type Error = AppError;
    fn try_from(m: entities::batch_gravity_readings::Model) -> Result<Self, AppError> {
        Ok(Self {
            id: m.id,
            batch_id: m.batch_id,
            recorded_at: m.recorded_at as i64,
            gravity: m.gravity,
            temp_c: m.temp_c,
            notes: m.notes,
        })
    }
}

impl TryFrom<entities::recipe_versions::Model> for RecipeVersionSummary {
    type Error = AppError;
    fn try_from(m: entities::recipe_versions::Model) -> Result<Self, AppError> {
        Ok(Self {
            id: m.id,
            recipe_id: m.recipe_id,
            version_number: m.version_number as i64,
            name: m.name,
            parent_version_id: m.parent_version_id,
            created_at: m.created_at as i64,
        })
    }
}
