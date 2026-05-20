pub mod abv;
pub mod carbonation;
pub mod color;
pub mod gravity;
pub mod hydro;
pub mod ibu;
pub mod og;
pub mod pitch;
pub mod refractometer;
pub mod srm;
pub mod strike;
pub mod volumes;
pub mod water;

use crate::models::{Recipe, RecipeStats};

const DEFAULT_EFFICIENCY_PCT: f64 = 72.0;
const DEFAULT_ATTENUATION_PCT: f64 = 75.0;
const DEFAULT_HOPSTAND_TEMP_C: f64 = 80.0;
const DEFAULT_EVAP_RATE_PCT_HR: f64 = 10.0;
const DEFAULT_TRUB_CHILLER_LOSS_L: f64 = 1.0;
const DEFAULT_FERMENTER_LOSS_L: f64 = 1.0;
const DEFAULT_TOP_UP_WATER_L: f64 = 0.0;

const DEFAULT_MASH_TUN_LOSS_L: f64 = 0.0;
const DEFAULT_HLT_DEADSPACE_L: f64 = 0.0;
const DEFAULT_COOLING_SHRINKAGE_PCT: f64 = 4.0;

pub fn calculate_stats(recipe: &Recipe) -> RecipeStats {
    let equipment = recipe.equipment_profile.as_ref();

    let efficiency = recipe
        .efficiency_pct
        .or_else(|| equipment.map(|e| e.efficiency_pct))
        .unwrap_or(DEFAULT_EFFICIENCY_PCT);

    let evaporation_rate = equipment
        .map(|e| e.evap_rate_pct_hr)
        .unwrap_or(DEFAULT_EVAP_RATE_PCT_HR);
    let trub_chiller_loss = equipment
        .map(|e| e.trub_chiller_loss_l)
        .unwrap_or(DEFAULT_TRUB_CHILLER_LOSS_L);
    let fermenter_loss = equipment
        .map(|e| e.fermenter_loss_l)
        .unwrap_or(DEFAULT_FERMENTER_LOSS_L);
    let top_up_water = equipment
        .map(|e| e.top_up_water_l)
        .unwrap_or(DEFAULT_TOP_UP_WATER_L);
    let mash_tun_loss = equipment
        .map(|e| e.mash_tun_loss_l)
        .unwrap_or(DEFAULT_MASH_TUN_LOSS_L);
    let hlt_deadspace = equipment
        .and_then(|e| e.hlt_deadspace_l)
        .unwrap_or(DEFAULT_HLT_DEADSPACE_L);
    let _cooling_shrinkage = equipment
        .map(|e| e.cooling_shrinkage_pct)
        .unwrap_or(DEFAULT_COOLING_SHRINKAGE_PCT);
    let aroma_hop_utilization_override: Option<f64> = equipment.and_then(|e| {
        if e.calc_aroma_hop_utilization {
            None
        } else {
            Some(e.aroma_hop_utilization_pct / 100.0)
        }
    });
    let whirlpool_time = equipment.and_then(|e| e.whirlpool_time_min).unwrap_or(0.0);
    let batch_volume_target = equipment
        .map(|e| e.batch_volume_target.as_str())
        .unwrap_or("fermenter");

    // When batch_volume_target = "kettle", batch_size_l is the post-boil kettle volume.
    // Fermenter volume is derived by subtracting losses from the kettle.
    // When "fermenter" (default), batch_size_l is the fermenter target and volumes derive normally.
    let (pre_boil_volume_l, post_boil_volume_l, fermenter_volume_l) = if batch_volume_target
        == "kettle"
    {
        let post_boil = recipe.batch_size_l;
        let boil_hours = recipe.boil_time_min / 60.0;
        let evap_fraction = evaporation_rate / 100.0 * boil_hours;
        let pre_boil = post_boil / (1.0 - evap_fraction) + mash_tun_loss;
        let fermenter = (post_boil - trub_chiller_loss - fermenter_loss + top_up_water).max(0.0);
        (pre_boil, post_boil, fermenter)
    } else {
        let (pre, post, _total) = volumes::calculate_boil_volumes(
            recipe.batch_size_l,
            recipe.boil_time_min,
            evaporation_rate,
            trub_chiller_loss,
            fermenter_loss,
            top_up_water,
            mash_tun_loss,
            hlt_deadspace,
        );
        (pre, post, recipe.batch_size_l)
    };

    let fermentable_inputs: Vec<(&f64, &f64, bool)> = recipe
        .fermentables
        .iter()
        .map(|f| (&f.yield_pct, &f.amount_kg, f.add_after_boil))
        .collect();

    let og = og::calculate_og(&fermentable_inputs, fermenter_volume_l, efficiency);

    let fg = recipe
        .yeasts
        .iter()
        .filter_map(|y| y.attenuation_pct)
        .next()
        .map(|attenuation| abv::calculate_fg(og, attenuation))
        .unwrap_or_else(|| abv::calculate_fg(og, DEFAULT_ATTENUATION_PCT));

    let abv_pct = abv::calculate_abv(og, fg);
    let calories = abv::calculate_calories_per_355ml(og, fg);

    let pre_boil_gravity =
        volumes::calculate_pre_boil_gravity(og, post_boil_volume_l, pre_boil_volume_l);

    let hopstand_default = recipe.hopstand_temp_c.unwrap_or(DEFAULT_HOPSTAND_TEMP_C);
    let hop_inputs: Vec<ibu::HopIbuInput> = recipe
        .hops
        .iter()
        .map(|h| ibu::HopIbuInput {
            alpha_pct: &h.alpha_pct,
            amount_kg: &h.amount_kg,
            time_min: &h.time_min,
            use_type: &h.use_,
            form: &h.form,
            hopstand_temp_c: h.hopstand_temp_c.unwrap_or(hopstand_default),
            whirlpool_time_min: whirlpool_time,
            aroma_utilization_override: aroma_hop_utilization_override,
        })
        .collect();

    let ibu = ibu::tinseth_ibu(&hop_inputs, og, post_boil_volume_l, recipe.boil_time_min);

    let srm_inputs: Vec<(&f64, &f64)> = recipe
        .fermentables
        .iter()
        .map(|f| (&f.color_lovibond, &f.amount_kg))
        .collect();

    let srm = srm::morey_srm(&srm_inputs, fermenter_volume_l);

    let gravity_units = (og - 1.0) * 1000.0;
    let bu_gu_ratio = if gravity_units > 0.0 {
        ibu / gravity_units
    } else {
        0.0
    };

    let strike_temp_c = recipe.mash.as_ref().and_then(|mash| {
        let grain_temp_c = mash.grain_temp_c;
        let target_temp_c = mash.steps.first()?.step_temp_c;
        let total_grain_kg: f64 = recipe.fermentables.iter().map(|f| f.amount_kg).sum();
        let derived_ratio = if total_grain_kg > 0.0 {
            mash.steps
                .iter()
                .find_map(|s| s.infuse_amount_l.map(|vol| vol / total_grain_kg))
        } else {
            None
        };
        let ratio = derived_ratio.or(mash.ratio_l_per_kg)?;
        Some(strike::calculate_strike_temp(
            grain_temp_c,
            target_temp_c,
            ratio,
        ))
    });

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
        strike_temp_c,
    }
}

