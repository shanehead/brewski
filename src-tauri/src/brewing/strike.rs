pub fn calculate_strike_temp(grain_temp_c: f64, target_temp_c: f64, ratio_l_per_kg: f64) -> f64 {
    (0.41 / ratio_l_per_kg) * (target_temp_c - grain_temp_c) + target_temp_c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strike_temp_reference_value() {
        // grain 20°C, target 67°C, ratio 3.0 L/kg → ~73.42°C
        // Formula: (0.41 / 3.0) * (67 - 20) + 67 = 0.1367 * 47 + 67 = 6.42 + 67 = 73.42
        let result = calculate_strike_temp(20.0, 67.0, 3.0);
        assert!((result - 73.42).abs() < 0.1, "expected ~73.42°C, got {result:.2}");
    }

    #[test]
    fn test_strike_temp_higher_ratio_needs_less_heating() {
        // More water per grain → less thermal mass adjustment → closer to target temp
        let low_ratio = calculate_strike_temp(20.0, 67.0, 2.0);
        let high_ratio = calculate_strike_temp(20.0, 67.0, 4.0);
        assert!(low_ratio > high_ratio, "lower ratio should require higher strike temp");
    }

    #[test]
    fn test_strike_temp_cold_grain_needs_hotter_strike() {
        let warm_grain = calculate_strike_temp(20.0, 67.0, 3.0);
        let cold_grain = calculate_strike_temp(10.0, 67.0, 3.0);
        assert!(cold_grain > warm_grain, "colder grain should require higher strike temp");
    }
}
