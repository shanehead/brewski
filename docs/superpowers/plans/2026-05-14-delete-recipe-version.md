# Delete Recipe Version Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Allow users to delete individual recipe versions from the version history panel, with safe handling of child versions, batch references, and recipe branch pointers.

**Architecture:** Add a `delete` method to `RecipeVersionRepository` that pre-checks for batch references, re-parents child versions, nullifies any recipe `branch_parent_id` pointing at the deleted version, then deletes the row (cascades clean up all snapshot tables). Wire it up as a Tauri command and add a "Delete" button to the version history panel's expanded row.

**Tech Stack:** Rust/SeaORM (backend), Tauri commands, SvelteKit + TypeScript (frontend)

---

## File Map

| File | Change |
|---|---|
| `src-tauri/src/error.rs` | Add `Conflict(String)` variant to `AppError` |
| `src-tauri/src/repositories/recipe_version.rs` | Add `delete` method + tests |
| `src-tauri/src/commands/batches.rs` | Add `delete_recipe_version` command |
| `src-tauri/src/lib.rs` | Register `delete_recipe_version` in `generate_handler!` |
| `src/lib/api.ts` | Add `deleteRecipeVersion` wrapper |
| `src/lib/components/VersionHistoryPanel.svelte` | Add `ondelete` prop + Delete button |
| `src/routes/recipe/[id]/+page.svelte` | Add `handleDeleteVersion` handler |

---

### Task 1: Add `Conflict` variant to `AppError`

**Files:**
- Modify: `src-tauri/src/error.rs`

- [ ] **Step 1: Write the failing test**

Add to the `#[cfg(test)]` block in `src-tauri/src/error.rs`:

```rust
#[test]
fn test_serialize_conflict() {
    assert_eq!(
        serde_json::to_string(&AppError::Conflict("version is referenced by a batch".into())).unwrap(),
        "\"conflict: version is referenced by a batch\""
    );
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cd src-tauri && cargo test error::tests::test_serialize_conflict 2>&1 | tail -5
```

Expected: compile error — `AppError::Conflict` does not exist.

- [ ] **Step 3: Add the `Conflict` variant**

In `src-tauri/src/error.rs`, update the enum:

```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(#[from] sea_orm::DbErr),
    #[error("conversion error: {0}")]
    Conversion(String),
    #[error("not found")]
    NotFound,
    #[error("internal error: {0}")]
    Internal(String),
    #[error("conflict: {0}")]
    Conflict(String),
}
```

- [ ] **Step 4: Run test to verify it passes**

```bash
cd src-tauri && cargo test error::tests 2>&1 | tail -5
```

Expected: all error tests PASS.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/error.rs
git commit -m "feat(backend): add Conflict variant to AppError"
```

---

### Task 2: Add `delete` method to `RecipeVersionRepository`

**Files:**
- Modify: `src-tauri/src/repositories/recipe_version.rs`

The method must:
1. Check if any batch references this version → `AppError::Conflict` if so
2. Re-parent child versions (set their `parent_version_id` to the deleted version's `parent_version_id`)
3. Nullify `recipes.branch_parent_id` if it points to this version
4. Delete the version row (cascades handle all `recipe_version_*` snapshot tables)

- [ ] **Step 1: Write failing tests**

Add to the `#[cfg(test)]` block at the bottom of `src-tauri/src/repositories/recipe_version.rs`:

```rust
#[tokio::test]
async fn test_delete_removes_version() {
    let db = setup_test_db().await;
    let recipe_id = make_recipe(&db).await;
    let repo = RecipeVersionRepository::new(&db);
    let v = repo.create_or_reuse(&recipe_id).await.unwrap();

    repo.delete(&v.id).await.unwrap();

    let versions = repo.list_for_recipe(&recipe_id).await.unwrap();
    assert!(versions.is_empty());
}

#[tokio::test]
async fn test_delete_reparents_children() {
    let db = setup_test_db().await;
    let recipe_id = make_recipe(&db).await;
    let repo = RecipeVersionRepository::new(&db);

    // Create v1
    let v1 = repo.create_or_reuse(&recipe_id).await.unwrap();

    // Create v2 (child of v1) by changing and re-snapshotting
    FermentableRepository::new(&db)
        .create(
            &recipe_id,
            CreateFermentableAdditionInput {
                fermentable_id: None,
                name: "Pale Malt".into(),
                type_: "grain".into(),
                yield_pct: 78.0,
                color_lovibond: 1.8,
                amount_kg: 4.5,
                add_after_boil: None,
            },
        )
        .await
        .unwrap();
    let v2 = repo.create_or_reuse(&recipe_id).await.unwrap();
    assert_eq!(v2.parent_version_id.as_deref(), Some(v1.id.as_str()));

    // Save a named v3 as child of v2
    let v3 = repo.save_named(&recipe_id, "checkpoint").await.unwrap();
    assert_eq!(v3.parent_version_id.as_deref(), Some(v2.id.as_str()));

    // Delete v2; v3 should be re-parented to v1
    repo.delete(&v2.id).await.unwrap();

    let versions = repo.list_for_recipe(&recipe_id).await.unwrap();
    let v3_updated = versions.iter().find(|v| v.id == v3.id).unwrap();
    assert_eq!(v3_updated.parent_version_id.as_deref(), Some(v1.id.as_str()));
}

#[tokio::test]
async fn test_delete_nullifies_recipe_branch_parent_id() {
    let db = setup_test_db().await;
    let recipe_id = make_recipe(&db).await;
    let repo = RecipeVersionRepository::new(&db);

    let v1 = repo.create_or_reuse(&recipe_id).await.unwrap();

    // Simulate branch_from having been called
    crate::entities::recipes::ActiveModel {
        id: Set(recipe_id.clone()),
        branch_parent_id: Set(Some(v1.id.clone())),
        ..Default::default()
    }
    .update(&db)
    .await
    .unwrap();

    repo.delete(&v1.id).await.unwrap();

    let recipe_row = crate::entities::recipes::Entity::find_by_id(&recipe_id)
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    assert!(recipe_row.branch_parent_id.is_none());
}

#[tokio::test]
async fn test_delete_blocked_when_batch_references_version() {
    let db = setup_test_db().await;
    let recipe_id = make_recipe(&db).await;
    let repo = RecipeVersionRepository::new(&db);

    // Creating a batch auto-creates a version via create_or_reuse
    let batch = crate::repositories::batches::BatchRepository::new(&db)
        .create(crate::models::CreateBatchInput {
            recipe_id: recipe_id.clone(),
            name: None,
        })
        .await
        .unwrap();

    let versions = repo.list_for_recipe(&recipe_id).await.unwrap();
    let version_id = versions[0].id.clone();

    let result = repo.delete(&version_id).await;
    assert!(matches!(result, Err(crate::error::AppError::Conflict(_))));

    // Batch should still exist
    let _ = crate::repositories::batches::BatchRepository::new(&db)
        .get(&batch.id)
        .await
        .unwrap();
}
```

- [ ] **Step 2: Run tests to verify they fail**

```bash
cd src-tauri && cargo test repositories::recipe_version::tests::test_delete 2>&1 | tail -10
```

Expected: compile errors — `delete` method does not exist.

- [ ] **Step 3: Implement the `delete` method**

Add to the `impl<'a> RecipeVersionRepository<'a>` block in `src-tauri/src/repositories/recipe_version.rs`, before the `#[cfg(test)]` block:

```rust
pub async fn delete(&self, version_id: &str) -> Result<(), AppError> {
    // 1. Confirm version exists
    let version = recipe_versions::Entity::find_by_id(version_id)
        .one(self.db)
        .await?
        .ok_or(AppError::NotFound)?;

    // 2. Block if any batch references this version
    let batch_count = crate::entities::batches::Entity::find()
        .filter(crate::entities::batches::Column::RecipeVersionId.eq(version_id))
        .count(self.db)
        .await?;
    if batch_count > 0 {
        return Err(AppError::Conflict(
            "version is referenced by a batch".to_string(),
        ));
    }

    // 3. Re-parent child versions
    recipe_versions::Entity::update_many()
        .col_expr(
            recipe_versions::Column::ParentVersionId,
            sea_orm::sea_query::Expr::value(version.parent_version_id.clone()),
        )
        .filter(recipe_versions::Column::ParentVersionId.eq(version_id))
        .exec(self.db)
        .await?;

    // 4. Nullify recipes.branch_parent_id if it points here
    recipes::Entity::update_many()
        .col_expr(
            recipes::Column::BranchParentId,
            sea_orm::sea_query::Expr::value::<Option<String>>(None),
        )
        .filter(recipes::Column::BranchParentId.eq(version_id))
        .exec(self.db)
        .await?;

    // 5. Delete the version (cascades clean up all recipe_version_* tables)
    recipe_versions::Entity::delete_by_id(version_id)
        .exec(self.db)
        .await?;

    Ok(())
}
```

Also add the `batches` entity import at the top of the file (inside the existing `use crate::entities::{ ... }` block):

```rust
use crate::entities::{
    batches,
    equipment_profiles, mash_steps, mashes, recipe_addition_fermentables, recipe_addition_hops,
    recipe_addition_miscs, recipe_addition_waters, recipe_addition_yeasts,
    recipe_version_fermentables, recipe_version_hops, recipe_version_mash,
    recipe_version_mash_steps, recipe_version_miscs, recipe_version_water_adjustments,
    recipe_version_waters, recipe_version_yeasts, recipe_versions, recipe_water_adjustments,
    recipes, styles,
};
```

- [ ] **Step 4: Run tests to verify they pass**

```bash
cd src-tauri && cargo test repositories::recipe_version::tests::test_delete 2>&1 | tail -10
```

Expected: all 4 `test_delete_*` tests PASS.

- [ ] **Step 5: Run full test suite to check for regressions**

```bash
cd src-tauri && cargo test 2>&1 | tail -10
```

Expected: all tests PASS.

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/repositories/recipe_version.rs
git commit -m "feat(backend): add delete method to RecipeVersionRepository"
```

---

### Task 3: Add `delete_recipe_version` Tauri command

**Files:**
- Modify: `src-tauri/src/commands/batches.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Add the command to `batches.rs`**

Append to the end of `src-tauri/src/commands/batches.rs`:

```rust
#[tauri::command]
pub async fn delete_recipe_version(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), AppError> {
    RecipeVersionRepository::new(&state.db)
        .delete(&id)
        .await
}
```

- [ ] **Step 2: Register the command in `lib.rs`**

In `src-tauri/src/lib.rs`, find the `generate_handler!` block and add after `branch_from_version`:

```rust
commands::batches::branch_from_version,
commands::batches::delete_recipe_version,
```

- [ ] **Step 3: Verify the backend compiles**

```bash
cd src-tauri && cargo build 2>&1 | tail -5
```

Expected: `Finished` with no errors.

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/commands/batches.rs src-tauri/src/lib.rs
git commit -m "feat(backend): add delete_recipe_version command"
```

---

### Task 4: Frontend — API wrapper and VersionHistoryPanel

**Files:**
- Modify: `src/lib/api.ts`
- Modify: `src/lib/components/VersionHistoryPanel.svelte`

- [ ] **Step 1: Add the API wrapper**

In `src/lib/api.ts`, after the `branchFromVersion` line:

```ts
export const deleteRecipeVersion = (id: string) =>
  invoke<void>("delete_recipe_version", { id });
```

- [ ] **Step 2: Add `ondelete` prop to `VersionHistoryPanel`**

In `src/lib/components/VersionHistoryPanel.svelte`, update the props destructuring (lines 3–16):

```svelte
<script lang="ts">
  import type { RecipeVersionSummary } from "$lib/api";

  let {
    versions,
    viewingVersionId,
    onview,
    onbranch,
    ondelete,
    onclose,
  }: {
    versions: RecipeVersionSummary[];
    viewingVersionId: string | null;
    onview: (version: RecipeVersionSummary) => void;
    onbranch: (version: RecipeVersionSummary) => void;
    ondelete: (version: RecipeVersionSummary) => void;
    onclose: () => void;
  } = $props();