#[cfg(test)]
mod beerxml_fixture;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brewing::beerxml_fixture::load_fixture;
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
            water_adjustments: vec![],
            mash_water_id: None,
            sparge_water_id: None,
            hopstand_temp_c: None,
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
            hopstand_temp_c: None,
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

        assert!(
            stats_81.abv_pct > stats_75.abv_pct,
            "higher attenuation → higher ABV"
        );
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

    fn mash_with_infusion(
        grain_temp_c: f64,
        step_temp_c: f64,
        infuse_amount_l: f64,
    ) -> crate::models::Mash {
        crate::models::Mash {
            id: "m1".into(),
            recipe_id: "r1".into(),
            name: "Single Infusion".into(),
            grain_temp_c,
            tun_temp_c: None,
            sparge_temp_c: None,
            ph: None,
            tun_weight_kg: None,
            tun_specific_heat: None,
            equip_adjust: false,
            ratio_l_per_kg: None,
            notes: None,
            steps: vec![crate::models::MashStep {
                id: "s1".into(),
                mash_id: "m1".into(),
                name: "Mash In".into(),
                type_: "infusion".into(),
                infuse_amount_l: Some(infuse_amount_l),
                step_temp_c,
                step_time_min: 60,
                ramp_time_min: None,
                end_temp_c: None,
                step_order: 0,
            }],
        }
    }

    #[test]
    fn test_strike_temp_derived_from_infuse_amount() {
        let mut recipe = minimal_recipe();
        recipe.fermentables = vec![pale_malt()]; // pale_malt() has amount_kg: 4.5
                                                 // ratio = 15.0 L / 4.5 kg = 3.333 L/kg
                                                 // strike = (0.41/3.333)*(67-20)+67 = 0.123*47+67 = 5.78+67 = 72.78°C
        recipe.mash = Some(mash_with_infusion(20.0, 67.0, 15.0));
        let stats = calculate_stats(&recipe);
        let strike = stats.strike_temp_c.expect("strike_temp_c should be Some");
        assert!(
            (strike - 72.78).abs() < 0.5,
            "expected ~72.78°C, got {strike:.2}"
        );
    }

    #[test]
    fn test_strike_temp_none_without_mash() {
        let recipe = minimal_recipe();
        let stats = calculate_stats(&recipe);
        assert!(stats.strike_temp_c.is_none());
    }

    #[test]
    fn test_strike_temp_fallback_to_stored_ratio() {
        let mut recipe = minimal_recipe();
        recipe.fermentables = vec![pale_malt()];
        let mut mash = mash_with_infusion(20.0, 67.0, 15.0);
        // Remove infuse amount from the step so auto-derive fails
        mash.steps[0].infuse_amount_l = None;
        // Set stored fallback ratio
        mash.ratio_l_per_kg = Some(3.333);
        recipe.mash = Some(mash);
        let stats = calculate_stats(&recipe);
        let strike = stats
            .strike_temp_c
            .expect("should fall back to stored ratio");
        assert!(
            (strike - 72.78).abs() < 0.5,
            "expected ~72.78°C, got {strike:.2}"
        );
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
            batch_volume_target: "fermenter".into(),
            mash_tun_loss_l: 0.0,
            hlt_deadspace_l: None,
            cooling_shrinkage_pct: 4.0,
            calc_mash_efficiency: true,
            mash_efficiency_pct: None,
            calc_aroma_hop_utilization: true,
            aroma_hop_utilization_pct: 23.0,
            whirlpool_time_min: None,
            altitude_adjustment: false,
            boil_temp_f: None,
            sparge_method: "no_sparge".into(),
            mash_volume_min_l: None,
            mash_volume_max_l: None,
            sparge_volume_min_l: None,
            sparge_volume_max_l: None,
            calc_strike_water_temp: false,
            created_at: 0,
            updated_at: 0,
        });
        let stats_with_equipment = calculate_stats(&recipe);

        recipe.efficiency_pct = Some(80.0);
        recipe.equipment_profile = None;
        let stats_explicit = calculate_stats(&recipe);

        assert!((stats_with_equipment.og - stats_explicit.og).abs() < 0.001);
    }

    fn equipment_profile_base() -> EquipmentProfile {
        EquipmentProfile {
            id: "eq1".into(),
            name: "Test".into(),
            notes: None,
            boil_size_l: 27.0,
            batch_size_l: 23.0,
            calc_boil_volume: true,
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
            efficiency_pct: 72.0,
            batch_volume_target: "fermenter".into(),
            mash_tun_loss_l: 0.0,
            hlt_deadspace_l: None,
            cooling_shrinkage_pct: 4.0,
            calc_mash_efficiency: true,
            mash_efficiency_pct: None,
            calc_aroma_hop_utilization: true,
            aroma_hop_utilization_pct: 23.0,
            whirlpool_time_min: None,
            altitude_adjustment: false,
            boil_temp_f: None,
            sparge_method: "no_sparge".into(),
            mash_volume_min_l: None,
            mash_volume_max_l: None,
            sparge_volume_min_l: None,
            sparge_volume_max_l: None,
            calc_strike_water_temp: false,
            created_at: 0,
            updated_at: 0,
        }
    }

    #[test]
    fn test_kettle_mode_og_matches_fermenter_mode_at_fermenter_volume() {
        // Kettle mode: batch_size_l=23 is post-boil kettle; fermenter gets 23-1(trub)-1(fermenter_loss)=21L
        // Fermenter mode: batch_size_l=21 targets the fermenter directly
        // Both should yield the same OG for the same grain bill.
        let mut recipe = minimal_recipe();
        recipe.fermentables = vec![pale_malt()];
        recipe.efficiency_pct = None;

        let mut eq_kettle = equipment_profile_base();
        eq_kettle.batch_volume_target = "kettle".into();
        eq_kettle.batch_size_l = 23.0; // post-boil kettle target
        recipe.batch_size_l = 23.0;
        recipe.equipment_profile = Some(eq_kettle);
        let stats_kettle = calculate_stats(&recipe);

        let mut eq_fermenter = equipment_profile_base();
        eq_fermenter.batch_volume_target = "fermenter".into();
        eq_fermenter.batch_size_l = 21.0; // 23 - 1 trub - 1 fermenter_loss = 21
        recipe.batch_size_l = 21.0;
        recipe.equipment_profile = Some(eq_fermenter);
        let stats_fermenter = calculate_stats(&recipe);

        assert!(
            (stats_kettle.og - stats_fermenter.og).abs() < 0.001,
            "kettle mode OG should match fermenter mode at fermenter volume: kettle={:.4}, fermenter={:.4}",
            stats_kettle.og,
            stats_fermenter.og
        );
    }

    #[test]
    fn test_kettle_mode_post_boil_equals_batch_size() {
        let mut recipe = minimal_recipe();
        recipe.fermentables = vec![pale_malt()];
        recipe.efficiency_pct = None;

        let mut eq = equipment_profile_base();
        eq.batch_volume_target = "kettle".into();
        eq.batch_size_l = 23.0;
        recipe.batch_size_l = 23.0;
        recipe.equipment_profile = Some(eq);
        let stats = calculate_stats(&recipe);

        assert!(
            (stats.post_boil_volume_l - 23.0).abs() < 0.01,
            "kettle mode: post_boil should equal batch_size_l=23, got {:.2}",
            stats.post_boil_volume_l
        );
        assert!(
            stats.pre_boil_volume_l > 23.0,
            "kettle mode: pre_boil should exceed batch_size_l due to evaporation, got {:.2}",
            stats.pre_boil_volume_l
        );
    }

    // --- fixture-based stats tests ---

    const OG_TOL: f64 = 0.003;
    const FG_TOL: f64 = 0.005;
    const IBU_TOL: f64 = 5.0;
    const SRM_TOL: f64 = 1.5;

    fn assert_within(label: &str, actual: f64, expected: f64, tol: f64) {
        assert!(
            (actual - expected).abs() <= tol,
            "{} expected ~{expected:.3}, got {actual:.3}",
            label
        );
    }

    macro_rules! fixture_test {
        ($fn:ident, $file:literal) => {
            #[test]
            fn $fn() {
                let (recipe, expected) = load_fixture($file);
                let stats = calculate_stats(&recipe);
                assert_within("OG", stats.og, expected.og, OG_TOL);
                assert_within("FG", stats.fg, expected.fg, FG_TOL);
                assert_within("IBU", stats.ibu, expected.ibu, IBU_TOL);
                assert_within("SRM", stats.srm, expected.srm, SRM_TOL);
            }
        };
    }

    fixture_test!(test_stats_punk_ipa_2007, "punk_ipa_2007.xml");
    fixture_test!(test_stats_alpha_dog, "alpha_dog.xml");
    fixture_test!(test_stats_jet_black_heart, "jet_black_heart.xml");
    fixture_test!(test_stats_nectaron_hazy_dipa, "nectaron_hazy_dipa.xml");
    fixture_test!(test_stats_american_pale_ale, "american_pale_ale.xml");
    fixture_test!(test_stats_english_bitter, "english_bitter.xml");
    fixture_test!(test_stats_oatmeal_stout, "oatmeal_stout.xml");
    fixture_test!(test_stats_german_hefeweizen, "german_hefeweizen.xml");
    fixture_test!(test_stats_belgian_tripel, "belgian_tripel.xml");
    fixture_test!(test_stats_american_amber_ale, "american_amber_ale.xml");
    fixture_test!(test_stats_irish_stout, "irish_stout.xml");
    fixture_test!(test_stats_american_barleywine, "american_barleywine.xml");
    fixture_test!(test_stats_session_ipa, "session_ipa.xml");
    fixture_test!(test_stats_english_porter, "english_porter.xml");
    fixture_test!(test_stats_czech_pilsner, "czech_pilsner.xml");
    fixture_test!(test_stats_scottish_80_shilling, "scottish_80_shilling.xml");
    fixture_test!(test_stats_saison, "saison.xml");
    fixture_test!(test_stats_dunkelweizen, "dunkelweizen.xml");
    fixture_test!(test_stats_neipa, "neipa.xml");
    fixture_test!(test_stats_marzen_oktoberfest, "marzen_oktoberfest.xml");
    fixture_test!(test_stats_belgian_witbier, "belgian_witbier.xml");
    fixture_test!(test_stats_double_ipa, "double_ipa.xml");
    fixture_test!(test_stats_american_brown_ale, "american_brown_ale.xml");
    fixture_test!(test_stats_imperial_stout, "imperial_stout.xml");
    fixture_test!(test_stats_american_wheat, "american_wheat.xml");
    fixture_test!(test_stats_blonde_ale, "blonde_ale.xml");
}
