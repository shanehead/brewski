# Batch Brew Day Recording Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Improve the batch overview with a stage callout banner showing planned targets, contextual measurement highlighting, a unified notes field, date auto-fill on status advance, and a new Conditioning stage.

**Architecture:** Four-task sequence: DB migration → type regeneration → Rust repository update → Svelte UI update. The migration recreates the `batches` table (SQLite constraint change requires full rebuild). Types flow from OpenAPI YAML → `just gen` → both `api.gen.ts` and `models.gen.rs`. The repository's `get()` loads planned targets by calling `calculate_stats` on the linked recipe version. The shared `BatchOverviewTab.svelte` handles all UI; no platform-specific batch files need changing.

**Tech Stack:** SQLite migration, SeaORM entity codegen (`just gen-entities`), cargo typify (`just gen-rust`), openapi-typescript (`just gen-ts`), Svelte 5 runes, Tailwind + CSS vars.

---

## File Map

| File | Action |
|------|--------|
| `src-tauri/migrations/006_batch_brew_day.sql` | Create — table recreation migration |
| `src-tauri/src/entities/batches.rs` | Regenerated via `just gen-entities` |
| `docs/openapi/components/schemas/Batch.yaml` | Modify — add new fields, remove old notes |
| `docs/openapi/components/schemas/UpdateBatchInput.yaml` | Modify — add conditioning_date + notes, remove old notes |
| `docs/openapi/components/schemas/BatchSummary.yaml` | Modify — update status description |
| `src/lib/api.gen.ts` | Regenerated via `just gen-ts` |
| `src-tauri/src/models.gen.rs` | Regenerated via `just gen-rust` |
| `src-tauri/src/repositories/batches.rs` | Modify — update create/update/get, add planned targets |
| `src/lib/components/batch/BatchOverviewTab.svelte` | Modify — full UI update |

---

## Task 1: DB Migration + SeaORM Entity Regeneration

**Files:**
- Create: `src-tauri/migrations/006_batch_brew_day.sql`
- Regenerated: `src-tauri/src/entities/batches.rs`
- Modify: `src-tauri/src/repositories/batches.rs` (fix compilation after entity change)

- [ ] **Step 1: Write the migration file**

Create `src-tauri/migrations/006_batch_brew_day.sql`:

```sql
-- Recreate batches with updated schema:
--   • status: adds 'conditioning', removes 'complete' (existing 'complete' rows → 'packaged')
--   • conditioning_date: new stage date column
--   • notes: replaces brew_day_notes / fermentation_notes / tasting_notes
CREATE TABLE batches_new (
  id                        TEXT PRIMARY KEY,
  recipe_id                 TEXT NOT NULL REFERENCES recipes(id) ON DELETE RESTRICT,
  recipe_version_id         TEXT NOT NULL REFERENCES recipe_versions(id),
  name                      TEXT,
  status                    TEXT NOT NULL DEFAULT 'planned'
                              CHECK (status IN ('planned', 'brewing', 'fermenting', 'conditioning', 'packaged')),
  brew_date                 INTEGER,
  fermenter_date            INTEGER,
  conditioning_date         INTEGER,
  packaging_date            INTEGER,
  actual_pre_boil_volume_l  REAL,
  actual_post_boil_volume_l REAL,
  actual_batch_size_l       REAL,
  actual_pre_boil_gravity   REAL,
  actual_og                 REAL,
  actual_fg                 REAL,
  notes                     TEXT,
  rating                    INTEGER CHECK (rating IS NULL OR (rating >= 1 AND rating <= 10)),
  created_at                INTEGER NOT NULL,
  updated_at                INTEGER NOT NULL
);

INSERT INTO batches_new (
  id, recipe_id, recipe_version_id, name,
  status,
  brew_date, fermenter_date, conditioning_date, packaging_date,
  actual_pre_boil_volume_l, actual_post_boil_volume_l, actual_batch_size_l,
  actual_pre_boil_gravity, actual_og, actual_fg,
  notes, rating, created_at, updated_at
)
SELECT
  id, recipe_id, recipe_version_id, name,
  CASE WHEN status = 'complete' THEN 'packaged' ELSE status END,
  brew_date, fermenter_date, NULL, packaging_date,
  actual_pre_boil_volume_l, actual_post_boil_volume_l, actual_batch_size_l,
  actual_pre_boil_gravity, actual_og, actual_fg,
  brew_day_notes, rating, created_at, updated_at
FROM batches;

DROP TABLE batches;
ALTER TABLE batches_new RENAME TO batches;

CREATE INDEX IF NOT EXISTS idx_batches_recipe_id ON batches(recipe_id);
CREATE INDEX IF NOT EXISTS idx_batches_recipe_version_id ON batches(recipe_version_id);
```

