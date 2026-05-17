# Cloud Sync Design

**Date:** 2026-05-16
**Status:** Approved

## Summary

Add cloud sync support for Brewski's SQLite database by letting users move the database file into a folder already managed by their cloud provider (Google Drive, Dropbox, OneDrive, iCloud Drive). The app auto-detects installed providers and surfaces a one-click "Move here" prompt in Settings. Conflict resolution is last-write-wins; this is documented in-app.

## Scope

- Single-user personal sync (not multi-user sharing)
- Cross-platform: macOS, Windows, Linux
- Supported providers: Google Drive, Dropbox, OneDrive, iCloud Drive (Mac only)
- No custom sync protocol — the cloud provider handles all replication

## Architecture

### Startup Config File

The database path must be resolved before the SQLite pool opens, so it cannot be stored inside the database. A separate JSON config file is used:

- **macOS:** `~/Library/Application Support/brewski/config.json`
- **Windows:** `%APPDATA%\brewski\config.json`
- **Linux:** `~/.config/brewski/config.json`

Schema:

```json
{ "database_path": "/absolute/path/to/brewski.db" }
```

If the file is absent or `database_path` is null, the app falls back to the default app data directory path (current behavior).

`lib.rs` reads this config at startup before calling `SqlitePool::connect`.

### Sync Folder Detector

A Rust function `detect_sync_folders() -> Vec<SyncFolder>` checks fixed well-known paths per platform and returns only those that exist on disk:

| Provider | macOS | Windows | Linux |
|---|---|---|---|
| Google Drive | `~/Google Drive/My Drive` | `%USERPROFILE%\Google Drive\My Drive` | `~/Google Drive/My Drive` |
| Dropbox | parsed from `~/.dropbox/info.json` → `path` key | `%APPDATA%\Dropbox\info.json` → `path` key | `~/.dropbox/info.json` → `path` key |
| OneDrive | `~/OneDrive` | `%USERPROFILE%\OneDrive` | — |
| iCloud Drive | `~/Library/Mobile Documents/com~apple~CloudDocs` | — | — |

Each result appends a `Brewski/` subdirectory to avoid cluttering the sync root (e.g. `~/Dropbox/Brewski/`). The `Brewski/` subdirectory is created during the move if it does not exist.

```rust
pub struct SyncFolder {
    pub name: String,   // "Google Drive", "Dropbox", etc.
    pub path: PathBuf,  // absolute path including /Brewski subdirectory
}
```

Exposed as Tauri command: `detect_sync_folders() -> Result<Vec<SyncFolder>, AppError>`

### Move Database Command

Tauri command: `move_database(target_path: String) -> Result<(), AppError>`

Steps:
1. Resolve `target_path` to an absolute `PathBuf`
2. Create the target directory if it does not exist (`std::fs::create_dir_all`)
3. Copy `brewski.db` to `target_path/brewski.db` via `std::fs::copy`
4. If the copy fails, return an error — the original file is untouched and config is not written
5. Write the new path to `config.json`
6. Restart the app via `tauri-plugin-process` (`app_handle.restart()`)

On next startup, `lib.rs` reads `config.json` and opens the pool from the new path. No changes to `AppState` or any command handler are required.

The original database file is **not deleted** after a successful move. The user is informed of this in the confirmation message so they can clean it up manually if desired.

### Custom Path

In addition to detected providers, the Settings UI includes a text input + folder-picker button (Tauri `dialog::open`) for manually specifying any path. This covers providers not on the detection list (e.g. Syncthing, a NAS mount).

## Settings UI

A new "Database Location" section in the Settings page contains:

1. **Current path** — read-only display of the active database file path
2. **Detected sync folders** — list of provider name + path pairs, each with a "Move here" button. Only shown if at least one provider is detected. Hidden after a successful move.
3. **Custom path** — text input + "Browse…" button + "Move here" button
4. **Last-write-wins callout** — always visible below the section:
   > *"Last write wins — if you edit on two devices without syncing in between, the device that syncs last will overwrite the other."*

The section is shown unconditionally (not just on first launch) so users can change the path later.

## Error Handling

| Scenario | Behaviour |
|---|---|
| Copy fails (disk full, permissions) | Toast error; original file and config unchanged |
| Target path is the same as current path | No-op with informational toast |
| Config file unreadable/corrupt on startup | Fall back to default path; log warning |
| Detected path disappears between detection and move | Error surfaced as toast |

## Testing

### Rust Unit Tests (`detect_sync_folders`)
- Create temp directories simulating each provider's folder structure
- Assert only existing folders are returned
- Assert Dropbox JSON parsing handles missing file, empty file, and malformed JSON
- Assert `Brewski/` subdirectory is appended to each result

### Rust Integration Tests (`move_database`)
- Happy path: copy succeeds, config updated (restart is not triggered in tests)
- Failure path: unwritable target → original file untouched, config unchanged
- Same-path no-op

### UI Tests (Playwright)
- Database Location section renders in Settings
- Detected folders list renders when mock command returns results
- "Move here" triggers `move_database` command with correct path
- Success confirmation toast appears
- Custom path browse button triggers file dialog

## Out of Scope

- Automatic conflict detection or merge
- Multi-user sharing
- Any cloud provider SDK or OAuth integration
- Deleting the original database file after move
- Sync status indicator (e.g. "last synced X minutes ago")
