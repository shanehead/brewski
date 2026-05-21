# Batch Carbonation Integration — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a carbonation section to the batch Overview tab (visible when status is conditioning/packaged) that calculates priming sugar and keg pressure pre-filled from batch and recipe data, and saves the plan back to the batch record.

**Architecture:** Four nullable columns are added to the `batches` DB table and surfaced through the OpenAPI schema, models.gen.rs, api.gen.ts, and repository. A new `BatchCarbonationSection` Svelte component hosts the UI; `BatchOverviewTab` fetches the full recipe and conditionally renders the section based on status.

**Tech Stack:** Rust/SeaORM (migration + entity + repository), OpenAPI YAML + `just gen` (codegen), SvelteKit 5 / Svelte 5 runes, Vitest + Testing Library (frontend tests), Tauri IPC (`calculate_priming_sugar`, `calculate_co2_pressure`).

---

## File Map

| File | Action |
|---|---|
| `src-tauri/migrations/007_batch_carbonation.sql` | Create — DB migration |
| `src-tauri/src/entities/batches.rs` | Modify — add 4 fields to SeaORM entity |
| `docs/openapi/components/schemas/Batch.yaml` | Modify — add 4 new fields |
| `docs/openapi/components/schemas/UpdateBatchInput.yaml` | Modify — add 4 new fields |
| `src/lib/api.gen.ts` | Generated — `just gen-ts` |
| `src-tauri/src/models.gen.rs` | Generated — `just gen-rust` |
| `src-tauri/src/repositories/batches.rs` | Modify — create/get/update + new test |
| `src/lib/components/batch/BatchCarbonationSection.svelte` | Create — new component |
| `tests/BatchCarbonationSection.test.ts` | Create — component unit tests |
| `src/lib/components/batch/BatchOverviewTab.svelte` | Modify — fetch recipe, render section |

---

### Task 1: DB Migration

**Files:**
- Create: `src-tauri/migrations/007_batch_carbonation.sql`

- [ ] **Step 1: Write the migration**

```sql
-- Add carbonation packaging fields to batches
ALTER TABLE batches ADD COLUMN packaging_temp_c REAL;
ALTER TABLE batches ADD COLUMN carbonation_sugar_type TEXT;
ALTER TABLE batches ADD COLUMN priming_sugar_g REAL;
ALTER TABLE batches ADD COLUMN serving_pressure_kpa REAL;
```

- [ ] **Step 2: Apply the migration**

```bash
just migrate
```

Expected: migration runs without error. If you see "table batches has no column named packaging_temp_c", the migration hasn't run yet — re-run.

- [ ] **Step 3: Commit**

```bash
git add src-tauri/migrations/007_batch_carbonation.sql
git commit -m "feat: migration 007 — add carbonation fields to batches"
```

---

### Task 2: Update SeaORM Entity

**Files:**
- Modify: `src-tauri/src/entities/batches.rs`

- [ ] **Step 1: Add four fields to the entity Model**

Open `src-tauri/src/entities/batches.rs`. After the `pub rating: Option<i32>` field (line ~33), add:

```rust
    pub packaging_temp_c: Option<f64>,
    #[sea_orm(column_type = "Text", nullable)]
    pub carbonation_sugar_type: Option<String>,
    pub priming_sugar_g: Option<f64>,
    pub serving_pressure_kpa: Option<f64>,
```

- [ ] **Step 2: Verify it compiles**

```bash
cd src-tauri && cargo check 2>&1 | grep -E "error|warning.*unused"
```

