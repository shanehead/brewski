# Recipe Scaling Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a "Scale Recipe" action to the recipe detail view that creates a new copy of a recipe with all ingredient amounts and volumes adjusted proportionally to a target batch size.

**Architecture:** A new `scale_recipe` Tauri command calls `RecipeRepository::scale`, which loads the source recipe, computes a ratio, inserts a new recipe row, copies and scales all ingredient additions (fermentables, hops, yeast, misc, water, water adjustments), and copies and scales mash steps. The frontend provides a modal with a volume input; on confirm it calls the IPC command and navigates to the new recipe.

**Tech Stack:** Rust/SeaORM (backend), Svelte 5 runes (frontend), Tauri IPC, Vitest + @testing-library/svelte (frontend tests), tokio + SQLite in-memory (Rust tests)

---

## File Map

| File | Action | Purpose |
|------|--------|---------|
| `src-tauri/src/repositories/recipe.rs` | Modify | Add `scale` method to `RecipeRepository` |
| `src-tauri/src/commands/recipes.rs` | Modify | Add `scale_recipe` Tauri command |
| `src-tauri/src/lib.rs` | Modify | Register `scale_recipe` in `invoke_handler` |
| `src/lib/api.ts` | Modify | Add `scaleRecipe` frontend function |
| `src/lib/components/ScaleRecipeModal.svelte` | Create | Modal with volume input + confirm/cancel |
| `tests/ScaleRecipeModal.test.ts` | Create | Component tests for the modal |
| `src/lib/desktop/RecipeView.svelte` | Modify | Add Scale button + modal in desktop header |
| `src/lib/mobile/RecipeView.svelte` | Modify | Add Scale button + modal in mobile header |

---

### Task 1: Backend — `scale` repository method, command, and registration

**Files:**
- Modify: `src-tauri/src/repositories/recipe.rs`
- Modify: `src-tauri/src/commands/recipes.rs`
- Modify: `src-tauri/src/lib.rs`

The `scale` method loads the source recipe, computes a ratio from the new vs. old batch size, inserts a new recipe row with scaled volumes, copies all ingredient additions with scaled amounts, and copies mash steps with scaled `infuse_amount_l`.

**Context — existing pattern:**
The `copy_additions` helper (line ~230 in `recipe.rs`) shows how each ingredient type is copied. The `create` method (line ~179) shows the `recipes::ActiveModel` field names. Mash steps are NOT copied by `copy_additions`; we must handle them explicitly using `MashRepository`.

The `recipes::ActiveModel` fields visible in `create` and `update` are: `id`, `name`, `r#type`, `brewer`, `asst_brewer`, `batch_size_l`, `boil_size_l`, `boil_time_min`, `efficiency_pct`, `style_id`, `equipment_profile_id`, `notes`, `taste_notes`, `taste_rating`, `fermentation_stages`, `primary_age_days`, `primary_temp_c`, `secondary_age_days`, `secondary_temp_c`, `tertiary_age_days`, `tertiary_temp_c`, `age_days`, `age_temp_c`, `carbonation_vols`, `forced_carbonation`, `priming_sugar_name`, `carbonation_temp_c`, `priming_sugar_equiv`, `keg_priming_factor`, `date`, `hopstand_temp_c`, `mash_water_id`, `sparge_water_id`, `created_at`, `updated_at`.

`forced_carbonation` is stored as `Option<i32>` in the DB and decoded as `bool` on the `Recipe` struct. Re-encode with `if src.forced_carbonation { 1 } else { 0 }`.

**Ingredient types and their scaled amount fields:**
- `RecipeAdditionFermentable`: `amount_kg: f64` → `f.amount_kg * ratio`
- `RecipeAdditionHop`: `amount_kg: f64` → `h.amount_kg * ratio`
- `RecipeAdditionYeast`: `amount: Option<f64>` → `y.amount.map(|a| a * ratio)`
- `RecipeAdditionMisc`: `amount: f64` → `m.amount * ratio`
- `RecipeAdditionWater`: `amount_l: f64` → `w.amount_l * ratio`
- `RecipeWaterAdjustment`: `amount: f64` (grams/ml) → `a.amount * ratio`
- `MashStep`: `infuse_amount_l: Option<f64>` → `step.infuse_amount_l.map(|v| v * ratio)`

