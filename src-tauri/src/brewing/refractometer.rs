pub fn brix_to_sg(brix: f64, wort_correction_factor: f64) -> f64 {
    let adjusted_brix = brix * wort_correction_factor;
    1.0 + (adjusted_brix / (258.6 - ((adjusted_brix / 258.2) * 227.1)))
}

pub fn correct_refractometer_fg(og_brix: f64, fg_brix: f64, wort_correction_factor: f64) -> f64 {
    let corrected_og = og_brix * wort_correction_factor;
    let corrected_fg = fg_brix * wort_correction_factor;

    1.0000 - 0.00085683 * corrected_og + 0.0034941 * corrected_fg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pre_fermentation_brix_to_sg_is_reasonable() {
        let sg = brix_to_sg(12.0, 1.04);
        assert!(sg > 1.045 && sg < 1.055, "got {sg:.4}");
    }

    #[test]
    fn post_fermentation_correction_lands_below_og() {
        let fg = correct_refractometer_fg(12.0, 6.0, 1.04);
        assert!(fg > 1.000 && fg < 1.020, "got {fg:.4}");
    }
}
