fn correction_factor(temp_f: f64) -> f64 {
    1.00130346 - (0.000134722124 * temp_f) + (0.00000204052596 * temp_f.powi(2))
        - (0.00000000232820948 * temp_f.powi(3))
}

pub fn correct_hydrometer_temp(
    measured_sg: f64,
    measured_temp_c: f64,
    calibration_temp_c: f64,
) -> f64 {
    let measured_temp_f = (measured_temp_c * 9.0 / 5.0) + 32.0;
    let calibration_temp_f = (calibration_temp_c * 9.0 / 5.0) + 32.0;

    measured_sg * (correction_factor(measured_temp_f) / correction_factor(calibration_temp_f))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warm_sample_corrects_upward() {
        let corrected = correct_hydrometer_temp(1.050, 30.0, 20.0);
        assert!(corrected > 1.050);
        assert!((corrected - 1.052).abs() < 0.002, "got {corrected:.4}");
    }

    #[test]
    fn calibration_temp_is_identity() {
        let corrected = correct_hydrometer_temp(1.050, 20.0, 20.0);
        assert!((corrected - 1.050).abs() < 0.00001);
    }
}
