# Strike Temperature Calculator Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a strike water temperature display to the Mash tab, calculated in Rust from existing recipe data and a stored water:grain ratio fallback.

**Architecture:** A new `brewing/strike.rs` pure function computes the result, which is wired into `calculate_stats` and returned as `RecipeStats.strike_temp_c`. The Mash tab reads this from `stats` and shows a ratio input field only when auto-derivation fails. A new DB migration adds `ratio_l_per_kg` to `mashes`.

**Tech Stack:** Rust (SeaORM, sea-orm-migration), SQLite, SvelteKit 5 / Svelte 5 runes, TypeScript, TailwindCSS 4. Test commands: `just test-rust` (cargo test), `just check-ts` (svelte-check).

---

## File Map

| File | Action |
|------|--------|
| `src-tauri/src/migration/sql/003_strike_temp.sql` | Create — ALTER TABLE SQL |
| `src-tauri/src/migration/m003_strike_temp.rs` | Create — Rust migration struct |
| `src-tauri/src/migration/mod.rs` | Modify — register m003 |
| `src-tauri/src/entities/mashes.rs` | Modify — add `ratio_l_per_kg` field (or regenerate via `just gen-entities`) |
| `src-tauri/src/brewing/strike.rs` | Create — pure calculation function + unit tests |
| `src-tauri/src/brewing/mod.rs` | Modify — register module, wire into `calculate_stats` |
| `src-tauri/src/models.rs` | Modify — `Mash`, `UpdateMashInput`, `RecipeStats` |
| `src-tauri/src/repositories/mash.rs` | Modify — map ratio from entity, apply in upsert |
| `src/lib/units.ts` | Modify — add ratio conversion helpers |
| `src/lib/api.ts` | Modify — `Mash`, `UpdateMashInput`, `updateMash`, `RecipeStats` |
| `src/routes/recipe/[id]/+page.svelte` | Modify — pass `stats` to MashTab |
| `src/lib/components/tabs/MashTab.svelte` | Modify — add `stats` prop, strike temp display, ratio input |

---

## Task 1: DB Migration and Entity

Add `ratio_l_per_kg` to the `mashes` table and update the SeaORM entity.

**Files:**
- Create: `src-tauri/src/migration/sql/003_strike_temp.sql`
- Create: `src-tauri/src/migration/m003_strike_temp.rs`
- Modify: `src-tauri/src/migration/mod.rs`
- Modify: `src-tauri/src/entities/mashes.rs`

- [ ] **Step 1: Create the SQL migration file**

Create `src-tauri/src/migration/sql/003_strike_temp.sql`:

```sql
ALTER TABLE mashes ADD COLUMN ratio_l_per_kg REAL;
```

- [ ] **Step 2: Create the Rust migration struct**

Create `src-tauri/src/migration/m003_strike_temp.rs`:

```rust
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str { "m003_strike_temp" }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(include_str!("sql/003_strike_temp.sql"))
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("ALTER TABLE mashes DROP COLUMN ratio_l_per_kg")
            .await?;
        Ok(())
    }
}
```

- [ ] **Step 3: Register the migration**

Open `src-tauri/src/migration/mod.rs`. Replace the entire file with:

```rust
use sea_orm_migration::prelude::*;

mod m001_initial;
mod m002_seed_data;
mod m003_strike_temp;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m001_initial::Migration),
            Box::new(m002_seed_data::Migration),
            Box::new(m003_strike_temp::Migration),
        ]
    }
}
```

- [ ] **Step 4: Update the SeaORM entity**

Open `src-tauri/src/entities/mashes.rs`. Add `ratio_l_per_kg` to the `Model` struct after `tun_specific_heat`:

```rust
    pub tun_specific_heat: Option<f64>,
    pub equip_adjust: Option<i32>,
    pub ratio_l_per_kg: Option<f64>,
```

The full updated struct becomes:

