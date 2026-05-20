pub struct HopIbuInput<'a> {
    pub alpha_pct: &'a f64,
    pub amount_kg: &'a f64,
    pub time_min: &'a f64,
    pub use_type: &'a str,
    pub form: &'a str,
    /// Pre-resolved: per-hop override → recipe default → 80.0
    pub hopstand_temp_c: f64,
    /// Extra whirlpool time added to hopstand additions, in minutes
    pub whirlpool_time_min: f64,
    /// When Some, use this flat utilization fraction instead of the Malowicki model
    pub aroma_utilization_override: Option<f64>,
}

/// Malowicki & Shellhammer (2005) isomerization rate model.
/// Returns the boil-equivalent minutes for actual_min at temp_c.
/// k1(T) = 7.9e11 * exp(-11858 / T), T in Kelvin.
pub fn malowicki_effective_time(actual_min: f64, temp_c: f64) -> f64 {
    let t = temp_c + 273.15;
    let k1_t = 7.9e11_f64 * f64::exp(-11858.0 / t);
    let k1_boil = 7.9e11_f64 * f64::exp(-11858.0 / 373.15);
    actual_min * (k1_t / k1_boil)
}

fn form_utilization(form_lower: &str) -> f64 {
    match form_lower {
        "leaf" | "plug" => 0.85,
        _ => 1.0,
    }
}

