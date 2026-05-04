pub fn calculate_fg(og: f64, attenuation_pct: f64) -> f64 {
    1.0 + (og - 1.0) * (1.0 - attenuation_pct / 100.0)
}

pub fn calculate_abv(og: f64, fg: f64) -> f64 {
    (og - fg) * 131.25
}

// ASBC formula: kcal per 355 mL (12 oz)
pub fn calculate_calories_per_355ml(og: f64, fg: f64) -> f64 {
    let abw = (og - fg) * 105.0;
    let re = 0.1808 * og_to_plato(og) + 0.8192 * og_to_plato(fg);
    let cal_per_ml = (6.9 * abw + 4.0 * (re - 0.1)) * fg * 10.0 / 1000.0;
    cal_per_ml * 355.0
}

fn og_to_plato(sg: f64) -> f64 {
    (-1.0 * 616.868) + (1111.14 * sg) - (630.272 * sg * sg) + (135.997 * sg * sg * sg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fg_from_attenuation() {
        // OG 1.052, 75% attenuation → FG 1.013
        let fg = calculate_fg(1.052, 75.0);
        assert!((fg - 1.013).abs() < 0.001, "FG was {fg:.4}, expected ~1.013");
    }

    #[test]
    fn test_abv_standard() {
        // OG 1.052, FG 1.013 → ~5.1% ABV
        let abv = calculate_abv(1.052, 1.013);
        assert!((abv - 5.1).abs() < 0.2, "ABV was {abv:.2}, expected ~5.1");
    }

    #[test]
    fn test_calories_reasonable_range() {
        // 355ml of 5% beer should be ~150 kcal
        let cal = calculate_calories_per_355ml(1.052, 1.013);
        assert!(cal > 130.0 && cal < 175.0, "Calories was {cal:.1}");
    }
}