```rust
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "mashes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub id: String,
    #[sea_orm(column_type = "Text", unique)]
    pub recipe_id: String,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    pub grain_temp_c: f64,
    pub tun_temp_c: Option<f64>,
    pub sparge_temp_c: Option<f64>,
    pub ph: Option<f64>,
    pub tun_weight_kg: Option<f64>,
    pub tun_specific_heat: Option<f64>,
    pub equip_adjust: Option<i32>,
    pub ratio_l_per_kg: Option<f64>,
    #[sea_orm(column_type = "Text", nullable)]
    pub notes: Option<String>,
}
```

- [ ] **Step 5: Verify the project compiles**

```bash
cd src-tauri && cargo build 2>&1 | head -30
```

Expected: no errors. Warnings about unused fields are OK.

- [ ] **Step 6: Verify tests still pass**

```bash
just test-rust
```

Expected: all tests pass (the migration runs in-memory in tests — the new `ALTER TABLE` applies cleanly to a fresh DB).

- [ ] **Step 7: Commit**

```bash
git add src-tauri/src/migration/sql/003_strike_temp.sql \
        src-tauri/src/migration/m003_strike_temp.rs \
        src-tauri/src/migration/mod.rs \
        src-tauri/src/entities/mashes.rs
git commit -m "feat: add ratio_l_per_kg column to mashes via migration m003"
```

---

## Task 2: Pure Calculation Function

**Files:**
- Create: `src-tauri/src/brewing/strike.rs`
- Modify: `src-tauri/src/brewing/mod.rs` (add `pub mod strike;`)

- [ ] **Step 1: Write the failing test**

Create `src-tauri/src/brewing/strike.rs` with the test first:

```rust
pub fn calculate_strike_temp(grain_temp_c: f64, target_temp_c: f64, ratio_l_per_kg: f64) -> f64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strike_temp_reference_value() {
        // grain 20°C, target 67°C, ratio 3.0 L/kg → ~71.4°C
        // Formula: (0.2 / 3.0) * (67 - 20) + 67 = 0.0667 * 47 + 67 = 3.13 + 67 = 70.13... 
        // Actual: (0.2/3.0)*(67-20)+67 = 0.06667*47+67 = 3.133+67 = 70.13°C
        let result = calculate_strike_temp(20.0, 67.0, 3.0);
        assert!((result - 70.13).abs() < 0.1, "expected ~70.13°C, got {result:.2}");
    }

    #[test]
    fn test_strike_temp_higher_ratio_needs_less_heating() {
        // More water per grain → less thermal mass adjustment → closer to target temp
        let low_ratio = calculate_strike_temp(20.0, 67.0, 2.0);
        let high_ratio = calculate_strike_temp(20.0, 67.0, 4.0);
        assert!(low_ratio > high_ratio, "lower ratio should require higher strike temp");
    }

    #[test]
    fn test_strike_temp_cold_grain_needs_hotter_strike() {
        let warm_grain = calculate_strike_temp(20.0, 67.0, 3.0);
        let cold_grain = calculate_strike_temp(10.0, 67.0, 3.0);
        assert!(cold_grain > warm_grain, "colder grain should require higher strike temp");
    }
}
```

- [ ] **Step 2: Run the test to confirm it fails**

```bash
cd src-tauri && cargo test brewing::strike 2>&1 | tail -10
```

Expected: FAIL — `todo!()` panics.

- [ ] **Step 3: Implement the function**

Replace `todo!()` with the formula:

```rust
pub fn calculate_strike_temp(grain_temp_c: f64, target_temp_c: f64, ratio_l_per_kg: f64) -> f64 {
    (0.2 / ratio_l_per_kg) * (target_temp_c - grain_temp_c) + target_temp_c
}
```

- [ ] **Step 4: Register the module**

Open `src-tauri/src/brewing/mod.rs`. Add `pub mod strike;` at the top alongside the other modules:

