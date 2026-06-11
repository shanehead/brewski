# Recipe Version Selection & Change Detection Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace silent `matches_current` auto-reuse with explicit version selection at batch creation, backed by a total content-hash change detector, and surface un-versioned recipe changes to the user.

**Architecture:** A pure `recipe_content_hash(&Recipe)` projection (include-by-default, serialize-then-strip via `serde_json`) fingerprints recipe identity. Each `recipe_versions` row stores that hash; a `status` query compares the live recipe's hash to the latest version's. `create_batch` requires an explicit `version_id`; the frontend resolves it via a unified brew flow (auto-latest when clean, a prompt when dirty). A dirty badge on the recipe view exposes un-versioned changes.

**Tech Stack:** Rust (SeaORM + sqlx, Tauri commands), SvelteKit 5 (runes), Tailwind v4, SQL migrations, OpenAPI-driven codegen (`just gen`, `just gen-entities`).

---

## Key conventions (read before starting)

- **Type changes flow through OpenAPI**, never hand-edit `*.gen.*`. Edit `docs/openapi/openapi.yaml`, then run `just gen` (regenerates `src/lib/api.gen.ts` + `src-tauri/src/models.gen.rs`).
- **DB schema changes:** add a numbered SQL file in `src-tauri/migrations/` (next is `017`), then `just gen-entities` (regenerates `src-tauri/src/entities/`). Tests use `sqlx::migrate!` against an in-memory DB via `setup_test_db()`.
- **Run backend tests:** `cd src-tauri && cargo test`. **Frontend tests:** `bun run test`. **Type/lint:** `just check`.
- Recipe identity decisions (from the spec): include-by-default; exclude metadata and resolved profile *values*; keep profile *selection* ids. See `docs/superpowers/specs/2026-06-10-recipe-version-selection-design.md`.

---

## Task 1: Content-hash module

**Files:**
- Create: `src-tauri/src/recipe_hash.rs`
- Modify: `src-tauri/src/lib.rs` (add `mod recipe_hash;`), `src-tauri/Cargo.toml` (add `sha2`)

- [ ] **Step 1: Add the `sha2` dependency**

In `src-tauri/Cargo.toml`, under `[dependencies]`, add:

```toml
sha2 = "0.10"
```

- [ ] **Step 2: Write the module with failing unit tests**

Create `src-tauri/src/recipe_hash.rs`:

```rust
//! Deterministic content fingerprint for recipe identity / change detection.
//!
//! Include-by-default: we serialize the whole `Recipe` and then strip the keys
//! that are NOT part of brew identity (metadata + resolved profile *values*).
//! A future recipe field is therefore part of the hash automatically unless it
//! is named like a surrogate key or added to the metadata exclude set.

use crate::error::AppError;
use crate::models::Recipe;
use serde_json::Value;
use sha2::{Digest, Sha256};

/// Bump when the canonical projection changes; stored hashes are prefixed with
/// this so stale ones can be recomputed instead of trusted.
pub const PROJECTION_VERSION: &str = "1";

/// Surrogate keys removed from EVERY object (live rows and snapshot rows use
/// different ids for identical content). Library reference ids like
/// `fermentable_id` / `equipment_profile_id` are deliberately NOT in this set.
const SURROGATE_KEYS: &[&str] = &["id", "recipe_id", "recipe_version_id", "mash_id"];

/// Top-level keys excluded from identity: metadata + the resolved profile value
/// objects. The profile *selection* ids (`equipment_profile_id`,
/// `mash_water_id`, `sparge_water_id`) are kept.
const TOP_EXCLUDE_KEYS: &[&str] = &[
    "created_at",
    "updated_at",
    "source",
    "taste_notes",
    "taste_rating",
    "image_path",
    "name",
    "brewer",
    "asst_brewer",
    "date",
    "style",     // resolved style object
    "style_id",  // descriptive classification, not brew content
    "equipment_profile", // resolved profile values (selection id kept)
];

fn scrub_surrogates(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for k in SURROGATE_KEYS {
                map.remove(*k);
            }
            for (_, v) in map.iter_mut() {
                scrub_surrogates(v);
            }
        }
        Value::Array(arr) => {
            for v in arr.iter_mut() {
                scrub_surrogates(v);
            }
        }
        _ => {}
    }
}

/// Returns the canonical JSON bytes used for hashing. Exposed for tests.
pub fn canonical_bytes(recipe: &Recipe) -> Result<Vec<u8>, AppError> {
    let mut value =
        serde_json::to_value(recipe).map_err(|e| AppError::Internal(e.to_string()))?;
    scrub_surrogates(&mut value);
    if let Value::Object(map) = &mut value {
        for k in TOP_EXCLUDE_KEYS {
            map.remove(*k);
        }
    }
    serde_json::to_vec(&value).map_err(|e| AppError::Internal(e.to_string()))
}

/// `"<projection>:<hex sha256>"` fingerprint of a recipe's brew identity.
pub fn recipe_content_hash(recipe: &Recipe) -> Result<String, AppError> {
    let bytes = canonical_bytes(recipe)?;
    let digest = Sha256::digest(&bytes);
    Ok(format!("{PROJECTION_VERSION}:{digest:x}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Recipe;

    // Minimal Recipe builder for hash tests. Adjust field set if `Recipe` gains
    // required fields; `..Default::default()` is not available on the generated
    // type, so build explicitly via serde_json to stay decoupled from field count.
    fn recipe_from_json(v: serde_json::Value) -> Recipe {
        serde_json::from_value(v).expect("valid Recipe json")
    }

    fn base_json() -> serde_json::Value {
        serde_json::json!({
            "id": "r1",
            "name": "Test IPA",
            "type": "All Grain",
            "brewer": null, "asst_brewer": null,
            "batch_size_l": 23.0, "boil_size_l": 27.0, "boil_time_min": 60.0,
            "efficiency_pct": 72.0,
            "style_id": null, "equipment_profile_id": "eq1",
            "notes": null, "taste_notes": null, "taste_rating": null,
            "og": null, "fg": null, "fermentation_stages": 1,
            "primary_age_days": null, "primary_temp_c": null,
            "secondary_age_days": null, "secondary_temp_c": null,
            "tertiary_age_days": null, "tertiary_temp_c": null,
            "age_days": null, "age_temp_c": null,
            "carbonation_vols": null, "forced_carbonation": false,
            "priming_sugar_name": null, "carbonation_temp_c": null,
            "priming_sugar_equiv": null, "keg_priming_factor": null,
            "date": null, "source": "user",
            "mash_water_id": null, "sparge_water_id": null,
            "hopstand_temp_c": 80.0, "image_path": null,
            "created_at": 1, "updated_at": 1,
            "style": null, "equipment_profile": null,
            "fermentables": [], "hops": [], "yeasts": [],
            "miscs": [], "waters": [], "water_adjustments": [], "mash": null
        })
    }

    #[test]
    fn identical_content_hashes_equal() {
        let a = recipe_from_json(base_json());
        let b = recipe_from_json(base_json());
        assert_eq!(recipe_content_hash(&a).unwrap(), recipe_content_hash(&b).unwrap());
    }

    #[test]
    fn surrogate_ids_do_not_affect_hash() {
        let a = recipe_from_json(base_json());
        let mut j = base_json();
        j["id"] = serde_json::json!("DIFFERENT");
        j["created_at"] = serde_json::json!(999);
        let b = recipe_from_json(j);
        assert_eq!(recipe_content_hash(&a).unwrap(), recipe_content_hash(&b).unwrap());
    }

    #[test]
    fn excluded_metadata_does_not_affect_hash() {
        let a = recipe_from_json(base_json());
        let mut j = base_json();
        j["name"] = serde_json::json!("Renamed");
        j["taste_notes"] = serde_json::json!("delicious");
        j["image_path"] = serde_json::json!("/x.png");
        let b = recipe_from_json(j);
        assert_eq!(recipe_content_hash(&a).unwrap(), recipe_content_hash(&b).unwrap());
    }

    #[test]
    fn equipment_profile_selection_changes_hash() {
        let a = recipe_from_json(base_json());
        let mut j = base_json();
        j["equipment_profile_id"] = serde_json::json!("eq2");
        let b = recipe_from_json(j);
        assert_ne!(recipe_content_hash(&a).unwrap(), recipe_content_hash(&b).unwrap());
    }

    #[test]
    fn brew_field_changes_hash() {
        let a = recipe_from_json(base_json());
        let mut j = base_json();
        j["batch_size_l"] = serde_json::json!(20.0);
        let b = recipe_from_json(j);
        assert_ne!(recipe_content_hash(&a).unwrap(), recipe_content_hash(&b).unwrap());
    }

    #[test]
    fn hash_has_projection_prefix() {
        let a = recipe_from_json(base_json());
        assert!(recipe_content_hash(&a).unwrap().starts_with("1:"));
    }
}
```

- [ ] **Step 3: Register the module**

In `src-tauri/src/lib.rs`, add alongside the other `mod` declarations:

```rust
mod recipe_hash;
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cd src-tauri && cargo test recipe_hash`
Expected: all `recipe_hash::tests::*` PASS. If `recipe_from_json` fails to deserialize, the generated `Recipe` field/JSON-key set drifted from `base_json()` — align the JSON keys with the current `Recipe` schema in `docs/openapi/openapi.yaml` (use the exact serde key names, e.g. `"type"` not `"type_"`).

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/recipe_hash.rs src-tauri/src/lib.rs src-tauri/Cargo.toml src-tauri/Cargo.lock
git commit -m "feat: add recipe content-hash module for change detection"
```

---

## Task 2: Migration — content_hash + freeze hopstand fidelity gaps

`get_full` currently hardcodes recipe-level `hopstand_temp_c: 80.0` and per-hop `hopstand_temp_c: None`, neither of which is stored in a snapshot. Both are in the hash's include-set, so they must be frozen or the live recipe would read as permanently dirty. This task adds the columns; Task 3 wires them.

**Files:**
- Create: `src-tauri/migrations/017_recipe_version_content_hash.sql`
- Regenerate: `src-tauri/src/entities/recipe_versions.rs`, `src-tauri/src/entities/recipe_version_hops.rs`

- [ ] **Step 1: Write the migration**

Create `src-tauri/migrations/017_recipe_version_content_hash.sql`:

```sql
-- Content fingerprint for change detection (NULL = recompute lazily).
ALTER TABLE recipe_versions ADD COLUMN content_hash TEXT;