- [ ] **Step 2: Apply the migration**

```bash
just migrate
```

Expected output: `Applying migration: m006_batch_brew_day` (or similar). If it fails, check the SQL syntax.

- [ ] **Step 3: Regenerate SeaORM entities from the updated DB**

```bash
just gen-entities
```

Expected: `src-tauri/src/entities/batches.rs` is rewritten. Open it and confirm it contains `conditioning_date: Option<i32>`, `notes: Option<String>`, and no longer has `brew_day_notes`, `fermentation_notes`, or `tasting_notes`.

- [ ] **Step 4: Fix compilation errors in `batches.rs` repository**

The repository's `create()` and `update()` methods still reference the old entity fields. Run:

```bash
cd src-tauri && cargo build 2>&1 | grep "error\[" | head -20
```

Expected: errors referencing `brew_day_notes`, `fermentation_notes`, `tasting_notes`, and missing `conditioning_date`/`notes`.

Update `src-tauri/src/repositories/batches.rs` — replace the `create()` `ActiveModel` block and `update()` field handlers.

In `create()`, replace the entire `batches::ActiveModel { ... }.insert(...)` block with:

```rust
batches::ActiveModel {
    id: Set(id.clone()),
    recipe_id: Set(input.recipe_id),
    recipe_version_id: Set(version.id),
    name: Set(input.name),
    status: Set("planned".to_string()),
    brew_date: Set(None),
    fermenter_date: Set(None),
    conditioning_date: Set(None),
    packaging_date: Set(None),
    actual_pre_boil_volume_l: Set(None),
    actual_post_boil_volume_l: Set(None),
    actual_batch_size_l: Set(None),
    actual_pre_boil_gravity: Set(None),
    actual_og: Set(None),
    actual_fg: Set(None),
    notes: Set(None),
    rating: Set(None),
    created_at: Set(now),
    updated_at: Set(now),
}
.insert(self.db)
.await?;
```

In `update()`, remove these three blocks entirely:

```rust
// DELETE these three blocks:
if let Some(v) = input.brew_day_notes {
    active.brew_day_notes = Set(Some(v));
}
if let Some(v) = input.fermentation_notes {
    active.fermentation_notes = Set(Some(v));
}
if let Some(v) = input.tasting_notes {
    active.tasting_notes = Set(Some(v));
}
```

And add these two blocks after `if let Some(v) = input.packaging_date { ... }`:

```rust
if let Some(v) = input.conditioning_date {
    active.conditioning_date = Set(Some(v as i32));
}
if let Some(v) = input.notes {
    active.notes = Set(Some(v));
}
```

- [ ] **Step 5: Write a failing test for conditioning_date in update**

Add to the `#[cfg(test)]` block in `batches.rs`:

```rust
#[tokio::test]
async fn test_update_conditioning_date_and_notes() {
    let db = setup_test_db().await;
    let (recipe_id, _) = setup(&db).await;
    let repo = BatchRepository::new(&db);
    let batch = repo
        .create(CreateBatchInput { recipe_id, name: None })
        .await
        .unwrap();
    let updated = repo
        .update(
            &batch.id,
            UpdateBatchInput {
                status: Some("conditioning".into()),
                conditioning_date: Some(1_700_000_000),
                notes: Some("Dry hop day 3".into()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(updated.status, "conditioning");
    assert_eq!(updated.conditioning_date, Some(1_700_000_000));
    assert_eq!(updated.notes, Some("Dry hop day 3".into()));
}
```

- [ ] **Step 6: Run the test (expected: compile error — fields not on model yet)**

```bash
cd src-tauri && cargo test repositories::batches::tests::test_update_conditioning_date_and_notes 2>&1 | tail -20
```

Expected: compilation errors — `conditioning_date` and `notes` don't exist on `Batch` or `UpdateBatchInput` yet. That's correct — Task 2 will add them.

- [ ] **Step 7: Also update the existing `test_update` to use `notes` instead of removed fields**

Find `test_update` in the test block and replace it:

```rust
#[tokio::test]
async fn test_update() {
    let db = setup_test_db().await;
    let (recipe_id, _) = setup(&db).await;
    let repo = BatchRepository::new(&db);
    let batch = repo
        .create(CreateBatchInput { recipe_id, name: None })
        .await
        .unwrap();
    let updated = repo
        .update(
            &batch.id,
            UpdateBatchInput {
                status: Some("brewing".into()),
                actual_og: Some(1.058),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(updated.status, "brewing");
    assert_eq!(updated.actual_og, Some(1.058));
}
```

- [ ] **Step 8: Commit the migration and entity-level changes**

```bash
git add src-tauri/migrations/006_batch_brew_day.sql \
        src-tauri/src/entities/batches.rs \
        src-tauri/src/repositories/batches.rs
git commit -m "feat: add conditioning stage and unified notes to batch DB schema"
```

---

## Task 2: OpenAPI Schema Updates + Type Regeneration

**Files:**
- Modify: `docs/openapi/components/schemas/Batch.yaml`
- Modify: `docs/openapi/components/schemas/UpdateBatchInput.yaml`
- Modify: `docs/openapi/components/schemas/BatchSummary.yaml`
- Regenerated: `src/lib/api.gen.ts`
- Regenerated: `src-tauri/src/models.gen.rs`

- [ ] **Step 1: Update `Batch.yaml`**

Replace the entire content of `docs/openapi/components/schemas/Batch.yaml` with:

```yaml
type: object
required:
  - id
  - recipe_id
  - recipe_name
  - recipe_version_id
  - status
  - gravity_readings
  - created_at
  - updated_at
properties:
  id:
    type: string
  recipe_id:
    type: string
  recipe_name:
    type: string
  recipe_version_id:
    type: string
  name:
    type: [string, "null"]
  status:
    type: string
    description: "planned | brewing | fermenting | conditioning | packaged"
  brew_date:
    type: [integer, "null"]
    format: int64
  fermenter_date:
    type: [integer, "null"]
    format: int64
  conditioning_date:
    type: [integer, "null"]
    format: int64
  packaging_date:
    type: [integer, "null"]
    format: int64
  actual_pre_boil_volume_l:
    type: [number, "null"]
  actual_post_boil_volume_l:
    type: [number, "null"]
  actual_batch_size_l:
    type: [number, "null"]
  actual_pre_boil_gravity:
    type: [number, "null"]
  actual_og:
    type: [number, "null"]
  actual_fg:
    type: [number, "null"]
  notes:
    type: [string, "null"]
  rating:
    type: [integer, "null"]
  planned_og:
    type: [number, "null"]
    description: Planned OG from recipe stats
  planned_fg:
    type: [number, "null"]
    description: Planned FG from recipe stats
  planned_pre_boil_gravity:
    type: [number, "null"]
    description: Planned pre-boil gravity from recipe stats
  planned_post_boil_volume_l:
    type: [number, "null"]
    description: Planned post-boil volume from recipe stats
  planned_batch_size_l:
    type: [number, "null"]
    description: Planned batch size from recipe
  gravity_readings:
    type: array
    items:
      $ref: "./GravityReading.yaml"
  created_at:
    type: integer
    format: int64
  updated_at:
    type: integer
    format: int64
```

- [ ] **Step 2: Update `UpdateBatchInput.yaml`**

Replace the entire content of `docs/openapi/components/schemas/UpdateBatchInput.yaml` with:

```yaml
type: object
properties:
  name:
    type: [string, "null"]
  status:
    type: [string, "null"]
  brew_date:
    type: [integer, "null"]
  fermenter_date:
    type: [integer, "null"]
  conditioning_date:
    type: [integer, "null"]
  packaging_date:
    type: [integer, "null"]
  actual_pre_boil_volume_l:
    type: [number, "null"]
  actual_post_boil_volume_l:
    type: [number, "null"]
  actual_batch_size_l:
    type: [number, "null"]
  actual_pre_boil_gravity:
    type: [number, "null"]
  actual_og:
    type: [number, "null"]
  actual_fg:
    type: [number, "null"]
  notes:
    type: [string, "null"]
  rating:
    type: [integer, "null"]
```

- [ ] **Step 3: Update `BatchSummary.yaml` status description**

In `docs/openapi/components/schemas/BatchSummary.yaml`, change:

```yaml
  status:
    type: string
    description: "planned | brewing | fermenting | packaged | complete"
```

to:

```yaml
  status:
    type: string
    description: "planned | brewing | fermenting | conditioning | packaged"
```

- [ ] **Step 4: Regenerate TypeScript types**

```bash
just gen-ts
```

Expected: `src/lib/api.gen.ts` updated. Confirm `conditioning_date`, `notes`, and `planned_og` appear in the `Batch` type and `UpdateBatchInput` type.

- [ ] **Step 5: Regenerate Rust types**

```bash
just gen-rust
```

Expected: `src-tauri/src/models.gen.rs` updated. Confirm `conditioning_date`, `notes`, `planned_og` etc. appear in the `Batch` and `UpdateBatchInput` structs.

- [ ] **Step 6: Verify tests compile**

```bash
cd src-tauri && cargo test repositories::batches 2>&1 | tail -20
```

Expected: the new test `test_update_conditioning_date_and_notes` fails at runtime (not compile time) because `get()` doesn't yet populate `conditioning_date` and `notes` from the DB row. The existing tests may also fail because `get()` still references the old fields. That's fine — Task 3 fixes `get()`.

- [ ] **Step 7: Commit schema and generated files**

```bash
git add docs/openapi/components/schemas/Batch.yaml \
        docs/openapi/components/schemas/UpdateBatchInput.yaml \
        docs/openapi/components/schemas/BatchSummary.yaml \
        src/lib/api.gen.ts \
        src-tauri/src/models.gen.rs
git commit -m "docs: update batch schema for conditioning stage, unified notes, planned targets"
```

---

## Task 3: BatchRepository — New Fields + Planned Targets

**Files:**
- Modify: `src-tauri/src/repositories/batches.rs`

- [ ] **Step 1: Update `get()` to populate all new fields and planned targets**

Replace the entire `get()` method body in `src-tauri/src/repositories/batches.rs` with:

```rust
pub async fn get(&self, id: &str) -> Result<Batch, AppError> {
    let (batch, recipe) = batches::Entity::find_by_id(id)
        .find_also_related(recipes::Entity)
        .one(self.db)
        .await?
        .ok_or(AppError::NotFound)?;

    let gravity_readings = batch_gravity_readings::Entity::find()
        .filter(batch_gravity_readings::Column::BatchId.eq(id))
        .order_by_asc(batch_gravity_readings::Column::RecordedAt)
        .all(self.db)
        .await?
        .into_iter()
        .map(GravityReading::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    let recipe_name = recipe.map(|r| r.name).unwrap_or_default();

    let full_recipe = RecipeVersionRepository::new(self.db)
        .get_full(&batch.recipe_version_id)
        .await?;
    let stats = crate::brewing::calculate_stats(&full_recipe);

    Ok(Batch {
        id: batch.id,
        recipe_id: batch.recipe_id,
        recipe_name,
        recipe_version_id: batch.recipe_version_id,
        name: batch.name,
        status: batch.status,
        brew_date: batch.brew_date.map(|v| v as i64),
        fermenter_date: batch.fermenter_date.map(|v| v as i64),
        conditioning_date: batch.conditioning_date.map(|v| v as i64),
        packaging_date: batch.packaging_date.map(|v| v as i64),
        actual_pre_boil_volume_l: batch.actual_pre_boil_volume_l,
        actual_post_boil_volume_l: batch.actual_post_boil_volume_l,
        actual_batch_size_l: batch.actual_batch_size_l,
        actual_pre_boil_gravity: batch.actual_pre_boil_gravity,
        actual_og: batch.actual_og,
        actual_fg: batch.actual_fg,
        notes: batch.notes,
        rating: batch.rating.map(|v| v as i64),
        gravity_readings,
        planned_og: Some(stats.og),
        planned_fg: Some(stats.fg),
        planned_pre_boil_gravity: Some(stats.pre_boil_gravity),
        planned_post_boil_volume_l: Some(stats.post_boil_volume_l),
        planned_batch_size_l: Some(full_recipe.batch_size_l),
        created_at: batch.created_at as i64,
        updated_at: batch.updated_at as i64,
    })
}
```

- [ ] **Step 2: Add a test for planned targets**

Add to the `#[cfg(test)]` block in `batches.rs`:

```rust
#[tokio::test]
async fn test_get_includes_planned_targets() {
    let db = setup_test_db().await;
    let (recipe_id, _) = setup(&db).await;
    let repo = BatchRepository::new(&db);
    let batch = repo
        .create(CreateBatchInput { recipe_id, name: None })
        .await
        .unwrap();
    let fetched = repo.get(&batch.id).await.unwrap();
    // Empty recipe has OG = 1.0 (no fermentables → no gravity contribution)
    assert_eq!(fetched.planned_og, Some(1.0));
    assert!(fetched.planned_batch_size_l.is_some());
}
```

- [ ] **Step 3: Run all batch tests**

```bash
cd src-tauri && cargo test repositories::batches 2>&1 | tail -20
```

Expected: all 6 tests pass:
- `test_create_and_get`
- `test_list_and_list_for_recipe`
- `test_update`
- `test_delete_cascades_readings`
- `test_gravity_readings`
- `test_update_conditioning_date_and_notes`
- `test_get_includes_planned_targets`

- [ ] **Step 4: Run all backend tests**

```bash
cd src-tauri && cargo test 2>&1 | tail -10
```

Expected: all tests pass.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/repositories/batches.rs
git commit -m "feat: populate conditioning_date, notes, and planned targets in batch get"
```

---

## Task 4: BatchOverviewTab.svelte UI

**Files:**
- Modify: `src/lib/components/batch/BatchOverviewTab.svelte`

This is the shared component used by both `src/lib/desktop/BatchView.svelte` and `src/lib/mobile/BatchView.svelte`. No changes needed in either BatchView file.

- [ ] **Step 1: Replace the full component with the updated version**

Replace the entire content of `src/lib/components/batch/BatchOverviewTab.svelte`:

```svelte
<!-- src/lib/components/batch/BatchOverviewTab.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import TabBar from "$lib/components/TabBar.svelte";
  import type { Batch, UpdateBatchInput, RecipeVersionSummary } from "$lib/api";
  import { listRecipeVersions } from "$lib/api";
  import { ipc } from "$lib/stores/error";

  let { batch, onUpdate }: { batch: Batch; onUpdate: (input: UpdateBatchInput) => void } = $props();

  const STATUSES = ["planned", "brewing", "fermenting", "conditioning", "packaged"] as const;

  let batchVersion = $state<RecipeVersionSummary | null>(null);

  onMount(async () => {
    const versions = await ipc(listRecipeVersions(batch.recipe_id));
    if (versions) {
      batchVersion = versions.find((v) => v.id === batch.recipe_version_id) ?? null;
    }
  });

  function toDateInput(ts: number | null | undefined): string {
    if (!ts) return "";
    return new Date(ts * 1000).toISOString().slice(0, 10);
  }

  function fromDateInput(val: string): number | null {
    if (!val) return null;
    return Math.floor(new Date(val).getTime() / 1000);
  }

  function onStatusChange(newStatus: string) {
    const update: UpdateBatchInput = { status: newStatus };
    const todayTs = Math.floor(Date.now() / 1000);
    if (newStatus === "brewing" && !batch.brew_date) update.brew_date = todayTs;
    if (newStatus === "fermenting" && !batch.fermenter_date) update.fermenter_date = todayTs;
    if (newStatus === "conditioning" && !batch.conditioning_date) update.conditioning_date = todayTs;
    if (newStatus === "packaged" && !batch.packaging_date) update.packaging_date = todayTs;
    onUpdate(update);
  }

  const HIGHLIGHTED: Record<string, string[]> = {
    planned: [],
    brewing: ["actual_pre_boil_gravity", "actual_og", "actual_post_boil_volume_l"],
    fermenting: ["actual_og", "actual_fg"],
    conditioning: ["actual_fg", "actual_batch_size_l"],
    packaged: ["actual_og", "actual_fg"],
  };

  const highlightedFields = $derived(new Set(HIGHLIGHTED[batch.status] ?? []));

  const stageTargets = $derived.by(() => {
    const { planned_og: og, planned_fg: fg, planned_pre_boil_gravity: pbg,
            planned_post_boil_volume_l: pbv, planned_batch_size_l: bs,
            actual_og, actual_fg } = batch;
    const targetAbv = og && fg ? ((og - fg) * 131.25).toFixed(1) : null;
    const actualAbv = actual_og && actual_fg ? ((actual_og - actual_fg) * 131.25).toFixed(1) : null;
    const items: { label: string; value: string }[] = [];
    switch (batch.status) {
      case "planned":
        if (og) items.push({ label: "OG", value: og.toFixed(3) });
        if (fg) items.push({ label: "FG", value: fg.toFixed(3) });
        if (bs) items.push({ label: "Batch", value: `${bs.toFixed(1)} L` });
        break;
      case "brewing":
        if (pbg) items.push({ label: "Pre-boil", value: pbg.toFixed(3) });
        if (og) items.push({ label: "OG", value: og.toFixed(3) });
        if (pbv) items.push({ label: "Post-boil", value: `${pbv.toFixed(1)} L` });
        break;
      case "fermenting":
        if (actual_og) items.push({ label: "Actual OG", value: actual_og.toFixed(3) });
        if (fg) items.push({ label: "Target FG", value: fg.toFixed(3) });
        if (targetAbv) items.push({ label: "Target ABV", value: `${targetAbv}%` });
        break;
      case "conditioning":
      case "packaged":
        if (actual_og) items.push({ label: "OG", value: actual_og.toFixed(3) });
        if (actual_fg) items.push({ label: "FG", value: actual_fg.toFixed(3) });
        if (actualAbv) items.push({ label: "ABV", value: `${actualAbv}%` });
        break;
    }
    return items;
  });
