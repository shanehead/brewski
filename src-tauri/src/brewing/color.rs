pub fn srm_to_ebc(srm: f64) -> f64 {
    srm * 1.97
}

pub fn ebc_to_srm(ebc: f64) -> f64 {
    ebc / 1.97
}

pub fn srm_to_lovibond(srm: f64) -> f64 {
    (srm + 0.76) / 1.3546
}

pub fn lovibond_to_srm(lovibond: f64) -> f64 {
    (1.3546 * lovibond) - 0.76
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn srm_and_ebc_round_trip() {
        let round_trip = ebc_to_srm(srm_to_ebc(10.0));
        assert!((round_trip - 10.0).abs() < 0.0001);
    }

    #[test]
    fn srm_and_lovibond_round_trip() {
        let round_trip = lovibond_to_srm(srm_to_lovibond(12.0));
        assert!((round_trip - 12.0).abs() < 0.0001);
    }
}
