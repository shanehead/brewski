use tauri::State;
use crate::AppState;
use crate::models::{CreateEquipmentProfileInput, EquipmentProfile, UpdateEquipmentProfileInput};
use crate::repositories::equipment::EquipmentRepository;

#[tauri::command]
pub async fn list_equipment_profiles(state: State<'_, AppState>) -> Result<Vec<EquipmentProfile>, String> {
    EquipmentRepository::new(&state.db).list().await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn create_equipment_profile(state: State<'_, AppState>, input: CreateEquipmentProfileInput) -> Result<EquipmentProfile, String> {
    EquipmentRepository::new(&state.db).create(input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_equipment_profile(state: State<'_, AppState>, id: String, input: UpdateEquipmentProfileInput) -> Result<EquipmentProfile, String> {
    EquipmentRepository::new(&state.db).update(&id, input).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_equipment_profile(state: State<'_, AppState>, id: String) -> Result<(), String> {
    EquipmentRepository::new(&state.db).delete(&id).await.map_err(|e| e.to_string())
}