-- Freeze hopstand temperatures into snapshots (previously re-resolved/hardcoded,
-- which would make recipes read as permanently changed once hashed).
ALTER TABLE recipe_versions ADD COLUMN hopstand_temp_c REAL;
ALTER TABLE recipe_version_hops ADD COLUMN hopstand_temp_c REAL;
```

- [ ] **Step 2: Apply migration to the dev DB and regenerate entities**

Run: `just gen-entities`
Expected: completes; `git diff src-tauri/src/entities/` shows `content_hash: Option<String>` and `hopstand_temp_c: Option<f64>` added to `recipe_versions::Model`, and `hopstand_temp_c: Option<f64>` added to `recipe_version_hops::Model`.

- [ ] **Step 3: Verify the test DB picks up the migration**

Run: `cd src-tauri && cargo test --no-run`
Expected: compiles. (The in-memory test DB runs all `migrations/`, so the new columns exist in tests.)

- [ ] **Step 4: Commit**

```bash
git add src-tauri/migrations/017_recipe_version_content_hash.sql src-tauri/src/entities/recipe_versions.rs src-tauri/src/entities/recipe_version_hops.rs
git commit -m "feat: add content_hash + hopstand snapshot columns to recipe versions"
```

---

## Task 3: Freeze hopstand in snapshot/restore + store content_hash

**Files:**
- Modify: `src-tauri/src/repositories/recipe_version.rs` (`snapshot`, `get_full`)

- [ ] **Step 1: Write a failing round-trip equality test**

Add to the `tests` module in `src-tauri/src/repositories/recipe_version.rs`:

```rust
#[tokio::test]
async fn live_and_snapshot_hash_match_for_unchanged_recipe() {
    use crate::recipe_hash::recipe_content_hash;
    let db = setup_test_db().await;
    let recipe_id = make_recipe(&db).await;
    let repo = RecipeVersionRepository::new(&db);

    // Create a version, then load both the live recipe and the snapshot.
    let v = repo.save_named(&recipe_id, None).await.unwrap();
    let live = RecipeRepository::new(&db).get(&recipe_id).await.unwrap();
    let snap = repo.get_full(&v.id).await.unwrap();

    assert_eq!(
        recipe_content_hash(&live).unwrap(),
        recipe_content_hash(&snap).unwrap(),
        "an unmodified recipe must hash identically to its own snapshot"
    );
}
```

> Note: this test calls `save_named(&recipe_id, None)` — Task 6 changes `save_named` to take `Option<String>`. If implementing strictly in order, temporarily call `save_named(&recipe_id, "v".into())` here and switch to `None` after Task 6. (Subagent-driven execution: prefer doing Task 6's `save_named` signature change first if convenient.)

- [ ] **Step 2: Run it to confirm it fails**

Run: `cd src-tauri && cargo test live_and_snapshot_hash_match_for_unchanged_recipe`
Expected: FAIL — hashes differ because `get_full` hardcodes `hopstand_temp_c: 80.0`/`None` while the live recipe carries real values.

- [ ] **Step 3: Freeze hopstand in `snapshot`**

In `snapshot`, in the `recipe_versions::ActiveModel { … }` initializer, add after `keg_priming_factor`:

```rust
            hopstand_temp_c: Set(Some(recipe.hopstand_temp_c)),
            content_hash: Set(Some(crate::recipe_hash::recipe_content_hash(recipe)?)),
```

In the per-hop loop (`for h in &recipe.hops { recipe_version_hops::ActiveModel { … } }`), add after `addition_order`:

```rust
                hopstand_temp_c: h.hopstand_temp_c,
```

- [ ] **Step 4: Read frozen hopstand in `get_full`**

In `get_full`, in the hops mapping closure, replace:

```rust
                hopstand_temp_c: None, // not captured in version snapshot
```

with:

```rust
                hopstand_temp_c: m.hopstand_temp_c,
```

And in the final `Ok(Recipe { … })`, replace:

```rust
            hopstand_temp_c: 80.0,
```

with:

```rust
            hopstand_temp_c: v.hopstand_temp_c.unwrap_or(80.0),
```

- [ ] **Step 5: Run the round-trip test**

Run: `cd src-tauri && cargo test live_and_snapshot_hash_match_for_unchanged_recipe`
Expected: PASS. If it still fails, another snapshot field diverges from live — diff the two `Recipe` values and either freeze that field in `snapshot`/`get_full` or add its key to `TOP_EXCLUDE_KEYS` (only if it is genuinely not brew identity). Re-run until green.

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/repositories/recipe_version.rs
git commit -m "feat: freeze hopstand temps and store content_hash in snapshots"
```

---

## Task 4: `status` query with lazy hash recompute

> **Spec note:** the spec mentioned a backfill migration to populate `content_hash`
> for pre-existing versions. This plan instead leaves old hashes `NULL` and
> recomputes them lazily on first comparison (`version_hash` below), persisting the
> result. This satisfies the same requirement (existing versions get correct
> hashes) without a data migration and is self-healing across projection bumps.

**Files:**
- Modify: `src-tauri/src/repositories/recipe_version.rs`
- Modify: `docs/openapi/openapi.yaml` (add `RecipeVersionStatus` schema)

- [ ] **Step 1: Add `RecipeVersionStatus` to the OpenAPI spec**

In `docs/openapi/openapi.yaml`, under `components/schemas`, add (match surrounding indentation/style; if schemas live in `docs/openapi/components/`, add a file there and `$ref` it as the neighbors do):

```yaml
    RecipeVersionStatus:
      type: object
      description: Whether a recipe has changes not captured in its latest version.
      required: [version_count, has_unversioned_changes]
      properties:
        version_count:
          type: integer
          format: int64
          description: Number of saved versions for this recipe.
        latest_version_id:
          type: string
          nullable: true
          description: Id of the most recent version, or null if none exist.
        has_unversioned_changes:
          type: boolean
          description: True when the live recipe differs from its latest version.
```

- [ ] **Step 2: Regenerate types**