Expected: no errors.

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/entities/batches.rs
git commit -m "feat: add carbonation fields to batches SeaORM entity"
```

---

### Task 3: Update OpenAPI Schemas and Regenerate Types

**Files:**
- Modify: `docs/openapi/components/schemas/Batch.yaml`
- Modify: `docs/openapi/components/schemas/UpdateBatchInput.yaml`
- Generated: `src/lib/api.gen.ts`
- Generated: `src-tauri/src/models.gen.rs`

- [ ] **Step 1: Add fields to Batch.yaml**

Open `docs/openapi/components/schemas/Batch.yaml`. After the `serving_temp_c` or last nullable field near the bottom of `properties:`, add:

```yaml
  packaging_temp_c:
    type: [number, "null"]
  carbonation_sugar_type:
    type: [string, "null"]
  priming_sugar_g:
    type: [number, "null"]
  serving_pressure_kpa:
    type: [number, "null"]
```

- [ ] **Step 2: Add fields to UpdateBatchInput.yaml**

Open `docs/openapi/components/schemas/UpdateBatchInput.yaml`. Add the same four fields to `properties:`:

```yaml
  packaging_temp_c:
    type: [number, "null"]
  carbonation_sugar_type:
    type: [string, "null"]
  priming_sugar_g:
    type: [number, "null"]
  serving_pressure_kpa:
    type: [number, "null"]
```

- [ ] **Step 3: Regenerate TypeScript and Rust types**

```bash
just gen
```

Expected: `src/lib/api.gen.ts` and `src-tauri/src/models.gen.rs` are updated. Verify the new fields appear:

```bash
grep "packaging_temp_c" src/lib/api.gen.ts src-tauri/src/models.gen.rs
```

Expected: matches in both files.

- [ ] **Step 4: Verify TypeScript types pass**

```bash
bun run check 2>&1 | tail -5
```

Expected: no errors.

- [ ] **Step 5: Commit**

```bash
git add docs/openapi/components/schemas/Batch.yaml \
        docs/openapi/components/schemas/UpdateBatchInput.yaml \
        src/lib/api.gen.ts \
        src-tauri/src/models.gen.rs
git commit -m "feat: add carbonation fields to Batch and UpdateBatchInput schemas"
```

---

### Task 4: Update Repository and Add Integration Test

**Files:**
- Modify: `src-tauri/src/repositories/batches.rs`

- [ ] **Step 1: Write the failing test**

In `src-tauri/src/repositories/batches.rs`, add this test inside the existing `#[cfg(test)] mod tests` block (after `test_gravity_readings`):

```rust
#[tokio::test]
async fn test_update_carbonation_fields() {
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
                packaging_temp_c: Some(18.0),
                carbonation_sugar_type: Some("corn_sugar".into()),
                priming_sugar_g: Some(134.5),
                serving_pressure_kpa: Some(97.2),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(updated.packaging_temp_c, Some(18.0));
    assert_eq!(updated.carbonation_sugar_type, Some("corn_sugar".into()));
    assert_eq!(updated.priming_sugar_g, Some(134.5));
    assert_eq!(updated.serving_pressure_kpa, Some(97.2));
}
```

- [ ] **Step 2: Run the test to verify it fails**

```bash
cd src-tauri && cargo test test_update_carbonation_fields 2>&1 | tail -20
```

Expected: FAIL — compilation error because `UpdateBatchInput` doesn't have `packaging_temp_c` yet (or a field mismatch), or runtime panic.

- [ ] **Step 3: Update the `create` method**

In the `create` method's `batches::ActiveModel { ... }` block, add four fields after `rating: Set(None)`:

```rust
            packaging_temp_c: Set(None),
            carbonation_sugar_type: Set(None),
            priming_sugar_g: Set(None),
            serving_pressure_kpa: Set(None),
```

- [ ] **Step 4: Update the `get` method**

In `get`, inside `Ok(Batch { ... })`, add four fields after `planned_batch_size_l`:

```rust
            packaging_temp_c: batch.packaging_temp_c,
            carbonation_sugar_type: batch.carbonation_sugar_type,
            priming_sugar_g: batch.priming_sugar_g,
            serving_pressure_kpa: batch.serving_pressure_kpa,
```

- [ ] **Step 5: Update the `update` method**

In `update`, after the `if let Some(v) = input.rating { ... }` block, add:

```rust
        if let Some(v) = input.packaging_temp_c {
            active.packaging_temp_c = Set(Some(v));
        }
        if let Some(v) = input.carbonation_sugar_type {
            active.carbonation_sugar_type = Set(Some(v));
        }
        if let Some(v) = input.priming_sugar_g {
            active.priming_sugar_g = Set(Some(v));
        }
        if let Some(v) = input.serving_pressure_kpa {
            active.serving_pressure_kpa = Set(Some(v));
        }
```

- [ ] **Step 6: Run the test to verify it passes**

```bash
cd src-tauri && cargo test test_update_carbonation_fields 2>&1 | tail -10
```

Expected: `test test_update_carbonation_fields ... ok`

- [ ] **Step 7: Run the full Rust test suite**

```bash
cd src-tauri && cargo test 2>&1 | tail -20
```

Expected: all tests pass.

- [ ] **Step 8: Commit**

```bash
git add src-tauri/src/repositories/batches.rs
git commit -m "feat: wire carbonation fields through batch repository"
```

---

### Task 5: Create BatchCarbonationSection Component

**Files:**
- Create: `src/lib/components/batch/BatchCarbonationSection.svelte`
- Create: `tests/BatchCarbonationSection.test.ts`

- [ ] **Step 1: Write the failing tests**

Create `tests/BatchCarbonationSection.test.ts`:

```typescript
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/svelte";
import { tick } from "svelte";
import userEvent from "@testing-library/user-event";
import type { Batch, UpdateBatchInput } from "$lib/api";
import BatchCarbonationSection from "$lib/components/batch/BatchCarbonationSection.svelte";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("$lib/stores/settings", () => ({
  settings: {
    subscribe: vi.fn((fn) => {
      fn({ theme: "midnight", units: "metric", default_equipment_profile_id: "" });
      return () => {};
    }),
  },
}));
vi.mock("$lib/stores/error", () => ({
  ipc: vi.fn((p) => p),
}));

const mockInvoke = vi.mocked((await import("@tauri-apps/api/core")).invoke);

function makeBatch(overrides: Partial<Batch> = {}): Batch {
  return {
    id: "b1",
    recipe_id: "r1",
    recipe_name: "Test IPA",
    recipe_version_id: "v1",
    name: null,
    status: "conditioning",
    brew_date: null,
    fermenter_date: null,
    conditioning_date: null,
    packaging_date: null,
    actual_pre_boil_volume_l: null,
    actual_post_boil_volume_l: null,
    actual_batch_size_l: 19,
    actual_pre_boil_gravity: null,
    actual_og: null,
    actual_fg: null,
    notes: null,
    rating: null,
    planned_og: null,
    planned_fg: null,
    planned_pre_boil_gravity: null,
    planned_post_boil_volume_l: null,
    planned_batch_size_l: null,
    packaging_temp_c: null,
    carbonation_sugar_type: null,
    priming_sugar_g: null,
    serving_pressure_kpa: null,
    gravity_readings: [],
    created_at: 0,
    updated_at: 0,
    ...overrides,
  } as unknown as Batch;
}

describe("BatchCarbonationSection", () => {
  beforeEach(() => {
    mockInvoke.mockReset();
    mockInvoke.mockResolvedValue(134.5); // default: priming sugar grams
  });

  it("pre-fills temp from recipePrimaryTempC when batch.packaging_temp_c is null", () => {
    render(BatchCarbonationSection, {
      batch: makeBatch({ packaging_temp_c: null }),
      recipePrimaryTempC: 18,
      recipeCarbonationVols: 2.4,
      onUpdate: vi.fn(),
    });
    const input = screen.getByLabelText(/Packaging Temp/i) as HTMLInputElement;
    expect(input.value).toBe("18.0");
  });

  it("uses batch.packaging_temp_c when already set", () => {
    render(BatchCarbonationSection, {
      batch: makeBatch({ packaging_temp_c: 22 }),
      recipePrimaryTempC: 18,
      recipeCarbonationVols: 2.4,
      onUpdate: vi.fn(),
    });
    const input = screen.getByLabelText(/Packaging Temp/i) as HTMLInputElement;
    expect(input.value).toBe("22.0");
  });

  it("shows prompt when no batch size is available", () => {
    render(BatchCarbonationSection, {
      batch: makeBatch({ actual_batch_size_l: null, planned_batch_size_l: null }),
      recipePrimaryTempC: 20,
      recipeCarbonationVols: 2.4,
      onUpdate: vi.fn(),
    });
    expect(screen.getByText(/Enter batch size/i)).toBeInTheDocument();
  });

  it("calls onUpdate with all four carbonation fields on sugar type change", async () => {
    mockInvoke
      .mockResolvedValueOnce(134.5)  // calculatePrimingSugar (initial effect)
      .mockResolvedValueOnce(97.2)   // calculateCo2Pressure (initial effect)
      .mockResolvedValueOnce(128.0)  // calculatePrimingSugar (after sugar change)
      .mockResolvedValueOnce(97.2);  // calculateCo2Pressure (after sugar change)
    const onUpdate = vi.fn();
    const user = userEvent.setup();
    render(BatchCarbonationSection, {
      batch: makeBatch(),
      recipePrimaryTempC: 20,
      recipeCarbonationVols: 2.4,
      onUpdate,
    });
    // Flush the initial $effect async calculation
    await tick();
    await new Promise((r) => setTimeout(r, 0));
    await tick();

    const select = screen.getByLabelText(/Sugar Type/i);
    await user.selectOptions(select, "table_sugar");

    // Flush the recalculation triggered by sugar type change
    await tick();
    await new Promise((r) => setTimeout(r, 0));
    await tick();

    expect(onUpdate).toHaveBeenCalledWith(
      expect.objectContaining({
        carbonation_sugar_type: "table_sugar",
        packaging_temp_c: expect.any(Number),
        priming_sugar_g: expect.any(Number),
        serving_pressure_kpa: expect.any(Number),
      })
    );
  });
});
```