**Running Rust tests** — from the `src-tauri/` directory:

```bash
cargo test repositories::recipe::tests 2>&1 | grep -E "test |FAILED|error"
```

- [ ] **Step 1: Write three failing tests** in the `#[cfg(test)]` block at the bottom of `src-tauri/src/repositories/recipe.rs`

Add these three tests. The existing test module already imports `setup_test_db`, `FermentableRepository`, `HopRepository`, and `use super::*`.

```rust
#[tokio::test]
async fn test_scale_creates_new_recipe() {
    let db = setup_test_db().await;
    let repo = RecipeRepository::new(&db);
    let original = repo.create(CreateRecipeInput {
        name: "My IPA".into(),
        batch_size_l: Some(23.0),
        boil_size_l: Some(27.0),
        ..Default::default()
    }).await.unwrap();

    let scaled = repo.scale(&original.id, 46.0).await.unwrap();

    assert_ne!(scaled.id, original.id);
    assert_eq!(scaled.name, "My IPA (scaled)");
    assert_eq!(scaled.batch_size_l, 46.0);
    assert!((scaled.boil_size_l - 54.0).abs() < 0.001);
    // original unchanged
    let still_original = repo.get(&original.id).await.unwrap();
    assert_eq!(still_original.batch_size_l, 23.0);
}

#[tokio::test]
async fn test_scale_ingredients() {
    let db = setup_test_db().await;
    let repo = RecipeRepository::new(&db);
    let original = repo.create(CreateRecipeInput {
        name: "My IPA".into(),
        batch_size_l: Some(23.0),
        boil_size_l: Some(27.0),
        ..Default::default()
    }).await.unwrap();

    FermentableRepository::new(&db).create(&original.id, CreateFermentableAdditionInput {
        fermentable_id: None,
        name: "Pale Malt".into(),
        type_: "grain".into(),
        yield_pct: 78.0,
        color_lovibond: 1.8,
        amount_kg: 4.5,
        add_after_boil: None,
    }).await.unwrap();

    HopRepository::new(&db).create(&original.id, CreateHopAdditionInput {
        hop_id: None,
        name: "Cascade".into(),
        alpha_pct: 5.5,
        form: None,
        amount_kg: 0.05,
        use_: "Boil".into(),
        time_min: 60.0,
        hopstand_temp_c: None,
    }).await.unwrap();

    let scaled = repo.scale(&original.id, 46.0).await.unwrap();

    assert_eq!(scaled.fermentables.len(), 1);
    assert!((scaled.fermentables[0].amount_kg - 9.0).abs() < 0.001);
    assert_eq!(scaled.hops.len(), 1);
    assert!((scaled.hops[0].amount_kg - 0.1).abs() < 0.0001);
}

#[tokio::test]
async fn test_scale_mash_steps() {
    use crate::models::{UpdateMashInput, CreateMashStepInput};
    use crate::repositories::mash::MashRepository;

    let db = setup_test_db().await;
    let repo = RecipeRepository::new(&db);
    let original = repo.create(CreateRecipeInput {
        name: "My IPA".into(),
        batch_size_l: Some(23.0),
        boil_size_l: Some(27.0),
        ..Default::default()
    }).await.unwrap();

    let mash_repo = MashRepository::new(&db);
    let mash = mash_repo.upsert_for_recipe(&original.id, UpdateMashInput {
        name: Some("Single Infusion".into()),
        grain_temp_c: Some(20.0),
        ..Default::default()
    }).await.unwrap();

    mash_repo.create_step(&mash.id, CreateMashStepInput {
        name: "Mash".into(),
        type_: Some("Infusion".into()),
        step_temp_c: 67.0,
        step_time_min: 60,
        infuse_amount_l: Some(15.0),
        ramp_time_min: None,
        end_temp_c: None,
    }).await.unwrap();

    let scaled = repo.scale(&original.id, 46.0).await.unwrap();

    let scaled_mash = scaled.mash.expect("scaled recipe should have mash");
    assert_eq!(scaled_mash.steps.len(), 1);
    let infuse = scaled_mash.steps[0].infuse_amount_l.expect("infuse should be set");
    assert!((infuse - 30.0).abs() < 0.001);
}
```