pub fn tinseth_ibu(
    hops: &[HopIbuInput],
    og: f64,
    post_boil_volume_l: f64,
    boil_time_min: f64,
) -> f64 {
    let volume_gallons = post_boil_volume_l * 0.264172;
    // Tinseth bigness factor: accounts for wort gravity suppressing utilization.
    // Constants 1.65 and 0.000125 are empirically derived by Glenn Tinseth.
    let bigness = 1.65 * 0.000125f64.powf(og - 1.0);

    hops.iter()
        .map(|h| {
            let use_lower = h.use_type.to_lowercase();
            // Mash and dry hop never contribute IBUs regardless of form.
            if matches!(use_lower.as_str(), "mash" | "dry hop") {
                return 0.0;
            }
            let ounces = *h.amount_kg * 35.274;
            let alpha_fraction = *h.alpha_pct / 100.0;
            let form_lower = h.form.to_lowercase();
            // CO2 extract: fully isomerized — full utilization regardless of boil time.
            // Bigness still applies (gravity suppresses utilization even for extracts).
            if form_lower == "co2 extract" {
                return (bigness * alpha_fraction * ounces * 7490.0) / volume_gallons;
            }
            let effective_time = match use_lower.as_str() {
                "first wort" => boil_time_min,
                "hopstand" => {
                    if let Some(flat_util) = h.aroma_utilization_override {
                        return (flat_util
                            * form_utilization(&form_lower)
                            * alpha_fraction
                            * ounces
                            * 7490.0)
                            / volume_gallons;
                    }
                    malowicki_effective_time(*h.time_min + h.whirlpool_time_min, h.hopstand_temp_c)
                }
                _ => *h.time_min,
            };
            if effective_time <= 0.0 {
                return 0.0;
            }
            // Tinseth time factor: models the exponential approach to maximum utilization.
            // -0.04 is the time decay constant; 4.15 normalises to a 0–1 range.
            let time_factor = (1.0 - f64::exp(-0.04 * effective_time)) / 4.15;
            let utilization = bigness * time_factor * form_utilization(&form_lower);
            // 7490 converts (utilization × AAU × oz / gal) to IBUs.
            // Derived from Tinseth's original formula constants.
            (utilization * alpha_fraction * ounces * 7490.0) / volume_gallons
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_malowicki_at_boiling_returns_actual_time() {
        let effective = malowicki_effective_time(20.0, 100.0);
        assert!((effective - 20.0).abs() < 0.01, "got {effective}");
    }

    #[test]
    fn test_malowicki_at_80c_reduces_time() {
        let effective = malowicki_effective_time(20.0, 80.0);
        assert!(effective > 2.0 && effective < 5.0, "got {effective}");
    }

    #[test]
    fn test_malowicki_at_0c_is_near_zero() {
        let effective = malowicki_effective_time(60.0, 0.0);
        assert!(effective < 0.001, "got {effective}");
    }

    #[test]
    fn test_ibu_single_addition() {
        // 28g (0.028 kg) of 10% AA hops, 60 min, OG 1.047, 23L → ~29 IBU
        let hops = vec![HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &60.0f64,
            use_type: "Boil",
            form: "Pellet",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        }];
        let ibu = tinseth_ibu(&hops, 1.047, 23.0, 60.0);
        assert!((ibu - 29.0).abs() < 3.0, "IBU was {ibu:.1}, expected ~29");
    }

    #[test]
    fn test_dry_hop_contributes_zero_ibu() {
        let hops = vec![HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &0.0f64,
            use_type: "Dry Hop",
            form: "Pellet",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        }];
        let ibu = tinseth_ibu(&hops, 1.047, 23.0, 60.0);
        assert_eq!(ibu, 0.0);
    }

    #[test]
    fn test_ibu_zero_with_no_hops() {
        let ibu = tinseth_ibu(&[], 1.047, 23.0, 60.0);
        assert_eq!(ibu, 0.0);
    }

    #[test]
    fn test_mash_hop_contributes_zero_ibu() {
        let hops = vec![HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &60.0f64,
            use_type: "Mash",
            form: "Pellet",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        }];
        let ibu = tinseth_ibu(&hops, 1.047, 23.0, 60.0);
        assert_eq!(ibu, 0.0);
    }

    #[test]
    fn test_hopstand_contributes_less_than_boil() {
        let boil_hops = vec![HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &20.0f64,
            use_type: "Boil",
            form: "Pellet",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        }];
        let hopstand_hops = vec![HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &20.0f64,
            use_type: "Hopstand",
            form: "Pellet",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        }];
        let boil_ibu = tinseth_ibu(&boil_hops, 1.047, 23.0, 60.0);
        let hopstand_ibu = tinseth_ibu(&hopstand_hops, 1.047, 23.0, 60.0);
        assert!(
            hopstand_ibu < boil_ibu,
            "hopstand {hopstand_ibu} should be < boil {boil_ibu}"
        );
        assert!(hopstand_ibu > 0.0, "hopstand ibu should be > 0");
    }

    #[test]
    fn test_hopstand_at_boiling_equals_boil() {
        let boil_hops = vec![HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &20.0f64,
            use_type: "Boil",
            form: "Pellet",
            hopstand_temp_c: 100.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        }];
        let hopstand_hops = vec![HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &20.0f64,
            use_type: "Hopstand",
            form: "Pellet",
            hopstand_temp_c: 100.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        }];
        let boil_ibu = tinseth_ibu(&boil_hops, 1.047, 23.0, 60.0);
        let hopstand_ibu = tinseth_ibu(&hopstand_hops, 1.047, 23.0, 60.0);
        assert!(
            (hopstand_ibu - boil_ibu).abs() < 0.01,
            "at 100°C: hopstand={hopstand_ibu}, boil={boil_ibu}"
        );
    }

    #[test]
    fn test_first_wort_uses_boil_time() {
        let first_wort = vec![HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &0.0f64,
            use_type: "First Wort",
            form: "Pellet",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        }];
        let boil_60 = vec![HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &60.0f64,
            use_type: "Boil",
            form: "Pellet",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        }];
        let fw_ibu = tinseth_ibu(&first_wort, 1.047, 23.0, 60.0);
        let boil_ibu = tinseth_ibu(&boil_60, 1.047, 23.0, 60.0);
        assert!(
            (fw_ibu - boil_ibu).abs() < 0.01,
            "FWH={fw_ibu}, Boil60={boil_ibu}"
        );
    }

    #[test]
    fn test_leaf_hop_reduces_ibu_by_15_percent() {
        let pellet = vec![HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &60.0f64,
            use_type: "Boil",
            form: "Pellet",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        }];
        let leaf = vec![HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &60.0f64,
            use_type: "Boil",
            form: "Leaf",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        }];
        let pellet_ibu = tinseth_ibu(&pellet, 1.047, 23.0, 60.0);
        let leaf_ibu = tinseth_ibu(&leaf, 1.047, 23.0, 60.0);
        let ratio = leaf_ibu / pellet_ibu;
        assert!(
            (ratio - 0.85).abs() < 0.01,
            "leaf/pellet ratio {ratio:.4}, expected 0.85"
        );
    }

    #[test]
    fn test_plug_hop_reduces_ibu_by_15_percent() {
        let pellet = vec![HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &60.0f64,
            use_type: "Boil",
            form: "Pellet",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        }];
        let plug = vec![HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &60.0f64,
            use_type: "Boil",
            form: "Plug",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        }];
        let pellet_ibu = tinseth_ibu(&pellet, 1.047, 23.0, 60.0);
        let plug_ibu = tinseth_ibu(&plug, 1.047, 23.0, 60.0);
        let ratio = plug_ibu / pellet_ibu;
        assert!(
            (ratio - 0.85).abs() < 0.01,
            "plug/pellet ratio {ratio:.4}, expected 0.85"
        );
    }

    #[test]
    fn test_cryo_hop_same_ibu_as_pellet() {
        let pellet = vec![HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &60.0f64,
            use_type: "Boil",
            form: "Pellet",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        }];
        let cryo = vec![HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &60.0f64,
            use_type: "Boil",
            form: "Cryo",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        }];
        let pellet_ibu = tinseth_ibu(&pellet, 1.047, 23.0, 60.0);
        let cryo_ibu = tinseth_ibu(&cryo, 1.047, 23.0, 60.0);
        assert!(
            (cryo_ibu - pellet_ibu).abs() < 0.01,
            "cryo IBU {cryo_ibu:.2} should equal pellet IBU {pellet_ibu:.2}"
        );
    }

    #[test]
    fn test_co2_extract_ignores_boil_time() {
        let short = vec![HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &1.0f64,
            use_type: "Boil",
            form: "CO2 Extract",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        }];
        let long = vec![HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &60.0f64,
            use_type: "Boil",
            form: "CO2 Extract",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        }];
        let short_ibu = tinseth_ibu(&short, 1.047, 23.0, 60.0);
        let long_ibu = tinseth_ibu(&long, 1.047, 23.0, 60.0);
        assert!(
            (short_ibu - long_ibu).abs() < 0.01,
            "CO2 extract IBU should not depend on boil time: 1min={short_ibu:.2}, 60min={long_ibu:.2}"
        );
    }

    #[test]
    fn test_co2_extract_dry_hop_contributes_zero_ibu() {
        let hops = vec![HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &0.0f64,
            use_type: "Dry Hop",
            form: "CO2 Extract",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        }];
        let ibu = tinseth_ibu(&hops, 1.047, 23.0, 60.0);
        assert_eq!(ibu, 0.0);
    }

    #[test]
    fn test_leaf_hopstand_with_utilization_override_reduces_ibu() {
        let pellet = vec![HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &20.0f64,
            use_type: "Hopstand",
            form: "Pellet",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: Some(0.23),
        }];
        let leaf = vec![HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &20.0f64,
            use_type: "Hopstand",
            form: "Leaf",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: Some(0.23),
        }];
        let pellet_ibu = tinseth_ibu(&pellet, 1.047, 23.0, 60.0);
        let leaf_ibu = tinseth_ibu(&leaf, 1.047, 23.0, 60.0);
        let ratio = leaf_ibu / pellet_ibu;
        assert!(
            (ratio - 0.85).abs() < 0.01,
            "leaf/pellet hopstand override ratio {ratio:.4}, expected 0.85"
        );
    }

    #[test]
    fn test_co2_extract_higher_utilization_than_pellet_at_60min() {
        let co2 = vec![HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &1.0f64,
            use_type: "Boil",
            form: "CO2 Extract",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        }];
        let pellet = vec![HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &60.0f64,
            use_type: "Boil",
            form: "Pellet",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        }];
        let co2_ibu = tinseth_ibu(&co2, 1.047, 23.0, 60.0);
        let pellet_ibu = tinseth_ibu(&pellet, 1.047, 23.0, 60.0);
        assert!(
            co2_ibu > pellet_ibu,
            "CO2 extract (full utilization) {co2_ibu:.2} should exceed pellet 60min {pellet_ibu:.2}"
        );
    }
}
