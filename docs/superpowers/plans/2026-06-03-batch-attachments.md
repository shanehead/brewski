# Batch Attachments Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a dedicated Attachments tab to the batch view where users can store photos (transcoded to JPEG) and arbitrary files (copied raw), view them in a masonry photo wall with inline file rows, and open them with the OS default app.

**Architecture:** SQLite `batch_attachments` table stores metadata; files live in `{appDataDir}/attachments/{batch_id}/`. Photos are resized via the existing `write_image` helper; non-images are copied raw. Four Tauri commands (add/list/delete/open) surface the feature through the standard OpenAPI-first pipeline. A shared Svelte component is wired into both desktop and mobile `BatchView`.

**Tech Stack:** Rust (SeaORM, tauri-plugin-opener, tauri-plugin-dialog), SQLite, SvelteKit 5, TypeScript, Tailwind CSS, Redocly OpenAPI

---

## File Map

| Action | File | Purpose |
|--------|------|---------|
| Create | `src-tauri/migrations/011_batch_attachments.sql` | DB schema |
| Create | `src-tauri/src/entities/batch_attachments.rs` | SeaORM entity (generated) |
| Modify | `src-tauri/src/entities/mod.rs` | Register entity module |
| Modify | `src-tauri/src/entities/prelude.rs` | Export entity |
| Modify | `src-tauri/src/entities/batches.rs` | Add `has_many` relation (generated) |
| Create | `docs/openapi/components/schemas/BatchAttachment.yaml` | OpenAPI schema |
| Create | `docs/openapi/paths/commands/add_batch_attachment.yaml` | OpenAPI path |
| Create | `docs/openapi/paths/commands/list_batch_attachments.yaml` | OpenAPI path |
| Create | `docs/openapi/paths/commands/delete_batch_attachment.yaml` | OpenAPI path |
| Create | `docs/openapi/paths/commands/open_batch_attachment.yaml` | OpenAPI path |
| Modify | `docs/openapi/openapi.yaml` | Register new paths + schema |
| Modify | `src-tauri/src/models.gen.rs` | Regenerated (contains BatchAttachment) |
| Modify | `src/lib/api.gen.ts` | Regenerated (contains BatchAttachment) |
| Modify | `src/lib/api.ts` | Add type export + 4 API functions |
| Create | `src-tauri/src/repositories/batch_attachments.rs` | DB CRUD |
| Modify | `src-tauri/src/repositories/mod.rs` | Register repository module |
| Create | `src-tauri/src/commands/batch_attachments.rs` | Tauri commands + storage helpers |
| Modify | `src-tauri/src/commands/mod.rs` | Register command module |
| Modify | `src-tauri/src/lib.rs` | Register 4 commands in invoke_handler |
| Create | `src/lib/components/batch/BatchAttachmentsTab.svelte` | Tab UI |
| Modify | `src/lib/desktop/BatchView.svelte` | Add Attachments tab |
| Modify | `src/lib/mobile/BatchView.svelte` | Add Attachments section |

---

### Task 1: Migration and entity

**Files:**
- Create: `src-tauri/migrations/011_batch_attachments.sql`
- Modify: `src-tauri/src/entities/mod.rs`
- Modify: `src-tauri/src/entities/prelude.rs`
- Generate: `src-tauri/src/entities/batch_attachments.rs` (via `just gen-entities`)

- [ ] **Step 1: Write the migration**

Create `src-tauri/migrations/011_batch_attachments.sql`:

```sql
CREATE TABLE batch_attachments (
    id            TEXT    PRIMARY KEY,
    batch_id      TEXT    NOT NULL REFERENCES batches(id) ON DELETE CASCADE,
    filename      TEXT    NOT NULL,
    original_name TEXT    NOT NULL,
    mime_type     TEXT,
    size_bytes    INTEGER NOT NULL,
    created_at    INTEGER NOT NULL
);
```

- [ ] **Step 2: Run `just gen-entities` to migrate and regenerate all entities**

```bash
just gen-entities
```

Expected: migration runs, `src-tauri/src/entities/batch_attachments.rs` is created, `src-tauri/src/entities/batches.rs` gets a `has_many = BatchAttachments` relation added.

- [ ] **Step 3: Register the new entity in `src-tauri/src/entities/mod.rs`**

Add this line in alphabetical order with the other modules:

```rust
pub mod batch_attachments;
```

- [ ] **Step 4: Export the entity in `src-tauri/src/entities/prelude.rs`**

Add this line in alphabetical order:

```rust
pub use super::batch_attachments::Entity as BatchAttachments;
```

- [ ] **Step 5: Verify the generated entity looks correct**

Read `src-tauri/src/entities/batch_attachments.rs` and confirm it contains:
- `pub struct Model` with fields: `id: String`, `batch_id: String`, `filename: String`, `original_name: String`, `mime_type: Option<String>`, `size_bytes: i32`, `created_at: i32`
- `Relation::Batches` with `on_delete = "Cascade"`

- [ ] **Step 6: Build to verify it compiles**

```bash
cd src-tauri && cargo build 2>&1 | tail -5
```

Expected: compiles without errors.

- [ ] **Step 7: Commit**

```bash
git add src-tauri/migrations/011_batch_attachments.sql src-tauri/src/entities/
git commit -m "feat: add batch_attachments migration and entity"
```

---

### Task 2: OpenAPI schema and code generation

**Files:**
- Create: `docs/openapi/components/schemas/BatchAttachment.yaml`
- Create: `docs/openapi/paths/commands/add_batch_attachment.yaml`
- Create: `docs/openapi/paths/commands/list_batch_attachments.yaml`
- Create: `docs/openapi/paths/commands/delete_batch_attachment.yaml`
- Create: `docs/openapi/paths/commands/open_batch_attachment.yaml`
- Modify: `docs/openapi/openapi.yaml`
- Modify: `src/lib/api.ts`

- [ ] **Step 1: Write the `BatchAttachment` schema**

Create `docs/openapi/components/schemas/BatchAttachment.yaml`:

```yaml
type: object
required:
  - id
  - batch_id
  - filename
  - original_name
  - size_bytes
  - created_at
properties:
  id:
    type: string
  batch_id:
    type: string
  filename:
    type: string
    description: UUID-based on-disk filename, e.g. "a1b2c3d4.jpg"
  original_name:
    type: string
    description: User-facing display name, e.g. "brew-day.jpg"
  mime_type:
    type: [string, "null"]
  size_bytes:
    type: integer
    format: int64
  created_at:
    type: integer
    format: int64
```

- [ ] **Step 2: Write the `add_batch_attachment` path**

Create `docs/openapi/paths/commands/add_batch_attachment.yaml`:

```yaml
post:
  operationId: addBatchAttachment
  summary: Add a file attachment to a batch
  tags:
    - Batches
  requestBody:
    required: true
    content:
      application/json:
        schema:
          type: object
          required:
            - batchId
            - sourcePath
            - originalName
          properties:
            batchId:
              type: string
            sourcePath:
              type: string
            originalName:
              type: string
  responses:
    "200":
      description: Created attachment
      content:
        application/json:
          schema:
            $ref: "../../components/schemas/BatchAttachment.yaml"
    "500":
      $ref: "../../components/responses/Error.yaml"
```

- [ ] **Step 3: Write the `list_batch_attachments` path**

Create `docs/openapi/paths/commands/list_batch_attachments.yaml`:

```yaml
post:
  operationId: listBatchAttachments
  summary: List all attachments for a batch, ordered by created_at ASC
  tags:
    - Batches
  requestBody:
    required: true
    content:
      application/json:
        schema:
          type: object
          required:
            - batchId
          properties:
            batchId:
              type: string
  responses:
    "200":
      description: Attachment list
      content:
        application/json:
          schema:
            type: array
            items:
              $ref: "../../components/schemas/BatchAttachment.yaml"
    "500":
      $ref: "../../components/responses/Error.yaml"
```

- [ ] **Step 4: Write the `delete_batch_attachment` path**

Create `docs/openapi/paths/commands/delete_batch_attachment.yaml`:

```yaml
post:
  operationId: deleteBatchAttachment
  summary: Delete a batch attachment (removes file from disk and DB record)
  tags:
    - Batches
  requestBody:
    required: true
    content:
      application/json:
        schema:
          type: object
          required:
            - id
          properties:
            id:
              type: string
  responses:
    "200":
      description: Deleted
      content:
        application/json:
          schema:
            type: "null"
    "500":
      $ref: "../../components/responses/Error.yaml"
```

- [ ] **Step 5: Write the `open_batch_attachment` path**

Create `docs/openapi/paths/commands/open_batch_attachment.yaml`:

```yaml
post:
  operationId: openBatchAttachment
  summary: Open a batch attachment with the OS default application
  tags:
    - Batches
  requestBody:
    required: true
    content:
      application/json:
        schema:
          type: object
          required:
            - id
          properties:
            id:
              type: string
  responses:
    "200":
      description: Opened
      content:
        application/json:
          schema:
            type: "null"
    "500":
      $ref: "../../components/responses/Error.yaml"
```

