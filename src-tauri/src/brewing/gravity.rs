use super::abv::og_to_plato;

pub fn sg_to_plato(sg: f64) -> f64 {
    og_to_plato(sg)
}

pub fn plato_to_sg(plato: f64) -> f64 {
    1.0 + (plato / (258.6 - ((plato / 258.2) * 227.1)))
}

pub fn sg_to_brix(sg: f64) -> f64 {
    (((182.4601 * sg) - 775.6821) * sg + 1262.7794) * sg - 669.5622
}

pub fn brix_to_sg_simple(brix: f64) -> f64 {
    1.0 + (brix / (258.6 - ((brix / 258.2) * 227.1)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sg_and_plato_round_trip() {
        let sg = 1.050;
        let round_trip = plato_to_sg(sg_to_plato(sg));
        assert!((round_trip - sg).abs() < 0.002, "got {round_trip:.4}");
    }

    #[test]
    fn sg_and_brix_round_trip() {
        let sg = 1.060;
        let round_trip = brix_to_sg_simple(sg_to_brix(sg));
        assert!((round_trip - sg).abs() < 0.002, "got {round_trip:.4}");
    }
}