```rust
pub mod abv;
pub mod ibu;
pub mod og;
pub mod srm;
pub mod strike;
pub mod volumes;
```

- [ ] **Step 5: Run the tests to confirm they pass**

```bash
cd src-tauri && cargo test brewing::strike 2>&1 | tail -10
```

Expected: 3 tests pass.

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/brewing/strike.rs src-tauri/src/brewing/mod.rs
git commit -m "feat: add calculate_strike_temp pure function with unit tests"
```

---

## Task 3: Data Model Changes

Add `ratio_l_per_kg` to `Mash` and `UpdateMashInput`, and `strike_temp_c` to `RecipeStats`.

**Files:**
- Modify: `src-tauri/src/models.rs`

- [ ] **Step 1: Add `ratio_l_per_kg` to the `Mash` struct**

Open `src-tauri/src/models.rs`. Find the `Mash` struct (around line 518). Add `ratio_l_per_kg` after `equip_adjust`:

```rust
pub struct Mash {
    pub id: String,
    pub recipe_id: String,
    pub name: String,
    pub grain_temp_c: f64,
    pub tun_temp_c: Option<f64>,
    pub sparge_temp_c: Option<f64>,
    pub ph: Option<f64>,
    pub tun_weight_kg: Option<f64>,
    pub tun_specific_heat: Option<f64>,
    pub equip_adjust: bool,
    pub ratio_l_per_kg: Option<f64>,
    pub notes: Option<String>,
    pub steps: Vec<MashStep>,
}
```

- [ ] **Step 2: Add `ratio_l_per_kg` to `UpdateMashInput`**

Find `UpdateMashInput` (around line 719). Add the field:

```rust
pub struct UpdateMashInput {
    pub name: Option<String>,
    pub grain_temp_c: Option<f64>,
    pub tun_temp_c: Option<f64>,
    pub sparge_temp_c: Option<f64>,
    pub ph: Option<f64>,
    pub notes: Option<String>,
    pub ratio_l_per_kg: Option<f64>,
}
```

- [ ] **Step 3: Add `strike_temp_c` to `RecipeStats`**

Find `RecipeStats` (around line 566). Add the field:

```rust
pub struct RecipeStats {
    pub og: f64,
    pub fg: f64,
    pub abv_pct: f64,
    pub ibu: f64,
    pub srm: f64,
    pub calories_per_355ml: f64,
    pub bu_gu_ratio: f64,
    pub pre_boil_gravity: f64,
    pub pre_boil_volume_l: f64,
    pub post_boil_volume_l: f64,
    pub strike_temp_c: Option<f64>,
}
```

- [ ] **Step 4: Add temporary `strike_temp_c: None` placeholder to `calculate_stats`**

Adding `strike_temp_c` to `RecipeStats` causes a compile error in `brewing/mod.rs` because the `RecipeStats { ... }` return literal is missing the field. Open `src-tauri/src/brewing/mod.rs` and add `strike_temp_c: None` to the `RecipeStats { ... }` literal at the end of `calculate_stats`:

```rust
    RecipeStats {
        og,
        fg,
        abv_pct,
        ibu,
        srm,
        calories_per_355ml: calories,
        bu_gu_ratio,
        pre_boil_gravity,
        pre_boil_volume_l,
        post_boil_volume_l,
        strike_temp_c: None,
    }
