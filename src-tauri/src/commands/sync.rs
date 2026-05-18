use crate::error::AppError;
use serde::Serialize;
use std::path::PathBuf;
use tauri::Manager;

#[derive(Debug, Serialize, Clone)]
pub struct SyncFolder {
    pub name: String,
    pub path: String,
}

pub fn find_sync_folders_with_home(home: &std::path::Path) -> Vec<SyncFolder> {
    let mut folders = Vec::new();

    // Google Drive
    let gd = home.join("Google Drive").join("My Drive");
    if gd.exists() {
        folders.push(SyncFolder {
            name: "Google Drive".into(),
            path: gd.join("Brewski").to_string_lossy().into_owned(),
        });
    }

    // Dropbox — path is inside ~/.dropbox/info.json
    let dropbox_info = home.join(".dropbox").join("info.json");
    if let Ok(contents) = std::fs::read_to_string(&dropbox_info) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&contents) {
            let path_str = json["personal"]["path"]
                .as_str()
                .or_else(|| json["business"]["path"].as_str());
            if let Some(p) = path_str {
                let root = PathBuf::from(p);
                if root.exists() {
                    folders.push(SyncFolder {
                        name: "Dropbox".into(),
                        path: root.join("Brewski").to_string_lossy().into_owned(),
                    });
                }
            }
        }
    }

    // OneDrive (macOS + Windows only)
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    {
        let od = home.join("OneDrive");
        if od.exists() {
            folders.push(SyncFolder {
                name: "OneDrive".into(),
                path: od.join("Brewski").to_string_lossy().into_owned(),
            });
        }
    }

    // iCloud Drive (macOS only)
    #[cfg(target_os = "macos")]
    {
        let icloud = home
            .join("Library")
            .join("Mobile Documents")
            .join("com~apple~CloudDocs");
        if icloud.exists() {
            folders.push(SyncFolder {
                name: "iCloud Drive".into(),
                path: icloud.join("Brewski").to_string_lossy().into_owned(),
            });
        }
    }

    folders
}

pub fn find_sync_folders() -> Vec<SyncFolder> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map(PathBuf::from);
    match home {
        Ok(h) => find_sync_folders_with_home(&h),
        Err(_) => vec![],
    }
}

use crate::AppState;
use tauri::State;

/// Pure function: copy src to target_dir/brewski.db, write config. Testable without Tauri.
pub fn execute_database_move(
    src: &std::path::Path,
    target_dir: &std::path::Path,
    config_dir: &std::path::Path,
) -> Result<(), AppError> {
    let target_file = target_dir.join("brewski.db");

    // No-op if already at the same location
    if src == target_file {
        return Ok(());
    }

    std::fs::create_dir_all(target_dir)
        .map_err(|e| AppError::Internal(format!("cannot create directory: {e}")))?;

    std::fs::copy(src, &target_file)
        .map_err(|e| AppError::Internal(format!("cannot copy database: {e}")))?;

    let config = crate::sync_config::SyncConfig {
        database_path: Some(target_file.to_string_lossy().into_owned()),
    };
    config
        .save(config_dir)
        .map_err(|e| AppError::Internal(format!("cannot save config: {e}")))?;

    Ok(())
}

#[tauri::command]
pub async fn detect_sync_folders() -> Result<Vec<SyncFolder>, AppError> {
    Ok(find_sync_folders())
}

#[tauri::command]
pub async fn move_database(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    target_path: String,
) -> Result<(), AppError> {
    let app_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| AppError::Internal(e.to_string()))?;

    execute_database_move(
        &state.db_path,
        &std::path::PathBuf::from(&target_path),
        &app_dir,
    )?;

    app.restart();
}

