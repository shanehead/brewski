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
    hops: &[(&f64, &f64, &f64, bool)], // (alpha_pct, amount_kg, time_min, is_dry_hop)
    og: f64,
    post_boil_volume_l: f64,
) -> f64 {
    let volume_gallons = post_boil_volume_l * 0.264172;
    hops.iter()
        .map(|(alpha_pct, amount_kg, time_min, is_dry_hop)| {
            if *is_dry_hop || **time_min <= 0.0 {
                return 0.0;
            }
            // Tinseth bigness factor: accounts for wort gravity suppressing utilization.
            // Constants 1.65 and 0.000125 are empirically derived by Glenn Tinseth.
            let bigness = 1.65 * 0.000125f64.powf(og - 1.0);
            // Tinseth time factor: models the exponential approach to maximum utilization.
            // -0.04 is the time decay constant; 4.15 normalises to a 0–1 range.
            let time_factor = (1.0 - f64::exp(-0.04 * *time_min)) / 4.15;
            let utilization = bigness * time_factor;
            let ounces = *amount_kg * 35.274;
            let alpha_fraction = *alpha_pct / 100.0;
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
    fn test_ibu_single_addition() {
        // 28g (0.028 kg) of 10% AA hops, 60 min, OG 1.047, 23L → ~29 IBU
        let hops = vec![(&10.0f64, &0.028f64, &60.0f64, false)];
        let ibu = tinseth_ibu(&hops, 1.047, 23.0);
        assert!((ibu - 29.0).abs() < 3.0, "IBU was {ibu:.1}, expected ~29");
    }

    #[test]
    fn test_dry_hop_contributes_zero_ibu() {
        let dry_hop = vec![(&10.0f64, &0.028f64, &0.0f64, true)];
        let ibu = tinseth_ibu(&dry_hop, 1.047, 23.0);
        assert_eq!(ibu, 0.0);
    }

    #[test]
    fn test_ibu_zero_with_no_hops() {
        let ibu = tinseth_ibu(&[], 1.047, 23.0);
        assert_eq!(ibu, 0.0);
    }

    #[test]
    fn test_malowicki_at_boiling_returns_actual_time() {
        // At 100°C, k1 ratio = 1.0 so effective time equals actual time
        let effective = malowicki_effective_time(20.0, 100.0);
        assert!((effective - 20.0).abs() < 0.01, "got {effective}");
    }

    #[test]
    fn test_malowicki_at_80c_reduces_time() {
        // At 80°C the rate is ~16.6% of boiling, so 20 min → ~3.3 effective min
        let effective = malowicki_effective_time(20.0, 80.0);
        assert!(effective > 2.0 && effective < 5.0, "got {effective}");
    }

    #[test]
    fn test_malowicki_at_0c_is_near_zero() {
        let effective = malowicki_effective_time(60.0, 0.0);
        assert!(effective < 0.001, "got {effective}");
    }
}
