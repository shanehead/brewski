# Hop Format Support Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Extend hop form support from `{Pellet, Plug, Leaf}` to `{Pellet, Plug, Leaf, Cryo, CO2 Extract}` with correct IBU utilization per form.

**Architecture:** No DB migration needed — `form` is already a plain TEXT column on all three hop tables. Backend changes are confined to `ibu.rs` and `mod.rs`. Frontend changes touch two platform-specific IngredientPicker files, the library editor modal, and the hops table component.

**Tech Stack:** Rust (Tauri backend, `cargo test`), Svelte 5, TypeScript, openapi-typescript for type generation.

---

## File Map

| File | Change |
|------|--------|
| `src-tauri/src/brewing/ibu.rs` | Add `form` to `HopIbuInput`, add `form_utilization` fn, CO2 extract code path, update all test fixtures |
| `src-tauri/src/brewing/mod.rs` | Pass `form: &h.form` when building `HopIbuInput` |
| `docs/openapi/components/schemas/Hop.yaml` | Update `form` description |
| `docs/openapi/components/schemas/CreateHopInput.yaml` | Update `form` description |
| `docs/openapi/components/schemas/RecipeAdditionHop.yaml` | Add `form` description |
| `docs/openapi/components/schemas/CreateHopAdditionInput.yaml` | Add `form` description |
| `src/lib/api.gen.ts` | Regenerate via `npx openapi-typescript` |
| `src/lib/components/ingredients/IngredientEditModal.svelte` | Add Cryo, CO2 Extract to hop form dropdown |
| `src/lib/desktop/IngredientPicker.svelte` | Add `form` to `AddPayload`, state, `$effect`, dropdown, `handleAdd` |
| `src/lib/mobile/IngredientPicker.svelte` | Same as desktop picker |
| `src/lib/components/ingredients/HopsTable.svelte` | Use `payload.form`, add non-Pellet form badge |

---

## Task 1: IBU Calculation — Tests and Implementation

**Files:**
- Modify: `src-tauri/src/brewing/ibu.rs`

- [ ] **Step 1: Add `form` field to `HopIbuInput` struct**

Replace the struct at the top of `ibu.rs`:

```rust
pub struct HopIbuInput<'a> {
    pub alpha_pct: &'a f64,
    pub amount_kg: &'a f64,
    pub time_min: &'a f64,
    pub use_type: &'a str,
    pub form: &'a str,
    /// Pre-resolved: per-hop override → recipe default → 80.0
    pub hopstand_temp_c: f64,
    /// Extra whirlpool time added to hopstand additions, in minutes
    pub whirlpool_time_min: f64,
    /// When Some, use this flat utilization fraction instead of the Malowicki model
    pub aroma_utilization_override: Option<f64>,
}
```

- [ ] **Step 2: Run tests to see compilation errors (expected)**

```bash
cd src-tauri && cargo test brewing::ibu 2>&1 | head -40
```

Expected: missing field `form` errors in all test `HopIbuInput` literals. This is expected — we'll fix them in the next step.

- [ ] **Step 3: Update all existing `HopIbuInput` literals in the test module**

In `ibu.rs`, add `form: "Pellet"` to every `HopIbuInput` in the `#[cfg(test)]` block. There are 10 instances across 7 tests. The pattern is identical for each — add one line after `use_type`:

```rust
// Before (example from test_ibu_single_addition):
HopIbuInput {
    alpha_pct: &10.0f64,
    amount_kg: &0.028f64,
    time_min: &60.0f64,
    use_type: "Boil",
    hopstand_temp_c: 80.0,
    whirlpool_time_min: 0.0,
    aroma_utilization_override: None,
}

// After:
HopIbuInput {
    alpha_pct: &10.0f64,
    amount_kg: &0.028f64,
    time_min: &60.0f64,
    use_type: "Boil",
    form: "Pellet",
    hopstand_temp_c: 80.0,
    whirlpool_time_min: 0.0,
    aroma_utilization_override: None,
}
```

