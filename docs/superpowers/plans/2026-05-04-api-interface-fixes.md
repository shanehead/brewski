# API Interface Fixes Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Bring the TypeScript API wrapper (`src/lib/api.ts`) into full alignment with the Rust models (`src-tauri/src/models.rs`) by adding missing types, missing invoke wrappers, and fixing incomplete interfaces.

**Architecture:** All changes are confined to `src/lib/api.ts`. Rust types are the source of truth — TypeScript interfaces must exactly mirror the serialized fields each Rust struct produces. No new files needed. Verification is `just check-ts` (runs `svelte-check`).

**Tech Stack:** TypeScript, SvelteKit, Tauri 2 `invoke()` IPC, Rust serde (snake_case serialization, `type_` fields serialized as `type_` in JSON — NOT remapped to camelCase by Tauri for struct fields).

**Important Tauri IPC naming note:**
- **Command parameters** (what you pass TO `invoke()`): Tauri auto-converts camelCase JS → snake_case Rust. So `invoke("cmd", { recipeId })` correctly reaches `recipe_id: String` in Rust.
- **Struct fields** (what comes BACK from `invoke()`): Serde serializes them as-is. `type_` in Rust → `type_` in JSON (NOT `type` — because of the `#[sqlx(rename = "type")]` annotation which only affects DB reads, not JSON). So TypeScript interfaces use `type_: string` to match.

---

## File Map

**Modify only:** `src/lib/api.ts`

**Reference (read-only):** `src-tauri/src/models.rs` — authoritative field list for each struct.

---

## Task 1: Add `Misc` and `Water` types and their invoke wrappers

These are **confirmed runtime bugs** — the Rust commands exist but are unreachable from the frontend.

**Files:**
- Modify: `src/lib/api.ts:183-268` (after the `Yeast` interface, before the `// --- Recipes ---` comment)

- [ ] **Step 1: Verify baseline passes**

```bash
just check-ts
```

Expected: exits 0 (no errors). Record this as your baseline.

- [ ] **Step 2: Add `Misc` interface after `Yeast` (line 192)**

Open `src/lib/api.ts`. After the closing `}` of the `Yeast` interface (currently line 192), insert:

```typescript
export interface Misc {
  id: string;
  name: string;
  type_: string;
  use_: string;
  time_min: number;
  notes: string | null;
  use_for: string | null;
  amount_is_weight: boolean;
}

export interface Water {
  id: string;
  name: string;
  calcium_ppm: number;
  bicarbonate_ppm: number;
  sulfate_ppm: number;
  chloride_ppm: number;
  sodium_ppm: number;
  magnesium_ppm: number;
  ph: number | null;
  notes: string | null;
}
```

- [ ] **Step 3: Add wrappers after `listYeastLibrary` (currently line 268)**

After the `listYeastLibrary` line, add:

```typescript
export const listMiscLibrary = () => invoke<Misc[]>("list_misc_library");
export const listWaterLibrary = () => invoke<Water[]>("list_water_library");
```

- [ ] **Step 4: Verify type check still passes**

```bash
just check-ts
```

Expected: exits 0.

- [ ] **Step 5: Commit**

```bash
git add src/lib/api.ts
git commit -m "fix: add missing Misc/Water types and listMiscLibrary/listWaterLibrary wrappers"
```

---

## Task 2: Fix incomplete addition interfaces

Three interfaces are missing fields that Rust returns. Adding them is purely additive and cannot break existing callers.

**Files:**
- Modify: `src/lib/api.ts:70-99` (the three addition interfaces)

Reference from `src-tauri/src/models.rs`:
- `RecipeAdditionYeast` (line 270-286): has `amount_is_weight: bool`, `add_to_secondary: bool`, `times_cultured: i64`
- `RecipeAdditionMisc` (line 288-302): has `misc_id: Option<String>`, `amount_is_weight: bool`
- `RecipeAdditionWater` (line 304-311): has `water_id: Option<String>`

- [ ] **Step 1: Fix `RecipeAdditionYeast`**

Replace the `RecipeAdditionYeast` interface (lines 70-81) with:

```typescript
export interface RecipeAdditionYeast {
  id: string;
  recipe_id: string;
  yeast_id: string | null;
  name: string;
  type_: string;
  form: string;
  laboratory: string | null;
  product_id: string | null;
  attenuation_pct: number | null;
  amount: number | null;
  amount_is_weight: boolean;
  add_to_secondary: boolean;
  times_cultured: number;
}
```