```

Task 5 replaces `strike_temp_c: None` with the real calculation.

- [ ] **Step 5: Verify it compiles (expect errors in repositories only)**

```bash
cd src-tauri && cargo build 2>&1 | grep "^error" | head -20
```

Expected: compile errors in `repositories/mash.rs` (missing `ratio_l_per_kg` in struct init). These are resolved in Task 4.

- [ ] **Step 7: Commit**

```bash
git add src-tauri/src/models.rs src-tauri/src/brewing/mod.rs
git commit -m "feat: add ratio_l_per_kg to Mash/UpdateMashInput, strike_temp_c to RecipeStats"
```

---

## Task 4: Repository Changes

Wire `ratio_l_per_kg` through the mash repository: read it from the entity and apply it in upsert.

**Files:**
- Modify: `src-tauri/src/repositories/mash.rs`

- [ ] **Step 1: Update `fetch_mash` to map the new field**

Open `src-tauri/src/repositories/mash.rs`. In `fetch_mash`, the `Mash { ... }` struct literal is missing `ratio_l_per_kg`. Add it:

```rust
        let mash = Mash {
            id: mash_row.id,
            recipe_id: mash_row.recipe_id,
            name: mash_row.name,
            grain_temp_c: mash_row.grain_temp_c,
            tun_temp_c: mash_row.tun_temp_c,
            sparge_temp_c: mash_row.sparge_temp_c,
            ph: mash_row.ph,
            tun_weight_kg: mash_row.tun_weight_kg,
            tun_specific_heat: mash_row.tun_specific_heat,
            equip_adjust,
            ratio_l_per_kg: mash_row.ratio_l_per_kg,
            notes: mash_row.notes,
            steps: steps?,
        };
```

- [ ] **Step 2: Handle `ratio_l_per_kg` in the UPDATE path of `upsert_for_recipe`**

In `upsert_for_recipe`, inside the `if let Some(mash_row) = existing` block, add after the `notes` handler (before `active.update`):

```rust
            if let Some(v) = input.ratio_l_per_kg {
                active.ratio_l_per_kg = Set(Some(v));
            }
```

- [ ] **Step 3: Handle `ratio_l_per_kg` in the INSERT path of `upsert_for_recipe`**

In the `else` block where a new mash is inserted, add `ratio_l_per_kg` to the `ActiveModel`:

```rust
            mashes::ActiveModel {
                id: Set(id.clone()),
                recipe_id: Set(recipe_id.to_string()),
                name: Set(input.name.unwrap_or_else(|| "Mash".to_string())),
                grain_temp_c: Set(input.grain_temp_c.unwrap_or(20.0)),
                tun_temp_c: Set(input.tun_temp_c),
                sparge_temp_c: Set(input.sparge_temp_c),
                ph: Set(input.ph),
                tun_weight_kg: Set(None),
                tun_specific_heat: Set(None),
                equip_adjust: Set(Some(0i32)),
                ratio_l_per_kg: Set(input.ratio_l_per_kg),
                notes: Set(input.notes),
            }
```

- [ ] **Step 4: Add a test for ratio round-trip**

In the `#[cfg(test)]` module at the bottom of `repositories/mash.rs`, add after `test_upsert_updates_existing_mash`:

```rust
    #[tokio::test]
    async fn test_ratio_l_per_kg_round_trip() {
        let db = setup_test_db().await;
        let recipe_id = create_recipe(&db).await;
        let repo = MashRepository::new(&db);

        let mash = repo
            .upsert_for_recipe(
                &recipe_id,
                UpdateMashInput {
                    name: Some("Mash".into()),
                    ratio_l_per_kg: Some(3.5),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        assert_eq!(mash.ratio_l_per_kg, Some(3.5));

        // Update the ratio
        let updated = repo
            .upsert_for_recipe(
                &recipe_id,
                UpdateMashInput {
                    ratio_l_per_kg: Some(2.8),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        assert_eq!(updated.ratio_l_per_kg, Some(2.8));
    }
```

- [ ] **Step 5: Run the tests**

```bash
just test-rust
```

Expected: all tests pass including the new `test_ratio_l_per_kg_round_trip`.

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/repositories/mash.rs
git commit -m "feat: wire ratio_l_per_kg through mash repository"
```

---

## Task 5: Wire Strike Temp into calculate_stats

**Files:**
- Modify: `src-tauri/src/brewing/mod.rs`

- [ ] **Step 1: Write a failing integration test in `brewing/mod.rs`**

Open `src-tauri/src/brewing/mod.rs`. In the `#[cfg(test)]` module, add a new test after `test_stats_with_hops`. The `minimal_recipe()` and `pale_malt()` helpers are already defined in this module — use them.