Apply this to all `HopIbuInput` literals in: `test_ibu_single_addition`, `test_dry_hop_contributes_zero_ibu`, `test_mash_hop_contributes_zero_ibu`, `test_hopstand_contributes_less_than_boil` (2 literals), `test_hopstand_at_boiling_equals_boil` (2 literals), `test_first_wort_uses_boil_time` (2 literals).

- [ ] **Step 4: Write failing tests for new forms**

Add these tests to the `#[cfg(test)]` block at the bottom of `ibu.rs`:

```rust
#[test]
fn test_leaf_hop_reduces_ibu_by_15_percent() {
    let pellet = vec![HopIbuInput {
        alpha_pct: &10.0f64, amount_kg: &0.028f64, time_min: &60.0f64,
        use_type: "Boil", form: "Pellet",
        hopstand_temp_c: 80.0, whirlpool_time_min: 0.0, aroma_utilization_override: None,
    }];
    let leaf = vec![HopIbuInput {
        alpha_pct: &10.0f64, amount_kg: &0.028f64, time_min: &60.0f64,
        use_type: "Boil", form: "Leaf",
        hopstand_temp_c: 80.0, whirlpool_time_min: 0.0, aroma_utilization_override: None,
    }];
    let pellet_ibu = tinseth_ibu(&pellet, 1.047, 23.0, 60.0);
    let leaf_ibu = tinseth_ibu(&leaf, 1.047, 23.0, 60.0);
    let ratio = leaf_ibu / pellet_ibu;
    assert!((ratio - 0.85).abs() < 0.01, "leaf/pellet ratio {ratio:.4}, expected 0.85");
}

#[test]
fn test_plug_hop_reduces_ibu_by_15_percent() {
    let pellet = vec![HopIbuInput {
        alpha_pct: &10.0f64, amount_kg: &0.028f64, time_min: &60.0f64,
        use_type: "Boil", form: "Pellet",
        hopstand_temp_c: 80.0, whirlpool_time_min: 0.0, aroma_utilization_override: None,
    }];
    let plug = vec![HopIbuInput {
        alpha_pct: &10.0f64, amount_kg: &0.028f64, time_min: &60.0f64,
        use_type: "Boil", form: "Plug",
        hopstand_temp_c: 80.0, whirlpool_time_min: 0.0, aroma_utilization_override: None,
    }];
    let pellet_ibu = tinseth_ibu(&pellet, 1.047, 23.0, 60.0);
    let plug_ibu = tinseth_ibu(&plug, 1.047, 23.0, 60.0);
    let ratio = plug_ibu / pellet_ibu;
    assert!((ratio - 0.85).abs() < 0.01, "plug/pellet ratio {ratio:.4}, expected 0.85");
}

#[test]
fn test_cryo_hop_same_ibu_as_pellet() {
    let pellet = vec![HopIbuInput {
        alpha_pct: &10.0f64, amount_kg: &0.028f64, time_min: &60.0f64,
        use_type: "Boil", form: "Pellet",
        hopstand_temp_c: 80.0, whirlpool_time_min: 0.0, aroma_utilization_override: None,
    }];
    let cryo = vec![HopIbuInput {
        alpha_pct: &10.0f64, amount_kg: &0.028f64, time_min: &60.0f64,
        use_type: "Boil", form: "Cryo",
        hopstand_temp_c: 80.0, whirlpool_time_min: 0.0, aroma_utilization_override: None,
    }];
    let pellet_ibu = tinseth_ibu(&pellet, 1.047, 23.0, 60.0);
    let cryo_ibu = tinseth_ibu(&cryo, 1.047, 23.0, 60.0);
    assert!(
        (cryo_ibu - pellet_ibu).abs() < 0.01,
        "cryo IBU {cryo_ibu:.2} should equal pellet IBU {pellet_ibu:.2}"
    );
}

#[test]
fn test_co2_extract_ignores_boil_time() {
    let short = vec![HopIbuInput {
        alpha_pct: &10.0f64, amount_kg: &0.028f64, time_min: &1.0f64,
        use_type: "Boil", form: "CO2 Extract",
        hopstand_temp_c: 80.0, whirlpool_time_min: 0.0, aroma_utilization_override: None,
    }];
    let long = vec![HopIbuInput {
        alpha_pct: &10.0f64, amount_kg: &0.028f64, time_min: &60.0f64,
        use_type: "Boil", form: "CO2 Extract",
        hopstand_temp_c: 80.0, whirlpool_time_min: 0.0, aroma_utilization_override: None,
    }];
    let short_ibu = tinseth_ibu(&short, 1.047, 23.0, 60.0);
    let long_ibu = tinseth_ibu(&long, 1.047, 23.0, 60.0);
    assert!(
        (short_ibu - long_ibu).abs() < 0.01,
        "CO2 extract IBU should not depend on boil time: 1min={short_ibu:.2}, 60min={long_ibu:.2}"
    );
}

#[test]
fn test_co2_extract_dry_hop_contributes_zero_ibu() {
    let hops = vec![HopIbuInput {
        alpha_pct: &10.0f64, amount_kg: &0.028f64, time_min: &0.0f64,
        use_type: "Dry Hop", form: "CO2 Extract",
        hopstand_temp_c: 80.0, whirlpool_time_min: 0.0, aroma_utilization_override: None,
    }];
    let ibu = tinseth_ibu(&hops, 1.047, 23.0, 60.0);
    assert_eq!(ibu, 0.0);
}

#[test]
fn test_co2_extract_higher_utilization_than_pellet_at_60min() {
    // CO2 extract has full utilization (bigness factor only); pellet at 60min has ~22% utilization.
    // So equal weight and AA should yield more IBUs from CO2 extract.
    let co2 = vec![HopIbuInput {
        alpha_pct: &10.0f64, amount_kg: &0.028f64, time_min: &1.0f64,
        use_type: "Boil", form: "CO2 Extract",
        hopstand_temp_c: 80.0, whirlpool_time_min: 0.0, aroma_utilization_override: None,
    }];
    let pellet = vec![HopIbuInput {
        alpha_pct: &10.0f64, amount_kg: &0.028f64, time_min: &60.0f64,
        use_type: "Boil", form: "Pellet",
        hopstand_temp_c: 80.0, whirlpool_time_min: 0.0, aroma_utilization_override: None,
    }];
    let co2_ibu = tinseth_ibu(&co2, 1.047, 23.0, 60.0);
    let pellet_ibu = tinseth_ibu(&pellet, 1.047, 23.0, 60.0);
    assert!(
        co2_ibu > pellet_ibu,
        "CO2 extract (full utilization) {co2_ibu:.2} should exceed pellet 60min {pellet_ibu:.2}"
    );
}
```