</script>

<div class="p-4 flex flex-col gap-6 overflow-y-auto">
  {#if batchVersion}
    <div class="text-xs" style="color: var(--color-text-muted);">
      Brewed with
      <button
        onclick={() => goto(`/recipe/${batch.recipe_id}`)}
        class="underline"
        style="color: var(--color-accent);"
      >
        Recipe v{batchVersion.version_number}{batchVersion.name ? ` · ${batchVersion.name}` : ""}
      </button>
    </div>
  {/if}

  <!-- Status -->
  <div>
    <div class="text-xs mb-2" style="color: var(--color-text-secondary);">STATUS</div>
    <!-- Mobile: native select -->
    <select
      class="md:hidden w-full px-3 py-2 rounded text-sm outline-none"
      style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);"
      value={batch.status}
      onchange={(e) => onStatusChange(e.currentTarget.value)}
    >
      {#each STATUSES as s}
        <option value={s}>{s.charAt(0).toUpperCase() + s.slice(1)}</option>
      {/each}
    </select>
    <!-- Desktop: tab bar -->
    <div class="hidden md:block">
      <TabBar
        tabs={STATUSES.map(s => ({ key: s, label: s.charAt(0).toUpperCase() + s.slice(1) }))}
        active={batch.status}
        onchange={(key) => onStatusChange(key)}
      />
    </div>
  </div>

  <!-- Stage callout banner -->
  {#if stageTargets.length > 0}
    <div
      class="flex items-center gap-4 flex-wrap px-3 py-2 rounded-lg text-sm"
      style="background: rgba(99,102,241,0.12); border: 1px solid rgba(99,102,241,0.25);"
    >
      <span class="text-xs font-bold uppercase tracking-wide" style="color: var(--color-text-secondary); min-width: 48px;">
        {batch.status === "fermenting" || batch.status === "conditioning" || batch.status === "packaged" ? "Actuals" : "Targets"}
      </span>
      {#each stageTargets as t}
        <span style="color: var(--color-text-secondary);">
          {t.label} <strong style="color: var(--color-text-primary);">{t.value}</strong>
        </span>
      {/each}
    </div>
  {/if}

  <!-- Measurements -->
  <div>
    <div class="text-xs mb-2" style="color: var(--color-text-secondary);">MEASUREMENTS</div>
    <div class="grid grid-cols-2 md:grid-cols-3 gap-3">
      {#each [
        { label: "Pre-Boil Gravity", field: "actual_pre_boil_gravity", value: batch.actual_pre_boil_gravity },
        { label: "Original Gravity (OG)", field: "actual_og", value: batch.actual_og },
        { label: "Final Gravity (FG)", field: "actual_fg", value: batch.actual_fg },
        { label: "Pre-Boil Volume (L)", field: "actual_pre_boil_volume_l", value: batch.actual_pre_boil_volume_l },
        { label: "Post-Boil Volume (L)", field: "actual_post_boil_volume_l", value: batch.actual_post_boil_volume_l },
        { label: "Batch Size (L)", field: "actual_batch_size_l", value: batch.actual_batch_size_l },
      ] as row}
        {@const highlighted = highlightedFields.has(row.field)}
        <div
          class="p-3 rounded"
          style="background: var(--color-bg-elevated);
                 border: 1px solid {highlighted ? 'rgba(99,102,241,0.4)' : 'var(--color-border)'};
                 opacity: {highlighted || row.value != null ? '1' : '0.55'};"
        >
          <label for="batch-{row.field}" class="text-xs block mb-1" style="color: var(--color-text-secondary);">{row.label}</label>
          <input
            id="batch-{row.field}"
            type="number" inputmode="decimal"
            step="0.001"
            value={row.value ?? ""}
            onblur={(e) => {
              const v = e.currentTarget.value;
              onUpdate({ [row.field]: v ? parseFloat(v) : null });
            }}
            placeholder="—"
            class="w-full bg-transparent text-sm outline-none"
            style="color: {highlighted ? 'var(--color-accent)' : 'var(--color-text-primary)'}; font-weight: {highlighted ? '600' : '400'};"
          />
        </div>
      {/each}
    </div>
    {#if batch.actual_og && batch.actual_fg}
      <div class="mt-3 text-sm" style="color: var(--color-text-muted);">
        Actual ABV: {((batch.actual_og - batch.actual_fg) * 131.25).toFixed(1)}%
      </div>
    {/if}
  </div>

  <!-- Dates -->
  <div>
    <div class="text-xs mb-2" style="color: var(--color-text-secondary);">DATES</div>
    <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
      {#each [
        { label: "Brew Date", field: "brew_date", value: batch.brew_date },
        { label: "Into Fermenter", field: "fermenter_date", value: batch.fermenter_date },
        { label: "Conditioning", field: "conditioning_date", value: batch.conditioning_date },
        { label: "Packaging", field: "packaging_date", value: batch.packaging_date },
      ] as item}
        <div>
          <label for="batch-{item.field}" class="text-xs block mb-1" style="color: var(--color-text-secondary);">{item.label}</label>
          <input
            id="batch-{item.field}"
            type="date"
            value={toDateInput(item.value)}
            onchange={(e) => onUpdate({ [item.field]: fromDateInput(e.currentTarget.value) })}
            class="w-full px-2 py-1.5 rounded text-sm outline-none"
            style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border); opacity: {item.value ? '1' : '0.55'};"
          />
        </div>
      {/each}
    </div>
  </div>

  <!-- Notes -->
  <div>
    <div class="text-xs mb-2" style="color: var(--color-text-secondary);">NOTES</div>
    <textarea
      value={batch.notes ?? ""}
      onblur={(e) => onUpdate({ notes: e.currentTarget.value || null })}
      placeholder="Brew day observations, gravity readings, anything worth remembering…"
      rows="4"
      class="w-full px-3 py-2 rounded text-sm outline-none resize-y"
      style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border); font-family: inherit;"
    ></textarea>
  </div>
