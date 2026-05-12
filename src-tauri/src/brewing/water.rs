/// Ion contribution constants (ppm per gram per US gallon)
/// Converted to ppm per gram per liter: divide by 3.785
pub struct IonContribution {
    pub calcium_ppm: f64,
    pub magnesium_ppm: f64,
    pub sodium_ppm: f64,
    pub chloride_ppm: f64,
    pub sulfate_ppm: f64,
    pub bicarbonate_ppm: f64,
}

impl IonContribution {
    /// Gypsum (CaSO₄·2H₂O)
    pub const GYPSUM: IonContribution = IonContribution {
        calcium_ppm: 61.5 / 3.785,
        magnesium_ppm: 0.0,
        sodium_ppm: 0.0,
        chloride_ppm: 0.0,
        sulfate_ppm: 147.4 / 3.785,
        bicarbonate_ppm: 0.0,
    };

    /// Calcium Chloride (CaCl₂·2H₂O)
    pub const CALCIUM_CHLORIDE: IonContribution = IonContribution {
        calcium_ppm: 72.0 / 3.785,
        magnesium_ppm: 0.0,
        sodium_ppm: 0.0,
        chloride_ppm: 127.5 / 3.785,
        sulfate_ppm: 0.0,
        bicarbonate_ppm: 0.0,
    };

    /// Epsom Salt (MgSO₄·7H₂O)
    pub const EPSOM_SALT: IonContribution = IonContribution {
        calcium_ppm: 0.0,
        magnesium_ppm: 26.1 / 3.785,
        sodium_ppm: 0.0,
        chloride_ppm: 0.0,
        sulfate_ppm: 103.0 / 3.785,
        bicarbonate_ppm: 0.0,
    };

    /// Table Salt (NaCl)
    pub const TABLE_SALT: IonContribution = IonContribution {
        calcium_ppm: 0.0,
        magnesium_ppm: 0.0,
        sodium_ppm: 104.0 / 3.785,
        chloride_ppm: 160.6 / 3.785,
        sulfate_ppm: 0.0,
        bicarbonate_ppm: 0.0,
    };

    /// Baking Soda (NaHCO₃)
    pub const BAKING_SODA: IonContribution = IonContribution {
        calcium_ppm: 0.0,
        magnesium_ppm: 0.0,
        sodium_ppm: 75.3 / 3.785,
        chloride_ppm: 0.0,
        sulfate_ppm: 0.0,
        bicarbonate_ppm: 190.7 / 3.785,
    };

    /// Chalk (CaCO₃)
    pub const CHALK: IonContribution = IonContribution {
        calcium_ppm: 105.9 / 3.785,
        magnesium_ppm: 0.0,
        sodium_ppm: 0.0,
        chloride_ppm: 0.0,
        sulfate_ppm: 0.0,
        bicarbonate_ppm: 158.4 / 3.785,
    };

    pub fn for_addition(addition: &str) -> Option<&'static IonContribution> {
        match addition {
            "gypsum" => Some(&Self::GYPSUM),
            "calcium_chloride" => Some(&Self::CALCIUM_CHLORIDE),
            "epsom_salt" => Some(&Self::EPSOM_SALT),
            "table_salt" => Some(&Self::TABLE_SALT),
            "baking_soda" => Some(&Self::BAKING_SODA),
            "chalk" => Some(&Self::CHALK),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gypsum_constants() {
        let g = &IonContribution::GYPSUM;
        assert!((g.calcium_ppm - 61.5 / 3.785).abs() < 0.01);
        assert!((g.sulfate_ppm - 147.4 / 3.785).abs() < 0.01);
    }

    #[test]
    fn test_for_addition() {
        assert!(IonContribution::for_addition("gypsum").is_some());
        assert!(IonContribution::for_addition("invalid").is_none());
    }
}