- [ ] **Step 2: Fix `RecipeAdditionMisc`**

Replace the `RecipeAdditionMisc` interface (lines 83-92) with:

```typescript
export interface RecipeAdditionMisc {
  id: string;
  recipe_id: string;
  misc_id: string | null;
  name: string;
  type_: string;
  use_: string;
  amount: number;
  amount_is_weight: boolean;
  time_min: number;
  addition_order: number;
}
```

- [ ] **Step 3: Fix `RecipeAdditionWater`**

Replace the `RecipeAdditionWater` interface (lines 94-99) with:

```typescript
export interface RecipeAdditionWater {
  id: string;
  recipe_id: string;
  water_id: string | null;
  name: string;
  amount_l: number;
}
```

- [ ] **Step 4: Verify**

```bash
just check-ts
```

Expected: exits 0.

- [ ] **Step 5: Commit**

```bash
git add src/lib/api.ts
git commit -m "fix: add missing fields to RecipeAdditionYeast, RecipeAdditionMisc, RecipeAdditionWater"
```

---

## Task 3: Fix `Mash` and `MashStep` interfaces

**Files:**
- Modify: `src/lib/api.ts:101-122`

Reference from `src-tauri/src/models.rs`:
- `Mash` (line 313-327): has `tun_temp_c: Option<f64>`, `tun_weight_kg: Option<f64>`, `tun_specific_heat: Option<f64>`, `equip_adjust: bool`
- `MashStep` (line 344-357): has `end_temp_c: Option<f64>`

- [ ] **Step 1: Fix `MashStep`**

Replace the `MashStep` interface (lines 101-111) with:

```typescript
export interface MashStep {
  id: string;
  mash_id: string;
  name: string;
  type_: string;
  infuse_amount_l: number | null;
  step_temp_c: number;
  step_time_min: number;
  ramp_time_min: number | null;
  end_temp_c: number | null;
  step_order: number;
}
```

- [ ] **Step 2: Fix `Mash`**

Replace the `Mash` interface (lines 113-122) with:

```typescript
export interface Mash {
  id: string;
  recipe_id: string;
  name: string;
  grain_temp_c: number;
  tun_temp_c: number | null;
  sparge_temp_c: number | null;
  ph: number | null;
  tun_weight_kg: number | null;
  tun_specific_heat: number | null;
  equip_adjust: boolean;
  notes: string | null;
  steps: MashStep[];
}
```

- [ ] **Step 3: Verify**

```bash
just check-ts
```

Expected: exits 0.

- [ ] **Step 4: Commit**

```bash
git add src/lib/api.ts
git commit -m "fix: add missing fields to Mash and MashStep interfaces"
```

---

## Task 4: Fix `Recipe`, `Style`, and `EquipmentProfile` interfaces

These have the most missing fields. All additions are non-breaking.

**Files:**
- Modify: `src/lib/api.ts:13-154`

Reference from `src-tauri/src/models.rs`:
- `Style` (line 3-29): has `category_number`, `style_letter`, `style_guide`, `carb_min_vols`, `carb_max_vols`, `notes`, `profile`, `ingredients`, `examples`
- `EquipmentProfile` (line 31-53): has `calc_boil_volume`, `tun_volume_l`, `tun_weight_kg`, `tun_specific_heat`, `lauter_deadspace_l`, `top_up_kettle_l`, `top_up_water_l`, `hop_utilization_pct`
- `Recipe` (line 155-199): has `asst_brewer`, `og`, `fg`, `tertiary_age_days`, `tertiary_temp_c`, `age_days`, `age_temp_c`, `priming_sugar_name`, `carbonation_temp_c`, `priming_sugar_equiv`, `keg_priming_factor`, `created_at`, `updated_at`

- [ ] **Step 1: Fix `EquipmentProfile`**

Replace the `EquipmentProfile` interface (lines 13-26) with:

```typescript
export interface EquipmentProfile {
  id: string;
  name: string;
  notes: string | null;
  boil_size_l: number;
  batch_size_l: number;
  calc_boil_volume: boolean;
  tun_volume_l: number | null;
  tun_weight_kg: number | null;
  tun_specific_heat: number | null;
  lauter_deadspace_l: number;
  top_up_kettle_l: number;
  trub_chiller_loss_l: number;
  evap_rate_pct_hr: number;
  boil_time_min: number;
  top_up_water_l: number;
  fermenter_loss_l: number;
  hop_utilization_pct: number;
  efficiency_pct: number;
  created_at: number;
  updated_at: number;
}
```

