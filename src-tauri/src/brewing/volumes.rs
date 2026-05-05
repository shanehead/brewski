/// Returns (pre_boil_volume_l, post_boil_volume_l)
pub fn calculate_boil_volumes(
    batch_size_l: f64,
    boil_time_min: f64,
    evap_rate_pct_hr: f64,
    trub_chiller_loss_l: f64,
    fermenter_loss_l: f64,
    top_up_water_l: f64,
) -> (f64, f64) {
    let post_boil_volume = batch_size_l + trub_chiller_loss_l + fermenter_loss_l - top_up_water_l;
    let boil_hours = boil_time_min / 60.0;
    let evaporation_fraction = evap_rate_pct_hr / 100.0 * boil_hours;
    let pre_boil_volume = post_boil_volume / (1.0 - evaporation_fraction);
    (pre_boil_volume, post_boil_volume)
}

pub fn calculate_pre_boil_gravity(og: f64, post_boil_volume_l: f64, pre_boil_volume_l: f64) -> f64 {
    let original_gravity_points = (og - 1.0) * 1000.0;
    let pre_boil_gravity_points = original_gravity_points * post_boil_volume_l / pre_boil_volume_l;
    1.0 + pre_boil_gravity_points / 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boil_volumes_standard() {
        // Batch 23L, 60 min boil, 10%/hr evap, 1L trub loss, 1L fermenter loss, 0 top-up
        let (pre, post) = calculate_boil_volumes(23.0, 60.0, 10.0, 1.0, 1.0, 0.0);
        // post_boil = batch + trub + fermenter - top_up = 23 + 1 + 1 = 25L
        assert!((post - 25.0).abs() < 0.5, "post_boil was {post:.2}L, expected ~25L");
        // pre_boil = post_boil / (1 - evap_rate * time_hr) = 25 / 0.9 ≈ 27.8L
        assert!((pre - 27.8).abs() < 0.5, "pre_boil was {pre:.2}L, expected ~27.8L");
    }

    #[test]
    fn test_pre_boil_gravity() {
        // OG 1.050, 25L post-boil, 27.8L pre-boil → pre-boil gravity lower
        let pbg = calculate_pre_boil_gravity(1.050, 25.0, 27.8);
        assert!(pbg < 1.050, "Pre-boil gravity {pbg:.4} should be less than OG 1.050");
        assert!((pbg - 1.045).abs() < 0.003, "pbg was {pbg:.4}, expected ~1.045");
    }
}
