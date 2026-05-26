use crate::error::AppError;
use crate::models::{
    CalculatedWaterProfile, CreateWaterAdjustmentInput, Recipe, RecipeWaterAdjustment,
    UpdateWaterAdjustmentInput,
};
use crate::repositories::water_chemistry::WaterChemistryRepository;
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn set_recipe_water_sources(
    state: State<'_, AppState>,
    recipe_id: String,
    mash_water_id: Option<String>,
    sparge_water_id: Option<String>,
) -> Result<Recipe, AppError> {
    WaterChemistryRepository::new(&state.db)
        .set_recipe_water_sources(&recipe_id, mash_water_id, sparge_water_id)
        .await
}

#[tauri::command]
pub async fn calculate_water_profile(
    state: State<'_, AppState>,
    recipe_id: String,
) -> Result<CalculatedWaterProfile, AppError> {
    WaterChemistryRepository::new(&state.db)
        .calculate_water_profile(&recipe_id)
        .await
}

#[tauri::command]
pub async fn create_water_adjustment(
    state: State<'_, AppState>,
    recipe_id: String,
    input: CreateWaterAdjustmentInput,
) -> Result<RecipeWaterAdjustment, AppError> {
    WaterChemistryRepository::new(&state.db)
        .create_water_adjustment(&recipe_id, input)
        .await
}

#[tauri::command]
pub async fn update_water_adjustment(
    state: State<'_, AppState>,
    id: String,
    input: UpdateWaterAdjustmentInput,
) -> Result<RecipeWaterAdjustment, AppError> {
    WaterChemistryRepository::new(&state.db)
        .update_water_adjustment(&id, input)
        .await
}

#[tauri::command]
pub async fn delete_water_adjustment(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), AppError> {
    WaterChemistryRepository::new(&state.db)
        .delete_water_adjustment(&id)
        .await
}