Add this helper and test:

```rust
    fn mash_with_infusion(grain_temp_c: f64, step_temp_c: f64, infuse_amount_l: f64) -> crate::models::Mash {
        crate::models::Mash {
            id: "m1".into(),
            recipe_id: "r1".into(),
            name: "Single Infusion".into(),
            grain_temp_c,
            tun_temp_c: None,
            sparge_temp_c: None,
            ph: None,
            tun_weight_kg: None,
            tun_specific_heat: None,
            equip_adjust: false,
            ratio_l_per_kg: None,
            notes: None,
            steps: vec![crate::models::MashStep {
                id: "s1".into(),
                mash_id: "m1".into(),
                name: "Mash In".into(),
                type_: "infusion".into(),
                infuse_amount_l: Some(infuse_amount_l),
                step_temp_c,
                step_time_min: 60,
                ramp_time_min: None,
                end_temp_c: None,
                step_order: 0,
            }],
        }
    }

    #[test]
    fn test_strike_temp_derived_from_infuse_amount() {
        let mut recipe = minimal_recipe();
        recipe.fermentables = vec![pale_malt()]; // pale_malt() has amount_kg: 4.5
        // ratio = 15.0 L / 4.5 kg = 3.333 L/kg
        // strike = (0.2/3.333)*(67-20)+67 = 0.06*47+67 = 2.82+67 = 69.82°C
        recipe.mash = Some(mash_with_infusion(20.0, 67.0, 15.0));
        let stats = calculate_stats(&recipe);
        let strike = stats.strike_temp_c.expect("strike_temp_c should be Some");
        assert!((strike - 69.82).abs() < 0.5, "expected ~69.82°C, got {strike:.2}");
    }

    #[test]
    fn test_strike_temp_none_without_mash() {
        let recipe = minimal_recipe();
        let stats = calculate_stats(&recipe);
        assert!(stats.strike_temp_c.is_none());
    }

    #[test]
    fn test_strike_temp_fallback_to_stored_ratio() {
        let mut recipe = minimal_recipe();
        recipe.fermentables = vec![pale_malt()];
        let mut mash = mash_with_infusion(20.0, 67.0, 15.0);
        // Remove infuse amount from the step so auto-derive fails
        mash.steps[0].infuse_amount_l = None;
        // Set stored fallback ratio
        mash.ratio_l_per_kg = Some(3.333);
        recipe.mash = Some(mash);
        let stats = calculate_stats(&recipe);
        let strike = stats.strike_temp_c.expect("should fall back to stored ratio");
        assert!((strike - 69.82).abs() < 0.5, "expected ~69.82°C, got {strike:.2}");
    }
```

- [ ] **Step 2: Run the tests to confirm they fail**

```bash
cd src-tauri && cargo test brewing::mod 2>&1 | tail -20
```

Expected: compile error — `calculate_stats` doesn't yet set `strike_temp_c` (the struct init is missing the field). Fix it: the test can't run until `strike_temp_c` is wired.

- [ ] **Step 3: Wire strike temp into `calculate_stats`**

Open `src-tauri/src/brewing/mod.rs`. At the bottom of `calculate_stats`, before the `RecipeStats { ... }` return, add:

```rust
    let strike_temp_c = recipe.mash.as_ref().and_then(|mash| {
        let grain_temp_c = mash.grain_temp_c;
        let target_temp_c = mash.steps.first()?.step_temp_c;
        let total_grain_kg: f64 = recipe.fermentables.iter().map(|f| f.amount_kg).sum();
        let derived_ratio = if total_grain_kg > 0.0 {
            mash.steps.iter().find_map(|s| s.infuse_amount_l.map(|vol| vol / total_grain_kg))
        } else {
            None
        };
        let ratio = derived_ratio.or(mash.ratio_l_per_kg)?;
        Some(strike::calculate_strike_temp(grain_temp_c, target_temp_c, ratio))
    });
```