- [ ] **Step 2: Run the tests to verify they fail**

```bash
bun run test -- BatchCarbonationSection 2>&1 | tail -20
```

Expected: FAIL — module not found or import error.

- [ ] **Step 3: Create the component**

Create `src/lib/components/batch/BatchCarbonationSection.svelte`:

```svelte
<script lang="ts">
  import type { Batch, UpdateBatchInput } from "$lib/api";
  import { calculatePrimingSugar, calculateCo2Pressure, type SugarType } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { settings } from "$lib/stores/settings";
  import { cToF, fToC, tempLabel, type Units } from "$lib/units";

  let {
    batch,
    recipePrimaryTempC,
    recipeCarbonationVols,
    onUpdate,
  }: {
    batch: Batch;
    recipePrimaryTempC: number | null;
    recipeCarbonationVols: number | null;
    onUpdate: (input: UpdateBatchInput) => void;
  } = $props();

  const units = $derived(($settings.units ?? "metric") as Units);
  const targetVols = $derived(recipeCarbonationVols ?? 2.4);
  const batchSizeL = $derived(batch.actual_batch_size_l ?? batch.planned_batch_size_l ?? null);

  let tempC = $state(batch.packaging_temp_c ?? recipePrimaryTempC ?? 20);
  let sugarType = $state<SugarType>(
    (batch.carbonation_sugar_type as SugarType) ?? "corn_sugar"
  );

  const tempDisplay = $derived(units === "imperial" ? cToF(tempC) : tempC);

  let primingSugarG = $state<number | null>(null);
  let pressureKpa = $state<number | null>(null);
  let hasInteracted = $state(false);

  $effect(() => {
    const vols = targetVols;
    const size = batchSizeL;
    const temp = tempC;
    const sugar = sugarType;

    if (!size || size <= 0 || vols <= 0) {
      primingSugarG = null;
      pressureKpa = null;
      return;
    }

    void (async () => {
      const [sg, kpa] = await Promise.all([
        ipc(calculatePrimingSugar(vols, size, temp, sugar)),
        ipc(calculateCo2Pressure(vols, temp)),
      ]);
      if (targetVols === vols && batchSizeL === size && tempC === temp && sugarType === sugar) {
        primingSugarG = sg ?? null;
        pressureKpa = kpa ?? null;
        if (hasInteracted && sg != null && kpa != null) {
          onUpdate({
            packaging_temp_c: temp,
            carbonation_sugar_type: sugar,
            priming_sugar_g: sg,
            serving_pressure_kpa: kpa,
          });
        }
      }
    })();
  });

  function updateTemp(value: string) {
    const next = Number(value);
    tempC = units === "imperial" ? fToC(next) : next;
  }

  function markInteracted() {
    hasInteracted = true;
  }
</script>

<div>
  <div
    class="text-xs mb-3 mt-4 pt-4 border-t font-semibold uppercase tracking-wide"
    style="color: var(--color-text-secondary); border-color: var(--color-border);"
  >
    Carbonation
  </div>

  <div class="grid grid-cols-2 md:grid-cols-3 gap-3 mb-4">
    <!-- Target vols — read only from recipe -->
    <div
      class="p-3 rounded"
      style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); opacity: 0.7;"
    >
      <div class="text-xs mb-1" style="color: var(--color-text-secondary);">Target CO₂ (vols)</div>
      <div class="text-sm font-medium" style="color: var(--color-text-primary);">{targetVols.toFixed(1)}</div>
      <div class="text-xs mt-0.5" style="color: var(--color-text-muted);">from recipe</div>
    </div>

    <!-- Packaging temp -->
    <div class="p-3 rounded" style="background: var(--color-bg-elevated); border: 1px solid var(--color-border);">
      <label
        for="carb-temp"
        class="text-xs block mb-1"
        style="color: var(--color-text-secondary);"
      >
        Packaging Temp ({tempLabel(units)})
      </label>
      <input
        id="carb-temp"
        type="number"
        inputmode="decimal"
        step="0.1"
        value={tempDisplay.toFixed(1)}
        oninput={(e) => updateTemp((e.target as HTMLInputElement).value)}
        onblur={markInteracted}
        class="w-full bg-transparent text-sm outline-none"
        style="color: var(--color-text-primary);"
      />
    </div>

    <!-- Sugar type -->
    <div
      class="p-3 rounded col-span-2 md:col-span-1"
      style="background: var(--color-bg-elevated); border: 1px solid var(--color-border);"
    >
      <label for="carb-sugar" class="text-xs block mb-1" style="color: var(--color-text-secondary);">
        Sugar Type
      </label>
      <select
        id="carb-sugar"
        bind:value={sugarType}
        onchange={markInteracted}
        class="w-full bg-transparent text-sm outline-none"
        style="color: var(--color-text-primary);"
      >
        <option value="corn_sugar">Corn sugar</option>
        <option value="table_sugar">Table sugar</option>
        <option value="dry_malt_extract">Dry malt extract</option>
      </select>
    </div>
  </div>

  {#if batchSizeL === null}
    <p class="text-sm" style="color: var(--color-text-muted);">
      Enter batch size in Measurements to calculate carbonation.
    </p>
  {:else if primingSugarG !== null && pressureKpa !== null}
    <div class="grid grid-cols-2 gap-3">
      <div
        class="p-4 rounded-lg"
        style="background: rgba(99,102,241,0.1); border: 1px solid rgba(99,102,241,0.3);"
      >
        <div
          class="text-xs font-bold uppercase tracking-wide mb-2"
          style="color: var(--color-text-secondary);"
        >
          Bottle Priming
        </div>
        <div class="text-3xl font-semibold" style="color: var(--color-text-primary);">
          {primingSugarG.toFixed(0)}<span
            class="text-base font-normal ml-1"
            style="color: var(--color-text-secondary);">g</span>
        </div>
        <div class="text-xs mt-1" style="color: var(--color-text-muted);">
          {sugarType.replace(/_/g, " ")} · {batchSizeL.toFixed(1)} L
        </div>
      </div>

      <div
        class="p-4 rounded-lg"
        style="background: rgba(99,102,241,0.1); border: 1px solid rgba(99,102,241,0.3);"
      >
        <div
          class="text-xs font-bold uppercase tracking-wide mb-2"
          style="color: var(--color-text-secondary);"
        >
          Keg Pressure
        </div>
        <div class="text-3xl font-semibold" style="color: var(--color-text-primary);">
          {pressureKpa.toFixed(0)}<span
            class="text-base font-normal ml-1"
            style="color: var(--color-text-secondary);">kPa</span>
        </div>
        <div class="text-xs mt-1" style="color: var(--color-text-muted);">
          {(pressureKpa * 0.145038).toFixed(1)} PSI
        </div>
      </div>
    </div>
  {/if}
</div>
```