- [ ] **Step 2: Run tests to verify they fail**

```bash
cd /Users/shead/Documents/code/brewski/src-tauri
cargo test repositories::recipe::tests::test_scale 2>&1 | grep -E "test |FAILED|error\[" | head -20
```

Expected: 3 errors — `error[E0599]: no method named 'scale' found`

- [ ] **Step 3: Add `CreateMashStepInput` and `UpdateMashInput` to the models import**

In `src-tauri/src/repositories/recipe.rs`, the existing models import (lines 7–11) currently reads:

```rust
use crate::models::{
    CreateFermentableAdditionInput, CreateHopAdditionInput, CreateMiscAdditionInput,
    CreateRecipeInput, CreateWaterAdditionInput, CreateWaterAdjustmentInput,
    CreateYeastAdditionInput, Recipe, RecipeSummary, UpdateRecipeInput,
};
```

Replace with:

```rust
use crate::models::{
    CreateFermentableAdditionInput, CreateHopAdditionInput, CreateMashStepInput,
    CreateMiscAdditionInput, CreateRecipeInput, CreateWaterAdditionInput,
    CreateWaterAdjustmentInput, CreateYeastAdditionInput, Recipe, RecipeSummary,
    UpdateMashInput, UpdateRecipeInput,
};
```

- [ ] **Step 4: Add the `scale` method to `RecipeRepository`**

Insert this method before the `update` method (around line 350) in `src-tauri/src/repositories/recipe.rs`:

```rust
pub async fn scale(&self, recipe_id: &str, new_batch_size_l: f64) -> Result<Recipe, AppError> {
    let src = self.get(recipe_id).await?;
    let ratio = new_batch_size_l / src.batch_size_l;
    let id = new_id();
    let now = now_secs() as i32;

    recipes::ActiveModel {
        id: Set(id.clone()),
        name: Set(format!("{} (scaled)", src.name)),
        r#type: Set(src.type_),
        brewer: Set(src.brewer),
        asst_brewer: Set(src.asst_brewer),
        batch_size_l: Set(new_batch_size_l),
        boil_size_l: Set(src.boil_size_l * ratio),
        boil_time_min: Set(src.boil_time_min),
        efficiency_pct: Set(src.efficiency_pct),
        style_id: Set(src.style_id),
        equipment_profile_id: Set(src.equipment_profile_id),
        notes: Set(src.notes),
        taste_notes: Set(src.taste_notes),
        taste_rating: Set(src.taste_rating),
        fermentation_stages: Set(Some(src.fermentation_stages as i32)),
        primary_age_days: Set(src.primary_age_days),
        primary_temp_c: Set(src.primary_temp_c),
        secondary_age_days: Set(src.secondary_age_days),
        secondary_temp_c: Set(src.secondary_temp_c),
        tertiary_age_days: Set(src.tertiary_age_days),
        tertiary_temp_c: Set(src.tertiary_temp_c),
        age_days: Set(src.age_days),
        age_temp_c: Set(src.age_temp_c),
        carbonation_vols: Set(src.carbonation_vols),
        forced_carbonation: Set(Some(if src.forced_carbonation { 1 } else { 0 })),
        priming_sugar_name: Set(src.priming_sugar_name),
        carbonation_temp_c: Set(src.carbonation_temp_c),
        priming_sugar_equiv: Set(src.priming_sugar_equiv),
        keg_priming_factor: Set(src.keg_priming_factor),
        date: Set(src.date),
        hopstand_temp_c: Set(src.hopstand_temp_c),
        mash_water_id: Set(src.mash_water_id),
        sparge_water_id: Set(src.sparge_water_id),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    }
    .insert(self.db)
    .await?;

    let fermentable_repo = FermentableRepository::new(self.db);
    for f in fermentable_repo.list(recipe_id).await? {
        fermentable_repo.create(&id, CreateFermentableAdditionInput {
            fermentable_id: f.fermentable_id,
            name: f.name,
            type_: f.type_,
            yield_pct: f.yield_pct,
            color_lovibond: f.color_lovibond,
            amount_kg: f.amount_kg * ratio,
            add_after_boil: Some(f.add_after_boil),
        }).await?;
    }

    let hop_repo = HopRepository::new(self.db);
    for h in hop_repo.list(recipe_id).await? {
        hop_repo.create(&id, CreateHopAdditionInput {
            hop_id: h.hop_id,
            name: h.name,
            alpha_pct: h.alpha_pct,
            form: Some(h.form),
            amount_kg: h.amount_kg * ratio,
            use_: h.use_,
            time_min: h.time_min,
            hopstand_temp_c: h.hopstand_temp_c,
        }).await?;
    }

    let yeast_repo = YeastRepository::new(self.db);
    for y in yeast_repo.list(recipe_id).await? {
        yeast_repo.create(&id, CreateYeastAdditionInput {
            yeast_id: y.yeast_id,
            name: y.name,
            type_: y.type_,
            form: y.form,
            laboratory: y.laboratory,
            product_id: y.product_id,
            attenuation_pct: y.attenuation_pct,
            amount: y.amount.map(|a| a * ratio),
            amount_is_weight: Some(y.amount_is_weight),
            add_to_secondary: Some(y.add_to_secondary),
            times_cultured: Some(y.times_cultured),
        }).await?;
    }

    let misc_repo = MiscRepository::new(self.db);
    for m in misc_repo.list(recipe_id).await? {
        misc_repo.create(&id, CreateMiscAdditionInput {
            misc_id: m.misc_id,
            name: m.name,
            type_: m.type_,
            use_: m.use_,
            amount: m.amount * ratio,
            amount_is_weight: Some(m.amount_is_weight),
            time_min: m.time_min,
        }).await?;
    }

    let water_repo = WaterRepository::new(self.db);
    for w in water_repo.list(recipe_id).await? {
        water_repo.create(&id, CreateWaterAdditionInput {
            water_id: w.water_id,
            name: w.name,
            amount_l: w.amount_l * ratio,
        }).await?;
    }

    let water_chem_repo = WaterChemistryRepository::new(self.db);
    for a in water_chem_repo.list_adjustments(recipe_id).await? {
        water_chem_repo.create_water_adjustment(&id, CreateWaterAdjustmentInput {
            addition: a.addition.to_string().parse()
                .map_err(|e| AppError::Internal(format!("{}", e)))?,
            target: a.target.to_string().parse()
                .map_err(|e| AppError::Internal(format!("{}", e)))?,
            amount: a.amount * ratio,
        }).await?;
    }

    if let Some(mash) = src.mash {
        let mash_repo = MashRepository::new(self.db);
        let new_mash = mash_repo.upsert_for_recipe(&id, UpdateMashInput {
            name: Some(mash.name),
            grain_temp_c: Some(mash.grain_temp_c),
            tun_temp_c: mash.tun_temp_c,
            sparge_temp_c: mash.sparge_temp_c,
            ph: mash.ph,
            ratio_l_per_kg: mash.ratio_l_per_kg,
            notes: mash.notes,
        }).await?;
        for step in mash.steps {
            mash_repo.create_step(&new_mash.id, CreateMashStepInput {
                name: step.name,
                type_: Some(step.type_),
                step_temp_c: step.step_temp_c,
                step_time_min: step.step_time_min,
                infuse_amount_l: step.infuse_amount_l.map(|v| v * ratio),
                ramp_time_min: step.ramp_time_min,
                end_temp_c: step.end_temp_c,
            }).await?;
        }
    }

    self.get(&id).await
}
```

