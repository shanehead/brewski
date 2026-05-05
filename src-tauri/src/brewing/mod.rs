pub mod abv;
pub mod ibu;
pub mod og;
pub mod srm;
pub mod volumes;

use crate::models::{Recipe, RecipeStats};

pub fn calculate_stats(recipe: &Recipe) -> RecipeStats {
    let efficiency = recipe.efficiency_pct
        .or_else(|| recipe.equipment_profile.as_ref().map(|e| e.efficiency_pct))
        .unwrap_or(72.0);

    let fermentable_inputs: Vec<(&f64, &f64, bool)> = recipe.fermentables.iter()
        .map(|f| (&f.yield_pct, &f.amount_kg, f.add_after_boil))
        .collect();

    let og = og::calculate_og(&fermentable_inputs, recipe.batch_size_l, efficiency);

    let fg = recipe.yeasts.iter()
        .filter_map(|y| y.attenuation_pct)
        .next()
        .map(|attenuation| abv::calculate_fg(og, attenuation))
        .unwrap_or_else(|| abv::calculate_fg(og, 75.0));

    let abv_pct = abv::calculate_abv(og, fg);
    let calories = abv::calculate_calories_per_355ml(og, fg);

    let equipment = recipe.equipment_profile.as_ref();
    let evaporation_rate = equipment.map(|e| e.evap_rate_pct_hr).unwrap_or(10.0);
    let trub_chiller_loss = equipment.map(|e| e.trub_chiller_loss_l).unwrap_or(1.0);
    let fermenter_loss = equipment.map(|e| e.fermenter_loss_l).unwrap_or(1.0);
    let top_up_water = equipment.map(|e| e.top_up_water_l).unwrap_or(0.0);

    let (pre_boil_volume_l, post_boil_volume_l) = volumes::calculate_boil_volumes(
        recipe.batch_size_l,
        recipe.boil_time_min,
        evaporation_rate,
        trub_chiller_loss,
        fermenter_loss,
        top_up_water,
    );

    let pre_boil_gravity = volumes::calculate_pre_boil_gravity(og, post_boil_volume_l, pre_boil_volume_l);

    let hop_inputs: Vec<(&f64, &f64, &f64, bool)> = recipe.hops.iter()
        .map(|h| (&h.alpha_pct, &h.amount_kg, &h.time_min, h.use_ == "dry hop"))
        .collect();

    let ibu = ibu::tinseth_ibu(&hop_inputs, og, post_boil_volume_l);

    let srm_inputs: Vec<(&f64, &f64)> = recipe.fermentables.iter()
        .map(|f| (&f.color_lovibond, &f.amount_kg))
        .collect();

    let srm = srm::morey_srm(&srm_inputs, recipe.batch_size_l);

    let gravity_units = (og - 1.0) * 1000.0;
    let bu_gu_ratio = if gravity_units > 0.0 { ibu / gravity_units } else { 0.0 };

    RecipeStats {
        og,
        fg,
        abv_pct,
        ibu,
        srm,
        calories_per_355ml: calories,
        bu_gu_ratio,
        pre_boil_gravity,
        pre_boil_volume_l,
        post_boil_volume_l,
    }
}