- [ ] **Step 6: Register the new paths and schema in `docs/openapi/openapi.yaml`**

In the `paths:` section, add after `/commands/delete_gravity_reading:`:

```yaml
  /commands/add_batch_attachment:
    $ref: ./paths/commands/add_batch_attachment.yaml
  /commands/list_batch_attachments:
    $ref: ./paths/commands/list_batch_attachments.yaml
  /commands/delete_batch_attachment:
    $ref: ./paths/commands/delete_batch_attachment.yaml
  /commands/open_batch_attachment:
    $ref: ./paths/commands/open_batch_attachment.yaml
```

In the `components.schemas:` section, add after `CreateGravityReadingInput:`:

```yaml
    BatchAttachment:
      $ref: ./components/schemas/BatchAttachment.yaml
```

- [ ] **Step 7: Lint the spec**

```bash
just lint-openapi
```

Expected: `validated in Xms` with no errors.

- [ ] **Step 8: Regenerate `api.gen.ts` and `models.gen.rs`**

```bash
just gen
```

Expected: `src/lib/api.gen.ts` and `src-tauri/src/models.gen.rs` updated without errors. Confirm `BatchAttachment` appears in both files:

```bash
grep "BatchAttachment" src/lib/api.gen.ts | head -3
grep "BatchAttachment" src-tauri/src/models.gen.rs | head -3
```

- [ ] **Step 9: Add the type export and API functions to `src/lib/api.ts`**

After the existing batch type exports (near `export type CreateGravityReadingInput`), add:

```typescript
export type BatchAttachment = components["schemas"]["BatchAttachment"];
```

After the existing `deleteGravityReading` function, add:

```typescript
// --- Batch Attachments ---
export const listBatchAttachments = (batchId: string) =>
  invoke<BatchAttachment[]>("list_batch_attachments", { batchId });
export const addBatchAttachment = (batchId: string, sourcePath: string, originalName: string) =>
  invoke<BatchAttachment>("add_batch_attachment", { batchId, sourcePath, originalName });
export const deleteBatchAttachment = (id: string) =>
  invoke<null>("delete_batch_attachment", { id });
export const openBatchAttachment = (id: string) =>
  invoke<null>("open_batch_attachment", { id });
```

- [ ] **Step 10: Run `just check`**

```bash
just check
```

Expected: `0 ERRORS`.

- [ ] **Step 11: Commit**

```bash
git add docs/openapi/ src/lib/api.gen.ts src/lib/api.ts src-tauri/src/models.gen.rs
git commit -m "feat: add BatchAttachment OpenAPI schema and generate types"
```

---

### Task 3: Repository

**Files:**
- Create: `src-tauri/src/repositories/batch_attachments.rs`
- Modify: `src-tauri/src/repositories/mod.rs`

- [ ] **Step 1: Write the failing tests**

Create `src-tauri/src/repositories/batch_attachments.rs` with tests first:

```rust
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};

use crate::entities::batch_attachments;
use crate::error::AppError;
use crate::models::BatchAttachment;

use super::{new_id, now_secs};

pub struct BatchAttachmentRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> BatchAttachmentRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        batch_id: &str,
        filename: &str,
        original_name: &str,
        mime_type: Option<&str>,
        size_bytes: i64,
    ) -> Result<BatchAttachment, AppError> {
        let id = new_id();
        let now = now_secs() as i32;
        batch_attachments::ActiveModel {
            id: Set(id.clone()),
            batch_id: Set(batch_id.to_string()),
            filename: Set(filename.to_string()),
            original_name: Set(original_name.to_string()),
            mime_type: Set(mime_type.map(|s| s.to_string())),
            size_bytes: Set(size_bytes as i32),
            created_at: Set(now),
        }
        .insert(self.db)
        .await?;
        self.get(&id).await
    }

    pub async fn list(&self, batch_id: &str) -> Result<Vec<BatchAttachment>, AppError> {
        let rows = batch_attachments::Entity::find()
            .filter(batch_attachments::Column::BatchId.eq(batch_id))
            .order_by_asc(batch_attachments::Column::CreatedAt)
            .all(self.db)
            .await?;
        Ok(rows.into_iter().map(Self::to_model).collect())
    }

    pub async fn get(&self, id: &str) -> Result<BatchAttachment, AppError> {
        batch_attachments::Entity::find_by_id(id)
            .one(self.db)
            .await?
            .map(Self::to_model)
            .ok_or_else(|| AppError::Internal(format!("attachment {id} not found")))
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        batch_attachments::Entity::delete_by_id(id)
            .exec(self.db)
            .await?;
        Ok(())
    }

    fn to_model(m: batch_attachments::Model) -> BatchAttachment {
        BatchAttachment {
            id: m.id,
            batch_id: m.batch_id,
            filename: m.filename,
            original_name: m.original_name,
            mime_type: m.mime_type,
            size_bytes: m.size_bytes as i64,
            created_at: m.created_at as i64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::CreateRecipeInput;
    use crate::repositories::batches::BatchRepository;
    use crate::repositories::recipe::RecipeRepository;
    use crate::test_helpers::setup_test_db;
    use crate::models::CreateBatchInput;

    async fn setup(db: &DatabaseConnection) -> String {
        let recipe = RecipeRepository::new(db)
            .create(CreateRecipeInput {
                name: "Test IPA".into(),
                ..Default::default()
            })
            .await
            .unwrap();
        let batch = BatchRepository::new(db)
            .create(CreateBatchInput {
                recipe_id: recipe.id,
                name: None,
            })
            .await
            .unwrap();
        batch.id
    }

    #[tokio::test]
    async fn test_create_and_get() {
        let db = setup_test_db().await;
        let batch_id = setup(&db).await;
        let repo = BatchAttachmentRepository::new(&db);
        let att = repo
            .create(&batch_id, "abc.jpg", "brew-day.jpg", Some("image/jpeg"), 102400)
            .await
            .unwrap();
        assert_eq!(att.batch_id, batch_id);
        assert_eq!(att.filename, "abc.jpg");
        assert_eq!(att.original_name, "brew-day.jpg");
        assert_eq!(att.mime_type.as_deref(), Some("image/jpeg"));
        assert_eq!(att.size_bytes, 102400);
    }

    #[tokio::test]
    async fn test_list_ordered_by_created_at() {
        let db = setup_test_db().await;
        let batch_id = setup(&db).await;
        let repo = BatchAttachmentRepository::new(&db);
        repo.create(&batch_id, "a.pdf", "a.pdf", Some("application/pdf"), 1000)
            .await
            .unwrap();
        repo.create(&batch_id, "b.jpg", "b.jpg", Some("image/jpeg"), 2000)
            .await
            .unwrap();
        let list = repo.list(&batch_id).await.unwrap();
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].filename, "a.pdf");
        assert_eq!(list[1].filename, "b.jpg");
    }

    #[tokio::test]
    async fn test_delete() {
        let db = setup_test_db().await;
        let batch_id = setup(&db).await;
        let repo = BatchAttachmentRepository::new(&db);
        let att = repo
            .create(&batch_id, "x.pdf", "x.pdf", None, 500)
            .await
            .unwrap();
        repo.delete(&att.id).await.unwrap();
        let list = repo.list(&batch_id).await.unwrap();
        assert!(list.is_empty());
    }

    #[tokio::test]
    async fn test_delete_batch_cascades() {
        let db = setup_test_db().await;
        let batch_id = setup(&db).await;
        let repo = BatchAttachmentRepository::new(&db);
        repo.create(&batch_id, "x.pdf", "x.pdf", None, 500)
            .await
            .unwrap();
        // Delete the parent batch — ON DELETE CASCADE should remove the attachment row
        BatchRepository::new(&db).delete(&batch_id).await.unwrap();
        let list = repo.list(&batch_id).await.unwrap();
        assert!(list.is_empty());
    }

    #[tokio::test]
    async fn test_get_nonexistent_returns_error() {
        let db = setup_test_db().await;
        let repo = BatchAttachmentRepository::new(&db);
        let result = repo.get("no-such-id").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_null_mime_type() {
        let db = setup_test_db().await;
        let batch_id = setup(&db).await;
        let repo = BatchAttachmentRepository::new(&db);
        let att = repo
            .create(&batch_id, "unknown.bin", "unknown.bin", None, 100)
            .await
            .unwrap();
        assert!(att.mime_type.is_none());
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

```bash
cd src-tauri && cargo test repositories::batch_attachments 2>&1 | tail -15
```

Expected: compilation errors (module not registered yet) or test failures.

- [ ] **Step 3: Register the module in `src-tauri/src/repositories/mod.rs`**

Add in alphabetical order:

```rust
pub mod batch_attachments;
```

- [ ] **Step 4: Run tests again**

```bash
cd src-tauri && cargo test repositories::batch_attachments 2>&1 | tail -15
```

Expected: all 6 tests pass.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/repositories/
git commit -m "feat: add BatchAttachmentRepository with tests"
```