- [ ] **Step 5: Run tests to confirm they fail**

```bash
cd src-tauri && cargo test brewing::ibu 2>&1 | tail -20
```

Expected: compilation error — `form` field exists in struct but `tinseth_ibu` doesn't use it yet, so tests should compile but the new form-specific tests will fail (leaf/plug/cryo IBUs will all equal pellet; CO2 won't ignore time).

- [ ] **Step 6: Implement `form_utilization` and update `tinseth_ibu`**

Add `form_utilization` before `tinseth_ibu`, and rewrite `tinseth_ibu` to use it:

```rust
fn form_utilization(form_lower: &str) -> f64 {
    match form_lower {
        "leaf" | "plug" => 0.85,
        _ => 1.0,
    }
}

pub fn tinseth_ibu(
    hops: &[HopIbuInput],
    og: f64,
    post_boil_volume_l: f64,
    boil_time_min: f64,
) -> f64 {
    let volume_gallons = post_boil_volume_l * 0.264172;
    // Tinseth bigness factor: accounts for wort gravity suppressing utilization.
    // Constants 1.65 and 0.000125 are empirically derived by Glenn Tinseth.
    let bigness = 1.65 * 0.000125f64.powf(og - 1.0);

    hops.iter()
        .map(|h| {
            let use_lower = h.use_type.to_lowercase();
            // Mash and dry hop never contribute IBUs regardless of form.
            if matches!(use_lower.as_str(), "mash" | "dry hop") {
                return 0.0;
            }
            let ounces = *h.amount_kg * 35.274;
            let alpha_fraction = *h.alpha_pct / 100.0;
            let form_lower = h.form.to_lowercase();
            // CO2 extract: fully isomerized — full utilization regardless of boil time.
            // Bigness still applies (gravity suppresses utilization even for extracts).
            if form_lower == "co2 extract" {
                return (bigness * alpha_fraction * ounces * 7490.0) / volume_gallons;
            }
            let effective_time = match use_lower.as_str() {
                "first wort" => boil_time_min,
                "hopstand" => {
                    if let Some(flat_util) = h.aroma_utilization_override {
                        return (flat_util * alpha_fraction * ounces * 7490.0) / volume_gallons;
                    }
                    malowicki_effective_time(*h.time_min + h.whirlpool_time_min, h.hopstand_temp_c)
                }
                _ => *h.time_min,
            };
            if effective_time <= 0.0 {
                return 0.0;
            }
            // Tinseth time factor: models the exponential approach to maximum utilization.
            // -0.04 is the time decay constant; 4.15 normalises to a 0–1 range.
            let time_factor = (1.0 - f64::exp(-0.04 * effective_time)) / 4.15;
            let utilization = bigness * time_factor * form_utilization(&form_lower);
            // 7490 converts (utilization × AAU × oz / gal) to IBUs.
            // Derived from Tinseth's original formula constants.
            (utilization * alpha_fraction * ounces * 7490.0) / volume_gallons
        })
        .sum()
}
```