- [ ] **Step 2: Fix `Style`**

Replace the `Style` interface (lines 28-42) with:

```typescript
export interface Style {
  id: string;
  name: string;
  category: string;
  category_number: string;
  style_letter: string;
  style_guide: string;
  type_: string;
  og_min: number;
  og_max: number;
  fg_min: number;
  fg_max: number;
  ibu_min: number;
  ibu_max: number;
  color_min_srm: number;
  color_max_srm: number;
  carb_min_vols: number | null;
  carb_max_vols: number | null;
  abv_min_pct: number | null;
  abv_max_pct: number | null;
  notes: string | null;
  profile: string | null;
  ingredients: string | null;
  examples: string | null;
}
```

- [ ] **Step 3: Fix `Recipe`**

Replace the `Recipe` interface (lines 124-154) with:

```typescript
export interface Recipe {
  id: string;
  name: string;
  type_: string;
  brewer: string | null;
  asst_brewer: string | null;
  batch_size_l: number;
  boil_size_l: number;
  boil_time_min: number;
  efficiency_pct: number | null;
  style_id: string | null;
  equipment_profile_id: string | null;
  notes: string | null;
  taste_notes: string | null;
  taste_rating: number | null;
  og: number | null;
  fg: number | null;
  fermentation_stages: number;
  primary_age_days: number | null;
  primary_temp_c: number | null;
  secondary_age_days: number | null;
  secondary_temp_c: number | null;
  tertiary_age_days: number | null;
  tertiary_temp_c: number | null;
  age_days: number | null;
  age_temp_c: number | null;
  carbonation_vols: number | null;
  forced_carbonation: boolean;
  priming_sugar_name: string | null;
  carbonation_temp_c: number | null;
  priming_sugar_equiv: number | null;
  keg_priming_factor: number | null;
  date: string | null;
  created_at: number;
  updated_at: number;
  equipment_profile: EquipmentProfile | null;
  style: Style | null;
  fermentables: RecipeAdditionFermentable[];
  hops: RecipeAdditionHop[];
  yeasts: RecipeAdditionYeast[];
  miscs: RecipeAdditionMisc[];
  waters: RecipeAdditionWater[];
  mash: Mash | null;
}
```

- [ ] **Step 4: Verify**

```bash
just check-ts
```

Expected: exits 0.

- [ ] **Step 5: Commit**

```bash
git add src/lib/api.ts
git commit -m "fix: complete Recipe, Style, and EquipmentProfile interfaces to match Rust models"
```

---

## Task 5: Add typed input interfaces and fix `updateRecipe`

Currently `updateRecipe` accepts `Partial<Recipe>` (wrong — lets callers pass `fermentables`, `id`, etc.) and all addition `update*` functions accept `object` (untyped). This task exports the Rust input types as TypeScript interfaces and wires them in.

**Files:**
- Modify: `src/lib/api.ts` — add input interfaces before the `// --- Recipes ---` comment, then update wrapper signatures.

Reference from `src-tauri/src/models.rs` (lines 386-564): `UpdateRecipeInput`, `UpdateFermentableAdditionInput`, `UpdateHopAdditionInput`, `UpdateYeastAdditionInput`, `UpdateMiscAdditionInput`, `UpdateWaterAdditionInput`.

- [ ] **Step 1: Add input interfaces**

Immediately before the `// --- Recipes ---` comment (currently around line 194), insert the following block:

```typescript
// --- Input types ---

export interface UpdateRecipeInput {
  name?: string;
  type_?: string;
  brewer?: string;
  asst_brewer?: string;
  batch_size_l?: number;
  boil_size_l?: number;
  boil_time_min?: number;
  efficiency_pct?: number;
  style_id?: string;
  equipment_profile_id?: string;
  notes?: string;
  taste_notes?: string;
  taste_rating?: number;
  fermentation_stages?: number;
  primary_age_days?: number;
  primary_temp_c?: number;
  secondary_age_days?: number;
  secondary_temp_c?: number;
  tertiary_age_days?: number;
  tertiary_temp_c?: number;
  age_days?: number;
  age_temp_c?: number;
  carbonation_vols?: number;
  forced_carbonation?: boolean;
  priming_sugar_name?: string;
  carbonation_temp_c?: number;
  date?: string;
}

export interface UpdateFermentableAdditionInput {
  amount_kg?: number;
  add_after_boil?: boolean;
  addition_order?: number;
}

export interface UpdateHopAdditionInput {
  amount_kg?: number;
  use_?: string;
  time_min?: number;
  addition_order?: number;
}

export interface UpdateYeastAdditionInput {
  attenuation_pct?: number;
  amount?: number;
  amount_is_weight?: boolean;
  add_to_secondary?: boolean;
  times_cultured?: number;
}

export interface UpdateMiscAdditionInput {
  amount?: number;
  amount_is_weight?: boolean;
  use_?: string;
  time_min?: number;
  addition_order?: number;
}

export interface UpdateWaterAdditionInput {
  amount_l?: number;
}
```