Run: `just gen`
Expected: `RecipeVersionStatus` appears in `src/lib/api.gen.ts` and `src-tauri/src/models.gen.rs`.

- [ ] **Step 3: Write failing tests for `status`**

Add to the `tests` module in `recipe_version.rs`:

```rust
#[tokio::test]
async fn status_zero_versions_is_not_dirty() {
    let db = setup_test_db().await;
    let recipe_id = make_recipe(&db).await;
    let st = RecipeVersionRepository::new(&db).status(&recipe_id).await.unwrap();
    assert_eq!(st.version_count, 0);
    assert!(st.latest_version_id.is_none());
    assert!(!st.has_unversioned_changes);
}

#[tokio::test]
async fn status_clean_after_snapshot() {
    let db = setup_test_db().await;
    let recipe_id = make_recipe(&db).await;
    let repo = RecipeVersionRepository::new(&db);
    repo.save_named(&recipe_id, None).await.unwrap();
    let st = repo.status(&recipe_id).await.unwrap();
    assert_eq!(st.version_count, 1);
    assert!(!st.has_unversioned_changes);
}

#[tokio::test]
async fn status_dirty_after_edit() {
    use crate::repositories::recipe::RecipeRepository;
    use crate::models::UpdateRecipeInput;
    let db = setup_test_db().await;
    let recipe_id = make_recipe(&db).await;
    let repo = RecipeVersionRepository::new(&db);
    repo.save_named(&recipe_id, None).await.unwrap();

    RecipeRepository::new(&db)
        .update(&recipe_id, UpdateRecipeInput { batch_size_l: Some(19.0), ..Default::default() })
        .await
        .unwrap();

    let st = repo.status(&recipe_id).await.unwrap();
    assert!(st.has_unversioned_changes, "editing batch size must mark the recipe dirty");
}
```

> If `UpdateRecipeInput` has no `Default`, set every field explicitly or use the existing update helper pattern from other tests in this file.

- [ ] **Step 4: Run them to confirm failure**

Run: `cd src-tauri && cargo test status_`
Expected: FAIL — `status` and `version_hash` not defined.

- [ ] **Step 5: Implement `version_hash` + `status`**

Add these methods to `impl RecipeVersionRepository` in `recipe_version.rs`:

```rust
/// Returns the content hash for a version, recomputing (and persisting) it when
/// missing or produced by an older projection.
async fn version_hash(
    &self,
    v: &recipe_versions::Model,
) -> Result<String, AppError> {
    let current_prefix = format!("{}:", crate::recipe_hash::PROJECTION_VERSION);
    if let Some(h) = &v.content_hash {
        if h.starts_with(&current_prefix) {
            return Ok(h.clone());
        }
    }
    let snap = self.get_full(&v.id).await?;
    let h = crate::recipe_hash::recipe_content_hash(&snap)?;
    recipe_versions::ActiveModel {
        id: Set(v.id.clone()),
        content_hash: Set(Some(h.clone())),
        ..Default::default()
    }
    .update(self.db)
    .await?;
    Ok(h)
}

pub async fn status(
    &self,
    recipe_id: &str,
) -> Result<crate::models::RecipeVersionStatus, AppError> {
    let versions = recipe_versions::Entity::find()
        .filter(recipe_versions::Column::RecipeId.eq(recipe_id))
        .order_by_desc(recipe_versions::Column::VersionNumber)
        .all(self.db)
        .await?;

    let has_unversioned_changes = if let Some(latest) = versions.first() {
        let live = RecipeRepository::new(self.db).get(recipe_id).await?;
        let live_hash = crate::recipe_hash::recipe_content_hash(&live)?;
        live_hash != self.version_hash(latest).await?
    } else {
        false
    };

    Ok(crate::models::RecipeVersionStatus {
        version_count: versions.len() as i64,
        latest_version_id: versions.first().map(|v| v.id.clone()),
        has_unversioned_changes,
    })
}
```

- [ ] **Step 6: Run the tests**

Run: `cd src-tauri && cargo test status_ && cargo test live_and_snapshot`
Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add docs/openapi/openapi.yaml src/lib/api.gen.ts src-tauri/src/models.gen.rs src-tauri/src/repositories/recipe_version.rs
git commit -m "feat: add recipe_version_status with lazy content-hash recompute"
```

---

## Task 5: Require `version_id`; remove `matches_current`/`create_or_reuse`; optional version name

**Files:**
- Modify: `docs/openapi/openapi.yaml` (`CreateBatchInput.version_id` required; `SaveRecipeVersionInput.name` nullable)
- Modify: `src-tauri/src/repositories/recipe_version.rs` (delete `matches_current`, `create_or_reuse`; `save_named` takes `Option<String>`)
- Modify: `src-tauri/src/repositories/batches.rs` (`create` requires version; seed helper)
- Modify: `src-tauri/src/commands/batches.rs` (`save_recipe_version` optional name; new `recipe_version_status` command)
- Modify: `src-tauri/src/lib.rs` (register `recipe_version_status`)

- [ ] **Step 1: Update the OpenAPI spec**

In `docs/openapi/openapi.yaml`:
- `CreateBatchInput`: add `version_id` to its `required` list and ensure the property is `type: string` (not nullable).
- `SaveRecipeVersionInput`: make `name` `nullable: true` and remove it from `required` if present.

- [ ] **Step 2: Regenerate types**

Run: `just gen`
Expected: `CreateBatchInput.version_id` becomes non-optional in `api.gen.ts`; `SaveRecipeVersionInput.name` becomes optional.

- [ ] **Step 3: Change `save_named` signature**

In `recipe_version.rs`, change `save_named` from `name: &str` to `name: Option<String>` and pass it straight through to `snapshot(..., name, ...)` (which already takes `Option<String>`). Update the existing `save_named` body accordingly (drop any `.to_string()` on the name).

- [ ] **Step 4: Delete `create_or_reuse` and `matches_current`**

Remove both methods entirely from `recipe_version.rs`. Remove the now-unused tests that exercised reuse behavior (`test_unchanged_recipe_reuses_version`, `test_changed_recipe_creates_new_version`, `test_first_brew_creates_version_1`, and any others calling `create_or_reuse`). Keep `branch_from`, `save_named`, `get_full`, `status`, and the round-trip/status tests.

- [ ] **Step 5: Make batch creation require an explicit version**

In `src-tauri/src/repositories/batches.rs` `create`, replace the `let version_id = if let Some(vid) = input.version_id { … } else { create_or_reuse … }` block with unconditional validation:

```rust
        let version = recipe_versions::Entity::find_by_id(&input.version_id)
            .one(self.db)
            .await?
            .ok_or(AppError::NotFound)?;
        if version.recipe_id != input.recipe_id {
            return Err(AppError::NotFound);
        }
        let version_id = version.id;