Then add `strike_temp_c` to the `RecipeStats { ... }` literal at the end of the function:

```rust
    RecipeStats {
        og,
        fg,
        abv_pct,
        ibu,
        srm,
        calories_per_355ml: calories,
        bu_gu_ratio,
        pre_boil_gravity,
        pre_boil_volume_l,
        post_boil_volume_l,
        strike_temp_c,
    }
```

Also update the existing `RecipeStats` literal in `test_stats_empty_recipe` and other tests that directly construct `RecipeStats` — but actually these tests call `calculate_stats()` which returns `RecipeStats`, so they don't need to be updated. Check that `test_stats_empty_recipe` passes since `strike_temp_c` will be `None` there.

- [ ] **Step 4: Run all Rust tests**

```bash
just test-rust
```

Expected: all tests pass including the three new strike temp tests.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/brewing/mod.rs
git commit -m "feat: wire strike_temp_c into calculate_stats"
```

---

## Task 6: TypeScript API and Unit Helpers

**Files:**
- Modify: `src/lib/units.ts`
- Modify: `src/lib/api.ts`

- [ ] **Step 1: Add ratio conversion helpers to `units.ts`**

Open `src/lib/units.ts`. Append three exports at the end:

```typescript
export function lPerKgToQtPerLb(ratio: number): number { return ratio / 2.0864; }
export function qtPerLbToLPerKg(ratio: number): number { return ratio * 2.0864; }
export function ratioLabel(units: Units): string { return units === "imperial" ? "qt/lb" : "L/kg"; }
```

- [ ] **Step 2: Add `ratio_l_per_kg` to the `Mash` interface**

Open `src/lib/api.ts`. Find the `Mash` interface (around line 138). Add the field after `equip_adjust`:

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
  ratio_l_per_kg: number | null;
  notes: string | null;
  steps: MashStep[];
}
```

- [ ] **Step 3: Add `UpdateMashInput` interface and fix `updateMash` signature**

Find `updateMash` (around line 379):

```typescript
export const updateMash = (recipeId: string, input: object) =>
  invoke<Mash>("update_mash", { recipeId, input });
```

Before that line (or grouped with the `// --- Mash ---` section), add the `UpdateMashInput` interface and update the function signature. Replace from the `// --- Mash ---` comment through `updateMash`:

```typescript
// --- Mash ---

export interface UpdateMashInput {
  name?: string;
  grain_temp_c?: number;
  tun_temp_c?: number;
  sparge_temp_c?: number;
  ph?: number;
  notes?: string;
  ratio_l_per_kg?: number;
}

export const getMash = (recipeId: string) => invoke<Mash>("get_mash", { recipeId });
export const updateMash = (recipeId: string, input: UpdateMashInput) =>
  invoke<Mash>("update_mash", { recipeId, input });
```

- [ ] **Step 4: Add `strike_temp_c` to `RecipeStats`**

Find the `RecipeStats` interface in `api.ts`. Add the field:

```typescript
export interface RecipeStats {
  og: number;
  fg: number;
  abv_pct: number;
  ibu: number;
  srm: number;
  calories_per_355ml: number;
  bu_gu_ratio: number;
  pre_boil_gravity: number;
  pre_boil_volume_l: number;
  post_boil_volume_l: number;
  strike_temp_c: number | null;
}
```

- [ ] **Step 5: Run type check**

```bash
just check-ts
```

Expected: exits 0. If there are errors from `MashTab.svelte` passing `object` where `UpdateMashInput` is now expected, they will be caught here — fix them in the next task.

- [ ] **Step 6: Commit**