#[tauri::command]
pub async fn get_db_path(state: State<'_, AppState>) -> Result<String, AppError> {
    Ok(state.db_path.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn make_dir(base: &TempDir, rel: &str) -> PathBuf {
        let path = base.path().join(rel);
        fs::create_dir_all(&path).unwrap();
        path
    }

    #[test]
    fn returns_empty_when_no_providers_installed() {
        // HOME set to an empty temp dir — no provider folders exist
        let home = tempfile::tempdir().unwrap();
        let folders = find_sync_folders_with_home(home.path());
        assert!(folders.is_empty());
    }

    #[test]
    fn detects_google_drive() {
        let home = tempfile::tempdir().unwrap();
        make_dir(&home, "Google Drive/My Drive");
        let folders = find_sync_folders_with_home(home.path());
        assert!(folders.iter().any(|f| f.name == "Google Drive"));
        assert!(folders.iter().any(|f| f.path.ends_with("Brewski")));
    }

    #[test]
    fn detects_dropbox_via_info_json() {
        let home = tempfile::tempdir().unwrap();
        let dropbox_root = make_dir(&home, "Dropbox");
        let info_dir = make_dir(&home, ".dropbox");
        let info = serde_json::json!({ "personal": { "path": dropbox_root.to_str().unwrap() } });
        fs::write(info_dir.join("info.json"), info.to_string()).unwrap();
        let folders = find_sync_folders_with_home(home.path());
        assert!(folders.iter().any(|f| f.name == "Dropbox"));
    }

    #[test]
    fn skips_dropbox_when_info_json_missing() {
        let home = tempfile::tempdir().unwrap();
        fs::create_dir_all(home.path().join(".dropbox")).unwrap();
        // no info.json written
        let folders = find_sync_folders_with_home(home.path());
        assert!(!folders.iter().any(|f| f.name == "Dropbox"));
    }

    #[test]
    fn skips_dropbox_when_info_json_corrupt() {
        let home = tempfile::tempdir().unwrap();
        let info_dir = make_dir(&home, ".dropbox");
        fs::write(info_dir.join("info.json"), b"not json").unwrap();
        let folders = find_sync_folders_with_home(home.path());
        assert!(!folders.iter().any(|f| f.name == "Dropbox"));
    }

    #[test]
    fn appends_brewski_subdirectory() {
        let home = tempfile::tempdir().unwrap();
        make_dir(&home, "Google Drive/My Drive");
        let folders = find_sync_folders_with_home(home.path());
        let gd = folders.iter().find(|f| f.name == "Google Drive").unwrap();
        assert!(gd.path.ends_with("Brewski") || gd.path.ends_with("Brewski/"));
    }

    #[test]
    fn execute_move_copies_file_and_writes_config() {
        let src_dir = tempfile::tempdir().unwrap();
        let config_dir = tempfile::tempdir().unwrap();
        let target_dir = tempfile::tempdir().unwrap();

        let src_file = src_dir.path().join("brewski.db");
        fs::write(&src_file, b"fake db content").unwrap();

        let target_path = target_dir.path().join("Brewski");

        execute_database_move(&src_file, &target_path, config_dir.path()).unwrap();

        // File was copied
        assert!(target_path.join("brewski.db").exists());
        // Config was written
        let config = crate::sync_config::SyncConfig::load(config_dir.path());
        assert_eq!(
            config.database_path,
            Some(
                target_path
                    .join("brewski.db")
                    .to_string_lossy()
                    .into_owned()
            )
        );
        // Original is still there
        assert!(src_file.exists());
    }

    #[test]
    fn execute_move_returns_error_on_bad_target() {
        let src_dir = tempfile::tempdir().unwrap();
        let config_dir = tempfile::tempdir().unwrap();
        let src_file = src_dir.path().join("brewski.db");
        fs::write(&src_file, b"fake db content").unwrap();

        // Use a path that cannot be created (file as parent)
        let not_a_dir = src_dir.path().join("brewski.db").join("nested");

        let result = execute_database_move(&src_file, &not_a_dir, config_dir.path());
        assert!(result.is_err());
        // Config was NOT written
        let config = crate::sync_config::SyncConfig::load(config_dir.path());
        assert!(config.database_path.is_none());
    }

    #[test]
    fn execute_move_no_op_when_same_path() {
        let src_dir = tempfile::tempdir().unwrap();
        let config_dir = tempfile::tempdir().unwrap();
        let src_file = src_dir.path().join("brewski.db");
        fs::write(&src_file, b"fake db content").unwrap();

        // Target resolves to the same file
        let result = execute_database_move(&src_file, src_dir.path(), config_dir.path());
        assert!(result.is_ok());
        // Config not written for no-op
        let config = crate::sync_config::SyncConfig::load(config_dir.path());
        assert!(config.database_path.is_none());
    }
}
