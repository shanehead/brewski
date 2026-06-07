/// Returns (pre_boil_volume_l, post_boil_volume_l, total_water_needed_l)
/// - pre_boil: cold volume needed in the kettle at start of boil
/// - post_boil: cold volume in kettle after boil
/// - total_water: pre_boil + hlt_deadspace (all water you need to start with)
#[allow(clippy::too_many_arguments)]
pub fn calculate_boil_volumes(
    batch_size_l: f64,
    boil_time_min: f64,
    evap_rate_l_hr: f64,
    trub_chiller_loss_l: f64,
    fermenter_loss_l: f64,
    top_up_water_l: f64,
    mash_tun_loss_l: f64,
    hlt_deadspace_l: f64,
) -> (f64, f64, f64) {
    let post_boil_volume = batch_size_l + trub_chiller_loss_l + fermenter_loss_l - top_up_water_l;
    let boil_hours = boil_time_min / 60.0;
    let pre_boil_volume = post_boil_volume + evap_rate_l_hr * boil_hours + mash_tun_loss_l;
    let total_water_needed = pre_boil_volume + hlt_deadspace_l;
    (pre_boil_volume, post_boil_volume, total_water_needed)
}

/// Convert a cold (room temperature) volume to the equivalent hot (boiling) volume.
/// Wort expands by cooling_shrinkage_pct when heated to boiling.
pub fn hot_volume(cold_volume_l: f64, cooling_shrinkage_pct: f64) -> f64 {
    cold_volume_l * (1.0 + cooling_shrinkage_pct / 100.0)
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
        // Batch 23L, 60 min boil, 2.5 L/hr evap (≈10%/hr on ~25L), 1L trub, 1L fermenter loss, 0 top-up
        let (pre, post, _total) = calculate_boil_volumes(23.0, 60.0, 2.5, 1.0, 1.0, 0.0, 0.0, 0.0);
        // post_boil = 23 + 1 + 1 = 25L
        assert!(
            (post - 25.0).abs() < 0.5,
            "post_boil was {post:.2}L, expected ~25L"
        );
        // pre_boil = 25 + 2.5 * 1hr = 27.5L
        assert!(
            (pre - 27.5).abs() < 0.5,
            "pre_boil was {pre:.2}L, expected ~27.5L"
        );
    }

    #[test]
    fn test_mash_tun_loss_increases_pre_boil() {
        let (pre_no_loss, _, _) = calculate_boil_volumes(23.0, 60.0, 2.5, 1.0, 1.0, 0.0, 0.0, 0.0);
        let (pre_with_loss, _, _) =
            calculate_boil_volumes(23.0, 60.0, 2.5, 1.0, 1.0, 0.0, 2.0, 0.0);
        assert!(
            (pre_with_loss - pre_no_loss - 2.0).abs() < 0.1,
            "mash_tun_loss should add ~2L to pre_boil: no_loss={pre_no_loss:.2}, with_loss={pre_with_loss:.2}"
        );
    }

    #[test]
    fn test_hlt_deadspace_returned_in_total_water() {
        let (pre, _post, total) = calculate_boil_volumes(23.0, 60.0, 2.5, 1.0, 1.0, 0.0, 0.0, 2.0);
        assert!(
            (total - pre - 2.0).abs() < 0.1,
            "total_water should be pre_boil + hlt_deadspace: pre={pre:.2}, total={total:.2}"
        );
    }

    #[test]
    fn test_hot_volume_conversion() {
        let cold = 25.0_f64;
        let hot = hot_volume(cold, 4.0);
        assert!((hot - 26.0).abs() < 0.1, "hot={hot:.2}, expected ~26.0");
    }

    #[test]
    fn test_pre_boil_gravity() {
        let pbg = calculate_pre_boil_gravity(1.050, 25.0, 27.5);
        assert!(pbg < 1.050);
        assert!((pbg - 1.045).abs() < 0.003);
    }
}