- [ ] **Step 7: Run all IBU tests and confirm they pass**

```bash
cd src-tauri && cargo test brewing::ibu -- --nocapture 2>&1 | tail -20
```

Expected: all tests pass including the 6 new ones. If `test_hopstand_at_boiling_equals_boil` fails, check that the `hopstand` branch still works correctly.

- [ ] **Step 8: Run all backend tests**

```bash
cd src-tauri && cargo test 2>&1 | tail -10
```

Expected: all tests pass.

- [ ] **Step 9: Commit**

```bash
git add src-tauri/src/brewing/ibu.rs
git commit -m "feat: add hop form utilization to IBU calculation

Leaf and Plug: 0.85× Tinseth utilization.
Cryo: 1.0× (concentration is in AA%, not utilization).
CO2 Extract: full utilization (bigness factor only, no time factor)."
```

---

## Task 2: Pass Form Through in `mod.rs`

**Files:**
- Modify: `src-tauri/src/brewing/mod.rs:119-131`

- [ ] **Step 1: Update `HopIbuInput` construction to include `form`**

In `mod.rs`, find the `hop_inputs` construction (around line 119-131) and add `form`:

```rust
let hop_inputs: Vec<ibu::HopIbuInput> = recipe
    .hops
    .iter()
    .map(|h| ibu::HopIbuInput {
        alpha_pct: &h.alpha_pct,
        amount_kg: &h.amount_kg,
        time_min: &h.time_min,
        use_type: &h.use_,
        form: &h.form,
        hopstand_temp_c: h.hopstand_temp_c.unwrap_or(hopstand_default),
        whirlpool_time_min: whirlpool_time,
        aroma_utilization_override: aroma_hop_utilization_override,
    })
    .collect();
```

- [ ] **Step 2: Run all backend tests**

```bash
cd src-tauri && cargo test 2>&1 | tail -10
```

