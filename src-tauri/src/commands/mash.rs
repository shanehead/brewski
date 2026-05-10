use crate::error::AppError;
use crate::models::{CreateMashStepInput, Mash, MashStep, UpdateMashInput, UpdateMashStepInput};
use crate::repositories::mash::MashRepository;
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_mash(state: State<'_, AppState>, recipe_id: String) -> Result<Mash, AppError> {
    MashRepository::new(&state.db)
        .get_for_recipe(&recipe_id)
        .await
}
#[tauri::command]
pub async fn update_mash(
    state: State<'_, AppState>,
    recipe_id: String,
    input: UpdateMashInput,
) -> Result<Mash, AppError> {
    MashRepository::new(&state.db)
        .upsert_for_recipe(&recipe_id, input)
        .await
}
#[tauri::command]
pub async fn create_mash_step(
    state: State<'_, AppState>,
    mash_id: String,
    input: CreateMashStepInput,
) -> Result<MashStep, AppError> {
    MashRepository::new(&state.db)
        .create_step(&mash_id, input)
        .await
}
#[tauri::command]
pub async fn update_mash_step(
    state: State<'_, AppState>,
    id: String,
    input: UpdateMashStepInput,
) -> Result<MashStep, AppError> {
    MashRepository::new(&state.db).update_step(&id, input).await
}
#[tauri::command]
pub async fn delete_mash_step(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
    MashRepository::new(&state.db).delete_step(&id).await
}
#[tauri::command]
pub async fn update_mash_step_order(
    state: State<'_, AppState>,
    ordered_ids: Vec<String>,
) -> Result<(), AppError> {
    MashRepository::new(&state.db)
        .update_step_order(ordered_ids)
        .await
}
