pub fn tinseth_ibu(
    hops: &[(&f64, &f64, &f64, bool)],  // (alpha_pct, amount_kg, time_min, is_dry_hop)
    og: f64,
    post_boil_volume_l: f64,
) -> f64 {
    let volume_gallons = post_boil_volume_l * 0.264172;
    hops.iter().map(|(alpha_pct, amount_kg, time_min, is_dry_hop)| {
        if *is_dry_hop || **time_min <= 0.0 {
            return 0.0;
        }
        let bigness = 1.65 * 0.000125f64.powf(og - 1.0);
        let time_factor = (1.0 - f64::exp(-0.04 * *time_min)) / 4.15;
        let utilization = bigness * time_factor;
        let ounces = *amount_kg * 35.274;
        let alpha_fraction = *alpha_pct / 100.0;
        (utilization * alpha_fraction * ounces * 7490.0) / volume_gallons
    }).sum()
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
}