Expected: all tests pass. The `test_stats_with_hops` test already has `form: "pellet".into()` on the `RecipeAdditionHop`, so it compiles cleanly.

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/brewing/mod.rs
git commit -m "feat: pass hop form to IBU calculation"
```

---

## Task 3: OpenAPI Schemas and Type Regeneration

**Files:**
- Modify: `docs/openapi/components/schemas/Hop.yaml`
- Modify: `docs/openapi/components/schemas/CreateHopInput.yaml`
- Modify: `docs/openapi/components/schemas/RecipeAdditionHop.yaml`
- Modify: `docs/openapi/components/schemas/CreateHopAdditionInput.yaml`
- Modify: `src/lib/api.gen.ts` (regenerated)

- [ ] **Step 1: Update `Hop.yaml` form description**

In `docs/openapi/components/schemas/Hop.yaml`, change:

```yaml
  form:
    type: string
    description: Pellet, Plug, Leaf
```

to:

```yaml
  form:
    type: string
    description: "Pellet, Plug, Leaf, Cryo, CO2 Extract"
```

- [ ] **Step 2: Update `CreateHopInput.yaml` form description**

In `docs/openapi/components/schemas/CreateHopInput.yaml`, change:

```yaml
  form:
    type: string
    description: Pellet, Plug, Leaf
```

to:

```yaml
  form:
    type: string
    description: "Pellet, Plug, Leaf, Cryo, CO2 Extract"
```

- [ ] **Step 3: Add form description to `RecipeAdditionHop.yaml`**

In `docs/openapi/components/schemas/RecipeAdditionHop.yaml`, change:

```yaml
  form:
    type: string
```

to:

```yaml
  form:
    type: string
    description: "Pellet, Plug, Leaf, Cryo, CO2 Extract"
```

- [ ] **Step 4: Add form description to `CreateHopAdditionInput.yaml`**

In `docs/openapi/components/schemas/CreateHopAdditionInput.yaml`, change:

```yaml
  form:
    type: string
```

to:

```yaml
  form:
    type: string
    description: "Pellet, Plug, Leaf, Cryo, CO2 Extract"
```

- [ ] **Step 5: Regenerate TypeScript types**

```bash
npx openapi-typescript
```

Expected output: `🚀 brewski@v1 → src/lib/api.gen.ts`

- [ ] **Step 6: Verify the generated types compile**

```bash
npm run check 2>&1 | tail -20
```

Expected: no errors.

- [ ] **Step 7: Commit**

```bash
git add docs/openapi/components/schemas/Hop.yaml \
        docs/openapi/components/schemas/CreateHopInput.yaml \
        docs/openapi/components/schemas/RecipeAdditionHop.yaml \
        docs/openapi/components/schemas/CreateHopAdditionInput.yaml \
        src/lib/api.gen.ts
git commit -m "docs: update hop form enum to include Cryo and CO2 Extract"
```

---

## Task 4: Library Editor — Add New Form Options

**Files:**
- Modify: `src/lib/components/ingredients/IngredientEditModal.svelte:358-362`

- [ ] **Step 1: Add Cryo and CO2 Extract to the hop form dropdown**

Find the hop Form select (around line 358-362):

```svelte
<select bind:value={hopForm} class="px-2 py-1.5 rounded text-sm"
        style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);">
  <option>Pellet</option><option>Plug</option><option>Leaf</option>
</select>
```

Change to:

```svelte
<select bind:value={hopForm} class="px-2 py-1.5 rounded text-sm"
        style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);">
  <option>Pellet</option><option>Cryo</option><option>CO2 Extract</option><option>Plug</option><option>Leaf</option>
</select>
```

- [ ] **Step 2: Run type check**

```bash
npm run check 2>&1 | tail -10
```

Expected: no errors.

- [ ] **Step 3: Verify manually**

Start the app (`npm run tauri dev`), open the ingredient library, create or edit a hop. Confirm the Form dropdown shows: Pellet, Cryo, CO2 Extract, Plug, Leaf. Save a hop with each new form and confirm it persists correctly.

- [ ] **Step 4: Commit**

```bash
git add src/lib/components/ingredients/IngredientEditModal.svelte
git commit -m "feat: add Cryo and CO2 Extract to hop library form options"
```

---

## Task 5: Ingredient Picker — Form Dropdown

Both picker files (`desktop` and `mobile`) have identical `AddPayload`, state, `$effect`, `handleAdd`, and footer structures. Apply the same changes to both.

**Files:**
- Modify: `src/lib/desktop/IngredientPicker.svelte`
- Modify: `src/lib/mobile/IngredientPicker.svelte`

- [ ] **Step 1: Update `AddPayload` type in both files**

In both pickers, change:

```typescript
export type AddPayload =
  | { type: 'hop'; item: Hop; amount_kg: number; use_: string; time_min: number; hopstand_temp_c: number | null }
  | { type: 'fermentable'; item: Fermentable; amount_kg: number }
  | { type: 'yeast'; item: Yeast; amount: number };