```

(`input.version_id` is now `String`.) Update the seed helper near `batches.rs:301` that called `create_or_reuse` to instead create a version explicitly, e.g.:

```rust
        let version_id = RecipeVersionRepository::new(db)
            .save_named(&recipe_id, None)
            .await
            .unwrap()
            .id;
```

Fix any test in `batches.rs` that built `CreateBatchInput { version_id: None, .. }` to pass a real version id (create one via `save_named(&recipe_id, None)` first).

- [ ] **Step 6: Update commands**

In `src-tauri/src/commands/batches.rs`:
- `save_recipe_version`: its `input.name` is now `Option<String>`; pass it through to `save_named(&input.recipe_id, input.name)`.
- Add:

```rust
#[tauri::command]
pub async fn recipe_version_status(
    state: State<'_, AppState>,
    recipe_id: String,
) -> Result<RecipeVersionStatus, AppError> {
    RecipeVersionRepository::new(&state.db).status(&recipe_id).await
}
```

Add `RecipeVersionStatus` to the `use crate::models::{…}` imports at the top of the file.

In `src-tauri/src/lib.rs`, add to the `tauri::generate_handler![…]` list:

```rust
            commands::batches::recipe_version_status,
```

- [ ] **Step 7: Build and run the full backend suite**

Run: `cd src-tauri && cargo test`
Expected: PASS (no references to `create_or_reuse`/`matches_current` remain; `cargo build` clean).

- [ ] **Step 8: Commit**

```bash
git add docs/openapi/openapi.yaml src/lib/api.gen.ts src-tauri/src/models.gen.rs src-tauri/src/repositories/recipe_version.rs src-tauri/src/repositories/batches.rs src-tauri/src/commands/batches.rs src-tauri/src/lib.rs
git commit -m "feat: require explicit version on batch creation; drop silent reuse"
```

---

## Task 6: Frontend API bindings

**Files:**
- Modify: `src/lib/api.ts`

- [ ] **Step 1: Add the status binding and type**

In `src/lib/api.ts`, in the `// --- Recipe Versions ---` section, add:

```ts
export type RecipeVersionStatus = components["schemas"]["RecipeVersionStatus"];

export const recipeVersionStatus = (recipeId: string) =>
  invoke<RecipeVersionStatus>("recipe_version_status", { recipeId });
```

- [ ] **Step 2: Type-check**

Run: `just check`
Expected: 0 errors. (`createBatch`/`saveRecipeVersion` already consume the regenerated input types from Task 5.)

- [ ] **Step 3: Commit**

```bash
git add src/lib/api.ts
git commit -m "feat: add recipeVersionStatus API binding"
```

---

## Task 7: Brew version modal (shared, desktop + mobile)

A single modal drives the dirty branch and the optional version picker. Follows the existing runes + inline-style-token-free Tailwind patterns (use the `text-text-*` / `bg-bg-*` utilities).

**Files:**
- Create: `src/lib/components/BrewVersionModal.svelte`
- Test: `tests/BrewVersionModal.test.ts`

- [ ] **Step 1: Write the component**

Create `src/lib/components/BrewVersionModal.svelte`:

```svelte
<script lang="ts">
  import type { RecipeVersionSummary, RecipeVersionStatus } from "$lib/api";

  let {
    status,
    versions,
    onBrewCurrent,
    onBrewVersion,
    onCancel,
  }: {
    status: RecipeVersionStatus;
    versions: RecipeVersionSummary[];
    onBrewCurrent: (name: string | null) => void;
    onBrewVersion: (versionId: string) => void;
    onCancel: () => void;
  } = $props();

  let newName = $state("");
  let selected = $state(status.latest_version_id ?? versions[0]?.id ?? "");

  function label(v: RecipeVersionSummary): string {
    return v.name ? `v${v.version_number} · ${v.name}` : `v${v.version_number}`;
  }
</script>

<div class="fixed inset-0 z-50 flex items-center justify-center" style="background: rgba(0,0,0,0.4);">
  <div class="w-[420px] max-w-[92vw] rounded-lg p-4 flex flex-col gap-3 bg-bg-surface border border-border">
    <h2 class="text-base font-semibold text-text-primary">Choose a version to brew</h2>

    {#if status.has_unversioned_changes}
      <div class="flex flex-col gap-2 p-3 rounded bg-bg-elevated border border-border">
        <div class="text-sm text-text-primary">⚠ This recipe has un-versioned changes.</div>
        <input
          class="px-2 py-1.5 rounded text-sm bg-bg-base text-text-primary border border-border"
          placeholder="Name (optional)"
          bind:value={newName} />
        <button class="px-3 py-1.5 rounded text-sm bg-accent self-start" style="color: #fff;"
                onclick={() => onBrewCurrent(newName.trim() || null)}>
          Brew with current changes
        </button>
      </div>
    {/if}

    <div class="flex flex-col gap-1">
      <div class="text-xs text-text-secondary">Saved versions</div>
      <select bind:value={selected}
              class="px-2 py-1.5 rounded text-sm bg-bg-elevated text-text-primary border border-border">
        {#each versions as v}
          <option value={v.id}>{label(v)}</option>
        {/each}
      </select>
      <button class="px-3 py-1.5 rounded text-sm mt-1 bg-bg-elevated text-text-primary border border-border self-start"
              disabled={!selected} onclick={() => onBrewVersion(selected)}>
        Brew a saved version
      </button>
    </div>

    <button class="text-xs text-text-secondary self-end" onclick={onCancel}>Cancel</button>
  </div>
</div>
```

- [ ] **Step 2: Write the test**

Create `tests/BrewVersionModal.test.ts`:

```ts
import { render, fireEvent } from "@testing-library/svelte";
import { describe, it, expect, vi } from "vitest";
import BrewVersionModal from "$lib/components/BrewVersionModal.svelte";

const versions = [
  { id: "v2", version_number: 2, name: null },
  { id: "v1", version_number: 1, name: "first" },
] as any;

describe("BrewVersionModal", () => {
  it("shows the current-changes option only when dirty", () => {
    const clean = render(BrewVersionModal, {
      props: {
        status: { version_count: 2, latest_version_id: "v2", has_unversioned_changes: false },
        versions, onBrewCurrent: vi.fn(), onBrewVersion: vi.fn(), onCancel: vi.fn(),
      },
    });
    expect(clean.queryByText(/un-versioned changes/i)).toBeNull();
  });

  it("emits onBrewCurrent with the typed name", async () => {
    const onBrewCurrent = vi.fn();
    const { getByPlaceholderText, getByText } = render(BrewVersionModal, {
      props: {
        status: { version_count: 1, latest_version_id: "v2", has_unversioned_changes: true },
        versions, onBrewCurrent, onBrewVersion: vi.fn(), onCancel: vi.fn(),
      },
    });
    await fireEvent.input(getByPlaceholderText("Name (optional)"), { target: { value: "hop bump" } });
    await fireEvent.click(getByText("Brew with current changes"));
    expect(onBrewCurrent).toHaveBeenCalledWith("hop bump");
  });

  it("emits onBrewVersion with the selected id", async () => {
    const onBrewVersion = vi.fn();
    const { getByText } = render(BrewVersionModal, {
      props: {
        status: { version_count: 2, latest_version_id: "v2", has_unversioned_changes: false },
        versions, onBrewCurrent: vi.fn(), onBrewVersion, onCancel: vi.fn(),
      },
    });
    await fireEvent.click(getByText("Brew a saved version"));
    expect(onBrewVersion).toHaveBeenCalledWith("v2");
  });
});
```

- [ ] **Step 3: Run the test**

Run: `bun run test -- BrewVersionModal`
Expected: PASS. (If selector queries need adjustment to match the markup, fix the test, not the behavior.)

- [ ] **Step 4: Commit**

```bash
git add src/lib/components/BrewVersionModal.svelte tests/BrewVersionModal.test.ts
git commit -m "feat: add shared BrewVersionModal for version selection at brew time"
```

---

## Task 8: Shared brew-flow helper

Centralizes the three-branch decision so both entry points and both platforms share it.

**Files:**
- Create: `src/lib/brewFlow.ts`
- Test: `tests/brewFlow.test.ts`

- [ ] **Step 1: Write the helper**

Create `src/lib/brewFlow.ts`:

```ts
import {
  recipeVersionStatus,
  listRecipeVersions,
  saveRecipeVersion,
  createBatch,
  type Batch,
  type RecipeVersionStatus,
  type RecipeVersionSummary,
} from "$lib/api";
import { ipc } from "$lib/stores/error";

export type BrewDecision =
  | { kind: "auto"; batch: Batch }                                   // 0 versions or clean: created directly
  | { kind: "prompt"; status: RecipeVersionStatus; versions: RecipeVersionSummary[] };

/**
 * Decides how to brew `recipeId`:
 * - 0 versions  → snapshot v1 + create batch (kind: "auto")
 * - clean       → create batch on latest version (kind: "auto")
 * - dirty       → return status + versions for the caller to show BrewVersionModal (kind: "prompt")
 */
export async function startBrew(recipeId: string): Promise<BrewDecision | null> {
  const status = await ipc(recipeVersionStatus(recipeId));
  if (!status) return null;

  if (status.version_count === 0) {
    const v = await ipc(saveRecipeVersion({ recipe_id: recipeId, name: null }));
    if (!v) return null;
    const batch = await ipc(createBatch({ recipe_id: recipeId, version_id: v.id, name: null }));
    return batch ? { kind: "auto", batch } : null;
  }

  if (!status.has_unversioned_changes && status.latest_version_id) {
    const batch = await ipc(
      createBatch({ recipe_id: recipeId, version_id: status.latest_version_id, name: null })
    );
    return batch ? { kind: "auto", batch } : null;
  }

  const versions = (await ipc(listRecipeVersions(recipeId))) ?? [];
  return { kind: "prompt", status, versions };
}

/** "Brew with current changes": snapshot (optional name) then create the batch. */
export async function brewCurrent(recipeId: string, name: string | null): Promise<Batch | null> {
  const v = await ipc(saveRecipeVersion({ recipe_id: recipeId, name }));
  if (!v) return null;
  return (await ipc(createBatch({ recipe_id: recipeId, version_id: v.id, name: null }))) ?? null;
}

/** "Brew a saved version". */
export async function brewVersion(recipeId: string, versionId: string): Promise<Batch | null> {
  return (await ipc(createBatch({ recipe_id: recipeId, version_id: versionId, name: null }))) ?? null;
}
```

