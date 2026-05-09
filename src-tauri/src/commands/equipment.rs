use tauri::State;
use crate::AppState;
use crate::error::AppError;
use crate::models::{CreateEquipmentProfileInput, EquipmentProfile, UpdateEquipmentProfileInput};
use crate::repositories::equipment::EquipmentRepository;

#[tauri::command]
pub async fn list_equipment_profiles(state: State<'_, AppState>) -> Result<Vec<EquipmentProfile>, AppError> {
    EquipmentRepository::new(&state.db).list().await
}
#[tauri::command]
pub async fn create_equipment_profile(state: State<'_, AppState>, input: CreateEquipmentProfileInput) -> Result<EquipmentProfile, AppError> {
    EquipmentRepository::new(&state.db).create(input).await
}
#[tauri::command]
pub async fn update_equipment_profile(state: State<'_, AppState>, id: String, input: UpdateEquipmentProfileInput) -> Result<EquipmentProfile, AppError> {
    EquipmentRepository::new(&state.db).update(&id, input).await
}
#[tauri::command]
pub async fn delete_equipment_profile(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
    EquipmentRepository::new(&state.db).delete(&id).await
}