---

### Task 4: Commands and storage helper

**Files:**
- Create: `src-tauri/src/commands/batch_attachments.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Write `src-tauri/src/commands/batch_attachments.rs`**

```rust
use crate::commands::recipe_image::write_image;
use crate::error::AppError;
use crate::models::BatchAttachment;
use crate::repositories::batch_attachments::BatchAttachmentRepository;
use crate::AppState;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager, State};

const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "gif", "webp", "heic"];

pub fn attachments_dir(app: &AppHandle, batch_id: &str) -> Result<PathBuf, AppError> {
    let base = app
        .path()
        .app_data_dir()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(base.join("attachments").join(batch_id))
}

fn file_extension(path: &Path) -> String {
    path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("bin")
        .to_lowercase()
}

fn mime_for_extension(ext: &str) -> String {
    match ext {
        "jpg" | "jpeg" => "image/jpeg".to_string(),
        "png" => "image/png".to_string(),
        "gif" => "image/gif".to_string(),
        "webp" => "image/webp".to_string(),
        "heic" => "image/heic".to_string(),
        "pdf" => "application/pdf".to_string(),
        "doc" => "application/msword".to_string(),
        "docx" => {
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string()
        }
        "xls" => "application/vnd.ms-excel".to_string(),
        "xlsx" => {
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string()
        }
        "txt" => "text/plain".to_string(),
        _ => "application/octet-stream".to_string(),
    }
}

#[tauri::command]
pub async fn add_batch_attachment(
    app: AppHandle,
    state: State<'_, AppState>,
    batch_id: String,
    source_path: String,
    original_name: String,
) -> Result<BatchAttachment, AppError> {
    let src = Path::new(&source_path);
    let ext = file_extension(src);
    let mime = mime_for_extension(&ext);
    let uuid = uuid::Uuid::new_v4().to_string();
    let filename = format!("{uuid}.{ext}");
    let dest_dir = attachments_dir(&app, &batch_id)?;
    let dest = dest_dir.join(&filename);

    std::fs::create_dir_all(&dest_dir)
        .map_err(|e| AppError::Internal(format!("create attachments dir: {e}")))?;

    if IMAGE_EXTENSIONS.contains(&ext.as_str()) {
        write_image(src, &dest)?;
    } else {
        std::fs::copy(src, &dest)
            .map_err(|e| AppError::Internal(format!("copy attachment: {e}")))?;
    }

    let size_bytes = std::fs::metadata(&dest)
        .map_err(|e| AppError::Internal(format!("stat attachment: {e}")))?
        .len() as i64;

    BatchAttachmentRepository::new(&state.db)
        .create(&batch_id, &filename, &original_name, Some(&mime), size_bytes)
        .await
}

#[tauri::command]
pub async fn list_batch_attachments(
    state: State<'_, AppState>,
    batch_id: String,
) -> Result<Vec<BatchAttachment>, AppError> {
    BatchAttachmentRepository::new(&state.db)
        .list(&batch_id)
        .await
}

#[tauri::command]
pub async fn delete_batch_attachment(
    app: AppHandle,
    state: State<'_, AppState>,
    id: String,
) -> Result<(), AppError> {
    let attachment = BatchAttachmentRepository::new(&state.db).get(&id).await?;
    let path = attachments_dir(&app, &attachment.batch_id)?.join(&attachment.filename);
    if let Err(e) = std::fs::remove_file(&path) {
        if e.kind() != std::io::ErrorKind::NotFound {
            return Err(AppError::Internal(format!("remove attachment file: {e}")));
        }
    }
    BatchAttachmentRepository::new(&state.db).delete(&id).await
}

