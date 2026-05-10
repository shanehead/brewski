#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SugarType {
    TableSugar,
    CornSugar,
    DryMaltExtract,
}

impl SugarType {
    fn grams_per_liter_per_volume(self) -> f64 {
        match self {
            Self::TableSugar => 4.01,
            Self::CornSugar => 4.22,
            Self::DryMaltExtract => 5.07,
        }
    }
}

fn residual_co2_vols(temp_c: f64) -> f64 {
    let temp_f = (temp_c * 9.0 / 5.0) + 32.0;
    3.0378 - (0.050062 * temp_f) + (0.00026555 * temp_f.powi(2))
}

pub fn priming_sugar_grams(
    target_vols: f64,
    batch_size_l: f64,
    temp_c: f64,
    sugar_type: SugarType,
) -> f64 {
    let additional_vols = (target_vols - residual_co2_vols(temp_c)).max(0.0);
    additional_vols * batch_size_l * sugar_type.grams_per_liter_per_volume()
}

pub fn co2_pressure_kpa(target_vols: f64, temp_c: f64) -> f64 {
    let temp_f = (temp_c * 9.0 / 5.0) + 32.0;
    let psi = -16.6999 - (0.0101059 * temp_f)
        + (0.00116512 * temp_f.powi(2))
        + (0.173354 * temp_f * target_vols)
        + (4.24267 * target_vols)
        - (0.0684226 * target_vols.powi(2));

    psi.max(0.0) * 6.89476
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priming_sugar_changes_by_sugar_type() {
        let table = priming_sugar_grams(2.4, 19.0, 20.0, SugarType::TableSugar);
        let dme = priming_sugar_grams(2.4, 19.0, 20.0, SugarType::DryMaltExtract);
        assert!(table > 0.0);
        assert!(dme > table);
    }

    #[test]
    fn colder_beer_needs_more_pressure() {
        let cold = co2_pressure_kpa(2.4, 4.0);
        let warm = co2_pressure_kpa(2.4, 20.0);
        assert!(warm > cold);
    }
}