- [ ] **Step 5: Run tests to verify they pass**

```bash
cd /Users/shead/Documents/code/brewski/src-tauri
cargo test repositories::recipe::tests::test_scale 2>&1 | grep -E "test |FAILED|error\[" | head -20
```

Expected: `test repositories::recipe::tests::test_scale_creates_new_recipe ... ok`, `test_scale_ingredients ... ok`, `test_scale_mash_steps ... ok`

- [ ] **Step 6: Add the `scale_recipe` Tauri command**

In `src-tauri/src/commands/recipes.rs`, add this function after the `delete_recipe` function:

```rust
#[tauri::command]
pub async fn scale_recipe(
    state: State<'_, AppState>,
    recipe_id: String,
    new_batch_size_l: f64,
) -> Result<Recipe, AppError> {
    RecipeRepository::new(&state.db).scale(&recipe_id, new_batch_size_l).await
}
```

Also add `Recipe` to the existing `use crate::models::` import if not already present. The current import likely includes `Recipe` already (visible in `get_recipe`'s return type). No change needed there.

- [ ] **Step 7: Register the command in `src-tauri/src/lib.rs`**

In `src/lib.rs`, inside the `.invoke_handler(tauri::generate_handler![...])` block, add `commands::recipes::scale_recipe,` after `commands::recipes::get_recipe_stats,`:

```rust
commands::recipes::get_recipe_stats,
commands::recipes::scale_recipe,
```

- [ ] **Step 8: Verify the backend compiles**

```bash
cd /Users/shead/Documents/code/brewski/src-tauri
cargo build 2>&1 | grep -E "error\[|warning: unused" | head -20
```

Expected: clean build (no errors)

- [ ] **Step 9: Commit**

```bash
cd /Users/shead/Documents/code/brewski
git add src-tauri/src/repositories/recipe.rs src-tauri/src/commands/recipes.rs src-tauri/src/lib.rs
git commit -m "feat: add scale_recipe backend command with proportional scaling"
```

---

### Task 2: Frontend API function

**Files:**
- Modify: `src/lib/api.ts`

- [ ] **Step 1: Add `scaleRecipe` to `src/lib/api.ts`**

In `src/lib/api.ts`, after the `deleteRecipe` export (around line 104), add:

```ts
export const scaleRecipe = (recipeId: string, newBatchSizeL: number) =>
  invoke<Recipe>("scale_recipe", { recipeId, newBatchSizeL });
```

- [ ] **Step 2: Verify type-check passes**

```bash
cd /Users/shead/Documents/code/brewski
npm run check 2>&1 | tail -5
```

Expected: `0 ERRORS`

- [ ] **Step 3: Commit**

```bash
git add src/lib/api.ts
git commit -m "feat: add scaleRecipe frontend API function"
```

---

### Task 3: `ScaleRecipeModal` component and tests

**Files:**
- Create: `src/lib/components/ScaleRecipeModal.svelte`
- Create: `tests/ScaleRecipeModal.test.ts`

**Context — existing patterns:**

The `ConfirmModal.svelte` component (`src/lib/components/ConfirmModal.svelte`) shows the modal shell pattern: a fixed overlay, a centered card, Cancel and Confirm buttons. Reuse its visual style.

Unit conversion utilities from `src/lib/units.ts`:
- `lToGal(l: number): number` — liters → US gallons
- `galToL(gal: number): number` — US gallons → liters
- `volumeLabel(units: Units): string` — returns `"gal"` or `"L"`
- `type Units = "metric" | "imperial"`

The `$settings` store is from `$lib/stores/settings`. Access `$settings.units` for the measurement system.

For IPC calls use `ipc(...)` from `$lib/stores/error` — same pattern as all other components.

Test mock pattern (from `tests/BatchGravityTab.test.ts`):
```ts
vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("$lib/stores/error", () => ({ ipc: vi.fn((p) => p) }));
vi.mock("$app/navigation", () => ({ goto: vi.fn() }));
```

- [ ] **Step 1: Write the failing tests**

Create `tests/ScaleRecipeModal.test.ts`:

```ts
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import { tick } from "svelte";
import ScaleRecipeModal from "$lib/components/ScaleRecipeModal.svelte";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("$lib/stores/error", () => ({ ipc: vi.fn((p) => p) }));
vi.mock("$app/navigation", () => ({ goto: vi.fn() }));

let currentUnits = "metric";

vi.mock("$lib/stores/settings", () => ({
  settings: {
    subscribe: vi.fn((fn) => {
      fn({ units: currentUnits });
      return () => {};
    }),
  },
}));

const mockInvoke = vi.mocked((await import("@tauri-apps/api/core")).invoke);
const { goto } = await import("$app/navigation");
const mockGoto = vi.mocked(goto);

beforeEach(() => {
  mockInvoke.mockReset();
  mockGoto.mockReset();
  currentUnits = "metric";
});

describe("ScaleRecipeModal", () => {
  it("pre-fills input with current batch size in liters (metric)", () => {
    render(ScaleRecipeModal, {
      props: { recipeId: "r1", currentBatchSizeL: 23.0, onClose: vi.fn() },
    });
    const input = screen.getByRole("spinbutton") as HTMLInputElement;
    expect(input.value).toBe("23");
    expect(screen.getByText("L")).toBeTruthy();
  });

  it("pre-fills input with current batch size in gallons (imperial)", async () => {
    currentUnits = "imperial";
    render(ScaleRecipeModal, {
      props: { recipeId: "r1", currentBatchSizeL: 23.0, onClose: vi.fn() },
    });
    const input = screen.getByRole("spinbutton") as HTMLInputElement;
    // 23L * 0.264172 ≈ 6.08 gal
    expect(parseFloat(input.value)).toBeCloseTo(6.08, 1);
    expect(screen.getByText("gal")).toBeTruthy();
  });

  it("calls scale_recipe with liters and navigates to new recipe on confirm", async () => {
    const user = userEvent.setup();
    mockInvoke.mockResolvedValue({ id: "new-recipe-id", name: "My IPA (scaled)" });

    render(ScaleRecipeModal, {
      props: { recipeId: "r1", currentBatchSizeL: 23.0, onClose: vi.fn() },
    });

    const input = screen.getByRole("spinbutton");
    await user.clear(input);
    await user.type(input, "46");

    await user.click(screen.getByRole("button", { name: /scale/i }));
    await tick();
    await tick();

    expect(mockInvoke).toHaveBeenCalledWith("scale_recipe", {
      recipeId: "r1",
      newBatchSizeL: 46,
    });
    expect(mockGoto).toHaveBeenCalledWith("/recipe/new-recipe-id");
  });

  it("calls onClose when Cancel is clicked", async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    render(ScaleRecipeModal, {
      props: { recipeId: "r1", currentBatchSizeL: 23.0, onClose },
    });
    await user.click(screen.getByRole("button", { name: /cancel/i }));
    expect(onClose).toHaveBeenCalled();
  });
});
```

- [ ] **Step 2: Run tests to verify they fail**

```bash
cd /Users/shead/Documents/code/brewski
npm test -- ScaleRecipeModal 2>&1 | tail -10
```

Expected: FAIL — `Cannot find module '$lib/components/ScaleRecipeModal.svelte'`

- [ ] **Step 3: Implement `ScaleRecipeModal.svelte`**

Create `src/lib/components/ScaleRecipeModal.svelte`:

```svelte
<script lang="ts">
  import { goto } from "$app/navigation";
  import { scaleRecipe } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { settings } from "$lib/stores/settings";
  import { lToGal, galToL, volumeLabel, type Units } from "$lib/units";

  let {
    recipeId,
    currentBatchSizeL,
    onClose,
  }: {
    recipeId: string;
    currentBatchSizeL: number;
    onClose: () => void;
  } = $props();

  const units = $derived<Units>($settings.units === "imperial" ? "imperial" : "metric");
  const initialValue = $derived(units === "imperial" ? lToGal(currentBatchSizeL) : currentBatchSizeL);

  let targetValue = $state(0);
  let scaling = $state(false);
  let error = $state<string | null>(null);

  $effect(() => {
    targetValue = parseFloat(initialValue.toFixed(2));
  });

  async function handleConfirm() {
    if (!targetValue || targetValue <= 0) return;
    scaling = true;
    error = null;
    const batchSizeL = units === "imperial" ? galToL(targetValue) : targetValue;
    const result = await ipc(scaleRecipe(recipeId, batchSizeL));
    scaling = false;
    if (result) {
      goto(`/recipe/${result.id}`);
    } else {
      error = "Scaling failed. Please try again.";
    }
  }
</script>

<div class="fixed inset-0 flex items-center justify-center" style="z-index: 1000;">
  <div
    class="absolute inset-0"
    style="background: rgba(0,0,0,0.4);"
    role="none"
    onclick={onClose}
    onkeydown={onClose}
  ></div>
  <div
    class="p-4 rounded relative flex flex-col gap-3"
    style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); z-index: 1001; min-width: 280px;"
  >
    <div class="text-sm font-semibold" style="color: var(--color-text-primary);">Scale Recipe</div>
    <div class="flex items-center gap-2">
      <label class="text-sm" style="color: var(--color-text-secondary);">Target Batch Size</label>
      <input
        type="number"
        bind:value={targetValue}
        min="0.1"
        step="0.1"
        class="px-2 py-1 rounded text-sm w-24 outline-none"
        style="background: var(--color-bg-surface); color: var(--color-text-primary); border: 1px solid var(--color-border);"
      />
      <span class="text-sm" style="color: var(--color-text-secondary);">{volumeLabel(units)}</span>
    </div>
    {#if error}
      <div class="text-xs" style="color: var(--color-text-danger, #e55);">{error}</div>
    {/if}
    <div class="flex justify-end gap-2">
      <button
        onclick={onClose}
        class="px-3 py-1 rounded text-sm"
        style="background: var(--color-bg-surface); color: var(--color-text-primary); border: 1px solid var(--color-border);"
      >Cancel</button>
      <button
        onclick={handleConfirm}
        disabled={scaling || !targetValue || targetValue <= 0}
        class="px-3 py-1 rounded text-sm"
        style="background: var(--color-accent); color: #fff;"
      >
        {scaling ? "Scaling…" : "Scale Recipe"}
      </button>
    </div>
  </div>
</div>
```

- [ ] **Step 4: Run tests to verify they pass**

```bash
npm test -- ScaleRecipeModal 2>&1 | tail -10
```

Expected: 4 tests pass

- [ ] **Step 5: Commit**

```bash
git add src/lib/components/ScaleRecipeModal.svelte tests/ScaleRecipeModal.test.ts
git commit -m "feat: add ScaleRecipeModal component"
```

---

### Task 4: Desktop RecipeView integration

**Files:**
- Modify: `src/lib/desktop/RecipeView.svelte`

**Context:**

The desktop header lives around line 228–354. The action buttons in the header follow this pattern:

```svelte
<button
  onclick={handler}
  class="text-xs px-2 py-1 rounded"
  style="color: var(--color-text-secondary); background: var(--color-bg-elevated); border: 1px solid var(--color-border);"
>
  Button Label
</button>
```

The `recipe` variable is `$state<Recipe | null>` and holds the current recipe. The `showDeleteModal` / `showBranchModal` state vars show the existing modal-toggle pattern.

Modals are rendered at the bottom of the `{#if recipe}` block (around line 423).

- [ ] **Step 1: Add `scaleRecipe` import and `showScaleModal` state**

In `src/lib/desktop/RecipeView.svelte`, add a `ScaleRecipeModal` component import after the `ConfirmModal` import (no change to the `$lib/api` import — `scaleRecipe` is called inside the modal, not here):

```ts
import ConfirmModal from "$lib/components/ConfirmModal.svelte";
import ScaleRecipeModal from "$lib/components/ScaleRecipeModal.svelte";
```

Add the state variable after the `showBranchModal` state:

```ts
// Scale modal state
let showScaleModal = $state(false);
```

- [ ] **Step 2: Add the Scale button to the header**

In the header (around line 294–354), add the Scale Recipe button after the "Save Version" `<div class="relative">` block and before the Export BeerXML button:

```svelte
<!-- Scale Recipe button -->
<button
  onclick={() => { showScaleModal = true; }}
  class="text-xs px-2 py-1 rounded"
  style="color: var(--color-text-secondary); background: var(--color-bg-elevated); border: 1px solid var(--color-border);"
>
  Scale Recipe
</button>
```

- [ ] **Step 3: Render the modal conditionally**

At the bottom of the `{#if recipe}` block (after the existing `{#if showBranchModal && branchCandidate}` block), add:

```svelte
{#if showScaleModal && recipe}
  <ScaleRecipeModal
    recipeId={recipe.id}
    currentBatchSizeL={recipe.batch_size_l}
    onClose={() => { showScaleModal = false; }}
  />
{/if}
```

- [ ] **Step 4: Run type-check**

```bash
cd /Users/shead/Documents/code/brewski
npm run check 2>&1 | tail -5
```

Expected: `0 ERRORS`

- [ ] **Step 5: Run all tests**

```bash
npm test 2>&1 | tail -8
```

Expected: all tests pass

- [ ] **Step 6: Commit**

```bash
git add src/lib/desktop/RecipeView.svelte
git commit -m "feat: add Scale Recipe button to desktop recipe view"
```

---

### Task 5: Mobile RecipeView integration

**Files:**
- Modify: `src/lib/mobile/RecipeView.svelte`

**Context:**

The mobile header lives around lines 107–129. It has a `‹ Recipes` back button, the recipe name, and an Export BeerXML icon button. Add the Scale button between the recipe name and the export button — or after the export button as a text button to match the desktop pattern.

The mobile view uses `goto` (already imported) and `ipc` (already imported).

- [ ] **Step 1: Add `ScaleRecipeModal` import and state**

In `src/lib/mobile/RecipeView.svelte`, add:

```ts
import ScaleRecipeModal from "$lib/components/ScaleRecipeModal.svelte";
```

after the `RecipeHero` import line.

Add the state variable near the other state declarations:

```ts
let showScaleModal = $state(false);
```

- [ ] **Step 2: Add the Scale button to the mobile header**

In the header block (around line 107–129), add a Scale button after the export icon button:

```svelte
<button
  onclick={() => { showScaleModal = true; }}
  class="flex items-center justify-center rounded flex-shrink-0 text-xs px-2"
  style="height: 28px; color: var(--color-text-secondary); background: var(--color-bg-elevated); border: 1px solid var(--color-border); border-radius: var(--radius-md);"
>
  Scale
</button>
```

- [ ] **Step 3: Render the modal conditionally**

After the `<input type="file" ...>` hidden input (around line 131), add:

```svelte
{#if showScaleModal && recipe}
  <ScaleRecipeModal
    recipeId={recipe.id}
    currentBatchSizeL={recipe.batch_size_l}
    onClose={() => { showScaleModal = false; }}
  />
{/if}
```

- [ ] **Step 4: Run type-check and all tests**

```bash
cd /Users/shead/Documents/code/brewski
npm run check 2>&1 | tail -5 && npm test 2>&1 | tail -8
```

Expected: `0 ERRORS`, all tests pass

- [ ] **Step 5: Commit**

```bash
git add src/lib/mobile/RecipeView.svelte
git commit -m "feat: add Scale Recipe button to mobile recipe view"
```