#[tauri::command]
pub async fn open_batch_attachment(
    app: AppHandle,
    state: State<'_, AppState>,
    id: String,
) -> Result<(), AppError> {
    let attachment = BatchAttachmentRepository::new(&state.db).get(&id).await?;
    let path = attachments_dir(&app, &attachment.batch_id)?.join(&attachment.filename);
    tauri_plugin_opener::open_path(path, Option::<&str>::None)
        .map_err(|e| AppError::Internal(format!("open attachment: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb};
    use tempfile::tempdir;

    fn make_png(path: &Path, width: u32, height: u32) {
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
            ImageBuffer::from_fn(width, height, |x, _y| Rgb([x as u8, 100, 200]));
        img.save(path).unwrap();
    }

    #[test]
    fn test_file_extension_jpg() {
        assert_eq!(file_extension(Path::new("brew-day.JPG")), "jpg");
    }

    #[test]
    fn test_file_extension_no_ext() {
        assert_eq!(file_extension(Path::new("noext")), "bin");
    }

    #[test]
    fn test_mime_for_extension_known() {
        assert_eq!(mime_for_extension("jpg"), "image/jpeg");
        assert_eq!(mime_for_extension("pdf"), "application/pdf");
        assert_eq!(mime_for_extension("png"), "image/png");
    }

    #[test]
    fn test_mime_for_extension_unknown() {
        assert_eq!(mime_for_extension("xyz"), "application/octet-stream");
    }

    #[test]
    fn test_image_extension_is_detected() {
        assert!(IMAGE_EXTENSIONS.contains(&"jpg"));
        assert!(IMAGE_EXTENSIONS.contains(&"jpeg"));
        assert!(IMAGE_EXTENSIONS.contains(&"png"));
        assert!(!IMAGE_EXTENSIONS.contains(&"pdf"));
        assert!(!IMAGE_EXTENSIONS.contains(&"docx"));
    }

    #[test]
    fn test_write_image_called_for_png() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("photo.png");
        let dest = dir.path().join("out/photo_out.jpg");
        make_png(&src, 200, 200);
        write_image(&src, &dest).unwrap();
        assert!(dest.exists());
        let img = image::open(&dest).unwrap();
        assert!(img.width() <= 1200);
    }

    #[test]
    fn test_copy_used_for_pdf() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("report.pdf");
        std::fs::write(&src, b"%PDF-1.4 fake content").unwrap();
        let dest_dir = dir.path().join("attachments").join("batch1");
        std::fs::create_dir_all(&dest_dir).unwrap();
        let dest = dest_dir.join("report_copy.pdf");
        std::fs::copy(&src, &dest).unwrap();
        assert_eq!(
            std::fs::read(&dest).unwrap(),
            b"%PDF-1.4 fake content"
        );
    }
}
```

- [ ] **Step 2: Register the module in `src-tauri/src/commands/mod.rs`**

Add in alphabetical order:

```rust
pub mod batch_attachments;
```

- [ ] **Step 3: Register all 4 commands in `src-tauri/src/lib.rs`**

In the `tauri::generate_handler![]` list, after `commands::batches::delete_recipe_version,`, add:

```rust
            commands::batch_attachments::add_batch_attachment,
            commands::batch_attachments::list_batch_attachments,
            commands::batch_attachments::delete_batch_attachment,
            commands::batch_attachments::open_batch_attachment,
```

- [ ] **Step 4: Run the storage helper tests**

```bash
cd src-tauri && cargo test commands::batch_attachments 2>&1 | tail -15
```

Expected: all 6 tests pass.

- [ ] **Step 5: Run the full Rust test suite**

```bash
just test-rust
```

Expected: all tests pass.

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/commands/ src-tauri/src/lib.rs
git commit -m "feat: add batch attachment Tauri commands"
```

---

### Task 5: BatchAttachmentsTab component

**Files:**
- Create: `src/lib/components/batch/BatchAttachmentsTab.svelte`

- [ ] **Step 1: Write the component**

Create `src/lib/components/batch/BatchAttachmentsTab.svelte`:

```svelte
<!-- src/lib/components/batch/BatchAttachmentsTab.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { open as openDialog } from "@tauri-apps/plugin-dialog";
  import { appDataDir as getAppDataDir } from "@tauri-apps/api/path";
  import type { Batch, BatchAttachment } from "$lib/api";
  import {
    listBatchAttachments,
    addBatchAttachment,
    deleteBatchAttachment,
    openBatchAttachment,
  } from "$lib/api";
  import { ipc } from "$lib/stores/error";

  let { batch }: { batch: Batch } = $props();

  let attachments = $state<BatchAttachment[]>([]);
  let appDataDir = $state("");
  let adding = $state(false);

  const photos = $derived(
    attachments.filter((a) => a.mime_type?.startsWith("image/")),
  );
  const files = $derived(
    attachments.filter((a) => !a.mime_type?.startsWith("image/")),
  );

  async function load() {
    attachments = (await ipc(listBatchAttachments(batch.id))) ?? [];
  }

  onMount(async () => {
    appDataDir = await getAppDataDir();
    await load();
  });

  function attachmentSrc(a: BatchAttachment): string {
    return convertFileSrc(`${appDataDir}/attachments/${batch.id}/${a.filename}`);
  }

  async function handleAdd() {
    const result = await openDialog({ multiple: true });
    if (!result) return;
    adding = true;
    try {
      const paths = Array.isArray(result) ? result : [result];
      for (const p of paths) {
        const name =
          p.split("/").pop() ?? p.split("\\").pop() ?? "attachment";
        await ipc(addBatchAttachment(batch.id, p, name));
      }
      await load();
    } finally {
      adding = false;
    }
  }

  async function handleDelete(id: string) {
    await ipc(deleteBatchAttachment(id));
    await load();
  }

  async function handleOpen(id: string) {
    await ipc(openBatchAttachment(id));
  }

  function fileIcon(mimeType: string | null | undefined): string {
    if (!mimeType) return "📄";
    if (mimeType === "application/pdf") return "📄";
    if (
      mimeType.includes("spreadsheet") ||
      mimeType.includes("excel") ||
      mimeType.includes("xlsx")
    )
      return "📊";
    return "📄";
  }

  function formatSize(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  }
</script>

<div class="p-4 flex flex-col gap-4">
  <!-- Header row -->
  <div class="flex items-center justify-between">
    <span class="text-xs" style="color: var(--color-text-muted);">
      {attachments.length} attachment{attachments.length === 1 ? "" : "s"}
    </span>
    <button
      onclick={handleAdd}
      disabled={adding}
      class="px-3 py-1.5 rounded text-sm font-medium"
      style="background: var(--color-accent); color: #fff;"
    >
      {adding ? "Adding…" : "+ Add"}
    </button>
  </div>

  <!-- Photo wall -->
  {#if photos.length > 0}
    <div
      class="photo-grid"
      style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 4px;"
    >
      {#each photos as photo (photo.id)}
        <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
        <div
          class="photo-cell"
          style="position: relative; aspect-ratio: 1; overflow: hidden; border-radius: 4px; background: var(--color-bg-elevated); cursor: pointer;"
          onclick={() => handleOpen(photo.id)}
        >
          <img
            src={attachmentSrc(photo)}
            alt={photo.original_name}
            onload={(e) => {
              const img = e.currentTarget as HTMLImageElement;
              if (img.naturalHeight > img.naturalWidth * 1.3) {
                (img.closest(".photo-cell") as HTMLElement).style.gridRow =
                  "span 2";
                (img.closest(".photo-cell") as HTMLElement).style.aspectRatio =
                  "auto";
              }
            }}
            style="width: 100%; height: 100%; object-fit: cover; display: block;"
          />
          <button
            onclick={(e) => { e.stopPropagation(); handleDelete(photo.id); }}
            aria-label="Delete {photo.original_name}"
            class="delete-btn"
            style="
              position: absolute; top: 4px; right: 4px;
              width: 22px; height: 22px;
              background: rgba(0,0,0,0.6); color: #fff;
              border-radius: 50%; border: none; cursor: pointer;
              font-size: 12px; display: flex; align-items: center; justify-content: center;
            "
          >×</button>
        </div>
      {/each}
    </div>
  {/if}

  <!-- File list -->
  {#if files.length > 0}
    <div class="flex flex-col gap-1">
      {#each files as file (file.id)}
        <div
          class="flex items-center gap-2 px-3 py-2 rounded cursor-pointer"
          style="background: var(--color-bg-elevated); border: 1px solid var(--color-border);"
          role="button"
          tabindex="0"
          onclick={() => handleOpen(file.id)}
          onkeydown={(e) => e.key === "Enter" && handleOpen(file.id)}
        >
          <span class="text-base leading-none">{fileIcon(file.mime_type)}</span>
          <span class="flex-1 text-sm truncate" style="color: var(--color-text-secondary);">
            {file.original_name}
          </span>
          <span class="text-xs flex-shrink-0" style="color: var(--color-text-muted);">
            {formatSize(file.size_bytes)}
          </span>
          <button
            onclick={(e) => { e.stopPropagation(); handleDelete(file.id); }}
            aria-label="Delete {file.original_name}"
            class="text-sm flex-shrink-0"
            style="color: var(--color-text-muted);"
          >×</button>
        </div>
      {/each}
    </div>
  {/if}

  {#if attachments.length === 0 && !adding}
    <p class="text-sm text-center py-8" style="color: var(--color-text-muted);">
      No attachments yet. Add photos, PDFs, or any files.
    </p>
  {/if}
</div>

<style>
  @media (max-width: 640px) {
    .photo-grid {
      grid-template-columns: repeat(2, 1fr) !important;
    }
    .delete-btn {
      width: 28px !important;
      height: 28px !important;
      font-size: 14px !important;
    }
  }
</style>
```

- [ ] **Step 2: Run `just check`**

```bash
just check
```

Expected: `0 ERRORS`.

- [ ] **Step 3: Commit**

```bash
git add src/lib/components/batch/BatchAttachmentsTab.svelte
git commit -m "feat: add BatchAttachmentsTab component"
```

---

### Task 6: Wire into desktop and mobile BatchView

**Files:**
- Modify: `src/lib/desktop/BatchView.svelte`
- Modify: `src/lib/mobile/BatchView.svelte`

- [ ] **Step 1: Update `src/lib/desktop/BatchView.svelte`**

Replace the script section and tab definitions. The current file starts with:

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import type { Batch, UpdateBatchInput } from "$lib/api";
  import { getBatch, updateBatch } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { batchList, refreshBatchList } from "$lib/stores/batches";
  import BatchList from "$lib/components/BatchList.svelte";
  import TabBar from "$lib/components/TabBar.svelte";
  import BatchOverviewTab from "$lib/components/batch/BatchOverviewTab.svelte";
  import BatchGravityTab from "$lib/components/batch/BatchGravityTab.svelte";
  import BatchTastingTab from "$lib/components/batch/BatchTastingTab.svelte";

  let { id }: { id: string } = $props();

  let batch = $state<Batch | null>(null);
  let activeTab = $state<"overview" | "gravity" | "tasting">("overview");

  const TABS = [
    { key: "overview", label: "Overview" },
    { key: "gravity", label: "Gravity Log" },
    { key: "tasting", label: "Tasting" },
  ] as const;
```

Replace with:

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import type { Batch, UpdateBatchInput } from "$lib/api";
  import { getBatch, updateBatch } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { batchList, refreshBatchList } from "$lib/stores/batches";
  import BatchList from "$lib/components/BatchList.svelte";
  import TabBar from "$lib/components/TabBar.svelte";
  import BatchOverviewTab from "$lib/components/batch/BatchOverviewTab.svelte";
  import BatchGravityTab from "$lib/components/batch/BatchGravityTab.svelte";
  import BatchTastingTab from "$lib/components/batch/BatchTastingTab.svelte";
  import BatchAttachmentsTab from "$lib/components/batch/BatchAttachmentsTab.svelte";

  let { id }: { id: string } = $props();

  let batch = $state<Batch | null>(null);
  let activeTab = $state<"overview" | "gravity" | "tasting" | "attachments">("overview");

  const TABS = [
    { key: "overview", label: "Overview" },
    { key: "gravity", label: "Gravity Log" },
    { key: "tasting", label: "Tasting" },
    { key: "attachments", label: "Attachments" },
  ] as const;
```

Then in the tab content section, find `{:else if activeTab === "tasting"}` and add after it:

```svelte
      {:else if activeTab === "attachments"}
        <BatchAttachmentsTab {batch} />
```

- [ ] **Step 2: Update `src/lib/mobile/BatchView.svelte`**

Add the import after the existing batch tab imports:

```svelte
  import BatchAttachmentsTab from "$lib/components/batch/BatchAttachmentsTab.svelte";
```

Add an Attachments section after the Tasting section (inside the `<div class="p-4 flex flex-col gap-6">`). Find:

```svelte
        <!-- Tasting -->
        <section>
          <div class="text-xs font-semibold uppercase tracking-wider mb-3"
               style="color: var(--color-text-secondary);">Tasting</div>
          <BatchTastingTab {batch} onUpdate={handleUpdate} />
        </section>
```

And add after it:

```svelte
        <!-- Attachments -->
        <section>
          <div class="text-xs font-semibold uppercase tracking-wider mb-3"
               style="color: var(--color-text-secondary);">Attachments</div>
          <BatchAttachmentsTab {batch} />
        </section>
```

- [ ] **Step 3: Run `just check`**

```bash
just check
```

Expected: `0 ERRORS`.

- [ ] **Step 4: Run `just test`**

```bash
just test
```

Expected: all tests pass.

- [ ] **Step 5: Commit**

```bash
git add src/lib/desktop/BatchView.svelte src/lib/mobile/BatchView.svelte
git commit -m "feat: wire BatchAttachmentsTab into desktop and mobile BatchView"
```
