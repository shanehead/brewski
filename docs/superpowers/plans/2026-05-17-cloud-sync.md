# Cloud Sync Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Let users move Brewski's SQLite database into a cloud-synced folder (Google Drive, Dropbox, OneDrive, iCloud), with auto-detection of installed providers and a one-click "Move here" prompt in Settings.

**Architecture:** A startup-time `config.json` (stored alongside the DB in `app_data_dir`) holds the custom DB path; `lib.rs` reads it before opening the pool. Two new Tauri commands handle detection and the move itself. After a successful move the app restarts so the new pool opens cleanly from the new path — no changes to `AppState` or any existing command handler.

**Tech Stack:** Rust / Tauri 2, `tauri-plugin-process` (app restart), `serde_json` (config file), SvelteKit 5 / Svelte 5 runes, Vitest + Testing Library (frontend tests)

---

## File Map

| Action | Path | Responsibility |
|--------|------|----------------|
| Create | `src-tauri/src/sync_config.rs` | `SyncConfig` struct — load/save `config.json` |
| Create | `src-tauri/src/commands/sync.rs` | `detect_sync_folders`, `move_database`, `get_db_path` commands + pure helpers |
| Create | `src/lib/components/DatabaseLocation.svelte` | Settings UI section for DB location |
| Create | `tests/DatabaseLocation.test.ts` | Vitest tests for `DatabaseLocation.svelte` |
| Modify | `src-tauri/Cargo.toml` | Add `tauri-plugin-process = "2"` |
| Modify | `src-tauri/src/lib.rs` | Read config at startup, add `db_path` to `AppState`, register new commands + process plugin |
| Modify | `src-tauri/src/commands/mod.rs` | Add `pub mod sync;` |
| Modify | `src/lib/api.ts` | Add `SyncFolder` type, `detectSyncFolders`, `moveDatabase`, `getDbPath` |
| Modify | `src/routes/settings/+page.svelte` | Import and render `<DatabaseLocation />` |

---

### Task 1: Add `tauri-plugin-process` dependency

**Files:**
- Modify: `src-tauri/Cargo.toml`

- [ ] **Step 1: Add the dependency**

Open `src-tauri/Cargo.toml` and add to `[dependencies]`:

```toml
tauri-plugin-process = "2"
```

- [ ] **Step 2: Fetch the crate**

```bash
cd src-tauri && cargo fetch
```

Expected: no errors, crate downloaded.

- [ ] **Step 3: Commit**

```bash
git add src-tauri/Cargo.toml src-tauri/Cargo.lock
git commit -m "chore: add tauri-plugin-process dependency"
```

---

### Task 2: Create `sync_config.rs` — load/save config.json

**Files:**
- Create: `src-tauri/src/sync_config.rs`

- [ ] **Step 1: Write the failing tests first**

Create `src-tauri/src/sync_config.rs` with the tests only:

```rust
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct SyncConfig {
    pub database_path: Option<String>,
}

impl SyncConfig {
    pub fn load(config_dir: &Path) -> Self {
        todo!()
    }

    pub fn save(&self, config_dir: &Path) -> Result<(), std::io::Error> {
        todo!()
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
```

- [ ] **Step 2: Add `tempfile` to dev-dependencies in Cargo.toml**

```toml
[dev-dependencies]
tokio = { version = "1", features = ["full", "test-util"] }
tempfile = "3"
```

- [ ] **Step 3: Run the tests to confirm they fail**

```bash
cd src-tauri && cargo test sync_config
```

Expected: FAIL — `todo!()` panics.

- [ ] **Step 4: Implement `load` and `save`**

Replace the `todo!()` stubs in `sync_config.rs`:

```rust
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
```

- [ ] **Step 5: Run the tests to confirm they pass**

```bash
cd src-tauri && cargo test sync_config
```

Expected: 3 tests pass.

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/sync_config.rs src-tauri/Cargo.toml src-tauri/Cargo.lock
git commit -m "feat(sync): add SyncConfig for persisting database path"
```

---

### Task 3: Update `lib.rs` — read config at startup, expose db_path in AppState

**Files:**
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Add `sync_config` module and update `AppState`**

In `src-tauri/src/lib.rs`, add the module declaration and update `AppState` to carry the DB path:

```rust
pub mod brewing;
mod commands;
pub mod entities;
mod error;
pub mod models;
#[path = "models.gen.rs"]
pub mod models_gen;
pub mod repositories;
pub mod sync_config;   // ← add this

#[cfg(test)]
mod test_helpers;

use sea_orm::SqlxSqliteConnector;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::path::PathBuf;
use tauri::Manager;

pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
    pub db_path: PathBuf,   // ← add this
}
```

- [ ] **Step 2: Read config before opening the pool**

Replace the `setup` closure body in `run()`:

```rust
.setup(|app| {
    let app_dir = app.path().app_data_dir()?;
    std::fs::create_dir_all(&app_dir)?;

    let config = crate::sync_config::SyncConfig::load(&app_dir);
    let db_path = config
        .database_path
        .map(PathBuf::from)
        .unwrap_or_else(|| app_dir.join("brewski.db"));

    let opts = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true);
    let pool = tauri::async_runtime::block_on(SqlitePool::connect_with(opts))?;
    tauri::async_runtime::block_on(sqlx::migrate!("./migrations").run(&pool))?;
    let db = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool);
    app.manage(AppState { db, db_path });
    Ok(())
})
```

- [ ] **Step 3: Register the process plugin**

Add `.plugin(tauri_plugin_process::init())` after `.plugin(tauri_plugin_opener::init())`:

```rust
tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_process::init())   // ← add this
    .setup(|app| {
```

- [ ] **Step 4: Verify the app still builds**

```bash
cd src-tauri && cargo build 2>&1 | tail -5
```

Expected: `Finished` with no errors.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/lib.rs
git commit -m "feat(sync): read db path from config.json at startup"
```

---

### Task 4: Create `commands/sync.rs` — sync folder detection

**Files:**
- Create: `src-tauri/src/commands/sync.rs`

- [ ] **Step 1: Write the failing tests**

Create `src-tauri/src/commands/sync.rs` with tests only:

```rust
use crate::error::AppError;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Serialize, Clone)]
pub struct SyncFolder {
    pub name: String,
    pub path: String,
}

pub fn find_sync_folders() -> Vec<SyncFolder> {
    todo!()
}

#[tauri::command]
pub async fn detect_sync_folders() -> Result<Vec<SyncFolder>, AppError> {
    Ok(find_sync_folders())
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
        std::env::set_var("BREWSKI_TEST_HOME", home.path().to_str().unwrap());
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
}
```

- [ ] **Step 2: Run to confirm tests fail**

First add `pub mod sync;` to `src-tauri/src/commands/mod.rs`:

```rust
pub mod additions;
pub mod batches;
pub mod equipment;
pub mod import_export;
pub mod library;
pub mod mash;
pub mod recipes;
pub mod settings;
pub mod sync;       // ← add this
pub mod tools;
pub mod water_chemistry;
```

Then run:

```bash
cd src-tauri && cargo test commands::sync
```

Expected: FAIL — `todo!()` panics.

- [ ] **Step 3: Implement `find_sync_folders_with_home` and `find_sync_folders`**

Replace the `todo!()` in `commands/sync.rs`:

```rust
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
```

- [ ] **Step 4: Run the tests to confirm they pass**

```bash
cd src-tauri && cargo test commands::sync
```

Expected: 6 tests pass.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/commands/sync.rs src-tauri/src/commands/mod.rs
git commit -m "feat(sync): add sync folder detection with tests"
```

---

### Task 5: Add `execute_database_move`, `move_database`, and `get_db_path`

**Files:**
- Modify: `src-tauri/src/commands/sync.rs`

- [ ] **Step 1: Write the failing tests for `execute_database_move`**

Add to the `#[cfg(test)]` block in `commands/sync.rs`:

```rust
    #[test]
    fn execute_move_copies_file_and_writes_config() {
        let src_dir = tempfile::tempdir().unwrap();
        let config_dir = tempfile::tempdir().unwrap();
        let target_dir = tempfile::tempdir().unwrap();

        let src_file = src_dir.path().join("brewski.db");
        fs::write(&src_file, b"fake db content").unwrap();

        let target_path = target_dir.path().join("Brewski");

        execute_database_move(
            &src_file,
            &target_path,
            config_dir.path(),
        )
        .unwrap();

        // File was copied
        assert!(target_path.join("brewski.db").exists());
        // Config was written
        let config = crate::sync_config::SyncConfig::load(config_dir.path());
        assert_eq!(
            config.database_path,
            Some(target_path.join("brewski.db").to_string_lossy().into_owned())
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
        let result = execute_database_move(
            &src_file,
            src_dir.path(),
            config_dir.path(),
        );
        assert!(result.is_ok());
        // Config not written for no-op
        let config = crate::sync_config::SyncConfig::load(config_dir.path());
        assert!(config.database_path.is_none());
    }
```

- [ ] **Step 2: Run to confirm new tests fail**

```bash
cd src-tauri && cargo test commands::sync
```

Expected: 3 new tests fail — `execute_database_move` not defined.

- [ ] **Step 3: Implement `execute_database_move`, `move_database`, and `get_db_path`**

Add above the `#[cfg(test)]` block in `commands/sync.rs`:

```rust
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
```

- [ ] **Step 4: Run all sync tests**

```bash
cd src-tauri && cargo test commands::sync
```

Expected: all 9 tests pass.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/commands/sync.rs
git commit -m "feat(sync): add move_database and get_db_path commands"
```

---

### Task 6: Register new commands in `lib.rs`

**Files:**
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Add the three new commands to the invoke handler**

In `src-tauri/src/lib.rs`, add to the `.invoke_handler(tauri::generate_handler![...])` block (after `commands::tools::convert_color`):

```rust
            commands::sync::detect_sync_folders,
            commands::sync::move_database,
            commands::sync::get_db_path,
```

- [ ] **Step 2: Verify the app builds cleanly**

```bash
cd src-tauri && cargo build 2>&1 | tail -5
```

Expected: `Finished` with no errors.

- [ ] **Step 3: Run the full test suite**

```bash
cd src-tauri && cargo test 2>&1 | tail -10
```

Expected: all tests pass.

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/lib.rs
git commit -m "feat(sync): register detect_sync_folders, move_database, get_db_path"
```

---

### Task 7: Add API wrappers in `api.ts`

**Files:**
- Modify: `src/lib/api.ts`

- [ ] **Step 1: Append the sync functions to the end of `src/lib/api.ts`**

```typescript
// --- Sync / Database location ---

export type SyncFolder = { name: string; path: string };

export const detectSyncFolders = (): Promise<SyncFolder[]> =>
  invoke<SyncFolder[]>("detect_sync_folders");

export const moveDatabase = (targetPath: string): Promise<void> =>
  invoke<void>("move_database", { targetPath });

export const getDbPath = (): Promise<string> =>
  invoke<string>("get_db_path");
```

- [ ] **Step 2: Verify TypeScript compiles**

```bash
cd /Users/shead/Documents/code/brewski && bun run check 2>&1 | tail -10
```

Expected: no type errors.

- [ ] **Step 3: Commit**

```bash
git add src/lib/api.ts
git commit -m "feat(sync): add detectSyncFolders, moveDatabase, getDbPath API wrappers"
```

---

### Task 8: Create `DatabaseLocation.svelte`

**Files:**
- Create: `src/lib/components/DatabaseLocation.svelte`

- [ ] **Step 1: Write the failing tests first**

Create `tests/DatabaseLocation.test.ts`:

```typescript
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render } from "@testing-library/svelte";
import { tick } from "svelte";
import DatabaseLocation from "../src/lib/components/DatabaseLocation.svelte";

vi.mock("$lib/api", () => ({
  getDbPath: vi.fn().mockResolvedValue("/home/user/.local/share/brewski/brewski.db"),
  detectSyncFolders: vi.fn().mockResolvedValue([
    { name: "Dropbox", path: "/home/user/Dropbox/Brewski" },
  ]),
  moveDatabase: vi.fn().mockResolvedValue(undefined),
}));

vi.mock("$lib/stores/error", () => ({
  ipc: vi.fn((p) => p),
}));

describe("DatabaseLocation", () => {
  it("renders the section heading", () => {
    const { getByText } = render(DatabaseLocation);
    expect(getByText("Database Location")).toBeInTheDocument();
  });

  it("renders the last-write-wins callout", () => {
    const { getByText } = render(DatabaseLocation);
    expect(getByText(/Last write wins/)).toBeInTheDocument();
  });

  it("renders the current db path after mount", async () => {
    const { getByText } = render(DatabaseLocation);
    await new Promise((r) => setTimeout(r, 10));
    await tick();
    expect(getByText(/brewski\.db/)).toBeInTheDocument();
  });

  it("renders detected sync folder names after mount", async () => {
    const { getByText } = render(DatabaseLocation);
    await new Promise((r) => setTimeout(r, 10));
    await tick();
    expect(getByText("Dropbox")).toBeInTheDocument();
  });

  it("renders a Move here button for each detected folder", async () => {
    const { getAllByText } = render(DatabaseLocation);
    await new Promise((r) => setTimeout(r, 10));
    await tick();
    expect(getAllByText("Move here").length).toBeGreaterThan(0);
  });
});
```

- [ ] **Step 2: Run tests to confirm they fail**

```bash
cd /Users/shead/Documents/code/brewski && bun run test -- DatabaseLocation 2>&1 | tail -10
```

Expected: FAIL — component not found.

- [ ] **Step 3: Create `DatabaseLocation.svelte`**

Create `src/lib/components/DatabaseLocation.svelte`:

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { detectSyncFolders, moveDatabase, getDbPath } from "$lib/api";
  import type { SyncFolder } from "$lib/api";
  import { ipc } from "$lib/stores/error";

  let currentPath = $state("");
  let syncFolders = $state<SyncFolder[]>([]);
  let customPath = $state("");
  let moving = $state(false);

  onMount(async () => {
    const [path, folders] = await Promise.all([
      ipc(getDbPath()),
      ipc(detectSyncFolders()),
    ]);
    currentPath = path ?? "";
    syncFolders = folders ?? [];
  });

  async function handleMove(path: string) {
    if (!path.trim()) return;
    moving = true;
    await ipc(moveDatabase(path));
    moving = false;
  }
</script>

<section class="flex flex-col gap-3">
  <h2 class="text-sm font-semibold" style="color: var(--color-text-secondary);">
    Database Location
  </h2>

  {#if currentPath}
    <p class="text-xs font-mono break-all" style="color: var(--color-text-muted);">
      {currentPath}
    </p>
  {/if}

  {#if syncFolders.length > 0}
    <div class="flex flex-col gap-2">
      {#each syncFolders as folder (folder.path)}
        <div class="flex items-center justify-between gap-3">
          <div class="flex flex-col">
            <span class="text-sm" style="color: var(--color-text-primary);">{folder.name}</span>
            <span class="text-xs font-mono" style="color: var(--color-text-muted);">{folder.path}</span>
          </div>
          <button
            disabled={moving}
            onclick={() => handleMove(folder.path)}
            class="px-3 py-1 text-sm rounded"
            style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);"
          >
            Move here
          </button>
        </div>
      {/each}
    </div>
  {/if}

  <div class="flex items-center gap-2">
    <input
      type="text"
      placeholder="Custom path..."
      bind:value={customPath}
      class="flex-1 px-2 py-1.5 rounded text-sm"
      style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);"
    />
    <button
      disabled={moving || !customPath.trim()}
      onclick={() => handleMove(customPath)}
      class="px-3 py-1.5 text-sm rounded"
      style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);"
    >
      Move here
    </button>
  </div>

  <p class="text-xs" style="color: var(--color-text-muted);">
    <strong>Last write wins</strong> — if you edit on two devices without syncing in between,
    the device that syncs last will overwrite the other.
  </p>
</section>
```

- [ ] **Step 4: Run the tests to confirm they pass**

```bash
cd /Users/shead/Documents/code/brewski && bun run test -- DatabaseLocation 2>&1 | tail -10
```

Expected: 5 tests pass.

- [ ] **Step 5: Commit**

```bash
git add src/lib/components/DatabaseLocation.svelte tests/DatabaseLocation.test.ts
git commit -m "feat(sync): add DatabaseLocation component with tests"
```

---

### Task 9: Add `DatabaseLocation` to the Settings page

**Files:**
- Modify: `src/routes/settings/+page.svelte`

- [ ] **Step 1: Import and render the component**

In `src/routes/settings/+page.svelte`, add the import at the top of the `<script>` block:

```typescript
import DatabaseLocation from "$lib/components/DatabaseLocation.svelte";
```

Then add the component at the bottom of the `<div class="flex flex-col gap-6 max-w-md">` container, after the Units section:

```svelte
    <DatabaseLocation />
```

- [ ] **Step 2: Run the full frontend test suite**

```bash
cd /Users/shead/Documents/code/brewski && bun run test 2>&1 | tail -10
```

Expected: all tests pass.

- [ ] **Step 3: Commit**

```bash
git add src/routes/settings/+page.svelte
git commit -m "feat(sync): add Database Location section to Settings page"
```

---

## Self-Review Notes

- **Spec coverage:** Config file ✓, sync folder detector ✓, move command ✓, restart-on-move ✓, custom path input ✓, last-write-wins callout ✓, error handling (bad target, same path, corrupt config) ✓, all test areas ✓.
- **Types consistent:** `SyncFolder { name: String, path: String }` used in Task 4 (Rust), Task 7 (`api.ts`), Task 8 (component). `execute_database_move(src, target_dir, config_dir)` defined Task 5, referenced only internally.
- **`tauri-plugin-process` Cargo feature:** The `app.restart()` call compiles because `tauri_plugin_process` is registered in `.setup()`. The return type of `restart()` is `!` (never returns), so the `move_database` command body ends there — no `Ok(())` needed after it.
- **`tempfile` crate:** Added to `[dev-dependencies]` in Task 2 and used in Tasks 4 and 5.