```

to:

```typescript
export type AddPayload =
  | { type: 'hop'; item: Hop; form: string; amount_kg: number; use_: string; time_min: number; hopstand_temp_c: number | null }
  | { type: 'fermentable'; item: Fermentable; amount_kg: number }
  | { type: 'yeast'; item: Yeast; amount: number };
```

- [ ] **Step 2: Add `HOP_FORMS` constant and `form` state in both files**

In both pickers, after the `const HOP_USES` line, add:

```typescript
const HOP_FORMS = ['Pellet', 'Cryo', 'CO2 Extract', 'Plug', 'Leaf'] as const;
```

After the `let hopstand_temp_c = $state(80);` line, add:

```typescript
let hopForm = $state('Pellet');
```

- [ ] **Step 3: Update the selection `$effect` to reset `hopForm` in both files**

In both pickers, find the `$effect` that resets hop fields on selection (around line 148-153 in desktop):

```typescript
$effect(() => {
  if (!selected) return;
  if (type === 'hop') { amount = hopDisplayToKg(units === 'imperial' ? 1 : 28, units); use_ = 'boil'; time = 60; hopstand_temp_c = 80; }
  else if (type === 'fermentable') { amount = units === 'imperial' ? lbToKg(2) : 1.0; }
  else { amount = 1; }
});
```

Change the hop branch to also set `hopForm`:

```typescript
$effect(() => {
  if (!selected) return;
  if (type === 'hop') {
    const h = selected as Hop;
    amount = hopDisplayToKg(units === 'imperial' ? 1 : 28, units);
    use_ = 'boil';
    time = 60;
    hopstand_temp_c = 80;
    hopForm = h.form;
  }
  else if (type === 'fermentable') { amount = units === 'imperial' ? lbToKg(2) : 1.0; }
  else { amount = 1; }
});
```

- [ ] **Step 4: Update `handleAdd` in both files**

In both pickers, change the hop `onadd` call in `handleAdd`:

```typescript
// Before:
onadd({ type: 'hop', item: selected as Hop, amount_kg: amount, use_, time_min: time, hopstand_temp_c: use_ === 'hopstand' ? hopstand_temp_c : null });

