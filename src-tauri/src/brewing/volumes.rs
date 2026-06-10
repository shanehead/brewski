pub struct WaterVolumes {
    pub mash_water_l: f64,
    pub sparge_water_l: f64,
    pub total_water_l: f64,
    pub mash_volume_l: f64,
    pub mash_volume_excess_l: Option<f64>,
    pub effective_top_up_l: f64,
    pub top_up_overflow_l: Option<f64>,
}

/// Resolves all water volumes, including no-sparge tun overflow redistribution to top-up.
///
/// When a no-sparge mash exceeds the tun limit, the excess water is redirected to post-boil
/// top-up, which changes pre-boil volume. This function iterates once to converge on a
/// self-consistent result. Sparge-mode overflow is handled inside `calculate_water_volumes`.
#[allow(clippy::too_many_arguments)]
pub fn resolve_water_volumes(
    pre_boil_volume_l: f64,
    grain_absorption_rate_l_per_kg: f64,
    mash_tun_deadspace_l: f64,
    total_grain_kg: f64,
    mash_infuse_l: Option<f64>,
    mash_ratio_l_per_kg: Option<f64>,
    sparge_method: &str,
    tun_volume_l: Option<f64>,
    top_up_water_l: f64,
    batch_size_l: f64,
    boil_time_min: f64,
    evap_rate_l_hr: f64,
    trub_chiller_loss_l: f64,
    fermenter_loss_l: f64,
    mash_tun_loss_l: f64,
    hlt_deadspace_l: f64,
    is_kettle_mode: bool,
) -> WaterVolumes {
    let (mash_water_l, sparge_water_l, total_water_l, mash_volume_l, mash_volume_excess_l) =
        calculate_water_volumes(
            pre_boil_volume_l,
            grain_absorption_rate_l_per_kg,
            mash_tun_deadspace_l,
            total_grain_kg,
            mash_infuse_l,
            mash_ratio_l_per_kg,
            sparge_method,
            tun_volume_l,
        );

    if sparge_method == "no_sparge" && !is_kettle_mode {
        if let Some(excess) = mash_volume_excess_l {
            let adj_top_up = top_up_water_l + excess;
            let (adj_pre_boil, _, _) = calculate_boil_volumes(
                batch_size_l,
                boil_time_min,
                evap_rate_l_hr,
                trub_chiller_loss_l,
                fermenter_loss_l,
                adj_top_up,
                mash_tun_loss_l,
                hlt_deadspace_l,
            );
            let (adj_mash, adj_sparge, adj_total, adj_mash_vol, adj_excess) =
                calculate_water_volumes(
                    adj_pre_boil,
                    grain_absorption_rate_l_per_kg,
                    mash_tun_deadspace_l,
                    total_grain_kg,
                    mash_infuse_l,
                    mash_ratio_l_per_kg,
                    sparge_method,
                    tun_volume_l,
                );
            return WaterVolumes {
                mash_water_l: adj_mash,
                sparge_water_l: adj_sparge,
                total_water_l: adj_total,
                mash_volume_l: adj_mash_vol,
                mash_volume_excess_l: adj_excess,
                effective_top_up_l: adj_top_up,
                top_up_overflow_l: Some(excess),
            };
        }
    }

    WaterVolumes {
        mash_water_l,
        sparge_water_l,
        total_water_l,
        mash_volume_l,
        mash_volume_excess_l,
        effective_top_up_l: top_up_water_l,
        top_up_overflow_l: None,
    }
}

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

// Standard homebrew value: milled grain displaces ~0.67 L/kg in water
const GRAIN_DISPLACEMENT_L_PER_KG: f64 = 0.67;

