pub struct HopIbuInput<'a> {
    pub alpha_pct: &'a f64,
    pub amount_kg: &'a f64,
    pub time_min: &'a f64,
    pub use_type: &'a str,
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
            let effective_time = match use_lower.as_str() {
                "mash" | "dry hop" => return 0.0,
                "first wort" => boil_time_min,
                "hopstand" => {
                    if let Some(flat_util) = h.aroma_utilization_override {
                        let ounces = *h.amount_kg * 35.274;
                        let alpha_fraction = *h.alpha_pct / 100.0;
                        let volume_gallons = post_boil_volume_l * 0.264172;
                        return (flat_util * alpha_fraction * ounces * 7490.0) / volume_gallons;
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
            let utilization = bigness * time_factor;
            let ounces = *h.amount_kg * 35.274;
            let alpha_fraction = *h.alpha_pct / 100.0;
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
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        }];
        let hopstand_hops = vec![HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &20.0f64,
            use_type: "Hopstand",
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
            hopstand_temp_c: 100.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        }];
        let hopstand_hops = vec![HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &20.0f64,
            use_type: "Hopstand",
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
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        }];
        let boil_60 = vec![HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &60.0f64,
            use_type: "Boil",
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
}