// After:
onadd({ type: 'hop', item: selected as Hop, form: hopForm, amount_kg: amount, use_, time_min: time, hopstand_temp_c: use_ === 'hopstand' ? hopstand_temp_c : null });
```

- [ ] **Step 5: Add form dropdown to the desktop picker footer**

In `src/lib/desktop/IngredientPicker.svelte`, find the hop footer section (the `{:else}` block starting around line 355 with `border-top`). Add the Form dropdown **after** the Amount block and **before** the Use block:

```svelte
<div>
  <div style="font-size: 11px; color: var(--color-text-secondary); margin-bottom: 4px;">Form</div>
  <select bind:value={hopForm} style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); border-radius: 5px; padding: 5px 8px; color: var(--color-text-primary); font-size: 13px;">
    {#each HOP_FORMS as f}<option value={f}>{f}</option>{/each}
  </select>
</div>
```

- [ ] **Step 6: Add form dropdown to the mobile picker footer**

In `src/lib/mobile/IngredientPicker.svelte`, find the hop add controls section (around the `select` for Use). Add the Form dropdown in the same position — after Amount, before Use. Match the existing mobile styling (uses Tailwind classes):

```svelte
<div class="flex flex-col gap-1">
  <label class="text-xs" style="color: var(--color-text-secondary);">Form</label>
  <select bind:value={hopForm} class="px-3 py-2 rounded-lg text-sm"
          style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); color: var(--color-text-primary);">
    {#each HOP_FORMS as f}<option value={f}>{f}</option>{/each}
  </select>
</div>
```

- [ ] **Step 7: Run type check**

```bash
npm run check 2>&1 | tail -20
```

Expected: no errors. Adding `form` to `AddPayload` is additive — `HopsTable.svelte` still compiles because it currently reads `payload.item.form`, which remains valid. Task 6 switches it to `payload.form`.

- [ ] **Step 8: Commit (together with Task 6)**

Commit together with Task 6 changes — see Task 6 Step 4.

---

## Task 6: Hops Table — Use `payload.form` and Add Form Badge

**Files:**
- Modify: `src/lib/components/ingredients/HopsTable.svelte`

- [ ] **Step 1: Use `payload.form` when creating a recipe hop**

In `HopsTable.svelte`, find `handlePickerAdd` (lines 15-30). Change `form: payload.item.form` to `form: payload.form`:

```typescript
async function handlePickerAdd(payload: AddPayload) {
  if (payload.type !== "hop") return;
  const result = await ipc(createRecipeHop(recipe.id, {
    hop_id: payload.item.id,
    name: payload.item.name,
    alpha_pct: payload.item.alpha_pct,
    form: payload.form,
    amount_kg: payload.amount_kg,
    use_: payload.use_,
    time_min: payload.time_min,
    hopstand_temp_c: payload.hopstand_temp_c ?? undefined,
  }));
  if (result === undefined) return;
  adding = false;
  onchange();
}
```

- [ ] **Step 2: Add form badge to the hop name cell**

In `HopsTable.svelte`, find the table row name cell (around line 70):

```svelte
<td class="py-1.5" style="color: var(--color-text-primary);">{h.name}</td>
```

Replace with:

```svelte
<td class="py-1.5" style="color: var(--color-text-primary);">
  {h.name}
  {#if h.form !== 'Pellet'}
    {@const badgeColor =
      h.form === 'Cryo' ? 'background: #d1fae5; color: #065f46;' :
      h.form === 'CO2 Extract' ? 'background: #ede9fe; color: #5b21b6;' :
      'background: var(--color-bg-elevated); color: var(--color-text-secondary);'}
    <span style="font-size: 10px; padding: 1px 5px; border-radius: 4px; font-weight: 600; margin-left: 4px; {badgeColor}">{h.form}</span>
  {/if}
</td>
```

- [ ] **Step 3: Run type check**

```bash
npm run check 2>&1 | tail -20
```

Expected: no errors.

- [ ] **Step 4: Commit Tasks 5 and 6 together**

```bash
git add src/lib/desktop/IngredientPicker.svelte \
        src/lib/mobile/IngredientPicker.svelte \
        src/lib/components/ingredients/HopsTable.svelte
git commit -m "feat: add hop form selector to ingredient picker and badge to hops table"
```

- [ ] **Step 5: Verify manually**

Start the app (`npm run tauri dev`) and test:

1. Open a recipe, click **+ Add** on hops. Select any hop from the library — confirm the Form dropdown appears pre-filled with the library hop's form (Pellet for seeded hops).
2. Change Form to **Cryo**, add to recipe. Confirm a green "Cryo" badge appears next to the hop name in the table.
3. Change Form to **CO2 Extract**, add to recipe. Confirm a purple "CO2 Extract" badge appears.
4. Add a regular Pellet hop — confirm no badge appears.
5. Verify that IBU changes when switching between Pellet, Leaf, and CO2 Extract for the same hop (check the stats sidebar).

---

## Final Check

- [ ] **Run all backend tests one more time**

```bash
cd src-tauri && cargo test 2>&1 | tail -10
```

Expected: all tests pass.

- [ ] **Run frontend type check**

```bash
npm run check 2>&1 | tail -10
```

Expected: no errors.