/// Calculate water volumes for a recipe.
///
/// Returns `(mash_water_l, sparge_water_l, total_water_l, mash_volume_l, mash_volume_excess_l)`
///
/// - `total_water_l`: all water before fermentation (pre_boil + grain absorption + lauter loss)
/// - `mash_water_l`: water added to the grain bill
/// - `sparge_water_l`: rinse water (0 when sparge_method == "no_sparge" or no split defined)
/// - `mash_volume_l`: physical volume in the mash tun (water + grain displacement)
/// - `mash_volume_excess_l`: Some(litres over limit) for no-sparge overflow that must be
///   handled upstream; None for sparge modes (overflow is auto-redirected to sparge water)
#[allow(clippy::too_many_arguments)]
pub fn calculate_water_volumes(
    pre_boil_volume_l: f64,
    grain_absorption_rate_l_per_kg: f64,
    mash_tun_deadspace_l: f64,
    total_grain_kg: f64,
    mash_infuse_l: Option<f64>,
    mash_ratio_l_per_kg: Option<f64>,
    sparge_method: &str,
    tun_volume_l: Option<f64>,
) -> (f64, f64, f64, f64, Option<f64>) {
    // Lauter deadspace only applies when sparging; no-sparge systems drain fully into the kettle.
    let effective_deadspace = if sparge_method == "no_sparge" {
        0.0
    } else {
        mash_tun_deadspace_l
    };
    let total_water_l =
        pre_boil_volume_l + grain_absorption_rate_l_per_kg * total_grain_kg + effective_deadspace;

    let mash_water_l = if sparge_method == "no_sparge" || total_grain_kg <= 0.0 {
        total_water_l
    } else {
        let from_infuse_or_ratio =
            mash_infuse_l.or_else(|| mash_ratio_l_per_kg.map(|r| r * total_grain_kg));
        match from_infuse_or_ratio {
            Some(v) => v.min(total_water_l),
            None => total_water_l,
        }
    };

    // Sparge modes: cap mash at tun limit and redirect excess to sparge (total water unchanged).
    // No-sparge overflow cannot be fixed here — caller must redistribute to top-up.
    let (mash_water_l, sparge_water_l) = if sparge_method == "no_sparge" {
        (mash_water_l, 0.0)
    } else {
        let raw_sparge = (total_water_l - mash_water_l).max(0.0);
        if let Some(tun) = tun_volume_l {
            let max_mash = (tun - total_grain_kg * GRAIN_DISPLACEMENT_L_PER_KG).max(0.0);
            if mash_water_l > max_mash {
                (max_mash, (total_water_l - max_mash).max(0.0))
            } else {
                (mash_water_l, raw_sparge)
            }
        } else {
            (mash_water_l, raw_sparge)
        }
    };

    let mash_volume_l = mash_water_l + total_grain_kg * GRAIN_DISPLACEMENT_L_PER_KG;

    let mash_volume_excess_l = tun_volume_l.and_then(|tun| {
        let excess = mash_volume_l - tun;
        if excess > 0.0 {
            Some(excess)
        } else {
            None
        }
    });

    (
        mash_water_l,
        sparge_water_l,
        total_water_l,
        mash_volume_l,
        mash_volume_excess_l,
    )
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

#[cfg(test)]
mod water_volume_tests {
    use super::*;

    fn call(
        pre_boil: f64,
        absorption: f64,
        lauter: f64,
        grain_kg: f64,
        infuse: Option<f64>,
        ratio: Option<f64>,
        sparge_method: &str,
        tun: Option<f64>,
    ) -> (f64, f64, f64, f64, Option<f64>) {
        calculate_water_volumes(
            pre_boil,
            absorption,
            lauter,
            grain_kg,
            infuse,
            ratio,
            sparge_method,
            tun,
        )
    }

    #[test]
    fn no_sparge_all_water_is_mash() {
        // pre_boil=20L, absorption=1L/kg, lauter=0, grain=4kg → total=24L
        let (mash, sparge, total, _, _) = call(20.0, 1.0, 0.0, 4.0, None, None, "no_sparge", None);
        assert!((total - 24.0).abs() < 0.01, "total={total:.3}");
        assert!((mash - 24.0).abs() < 0.01, "mash={mash:.3}");
        assert!((sparge - 0.0).abs() < 0.01, "sparge={sparge:.3}");
    }

    #[test]
    fn sparge_with_infuse_amount_splits_correctly() {
        // total=24L, infuse=16L → sparge=8L
        let (mash, sparge, total, _, _) =
            call(20.0, 1.0, 0.0, 4.0, Some(16.0), None, "fly_sparge", None);
        assert!((total - 24.0).abs() < 0.01, "total={total:.3}");
        assert!((mash - 16.0).abs() < 0.01, "mash={mash:.3}");
        assert!((sparge - 8.0).abs() < 0.01, "sparge={sparge:.3}");
    }

    #[test]
    fn sparge_with_ratio_splits_correctly() {
        // ratio=3L/kg * 4kg = 12L mash; total=24L → sparge=12L
        let (mash, sparge, total, _, _) =
            call(20.0, 1.0, 0.0, 4.0, None, Some(3.0), "batch_sparge", None);
        assert!((total - 24.0).abs() < 0.01, "total={total:.3}");
        assert!((mash - 12.0).abs() < 0.01, "mash={mash:.3}");
        assert!((sparge - 12.0).abs() < 0.01, "sparge={sparge:.3}");
    }

    #[test]
    fn sparge_without_infuse_or_ratio_falls_back_to_no_sparge() {
        // No infuse, no ratio → mash=total, sparge=0
        let (_mash, sparge, _, _, _) = call(20.0, 1.0, 0.0, 4.0, None, None, "fly_sparge", None);
        assert!(
            (sparge - 0.0).abs() < 0.01,
            "sparge should be 0, got {sparge:.3}"
        );
    }

    #[test]
    fn mash_volume_includes_grain_displacement() {
        // mash=24L, grain=4kg → mash_volume = 24 + 4*0.67 = 26.68L
        let (mash, _, _, mash_vol, _) = call(20.0, 1.0, 0.0, 4.0, None, None, "no_sparge", None);
        let expected = mash + 4.0 * 0.67;
        assert!(
            (mash_vol - expected).abs() < 0.01,
            "mash_vol={mash_vol:.3}, expected={expected:.3}"
        );
    }

    #[test]
    fn tun_overflow_warning_when_exceeded() {
        // mash_vol ≈ 26.68L, tun=25L → excess ≈ 1.68L
        let (_, _, _, mash_vol, excess) =
            call(20.0, 1.0, 0.0, 4.0, None, None, "no_sparge", Some(25.0));
        assert!(excess.is_some(), "should have overflow");
        let e = excess.unwrap();
        assert!((e - (mash_vol - 25.0)).abs() < 0.01, "excess={e:.3}");
    }

    #[test]
    fn no_overflow_when_within_tun_volume() {
        let (_, _, _, _, excess) = call(20.0, 1.0, 0.0, 4.0, None, None, "no_sparge", Some(30.0));
        assert!(excess.is_none(), "should be None when under tun volume");
    }

    #[test]
    fn no_overflow_when_no_tun_volume_set() {
        let (_, _, _, _, excess) = call(20.0, 1.0, 0.0, 4.0, None, None, "no_sparge", None);
        assert!(excess.is_none(), "should be None when tun_volume_l is None");
    }

    #[test]
    fn no_sparge_ignores_mash_tun_deadspace() {
        // With deadspace=5L and no_sparge, total should equal pre_boil + absorption only
        let (_, _, total_with, _, _) = call(20.0, 1.0, 5.0, 4.0, None, None, "no_sparge", None);
        let (_, _, total_without, _, _) = call(20.0, 1.0, 0.0, 4.0, None, None, "no_sparge", None);
        assert!(
            (total_with - total_without).abs() < 0.01,
            "no_sparge should ignore deadspace: with={total_with:.3}, without={total_without:.3}"
        );
    }

    #[test]
    fn sparge_includes_mash_tun_deadspace_in_total() {
        // With deadspace=2L and fly_sparge, total should be 2L more than without
        let (_, _, total_with, _, _) =
            call(20.0, 1.0, 2.0, 4.0, None, Some(3.0), "fly_sparge", None);
        let (_, _, total_without, _, _) =
            call(20.0, 1.0, 0.0, 4.0, None, Some(3.0), "fly_sparge", None);
        assert!(
            (total_with - total_without - 2.0).abs() < 0.01,
            "sparge should include deadspace: with={total_with:.3}, without={total_without:.3}"
        );
    }

    #[test]
    fn zero_grain_falls_back_to_full_mash_water() {
        let (mash, sparge, total, _, _) = call(20.0, 1.0, 0.0, 0.0, None, None, "fly_sparge", None);
        assert!((total - 20.0).abs() < 0.01, "total={total:.3}");
        assert!((mash - 20.0).abs() < 0.01, "mash={mash:.3}");
        assert!((sparge - 0.0).abs() < 0.01, "sparge={sparge:.3}");
    }
}