</div>
```

- [ ] **Step 2: Run TypeScript check**

```bash
npm run check 2>&1 | tail -15
```

Expected: 0 errors, 0 warnings.

- [ ] **Step 3: Smoke test on desktop**

Run `just dev` and open a batch. Verify:

1. Status tabs show: Planned, Brewing, Fermenting, Conditioning, Packaged (no "Complete")
2. Advancing from Planned → Brewing auto-fills Brew Date to today
3. Advancing from Brewing → Fermenting auto-fills Fermenter Date to today
4. Advancing from Fermenting → Conditioning auto-fills Conditioning Date to today
5. Advancing from Conditioning → Packaged auto-fills Packaging Date to today
6. Stage callout banner appears with correct content per stage (e.g., on Brewing: shows pre-boil gravity target, OG target, post-boil volume)
7. Highlighted measurement cards (indigo-bordered) match the current stage
8. Notes textarea is visible at all stages; text persists after blurring
9. Dates grid shows 4 fields in a 2×2 grid on mobile, 4-column row on desktop

- [ ] **Step 4: Smoke test on mobile**

Run `just dev-ios` (or `just dev-android`) and open a batch. Verify the same checklist — particularly that the status select dropdown shows Conditioning (not Complete), the notes textarea is present, and the 4 dates appear in a 2×2 grid.

- [ ] **Step 5: Commit**

```bash
git add src/lib/components/batch/BatchOverviewTab.svelte
git commit -m "feat: batch brew day UI — callout banner, conditioning stage, notes, date auto-fill"
```
