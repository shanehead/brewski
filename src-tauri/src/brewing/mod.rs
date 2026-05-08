pub mod abv;
pub mod ibu;
pub mod og;
pub mod srm;
pub mod strike;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        EquipmentProfile, Recipe, RecipeAdditionFermentable, RecipeAdditionHop, RecipeAdditionYeast,
    };

    fn minimal_recipe() -> Recipe {
        Recipe {
            id: "r1".into(),
            name: "Test".into(),
            type_: "all_grain".into(),
            brewer: None,
            asst_brewer: None,
            batch_size_l: 23.0,
            boil_size_l: 27.0,
            boil_time_min: 60.0,
            efficiency_pct: Some(75.0),
            style_id: None,
            equipment_profile_id: None,
            notes: None,
            taste_notes: None,
            taste_rating: None,
            og: None,
            fg: None,
            fermentation_stages: 1,
            primary_age_days: None,
            primary_temp_c: None,
            secondary_age_days: None,
            secondary_temp_c: None,
            tertiary_age_days: None,
            tertiary_temp_c: None,
            age_days: None,
            age_temp_c: None,
            carbonation_vols: None,
            forced_carbonation: false,
            priming_sugar_name: None,
            carbonation_temp_c: None,
            priming_sugar_equiv: None,
            keg_priming_factor: None,
            date: None,
            created_at: 0,
            updated_at: 0,
            equipment_profile: None,
            style: None,
            fermentables: vec![],
            hops: vec![],
            yeasts: vec![],
            miscs: vec![],
            waters: vec![],
            mash: None,
        }
    }

    fn pale_malt() -> RecipeAdditionFermentable {
        RecipeAdditionFermentable {
            id: "f1".into(),
            recipe_id: "r1".into(),
            fermentable_id: None,
            name: "Pale Malt".into(),
            type_: "grain".into(),
            yield_pct: 78.0,
            color_lovibond: 1.8,
            amount_kg: 4.5,
            add_after_boil: false,
            addition_order: 0,
        }
    }

    #[test]
    fn test_stats_empty_recipe() {
        let stats = calculate_stats(&minimal_recipe());
        assert_eq!(stats.og, 1.0);
        assert_eq!(stats.ibu, 0.0);
        assert_eq!(stats.srm, 0.0);
        assert_eq!(stats.abv_pct, 0.0);
        assert_eq!(stats.bu_gu_ratio, 0.0);
    }

    #[test]
    fn test_stats_with_grain() {
        let mut recipe = minimal_recipe();
        recipe.fermentables = vec![pale_malt()];
        let stats = calculate_stats(&recipe);
        assert!(stats.og > 1.0);
        assert!(stats.srm > 0.0);
        assert!(stats.fg < stats.og);
        assert!(stats.abv_pct > 0.0);
    }

    #[test]
    fn test_stats_with_hops() {
        let mut recipe = minimal_recipe();
        recipe.fermentables = vec![pale_malt()];
        recipe.hops = vec![RecipeAdditionHop {
            id: "h1".into(),
            recipe_id: "r1".into(),
            hop_id: None,
            name: "Cascade".into(),
            alpha_pct: 5.5,
            form: "pellet".into(),
            amount_kg: 0.05,
            use_: "Boil".into(),
            time_min: 60.0,
            addition_order: 0,
        }];
        let stats = calculate_stats(&recipe);
        assert!(stats.ibu > 0.0);
        assert!(stats.bu_gu_ratio > 0.0);
    }

    #[test]
    fn test_stats_yeast_attenuation_used() {
        let mut recipe = minimal_recipe();
        recipe.fermentables = vec![pale_malt()];
        recipe.yeasts = vec![RecipeAdditionYeast {
            id: "y1".into(),
            recipe_id: "r1".into(),
            yeast_id: None,
            name: "US-05".into(),
            type_: "ale".into(),
            form: "dry".into(),
            laboratory: None,
            product_id: None,
            attenuation_pct: Some(81.0),
            amount: Some(1.0),
            amount_is_weight: true,
            add_to_secondary: false,
            times_cultured: 0,
        }];
        let stats_81 = calculate_stats(&recipe);

        recipe.yeasts[0].attenuation_pct = Some(75.0);
        let stats_75 = calculate_stats(&recipe);

        assert!(stats_81.abv_pct > stats_75.abv_pct, "higher attenuation → higher ABV");
    }

    #[test]
    fn test_stats_efficiency_falls_back_to_default() {
        let mut recipe = minimal_recipe();
        recipe.fermentables = vec![pale_malt()];
        recipe.efficiency_pct = None;
        let stats_default = calculate_stats(&recipe);

        recipe.efficiency_pct = Some(72.0);
        let stats_explicit = calculate_stats(&recipe);

        assert!((stats_default.og - stats_explicit.og).abs() < 0.001);
    }

    #[test]
    fn test_stats_equipment_profile_used() {
        let mut recipe = minimal_recipe();
        recipe.fermentables = vec![pale_malt()];
        recipe.efficiency_pct = None;
        recipe.equipment_profile = Some(EquipmentProfile {
            id: "ep1".into(),
            name: "Test Equipment".into(),
            notes: None,
            boil_size_l: 27.0,
            batch_size_l: 23.0,
            calc_boil_volume: false,
            tun_volume_l: None,
            tun_weight_kg: None,
            tun_specific_heat: None,
            lauter_deadspace_l: 0.0,
            top_up_kettle_l: 0.0,
            trub_chiller_loss_l: 1.0,
            evap_rate_pct_hr: 10.0,
            boil_time_min: 60.0,
            top_up_water_l: 0.0,
            fermenter_loss_l: 1.0,
            hop_utilization_pct: 100.0,
            efficiency_pct: 80.0,
            created_at: 0,
            updated_at: 0,
        });
        let stats_with_equipment = calculate_stats(&recipe);

        recipe.efficiency_pct = Some(80.0);
        recipe.equipment_profile = None;
        let stats_explicit = calculate_stats(&recipe);

        assert!((stats_with_equipment.og - stats_explicit.og).abs() < 0.001);
    }
}
