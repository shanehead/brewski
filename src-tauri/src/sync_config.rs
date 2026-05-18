use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct SyncConfig {
    pub database_path: Option<String>,
}

impl SyncConfig {
    pub fn load(config_dir: &Path) -> Self {
        let path = config_dir.join("config.json");
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self, config_dir: &Path) -> Result<(), std::io::Error> {
        let path = config_dir.join("config.json");
        std::fs::create_dir_all(config_dir)?;
        let json = serde_json::to_string_pretty(self).expect("serialization never fails");
        std::fs::write(&path, json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn tmp() -> TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    fn load_returns_default_when_file_missing() {
        let dir = tmp();
        let config = SyncConfig::load(dir.path());
        assert_eq!(config, SyncConfig::default());
    }

    #[test]
    fn load_returns_default_when_file_corrupt() {
        let dir = tmp();
        fs::write(dir.path().join("config.json"), b"not valid json").unwrap();
        let config = SyncConfig::load(dir.path());
        assert_eq!(config, SyncConfig::default());
    }

    #[test]
    fn save_and_load_roundtrip() {
        let dir = tmp();
        let original = SyncConfig {
            database_path: Some("/some/path/brewski.db".into()),
        };
        original.save(dir.path()).unwrap();
        let loaded = SyncConfig::load(dir.path());
        assert_eq!(loaded, original);
    }
}
