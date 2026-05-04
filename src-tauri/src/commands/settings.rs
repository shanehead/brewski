use tauri::State;
use crate::AppState;
use crate::db;
use std::collections::HashMap;

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<HashMap<String, String>, String> {
    db::settings::get_all(&state.db).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_setting(state: State<'_, AppState>, key: String, value: String) -> Result<(), String> {
    db::settings::set(&state.db, &key, &value).await.map_err(|e| e.to_string())
}