```

- [ ] **Step 3: Add the Delete button to the expanded version row**

In `src/lib/components/VersionHistoryPanel.svelte`, find the section that renders the "Branch from here" button (around line 93):

```svelte
        {#if viewingVersionId === version.id}
          <button
            onclick={() => onbranch(version)}
            class="mt-1 text-xs px-2 py-0.5 rounded"
            style="background: var(--color-accent); color: #fff;"
          >
            Branch from here
          </button>
        {/if}
```

Replace it with:

```svelte
        {#if viewingVersionId === version.id}
          <div class="mt-1 flex gap-1">
            <button
              onclick={() => onbranch(version)}
              class="text-xs px-2 py-0.5 rounded"
              style="background: var(--color-accent); color: #fff;"
            >
              Branch from here
            </button>
            <button
              onclick={(e) => { e.stopPropagation(); ondelete(version); }}
              class="text-xs px-2 py-0.5 rounded"
              style="background: var(--color-bg-elevated); color: var(--color-text-muted); border: 1px solid var(--color-border);"
            >
              Delete
            </button>
          </div>
        {/if}
```

- [ ] **Step 4: Verify TypeScript compiles**

```bash
npm run check 2>&1 | tail -10
```

Expected: no type errors.

- [ ] **Step 5: Commit**

```bash
git add src/lib/api.ts src/lib/components/VersionHistoryPanel.svelte
git commit -m "feat(frontend): add deleteRecipeVersion api wrapper and Delete button to VersionHistoryPanel"
```

---

### Task 5: Frontend — wire up `handleDeleteVersion` in recipe page

**Files:**
- Modify: `src/routes/recipe/[id]/+page.svelte`

- [ ] **Step 1: Add the import**

In `src/routes/recipe/[id]/+page.svelte`, update the import from `$lib/api`:

```ts
  import {
    getRecipe,
    getRecipeStats,
    updateRecipe,
    listRecipeVersions,
    getRecipeVersion,
    saveRecipeVersion,
    branchFromVersion,
    deleteRecipeVersion,
  } from "$lib/api";
```

- [ ] **Step 2: Add the handler function**

In `src/routes/recipe/[id]/+page.svelte`, after the `handleBranchFromVersion` function:

```ts
  async function handleDeleteVersion(version: RecipeVersionSummary) {
    if (!recipe) return;
    const confirmed = confirm(
      `Delete v${version.version_number}${version.name ? ` "${version.name}"` : ""}? This cannot be undone.`
    );
    if (!confirmed) return;
    await ipc(deleteRecipeVersion(version.id));
    await refreshRecipe();
  }
```

- [ ] **Step 3: Pass `ondelete` to `VersionHistoryPanel`**

Find the `<VersionHistoryPanel` usage in `src/routes/recipe/[id]/+page.svelte` and add the `ondelete` prop:

```svelte
        <VersionHistoryPanel
          {versions}
          viewingVersionId={viewingVersion?.id ?? null}
          onview={handleViewVersion}
          onbranch={handleBranchFromVersion}
          ondelete={handleDeleteVersion}
          onclose={() => { showVersionPanel = false; }}
        />
```

- [ ] **Step 4: Verify TypeScript compiles**

```bash
npm run check 2>&1 | tail -10
```

Expected: no type errors.

- [ ] **Step 5: Manual smoke test**

1. Start the app: `npm run tauri dev`
2. Open a recipe, open Version History
3. Click a version to expand it — confirm "Delete" button appears next to "Branch from here"
4. Click Delete, dismiss the confirm dialog — version should remain
5. Click Delete, confirm — version should disappear from the list
6. Create a batch for a recipe, open Version History, try to delete the version linked to the batch — confirm an error is shown (the `ipc` error store will surface it as a notification/alert)

- [ ] **Step 6: Commit**

```bash
git add src/routes/recipe/\[id\]/+page.svelte
git commit -m "feat(frontend): wire up handleDeleteVersion on recipe page"
```
