use tauri::State;
use crate::AppState;
use crate::repositories::settings::SettingsRepository;
use std::collections::HashMap;

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<HashMap<String, String>, String> {
    SettingsRepository::new(&state.db).get_all().await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn update_setting(state: State<'_, AppState>, key: String, value: String) -> Result<(), String> {
    SettingsRepository::new(&state.db).set(&key, &value).await.map_err(|e| e.to_string())
}