- [ ] **Step 2: Write the test**

Create `tests/brewFlow.test.ts`:

```ts
import { describe, it, expect, vi, beforeEach } from "vitest";

vi.mock("$lib/api", () => ({
  recipeVersionStatus: vi.fn(),
  listRecipeVersions: vi.fn(),
  saveRecipeVersion: vi.fn(),
  createBatch: vi.fn(),
}));
vi.mock("$lib/stores/error", () => ({ ipc: (p: any) => p }));

import * as api from "$lib/api";
import { startBrew } from "$lib/brewFlow";

beforeEach(() => vi.clearAllMocks());

describe("startBrew", () => {
  it("auto-snapshots v1 when there are no versions", async () => {
    (api.recipeVersionStatus as any).mockResolvedValue({ version_count: 0, latest_version_id: null, has_unversioned_changes: false });
    (api.saveRecipeVersion as any).mockResolvedValue({ id: "v1" });
    (api.createBatch as any).mockResolvedValue({ id: "b1" });
    const r = await startBrew("r1");
    expect(api.saveRecipeVersion).toHaveBeenCalledWith({ recipe_id: "r1", name: null });
    expect(r).toEqual({ kind: "auto", batch: { id: "b1" } });
  });

  it("auto-creates on latest when clean", async () => {
    (api.recipeVersionStatus as any).mockResolvedValue({ version_count: 2, latest_version_id: "v2", has_unversioned_changes: false });
    (api.createBatch as any).mockResolvedValue({ id: "b2" });
    const r = await startBrew("r1");
    expect(api.createBatch).toHaveBeenCalledWith({ recipe_id: "r1", version_id: "v2", name: null });
    expect(r).toEqual({ kind: "auto", batch: { id: "b2" } });
  });

  it("prompts when dirty", async () => {
    (api.recipeVersionStatus as any).mockResolvedValue({ version_count: 1, latest_version_id: "v1", has_unversioned_changes: true });
    (api.listRecipeVersions as any).mockResolvedValue([{ id: "v1", version_number: 1, name: null }]);
    const r = await startBrew("r1");
    expect(r?.kind).toBe("prompt");
  });
});
```

- [ ] **Step 3: Run the test**

Run: `bun run test -- brewFlow`
Expected: PASS.

- [ ] **Step 4: Commit**

```bash
git add src/lib/brewFlow.ts tests/brewFlow.test.ts
git commit -m "feat: add shared brew-flow helper (auto/prompt branches)"
```

---

## Task 9: Wire entry points to the brew flow (desktop + mobile)

**Files:**
- Modify: `src/lib/desktop/BatchesHome.svelte`, `src/lib/mobile/BatchesHome.svelte`
- Modify: `src/lib/components/tabs/BatchesTab.svelte`

- [ ] **Step 1: Desktop BatchesHome — replace the version branch**

In `src/lib/desktop/BatchesHome.svelte`:
- Import: `import BrewVersionModal from "$lib/components/BrewVersionModal.svelte";` and `import { startBrew, brewCurrent, brewVersion } from "$lib/brewFlow";` and the `RecipeVersionStatus` type.
- Replace the existing `handlePickRecipe` / `handlePickVersion` logic with:

```ts
  let promptStatus = $state<import("$lib/api").RecipeVersionStatus | null>(null);
  let promptVersions = $state<import("$lib/api").RecipeVersionSummary[]>([]);
  let promptRecipeId = $state<string | null>(null);

  async function handlePickRecipe(recipe: RecipeSummary) {
    showPicker = false;
    const decision = await startBrew(recipe.id);
    if (!decision) return;
    if (decision.kind === "auto") {
      await ipc(refreshBatchList());
      goto(`/batches/${decision.batch.id}`);
      return;
    }
    promptRecipeId = recipe.id;
    promptStatus = decision.status;
    promptVersions = decision.versions;
  }

  async function finishBrew(batch: { id: string } | null) {
    promptStatus = null;
    if (!batch) return;
    await ipc(refreshBatchList());
    goto(`/batches/${batch.id}`);
  }
```

- Replace the `step === "version"` picker markup with the modal (keep the recipe-picker markup):

```svelte
{#if promptStatus && promptRecipeId}
  <BrewVersionModal
    status={promptStatus}
    versions={promptVersions}
    onBrewCurrent={async (name) => finishBrew(await brewCurrent(promptRecipeId!, name))}
    onBrewVersion={async (vid) => finishBrew(await brewVersion(promptRecipeId!, vid))}
    onCancel={() => (promptStatus = null)} />
{/if}
```

Remove the now-unused `step`/`versions`/`handlePickVersion`/`handleBack` version-picker state.

- [ ] **Step 2: Mirror in mobile BatchesHome**

