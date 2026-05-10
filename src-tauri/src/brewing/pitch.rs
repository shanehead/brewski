use super::gravity::sg_to_plato;

pub fn required_cell_count(og: f64, batch_size_l: f64, pitch_rate_m_per_ml_per_plato: f64) -> f64 {
    let batch_size_ml = batch_size_l * 1000.0;
    let plato = sg_to_plato(og);
    (pitch_rate_m_per_ml_per_plato * batch_size_ml * plato) / 1000.0
}

pub fn starter_volume_l(required_cells: f64, yeast_pack_cells: f64, viability_pct: f64) -> f64 {
    let available_cells = yeast_pack_cells * (viability_pct / 100.0);
    let additional_cells_needed = (required_cells - available_cells).max(0.0);
    additional_cells_needed / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_cell_count_for_standard_ale_is_reasonable() {
        let required = required_cell_count(1.050, 20.0, 0.75);
        assert!(required > 150.0 && required < 250.0, "got {required:.1}");
    }

    #[test]
    fn starter_needed_only_when_pack_is_short() {
        let starter = starter_volume_l(200.0, 100.0, 75.0);
        let no_starter = starter_volume_l(200.0, 300.0, 100.0);
        assert!(starter > 0.0);
        assert_eq!(no_starter, 0.0);
    }
}