```bash
git add src/lib/units.ts src/lib/api.ts
git commit -m "feat: add UpdateMashInput, ratio/strike fields to TypeScript API types"
```

---

## Task 7: Frontend — MashTab Display

**Files:**
- Modify: `src/routes/recipe/[id]/+page.svelte`
- Modify: `src/lib/components/tabs/MashTab.svelte`

- [ ] **Step 1: Pass `stats` to MashTab in `+page.svelte`**

Open `src/routes/recipe/[id]/+page.svelte`. Find the line:

```svelte
        {:else if activeTab === "mash"}
          <MashTab {recipe} onchange={refreshRecipe} />
```

Replace with:

```svelte
        {:else if activeTab === "mash"}
          <MashTab {recipe} {stats} onchange={refreshRecipe} />
```

- [ ] **Step 2: Update `MashTab.svelte` props and imports**

Open `src/lib/components/tabs/MashTab.svelte`. Update the script section top:

Replace:
```typescript
  import type { Recipe, MashStep } from "$lib/api";
  import { updateMash, createMashStep, deleteMashStep } from "$lib/api";
  import { settings } from "$lib/stores/settings";
  import { type Units, cToF, fToC, lToGal, galToL, tempLabel, volumeLabel } from "$lib/units";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();
```

With:
```typescript
  import type { Recipe, MashStep, RecipeStats, UpdateMashInput } from "$lib/api";
  import { updateMash, createMashStep, deleteMashStep } from "$lib/api";
  import { settings } from "$lib/stores/settings";
  import { type Units, cToF, fToC, lToGal, galToL, tempLabel, volumeLabel, lPerKgToQtPerLb, qtPerLbToLPerKg, ratioLabel } from "$lib/units";

  let { recipe, stats, onchange }: { recipe: Recipe; stats: RecipeStats | null; onchange: () => void } = $props();
```

- [ ] **Step 3: Update `handleMashField` to use the typed input**

Find:
```typescript
  async function handleMashField(field: string, value: unknown) {
    await ensureMash();
    await updateMash(recipe.id, { [field]: value });
    onchange();
  }
```

Replace with:
```typescript
  async function handleMashField(input: UpdateMashInput) {
    await ensureMash();
    await updateMash(recipe.id, input);
    onchange();
  }
```

- [ ] **Step 4: Fix all `handleMashField` call sites**

Each existing call passes a field name string — update them to pass an object. Find and update:

```svelte
onblur={(e) => handleMashField("name", (e.target as HTMLInputElement).value)}
```
→
```svelte
onblur={(e) => handleMashField({ name: (e.target as HTMLInputElement).value })}
```

```svelte
onblur={(e) => { const v = parseFloat((e.target as HTMLInputElement).value); handleMashField("grain_temp_c", units === "imperial" ? fToC(v) : v); }}
```
→
```svelte
onblur={(e) => { const v = parseFloat((e.target as HTMLInputElement).value); handleMashField({ grain_temp_c: units === "imperial" ? fToC(v) : v }); }}
```

```svelte
onblur={(e) => {
  const v = (e.target as HTMLInputElement).value;
  handleMashField("sparge_temp_c", v ? (units === "imperial" ? fToC(parseFloat(v)) : parseFloat(v)) : null);
}}
```
→
```svelte
onblur={(e) => {
  const v = (e.target as HTMLInputElement).value;
  handleMashField({ sparge_temp_c: v ? (units === "imperial" ? fToC(parseFloat(v)) : parseFloat(v)) : null });
}}
```

```svelte
onblur={(e) => {
  const v = (e.target as HTMLInputElement).value;
  handleMashField("ph", v ? parseFloat(v) : null);
}}
```
→
```svelte
onblur={(e) => {
  const v = (e.target as HTMLInputElement).value;
  handleMashField({ ph: v ? parseFloat(v) : null });
}}
```

- [ ] **Step 5: Add derived values for auto-derive availability**

In the script section, after `const mash = $derived(recipe.mash);`, add:

```typescript
  const totalGrainKg = $derived(
    recipe.fermentables.reduce((sum, f) => sum + f.amount_kg, 0)
  );
  const firstInfuseAmount = $derived(
    recipe.mash?.steps.find(s => s.infuse_amount_l != null)?.infuse_amount_l ?? null
  );
  const canAutoDerive = $derived(totalGrainKg > 0 && firstInfuseAmount != null);
```

- [ ] **Step 6: Add the strike temp display and ratio input to the template**

In the template, find the closing `</div>` of the 2-column grid (after the mash pH field, before `<!-- Mash steps -->`). Insert inside that grid div, after the pH field:

```svelte
    {#if stats?.strike_temp_c != null}
      <div class="flex flex-col gap-1">
        <span class="text-xs font-medium" style="color: var(--color-text-secondary);">Strike Temp ({tempLabel(units)})</span>
        <span class="px-2 py-1.5 text-sm" style="color: var(--color-text-primary);">
          {(units === "imperial" ? cToF(stats.strike_temp_c) : stats.strike_temp_c).toFixed(1)}{tempLabel(units)}
        </span>
      </div>
    {/if}

    {#if mash && !canAutoDerive}
      <div class="flex flex-col gap-1">
        <label for="mash-ratio" class="text-xs font-medium" style="color: var(--color-text-secondary);">Water:Grain Ratio ({ratioLabel(units)})</label>
        <input id="mash-ratio" type="number" step="0.1"
               value={mash.ratio_l_per_kg != null
                 ? (units === "imperial" ? lPerKgToQtPerLb(mash.ratio_l_per_kg) : mash.ratio_l_per_kg).toFixed(2)
                 : ""}
               placeholder={units === "imperial" ? "1.5" : "3.0"}
               onblur={(e) => {
                 const v = (e.target as HTMLInputElement).value;
                 if (v) {
                   const parsed = parseFloat(v);
                   handleMashField({ ratio_l_per_kg: units === "imperial" ? qtPerLbToLPerKg(parsed) : parsed });
                 }
               }}
               class="px-2 py-1.5 rounded text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </div>
    {/if}
```

- [ ] **Step 7: Run type check**

```bash
just check-ts
```

Expected: exits 0.

- [ ] **Step 8: Commit**

```bash
git add src/routes/recipe/[id]/+page.svelte src/lib/components/tabs/MashTab.svelte
git commit -m "feat: display strike temp and ratio input in MashTab"
```

---

## Self-Review

**Spec coverage:**

| Spec requirement | Task |
|---|---|
| New SQL migration adds `ratio_l_per_kg` | Task 1 |
| SeaORM entity updated | Task 1 |
| `brewing/strike.rs` pure function | Task 2 |
| `calculate_strike_temp` unit tested | Task 2 |
| `Mash` model gets `ratio_l_per_kg` | Task 3 |
| `UpdateMashInput` gets `ratio_l_per_kg` | Task 3 |
| `RecipeStats` gets `strike_temp_c` | Task 3 |
| Repository maps ratio from entity | Task 4 |
| Repository applies ratio in upsert (update + insert) | Task 4 |
| Ratio round-trip integration test | Task 4 |
| `calculate_stats` derives ratio from infuse/grain weight | Task 5 |
| Fallback to `mash.ratio_l_per_kg` | Task 5 |
| `strike_temp_c` is `None` without mash | Task 5 |
| `api.ts` Mash, UpdateMashInput, RecipeStats updated | Task 6 |
| `updateMash` signature typed | Task 6 |
| `units.ts` ratio helpers | Task 6 |
| `+page.svelte` passes stats to MashTab | Task 7 |
| MashTab shows strike temp read-only | Task 7 |
| Ratio input shown only when auto-derive fails | Task 7 |
| Imperial unit conversion (°F, qt/lb) | Task 7 |

All requirements covered. No placeholders.
