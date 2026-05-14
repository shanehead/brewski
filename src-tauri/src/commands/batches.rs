use crate::error::AppError;
use crate::models::*;
use crate::repositories::batches::BatchRepository;
use crate::repositories::recipe_version::RecipeVersionRepository;
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn create_batch(
    state: State<'_, AppState>,
    input: CreateBatchInput,
) -> Result<Batch, AppError> {
    BatchRepository::new(&state.db).create(input).await
}

#[tauri::command]
pub async fn list_batches(state: State<'_, AppState>) -> Result<Vec<BatchSummary>, AppError> {
    BatchRepository::new(&state.db).list().await
}

#[tauri::command]
pub async fn list_batches_for_recipe(
    state: State<'_, AppState>,
    recipe_id: String,
) -> Result<Vec<BatchSummary>, AppError> {
    BatchRepository::new(&state.db)
        .list_for_recipe(&recipe_id)
        .await
}

#[tauri::command]
pub async fn get_batch(state: State<'_, AppState>, id: String) -> Result<Batch, AppError> {
    BatchRepository::new(&state.db).get(&id).await
}

#[tauri::command]
pub async fn update_batch(
    state: State<'_, AppState>,
    id: String,
    input: UpdateBatchInput,
) -> Result<Batch, AppError> {
    BatchRepository::new(&state.db).update(&id, input).await
}

#[tauri::command]
pub async fn delete_batch(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
    BatchRepository::new(&state.db).delete(&id).await
}

#[tauri::command]
pub async fn add_gravity_reading(
    state: State<'_, AppState>,
    batch_id: String,
    input: CreateGravityReadingInput,
) -> Result<GravityReading, AppError> {
    BatchRepository::new(&state.db)
        .add_gravity_reading(&batch_id, input)
        .await
}

#[tauri::command]
pub async fn delete_gravity_reading(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), AppError> {
    BatchRepository::new(&state.db)
        .delete_gravity_reading(&id)
        .await
}

#[tauri::command]
pub async fn list_recipe_versions(
    state: State<'_, AppState>,
    recipe_id: String,
) -> Result<Vec<RecipeVersionSummary>, AppError> {
    RecipeVersionRepository::new(&state.db)
        .list_for_recipe(&recipe_id)
        .await
}
