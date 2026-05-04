use tauri::State;
use crate::AppState;
use crate::models::{EquipmentProfile, CreateEquipmentProfileInput, UpdateEquipmentProfileInput};
use crate::db;

#[tauri::command]
pub async fn list_equipment_profiles(
    state: State<'_, AppState>,
) -> Result<Vec<EquipmentProfile>, String> {
    db::equipment::list(&state.db).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_equipment_profile(
    state: State<'_, AppState>,
    input: CreateEquipmentProfileInput,
) -> Result<EquipmentProfile, String> {
    db::equipment::create(&state.db, input).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_equipment_profile(
    state: State<'_, AppState>,
    id: String,
    input: UpdateEquipmentProfileInput,
) -> Result<EquipmentProfile, String> {
    db::equipment::update(&state.db, &id, input).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_equipment_profile(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    db::equipment::delete(&state.db, &id).await.map_err(|e| e.to_string())
}