- [ ] **Step 2: Fix `updateRecipe` signature**

Find this line (currently around line 206):

```typescript
export const updateRecipe = (id: string, input: Partial<Recipe>) =>
  invoke<Recipe>("update_recipe", { id, input });
```

Replace with:

```typescript
export const updateRecipe = (id: string, input: UpdateRecipeInput) =>
  invoke<Recipe>("update_recipe", { id, input });
```

- [ ] **Step 3: Fix addition update wrappers**

Find and replace each of the following (they all currently accept `object`):

```typescript
export const updateRecipeFermentable = (id: string, input: object) =>
  invoke<RecipeAdditionFermentable>("update_recipe_fermentable", { id, input });
```
→
```typescript
export const updateRecipeFermentable = (id: string, input: UpdateFermentableAdditionInput) =>
  invoke<RecipeAdditionFermentable>("update_recipe_fermentable", { id, input });
```

```typescript
export const updateRecipeHop = (id: string, input: object) =>
  invoke<RecipeAdditionHop>("update_recipe_hop", { id, input });
```
→
```typescript
export const updateRecipeHop = (id: string, input: UpdateHopAdditionInput) =>
  invoke<RecipeAdditionHop>("update_recipe_hop", { id, input });
```

```typescript
export const updateRecipeYeast = (id: string, input: object) =>
  invoke<RecipeAdditionYeast>("update_recipe_yeast", { id, input });
```
→
```typescript
export const updateRecipeYeast = (id: string, input: UpdateYeastAdditionInput) =>
  invoke<RecipeAdditionYeast>("update_recipe_yeast", { id, input });
```

```typescript
export const updateRecipeMisc = (id: string, input: object) =>
  invoke<RecipeAdditionMisc>("update_recipe_misc", { id, input });
```
→
```typescript
export const updateRecipeMisc = (id: string, input: UpdateMiscAdditionInput) =>
  invoke<RecipeAdditionMisc>("update_recipe_misc", { id, input });
```

```typescript
export const updateRecipeWater = (id: string, input: object) =>
  invoke<RecipeAdditionWater>("update_recipe_water", { id, input });
```
→
```typescript
export const updateRecipeWater = (id: string, input: UpdateWaterAdditionInput) =>
  invoke<RecipeAdditionWater>("update_recipe_water", { id, input });
```

- [ ] **Step 4: Verify — expect possible call-site errors**

```bash
just check-ts
```

If any `.svelte` files pass objects with extra/wrong fields to the update wrappers, `svelte-check` will now catch them. Read the error output carefully. For each error: the field being passed either doesn't belong in the input (remove it) or is a valid field missing from the input interface (add it after verifying in `src-tauri/src/models.rs`). Fix all errors until the command exits 0.

- [ ] **Step 5: Commit**

```bash
git add src/lib/api.ts
git commit -m "fix: add typed input interfaces and remove Partial<Recipe> from updateRecipe"
```

---

## Self-Review

**Spec coverage check:**

| Issue from audit | Task that covers it |
|---|---|
| Missing `list_misc_library` / `list_water_library` wrappers | Task 1 |
| Missing `Misc` and `Water` interfaces | Task 1 |
| `RecipeAdditionYeast` missing 3 fields | Task 2 |
| `RecipeAdditionMisc` missing 2 fields | Task 2 |
| `RecipeAdditionWater` missing 1 field | Task 2 |
| `Mash` missing 4 fields | Task 3 |
| `MashStep` missing `end_temp_c` | Task 3 |
| `Recipe` missing 13 fields | Task 4 |
| `Style` missing 9 fields | Task 4 |
| `EquipmentProfile` missing 8 fields | Task 4 |
| `updateRecipe` takes `Partial<Recipe>` | Task 5 |
| Addition update wrappers accept `object` | Task 5 |

All issues covered. No placeholders in plan. Types used in later tasks match definitions from earlier tasks.
