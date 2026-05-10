use crate::brewing::{
    abv,
    carbonation::{self, SugarType},
    color, gravity, hydro, pitch, refractometer,
};
use crate::error::AppError;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AbvCaloriesResult {
    abv_pct: f64,
    attenuation_pct: f64,
    calories_per_355ml: f64,
}

#[derive(serde::Serialize)]
pub struct RefractometerResult {
    sg: f64,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefractometerFgResult {
    fg_sg: f64,
}

#[derive(serde::Serialize)]
pub struct GravityConversionResult {
    sg: f64,
    plato: f64,
    brix: f64,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PitchRateResult {
    required_cells: f64,
    starter_volume_l: f64,
}

#[derive(serde::Serialize)]
pub struct ColorConversionResult {
    srm: f64,
    ebc: f64,
    lovibond: f64,
}

fn parse_sugar_type(sugar_type: &str) -> Result<SugarType, AppError> {
    match sugar_type {
        "table_sugar" => Ok(SugarType::TableSugar),
        "corn_sugar" => Ok(SugarType::CornSugar),
        "dry_malt_extract" => Ok(SugarType::DryMaltExtract),
        _ => Err(AppError::Conversion(format!(
            "unknown sugar type: {sugar_type}"
        ))),
    }
}

#[tauri::command]
pub async fn calculate_abv_calories(og: f64, fg: f64) -> Result<AbvCaloriesResult, AppError> {
    let attenuation_pct = if og > 1.0 {
        ((og - fg) / (og - 1.0)) * 100.0
    } else {
        0.0
    };

    Ok(AbvCaloriesResult {
        abv_pct: abv::calculate_abv(og, fg),
        attenuation_pct,
        calories_per_355ml: abv::calculate_calories_per_355ml(og, fg),
    })
}

#[tauri::command]
pub async fn correct_hydrometer_temp(
    measured_sg: f64,
    measured_temp_c: f64,
    calibration_temp_c: f64,
) -> Result<f64, AppError> {
    Ok(hydro::correct_hydrometer_temp(
        measured_sg,
        measured_temp_c,
        calibration_temp_c,
    ))
}

#[tauri::command]
pub async fn calculate_refractometer(
    brix: f64,
    wort_correction_factor: f64,
) -> Result<RefractometerResult, AppError> {
    Ok(RefractometerResult {
        sg: refractometer::brix_to_sg(brix, wort_correction_factor),
    })
}

#[tauri::command]
pub async fn correct_refractometer_fg(
    og_brix: f64,
    fg_brix: f64,
    wort_correction_factor: f64,
) -> Result<RefractometerFgResult, AppError> {
    Ok(RefractometerFgResult {
        fg_sg: refractometer::correct_refractometer_fg(og_brix, fg_brix, wort_correction_factor),
    })
}

#[tauri::command]
pub async fn calculate_priming_sugar(
    target_vols: f64,
    batch_size_l: f64,
    temp_c: f64,
    sugar_type: String,
) -> Result<f64, AppError> {
    Ok(carbonation::priming_sugar_grams(
        target_vols,
        batch_size_l,
        temp_c,
        parse_sugar_type(&sugar_type)?,
    ))
}

#[tauri::command]
pub async fn calculate_co2_pressure(target_vols: f64, temp_c: f64) -> Result<f64, AppError> {
    Ok(carbonation::co2_pressure_kpa(target_vols, temp_c))
}

#[tauri::command]
pub async fn convert_gravity(
    value: f64,
    from_unit: String,
) -> Result<GravityConversionResult, AppError> {
    let sg = match from_unit.as_str() {
        "sg" => value,
        "plato" => gravity::plato_to_sg(value),
        "brix" => gravity::brix_to_sg_simple(value),
        _ => {
            return Err(AppError::Conversion(format!(
                "unknown gravity unit: {from_unit}"
            )))
        }
    };

    Ok(GravityConversionResult {
        sg,
        plato: gravity::sg_to_plato(sg),
        brix: gravity::sg_to_brix(sg),
    })
}

#[tauri::command]
pub async fn calculate_pitch_rate(
    og: f64,
    batch_size_l: f64,
    pitch_rate: f64,
    yeast_pack_cells: f64,
    viability_pct: f64,
) -> Result<PitchRateResult, AppError> {
    let required_cells = pitch::required_cell_count(og, batch_size_l, pitch_rate);

    Ok(PitchRateResult {
        required_cells,
        starter_volume_l: pitch::starter_volume_l(required_cells, yeast_pack_cells, viability_pct),
    })
}

#[tauri::command]
pub async fn convert_color(
    value: f64,
    from_unit: String,
) -> Result<ColorConversionResult, AppError> {
    let srm = match from_unit.as_str() {
        "srm" => value,
        "ebc" => color::ebc_to_srm(value),
        "lovibond" => color::lovibond_to_srm(value),
        _ => {
            return Err(AppError::Conversion(format!(
                "unknown color unit: {from_unit}"
            )))
        }
    };

    Ok(ColorConversionResult {
        srm,
        ebc: color::srm_to_ebc(srm),
        lovibond: color::srm_to_lovibond(srm),
    })
}