Apply the same change to `src/lib/mobile/BatchesHome.svelte` (same imports, same handler/markup, using its existing `goto`/`refreshBatchList` equivalents).

- [ ] **Step 3: Route BatchesTab through the flow**

In `src/lib/components/tabs/BatchesTab.svelte`, replace the direct `createBatch({ recipe_id, name: null })` call with the shared flow + modal (same pattern as Step 1, using `recipeId`). The tab is already inside a recipe, so there is no recipe-pick step — call `startBrew(recipeId)` directly on the "new batch" action and render `BrewVersionModal` on the prompt branch.

- [ ] **Step 4: Type-check and run frontend tests**

Run: `just check && bun run test`
Expected: 0 type errors; all tests pass.

- [ ] **Step 5: Commit**

```bash
git add src/lib/desktop/BatchesHome.svelte src/lib/mobile/BatchesHome.svelte src/lib/components/tabs/BatchesTab.svelte
git commit -m "feat: route batch creation through unified brew flow (desktop+mobile)"
```

---

## Task 10: Dirty badge + Save-as-version on the recipe view (desktop + mobile)

**Files:**
- Modify: `src/lib/components/VersionHistoryPanel.svelte` (badge + save action)
- Modify: `src/lib/desktop/RecipeView.svelte`, `src/lib/mobile/RecipeView.svelte` (fetch status, pass down; mobile gets a host row)

- [ ] **Step 1: Add status fetch to desktop RecipeView**

In `src/lib/desktop/RecipeView.svelte`:
- Import `recipeVersionStatus, saveRecipeVersion, type RecipeVersionStatus`.
- Add state + loader, refreshed on load and after edits:

```ts
  let versionStatus = $state<RecipeVersionStatus | null>(null);
  async function refreshVersionStatus() {
    if (displayRecipe) versionStatus = await ipc(recipeVersionStatus(displayRecipe.id));
  }
```

- Call `refreshVersionStatus()` inside `onMount` after the recipe loads and at the end of `refreshRecipe()`.

- [ ] **Step 2: Badge + save action in VersionHistoryPanel**

In `src/lib/components/VersionHistoryPanel.svelte`, add props `hasUnversionedChanges: boolean` and `onSaved: () => void`, and render near the panel header:

```svelte
{#if hasUnversionedChanges}
  <div class="flex items-center justify-between gap-2 mb-2 px-2 py-1.5 rounded bg-bg-elevated border border-border">
    <span class="text-xs text-text-secondary">⚠ un-versioned changes</span>
    <button class="text-xs px-2 py-1 rounded bg-accent" style="color: #fff;"
            onclick={saveCurrent}>Save as version</button>
  </div>
{/if}
```

```ts
  import { saveRecipeVersion } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  let { recipeId, hasUnversionedChanges, onSaved /* …existing props… */ } = $props();
  async function saveCurrent() {
    const name = window.prompt("Version name (optional)") ?? null;
    const v = await ipc(saveRecipeVersion({ recipe_id: recipeId, name: name?.trim() || null }));
    if (v) onSaved();
  }
```

Pass the new props from desktop RecipeView's `<VersionHistoryPanel … hasUnversionedChanges={versionStatus?.has_unversioned_changes ?? false} onSaved={refreshVersionStatus} recipeId={displayRecipe.id} />`. (If a `prompt`-based dialog is undesirable, reuse `BrewVersionModal`'s name field in a tiny inline modal; `window.prompt` is acceptable for v1 and matches the app's existing simple dialogs.)

- [ ] **Step 3: Mobile parity**

In `src/lib/mobile/RecipeView.svelte`, add the same `versionStatus` fetch and render a compact row hosting the badge + "Save as version" button (mobile has no `VersionHistoryPanel`). Use the same `saveRecipeVersion` + `refreshVersionStatus` wiring and the `text-text-*` / `bg-bg-*` utilities.

- [ ] **Step 4: Type-check and run frontend tests**

Run: `just check && bun run test`
Expected: 0 type errors; tests pass.

- [ ] **Step 5: Commit**

```bash
git add src/lib/components/VersionHistoryPanel.svelte src/lib/desktop/RecipeView.svelte src/lib/mobile/RecipeView.svelte
git commit -m "feat: surface un-versioned changes badge + save-as-version (desktop+mobile)"
```

---

## Task 11: Docs — shared/live profiles contract

**Files:**
- Modify: the user-facing docs site (`docs/site/`), in the recipe-versioning / batches page.

- [ ] **Step 1: Add the note**

Add a short subsection where versioning is described, stating: equipment profiles and source-water profiles are **shared and live** — editing a profile's values is reflected immediately in all recipes and batches that reference it and is intentionally not captured by recipe versions. Switching *which* profile a recipe uses *is* recorded as a recipe change. Follow the docs writing-style memory (voice/formatting).

- [ ] **Step 2: Build docs to verify**

Run: `bun run docs:build`
Expected: builds without error.

- [ ] **Step 3: Commit**

```bash
git add docs/site
git commit -m "docs: note that equipment/water profiles are shared and live, not versioned"
```

---

## Final verification

- [ ] `cd src-tauri && cargo test` — all pass
- [ ] `just check` — 0 type errors
- [ ] `bun run test` — all pass
- [ ] `cargo clippy --manifest-path src-tauri/Cargo.toml` — clean
- [ ] Manual smoke (via `just` dev run): brew an unchanged recipe (one click → latest); edit it (badge appears); brew again (prompt → "brew with current changes" with a name → new version + batch); confirm "save as version" clears the badge.
