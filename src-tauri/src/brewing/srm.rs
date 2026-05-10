pub fn morey_srm(
    fermentables: &[(&f64, &f64)], // (color_lovibond, amount_kg)
    batch_size_l: f64,
) -> f64 {
    if fermentables.is_empty() {
        return 0.0;
    }
    let batch_gallons = batch_size_l * 0.264172;
    let malt_color_units: f64 = fermentables
        .iter()
        .map(|(color_lovibond, amount_kg)| {
            let pounds = *amount_kg * 2.20462;
            (*color_lovibond * pounds) / batch_gallons
        })
        .sum();
    // Morey equation: SRM = 1.4922 × MCU^0.6859
    // Empirically fitted by Dan Morey to correct the linear MCU formula at high color values.
    1.4922 * malt_color_units.powf(0.6859)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_srm_pale_ale() {
        // 5 kg pale malt at 1.5 lovibond, 23L → light golden, ~3 SRM
        let fermentables = vec![(&1.5f64, &5.0f64)];
        let srm = morey_srm(&fermentables, 23.0);
        assert!(srm > 2.0 && srm < 5.0, "SRM was {srm:.2}, expected ~3");
    }

    #[test]
    fn test_srm_stout() {
        // Mix of pale + roasted for a stout should give dark color
        let fermentables = vec![
            (&3.5f64, &5.0f64),   // pale malt
            (&300.0f64, &0.5f64), // roasted barley
        ];
        let srm = morey_srm(&fermentables, 23.0);
        assert!(srm > 20.0, "SRM was {srm:.1}, expected dark (>20)");
    }

    #[test]
    fn test_srm_empty() {
        let srm = morey_srm(&[], 23.0);
        assert_eq!(srm, 0.0);
    }
}
