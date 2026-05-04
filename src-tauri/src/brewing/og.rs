pub fn calculate_og(
    fermentables: &[(&f64, &f64, bool)],  // (yield_pct, amount_kg, add_after_boil)
    batch_size_l: f64,
    efficiency_pct: f64,
) -> f64 {
    let batch_gal = batch_size_l * 0.264172;
    let total_points: f64 = fermentables.iter().map(|(yield_pct, amount_kg, add_after_boil)| {
        let eff = if *add_after_boil { 100.0 } else { efficiency_pct };
        let lbs = *amount_kg * 2.20462;
        let ppg = *yield_pct / 100.0 * 46.0;
        lbs * ppg * (eff / 100.0)
    }).sum();
    1.0 + (total_points / batch_gal) / 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_og_pale_ale() {
        // 5 kg pale malt (75% yield), 23L batch, 75% efficiency
        let fermentables = vec![(&75.0f64, &5.0f64, false)];
        let og = calculate_og(&fermentables, 23.0, 75.0);
        // Expected ~1.047 (within ±0.002 tolerance)
        assert!((og - 1.047).abs() < 0.002, "OG was {og:.4}, expected ~1.047");
    }

    #[test]
    fn test_og_late_addition_gets_full_efficiency() {
        let normal = vec![(&75.0f64, &2.5f64, false)];
        let late = vec![(&75.0f64, &2.5f64, true)];
        let og_normal = calculate_og(&normal, 23.0, 75.0);
        let og_late = calculate_og(&late, 23.0, 75.0);
        // Late additions bypass mash efficiency, so og_late > og_normal
        assert!(og_late > og_normal);
    }

    #[test]
    fn test_og_empty_grain_bill() {
        let og = calculate_og(&[], 23.0, 75.0);
        assert_eq!(og, 1.0);
    }
}