- [ ] **Step 4: Run tests to verify they pass**

```bash
bun run test -- BatchCarbonationSection 2>&1 | tail -20
```

Expected: all tests pass.

- [ ] **Step 5: Commit**

```bash
git add src/lib/components/batch/BatchCarbonationSection.svelte \
        tests/BatchCarbonationSection.test.ts
git commit -m "feat: add BatchCarbonationSection component"
```

---

### Task 6: Update BatchOverviewTab

**Files:**
- Modify: `src/lib/components/batch/BatchOverviewTab.svelte`

- [ ] **Step 1: Add recipe import and state**

In `BatchOverviewTab.svelte`, update the `<script>` imports and state to fetch the recipe. Add `getRecipe` and `Recipe` to the import:

```typescript
import type { Batch, UpdateBatchInput, RecipeVersionSummary, Recipe } from "$lib/api";
import { listRecipeVersions, getRecipe } from "$lib/api";
```

Add recipe state after `let batchVersion`:

```typescript
let recipe = $state<Recipe | null>(null);
```

- [ ] **Step 2: Fetch recipe in onMount**

In `onMount`, add the `getRecipe` call alongside the existing `listRecipeVersions` call:

```typescript
onMount(async () => {
  const [versions, fetchedRecipe] = await Promise.all([
    ipc(listRecipeVersions(batch.recipe_id)),
    ipc(getRecipe(batch.recipe_id)),
  ]);
  if (versions) {
    batchVersion = versions.find((v) => v.id === batch.recipe_version_id) ?? null;
  }
  if (fetchedRecipe) {
    recipe = fetchedRecipe;
  }
});
```

- [ ] **Step 3: Import BatchCarbonationSection and render it**

Add the import at the top of `<script>`:

```typescript
import BatchCarbonationSection from "$lib/components/batch/BatchCarbonationSection.svelte";
```

At the bottom of the scrollable `<div class="p-4 flex flex-col gap-6 overflow-y-auto">`, after the Notes section, add:

```svelte
{#if batch.status === "conditioning" || batch.status === "packaged"}
  <BatchCarbonationSection
    {batch}
    recipePrimaryTempC={recipe?.primary_temp_c ?? null}
    recipeCarbonationVols={recipe?.carbonation_vols ?? null}
    {onUpdate}
  />
{/if}
```

- [ ] **Step 4: Run the full test suite**

```bash
just test 2>&1 | tail -30
```

Expected: all tests pass (Rust + frontend).

- [ ] **Step 5: Run TypeScript check**

```bash
bun run check 2>&1 | tail -10
```

Expected: no errors.

- [ ] **Step 6: Commit**

```bash
git add src/lib/components/batch/BatchOverviewTab.svelte
git commit -m "feat: show carbonation section in batch overview for conditioning/packaged"
```
