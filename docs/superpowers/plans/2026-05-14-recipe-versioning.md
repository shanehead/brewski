# Recipe Versioning Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a tree-structured recipe versioning system allowing manual named snapshots, browsing past versions, and branching from any version to create a new line of development.

**Architecture:** The existing `recipe_versions` snapshot system is extended with a `parent_version_id` FK to express tree structure, and a transient `branch_parent_id` on `recipes` to route the next snapshot's parent. Three new Tauri commands (`get_recipe_version`, `save_recipe_version`, `branch_from_version`) plus a version history panel in the recipe editor complete the feature.

**Tech Stack:** Rust/SeaORM/SQLite (backend), SvelteKit/TypeScript (frontend), OpenAPI codegen via `just gen` and `just gen-entities`.

---

## File Map

**New files:**
- `src-tauri/migrations/002_recipe_versioning.sql`
- `docs/openapi/components/schemas/SaveRecipeVersionInput.yaml`
- `docs/openapi/paths/commands/save_recipe_version.yaml`
- `docs/openapi/paths/commands/branch_from_version.yaml`
- `src/lib/components/VersionHistoryPanel.svelte`

**Modified files:**
- `src-tauri/src/entities/recipe_versions.rs` — add `parent_version_id: Option<String>`
- `src-tauri/src/entities/recipes.rs` — add `branch_parent_id: Option<String>`
- `src-tauri/src/models.rs` — update `RecipeVersionSummary` TryFrom; add version → Recipe mappers
- `src-tauri/src/repositories/recipe_version.rs` — add `get_full`, `save_named`, `branch_from`; update `create_or_reuse` and `snapshot`
- `src-tauri/src/commands/batches.rs` — add `get_recipe_version`, `save_recipe_version`, `branch_from_version` commands
- `src-tauri/src/lib.rs` — register three new commands
- `docs/openapi/components/schemas/RecipeVersionSummary.yaml` — add `parent_version_id`
- `docs/openapi/openapi.yaml` — add two new paths + schema
- `src/lib/api.gen.ts` — regenerated (do not edit by hand)
- `src-tauri/src/models.gen.rs` — regenerated (do not edit by hand)
- `src/lib/api.ts` — add three new invoke wrappers
- `src/routes/recipe/[id]/+page.svelte` — version panel, read-only mode, Save Version button
- `src/lib/components/batch/BatchOverviewTab.svelte` — recipe version link

---

## Task 1: Database migration

**Files:**
- Create: `src-tauri/migrations/002_recipe_versioning.sql`
- Modify: `src-tauri/src/entities/recipe_versions.rs` (via `just gen-entities`)
- Modify: `src-tauri/src/entities/recipes.rs` (via `just gen-entities`)

- [ ] **Step 1: Write the migration**

Create `src-tauri/migrations/002_recipe_versioning.sql`:

```sql
ALTER TABLE recipe_versions ADD COLUMN parent_version_id TEXT REFERENCES recipe_versions(id);
ALTER TABLE recipes ADD COLUMN branch_parent_id TEXT REFERENCES recipe_versions(id);
```

- [ ] **Step 2: Apply migration and regenerate entities**

```bash
just gen-entities
```

Expected: outputs something like "Entity post-processing complete. Files processed: 24"

- [ ] **Step 3: Verify generated entities have the new columns**

Check that `src-tauri/src/entities/recipe_versions.rs` now has:
```rust
#[sea_orm(column_type = "Text", nullable)]
pub parent_version_id: Option<String>,
```

Check that `src-tauri/src/entities/recipes.rs` now has:
```rust
#[sea_orm(column_type = "Text", nullable)]
pub branch_parent_id: Option<String>,
```

- [ ] **Step 4: Run Rust tests to confirm migration is valid**

```bash
cd src-tauri && cargo test
```

Expected: all existing tests pass (the in-memory test DB runs migrations from the `migrations/` folder).

- [ ] **Step 5: Commit**

```bash
git add src-tauri/migrations/002_recipe_versioning.sql \
        src-tauri/src/entities/recipe_versions.rs \
        src-tauri/src/entities/recipes.rs
git commit -m "feat(db): add parent_version_id and branch_parent_id columns"
```

---

## Task 2: OpenAPI schema updates + codegen

**Files:**
- Modify: `docs/openapi/components/schemas/RecipeVersionSummary.yaml`
- Create: `docs/openapi/components/schemas/SaveRecipeVersionInput.yaml`
- Create: `docs/openapi/paths/commands/save_recipe_version.yaml`
- Create: `docs/openapi/paths/commands/branch_from_version.yaml`
- Modify: `docs/openapi/openapi.yaml`
- Regenerate: `src/lib/api.gen.ts`, `src-tauri/src/models.gen.rs`

- [ ] **Step 1: Add `parent_version_id` to `RecipeVersionSummary.yaml`**

Replace the entire contents of `docs/openapi/components/schemas/RecipeVersionSummary.yaml`:

```yaml
type: object
required:
  - id
  - recipe_id
  - version_number
  - created_at
properties:
  id:
    type: string
  recipe_id:
    type: string
  version_number:
    type: integer
  name:
    type: [string, "null"]
  parent_version_id:
    type: [string, "null"]
  created_at:
    type: integer
    format: int64
```

- [ ] **Step 2: Create `SaveRecipeVersionInput.yaml`**

Create `docs/openapi/components/schemas/SaveRecipeVersionInput.yaml`:

```yaml
type: object
required:
  - recipe_id
  - name
properties:
  recipe_id:
    type: string
  name:
    type: string
```

- [ ] **Step 3: Create `save_recipe_version.yaml` path**

Create `docs/openapi/paths/commands/save_recipe_version.yaml`:

```yaml
post:
  operationId: saveRecipeVersion
  summary: Manually save a named version snapshot of a recipe
  tags: [Batches]
  requestBody:
    required: true
    content:
      application/json:
        schema:
          $ref: "../../components/schemas/SaveRecipeVersionInput.yaml"
  responses:
    "200":
      description: The newly created version summary
      content:
        application/json:
          schema:
            $ref: "../../components/schemas/RecipeVersionSummary.yaml"
    "500":
      $ref: "../../components/responses/Error.yaml"
```

- [ ] **Step 4: Create `branch_from_version.yaml` path**

Create `docs/openapi/paths/commands/branch_from_version.yaml`:

```yaml
post:
  operationId: branchFromVersion
  summary: Restore a past version's data to the live recipe and set it as the branch parent
  tags: [Batches]
  requestBody:
    required: true
    content:
      application/json:
        schema:
          type: object
          required:
            - recipe_id
            - version_id
          properties:
            recipe_id:
              type: string
            version_id:
              type: string
  responses:
    "200":
      description: Success (no body)
    "500":
      $ref: "../../components/responses/Error.yaml"
```

- [ ] **Step 5: Register the new paths and schema in `openapi.yaml`**

In `docs/openapi/openapi.yaml`, add to the `paths:` section (after `/commands/list_recipe_versions:`):

```yaml
  /commands/save_recipe_version:
    $ref: ./paths/commands/save_recipe_version.yaml
  /commands/branch_from_version:
    $ref: ./paths/commands/branch_from_version.yaml
```

In the `components.schemas:` section (after `RecipeVersionSummary:`):

```yaml
    SaveRecipeVersionInput:
      $ref: ./components/schemas/SaveRecipeVersionInput.yaml
```

- [ ] **Step 6: Run codegen**

```bash
just gen
```

Expected: `src/lib/api.gen.ts` and `src-tauri/src/models.gen.rs` are regenerated with no errors.

- [ ] **Step 7: Update `TryFrom<entities::recipe_versions::Model> for RecipeVersionSummary` in `models.rs`**

In `src-tauri/src/models.rs`, find the existing impl (around line 346) and add `parent_version_id`:

```rust
impl TryFrom<entities::recipe_versions::Model> for RecipeVersionSummary {
    type Error = AppError;
    fn try_from(m: entities::recipe_versions::Model) -> Result<Self, AppError> {
        Ok(Self {
            id: m.id,
            recipe_id: m.recipe_id,
            version_number: m.version_number as i64,
            name: m.name,
            parent_version_id: m.parent_version_id,
            created_at: m.created_at as i64,
        })
    }
}
```

- [ ] **Step 8: Run Rust tests**

```bash
cd src-tauri && cargo test
```

Expected: all tests pass.

- [ ] **Step 9: Run TypeScript check**

```bash
bun run check
```

Expected: no type errors.

- [ ] **Step 10: Commit**

```bash
git add docs/openapi/ src/lib/api.gen.ts src-tauri/src/models.gen.rs src-tauri/src/models.rs
git commit -m "feat(openapi): add parent_version_id, save_recipe_version, branch_from_version"
```

---

## Task 3: Backend — `get_full` repository method

**Files:**
- Modify: `src-tauri/src/repositories/recipe_version.rs`

The `get_full` method reads a version snapshot from the `recipe_version_*` tables and assembles a `Recipe`-shaped struct. `id` is set to `version.recipe_id` so the frontend can still navigate to the recipe. `updated_at` uses `created_at` since versions are immutable.

Note: `recipe_version_hops` does not store `hopstand_temp_c` (column was added after the versioning schema was designed), so that field defaults to `None`.

- [ ] **Step 1: Write the failing test**

At the bottom of `src-tauri/src/repositories/recipe_version.rs`, add to the `#[cfg(test)]` module:

```rust
#[tokio::test]
async fn test_get_full_returns_recipe_shaped_data() {
    let db = setup_test_db().await;
    let recipe_id = make_recipe(&db).await;

    // Add a fermentable so there's something to round-trip
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

    let repo = RecipeVersionRepository::new(&db);
    let v = repo.create_or_reuse(&recipe_id).await.unwrap();
    let full = repo.get_full(&v.id).await.unwrap();

    assert_eq!(full.fermentables.len(), 1);
    assert_eq!(full.fermentables[0].name, "Pale Malt");
    assert_eq!(full.fermentables[0].amount_kg, 4.5);
}
```

- [ ] **Step 2: Run test to confirm it fails**

```bash
cd src-tauri && cargo test test_get_full_returns_recipe_shaped_data -- --nocapture
```

Expected: FAIL — `get_full` not found.

- [ ] **Step 3: Add required imports to `recipe_version.rs`**

At the top of the file, add to the existing `use crate::entities::` block:

```rust
use crate::entities::{
    equipment_profiles, recipe_addition_fermentables, recipe_addition_hops,
    recipe_addition_miscs, recipe_addition_waters, recipe_addition_yeasts,
    recipe_version_fermentables, recipe_version_hops, recipe_version_mash,
    recipe_version_mash_steps, recipe_version_miscs, recipe_version_water_adjustments,
    recipe_version_waters, recipe_version_yeasts, recipe_versions, recipes, styles,
};
use crate::models::{
    Mash, MashStep, Recipe, RecipeAdditionFermentable, RecipeAdditionHop,
    RecipeAdditionMisc, RecipeAdditionWater,
    RecipeAdditionYeast, RecipeVersionSummary, RecipeWaterAdjustment,
};
```

Note: keep the existing imports; replace the entire `use crate::entities` and `use crate::models` blocks with the above consolidated versions.

- [ ] **Step 4: Implement `get_full`**

Inside `impl<'a> RecipeVersionRepository<'a>`, add:

```rust
pub async fn get_full(&self, version_id: &str) -> Result<Recipe, AppError> {
    let v = recipe_versions::Entity::find_by_id(version_id)
        .one(self.db)
        .await?
        .ok_or(AppError::NotFound)?;

    let recipe_row = recipes::Entity::find_by_id(&v.recipe_id)
        .one(self.db)
        .await?
        .ok_or(AppError::NotFound)?;

    let style = if let Some(sid) = &v.style_id {
        use crate::models::Style;
        styles::Entity::find_by_id(sid)
            .one(self.db)
            .await?
            .map(Style::try_from)
            .transpose()?
    } else {
        None
    };

    let equipment_profile = if let Some(eid) = &v.equipment_profile_id {
        use crate::models::EquipmentProfile;
        equipment_profiles::Entity::find_by_id(eid)
            .one(self.db)
            .await?
            .map(EquipmentProfile::try_from)
            .transpose()?
    } else {
        None
    };

    let fermentables = recipe_version_fermentables::Entity::find()
        .filter(recipe_version_fermentables::Column::RecipeVersionId.eq(version_id))
        .order_by_asc(recipe_version_fermentables::Column::AdditionOrder)
        .all(self.db)
        .await?
        .into_iter()
        .map(|m| RecipeAdditionFermentable {
            id: m.id,
            recipe_id: v.recipe_id.clone(),
            fermentable_id: m.fermentable_id,
            name: m.name,
            type_: m.r#type,
            yield_pct: m.yield_pct,
            color_lovibond: m.color_lovibond,
            amount_kg: m.amount_kg,
            add_after_boil: m.add_after_boil.unwrap_or(0) != 0,
            addition_order: m.addition_order as i64,
        })
        .collect();

    let hops = recipe_version_hops::Entity::find()
        .filter(recipe_version_hops::Column::RecipeVersionId.eq(version_id))
        .order_by_asc(recipe_version_hops::Column::AdditionOrder)
        .all(self.db)
        .await?
        .into_iter()
        .map(|m| RecipeAdditionHop {
            id: m.id,
            recipe_id: v.recipe_id.clone(),
            hop_id: m.hop_id,
            name: m.name,
            alpha_pct: m.alpha_pct,
            form: m.form,
            amount_kg: m.amount_kg,
            use_: m.r#use,
            time_min: m.time_min,
            addition_order: m.addition_order as i64,
            hopstand_temp_c: None, // not captured in version snapshot
        })
        .collect();

    let yeasts = recipe_version_yeasts::Entity::find()
        .filter(recipe_version_yeasts::Column::RecipeVersionId.eq(version_id))
        .all(self.db)
        .await?
        .into_iter()
        .map(|m| RecipeAdditionYeast {
            id: m.id,
            recipe_id: v.recipe_id.clone(),
            yeast_id: m.yeast_id,
            name: m.name,
            type_: m.r#type,
            form: m.form,
            laboratory: m.laboratory,
            product_id: m.product_id,
            attenuation_pct: m.attenuation_pct,
            amount: m.amount,
            amount_is_weight: m.amount_is_weight.unwrap_or(0) != 0,
            add_to_secondary: m.add_to_secondary.unwrap_or(0) != 0,
            times_cultured: m.times_cultured.unwrap_or(0) as i64,
        })
        .collect();

    let miscs = recipe_version_miscs::Entity::find()
        .filter(recipe_version_miscs::Column::RecipeVersionId.eq(version_id))
        .order_by_asc(recipe_version_miscs::Column::AdditionOrder)
        .all(self.db)
        .await?
        .into_iter()
        .map(|m| RecipeAdditionMisc {
            id: m.id,
            recipe_id: v.recipe_id.clone(),
            misc_id: m.misc_id,
            name: m.name,
            type_: m.r#type,
            use_: m.r#use,
            amount: m.amount,
            amount_is_weight: m.amount_is_weight.unwrap_or(0) != 0,
            time_min: m.time_min,
            addition_order: m.addition_order as i64,
        })
        .collect();

    let waters = recipe_version_waters::Entity::find()
        .filter(recipe_version_waters::Column::RecipeVersionId.eq(version_id))
        .all(self.db)
        .await?
        .into_iter()
        .map(|m| RecipeAdditionWater {
            id: m.id,
            recipe_id: v.recipe_id.clone(),
            water_id: m.water_id,
            name: m.name,
            amount_l: m.amount_l,
        })
        .collect();

    let water_adjustments = recipe_version_water_adjustments::Entity::find()
        .filter(recipe_version_water_adjustments::Column::RecipeVersionId.eq(version_id))
        .all(self.db)
        .await?
        .into_iter()
        .map(|m| -> Result<RecipeWaterAdjustment, AppError> {
            Ok(RecipeWaterAdjustment {
                id: m.id,
                recipe_id: v.recipe_id.clone(),
                addition: m.addition.parse().map_err(|e| AppError::Internal(format!("{e}")))?,
                target: m.target.parse().map_err(|e| AppError::Internal(format!("{e}")))?,
                amount: m.amount,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mash = if let Some(vm) = recipe_version_mash::Entity::find()
        .filter(recipe_version_mash::Column::RecipeVersionId.eq(version_id))
        .one(self.db)
        .await?
    {
        let steps = recipe_version_mash_steps::Entity::find()
            .filter(recipe_version_mash_steps::Column::RecipeVersionMashId.eq(&vm.id))
            .order_by_asc(recipe_version_mash_steps::Column::StepOrder)
            .all(self.db)
            .await?
            .into_iter()
            .map(|s| MashStep {
                id: s.id,
                mash_id: vm.id.clone(),
                name: s.name,
                type_: s.r#type,
                infuse_amount_l: s.infuse_amount_l,
                step_temp_c: s.step_temp_c,
                step_time_min: s.step_time_min as i64,
                ramp_time_min: s.ramp_time_min.map(|v| v as i64),
                end_temp_c: s.end_temp_c,
                step_order: s.step_order as i64,
            })
            .collect();
        Some(Mash {
            id: vm.id,
            recipe_id: v.recipe_id.clone(),
            name: vm.name,
            grain_temp_c: vm.grain_temp_c,
            tun_temp_c: vm.tun_temp_c,
            sparge_temp_c: vm.sparge_temp_c,
            ph: vm.ph,
            notes: vm.notes,
            ratio_l_per_kg: vm.ratio_l_per_kg,
            tun_weight_kg: vm.tun_weight_kg,
            tun_specific_heat: vm.tun_specific_heat,
            equip_adjust: vm.equip_adjust.unwrap_or(0) != 0,
            steps,
        })
    } else {
        None
    };

    Ok(Recipe {
        id: v.recipe_id.clone(),
        name: recipe_row.name,
        type_: v.r#type,
        brewer: v.brewer,
        asst_brewer: v.asst_brewer,
        batch_size_l: v.batch_size_l,
        boil_size_l: v.boil_size_l,
        boil_time_min: v.boil_time_min,
        efficiency_pct: v.efficiency_pct,
        style_id: v.style_id,
        equipment_profile_id: v.equipment_profile_id,
        notes: v.notes,
        taste_notes: None,
        taste_rating: None,
        og: v.og,
        fg: v.fg,
        fermentation_stages: v.fermentation_stages.unwrap_or(1) as i64,
        primary_age_days: v.primary_age_days,
        primary_temp_c: v.primary_temp_c,
        secondary_age_days: v.secondary_age_days,
        secondary_temp_c: v.secondary_temp_c,
        tertiary_age_days: v.tertiary_age_days,
        tertiary_temp_c: v.tertiary_temp_c,
        age_days: v.age_days,
        age_temp_c: v.age_temp_c,
        carbonation_vols: v.carbonation_vols,
        forced_carbonation: v.forced_carbonation.unwrap_or(0) != 0,
        priming_sugar_name: v.priming_sugar_name,
        carbonation_temp_c: v.carbonation_temp_c,
        priming_sugar_equiv: v.priming_sugar_equiv,
        keg_priming_factor: v.keg_priming_factor,
        date: None,
        mash_water_id: v.mash_water_id,
        sparge_water_id: v.sparge_water_id,
        hopstand_temp_c: None,
        created_at: v.created_at as i64,
        updated_at: v.created_at as i64,
        style,
        equipment_profile,
        fermentables,
        hops,
        yeasts,
        miscs,
        waters,
        water_adjustments,
        mash,
    })
}
```

- [ ] **Step 5: Run the test to confirm it passes**

```bash
cd src-tauri && cargo test test_get_full_returns_recipe_shaped_data -- --nocapture
```

Expected: PASS

- [ ] **Step 6: Run all tests**

```bash
cd src-tauri && cargo test
```

Expected: all tests pass.

- [ ] **Step 7: Commit**

```bash
git add src-tauri/src/repositories/recipe_version.rs src-tauri/src/models.rs
git commit -m "feat(backend): add get_full to RecipeVersionRepository"
```

---

## Task 4: Backend — `save_named` repository method + update `snapshot` signature

**Files:**
- Modify: `src-tauri/src/repositories/recipe_version.rs`

`save_named` always creates a new snapshot (bypasses unchanged check) with a given name. It respects `branch_parent_id` if set on the recipe.

First, update `snapshot` to accept an optional name. Then add `save_named`.

- [ ] **Step 1: Write the failing test**

Add to the `#[cfg(test)]` module in `recipe_version.rs`:

```rust
#[tokio::test]
async fn test_save_named_always_creates_new_version() {
    let db = setup_test_db().await;
    let recipe_id = make_recipe(&db).await;
    let repo = RecipeVersionRepository::new(&db);

    // First brew creates v1
    let v1 = repo.create_or_reuse(&recipe_id).await.unwrap();

    // Manual save with same unchanged recipe still creates v2
    let v2 = repo.save_named(&recipe_id, "My checkpoint").await.unwrap();

    assert_ne!(v1.id, v2.id);
    assert_eq!(v2.version_number, 2);
    assert_eq!(v2.name.as_deref(), Some("My checkpoint"));
    assert_eq!(v2.parent_version_id.as_deref(), Some(v1.id.as_str()));
}
```

- [ ] **Step 2: Run the test to confirm it fails**

```bash
cd src-tauri && cargo test test_save_named_always_creates_new_version -- --nocapture
```

Expected: FAIL — `save_named` not found.

- [ ] **Step 3: Update `snapshot` to accept an optional name and parent**

Change the `snapshot` method signature and body. Find the existing `snapshot` function and replace it:

```rust
async fn snapshot(
    &self,
    recipe_id: &str,
    recipe: &crate::models::Recipe,
    version_number: i32,
    name: Option<String>,
    parent_version_id: Option<String>,
) -> Result<RecipeVersionSummary, AppError> {
    let version_id = new_id();
    let now = now_secs() as i32;

    recipe_versions::ActiveModel {
        id: Set(version_id.clone()),
        recipe_id: Set(recipe_id.to_string()),
        version_number: Set(version_number),
        name: Set(name),
        parent_version_id: Set(parent_version_id),
        // ... rest of the fields unchanged from current implementation
```

The fields after `parent_version_id` are exactly as today. Update all three call sites of `snapshot` inside `create_or_reuse`:

```rust
// first call (no last version):
self.snapshot(recipe_id, &recipe, 1, None, None).await

// second call (unchanged, won't reach here — handled by early return)

// third call (changed recipe):
self.snapshot(recipe_id, &recipe, next_number, None, Some(last_version.id.clone())).await
```

- [ ] **Step 4: Add `save_named` method**

```rust
pub async fn save_named(
    &self,
    recipe_id: &str,
    name: &str,
) -> Result<RecipeVersionSummary, AppError> {
    let recipe = RecipeRepository::new(self.db).get(recipe_id).await?;

    let recipe_row = recipes::Entity::find_by_id(recipe_id)
        .one(self.db)
        .await?
        .ok_or(AppError::NotFound)?;

    let branch_parent_id = recipe_row.branch_parent_id.clone();

    let parent_id = if let Some(bp) = branch_parent_id {
        // Clear branch_parent_id now that we're consuming it
        recipes::ActiveModel {
            id: Set(recipe_id.to_string()),
            branch_parent_id: Set(None),
            ..Default::default()
        }
        .update(self.db)
        .await?;
        Some(bp)
    } else {
        recipe_versions::Entity::find()
            .filter(recipe_versions::Column::RecipeId.eq(recipe_id))
            .order_by_desc(recipe_versions::Column::CreatedAt)
            .one(self.db)
            .await?
            .map(|v| v.id)
    };

    let next_number = recipe_versions::Entity::find()
        .filter(recipe_versions::Column::RecipeId.eq(recipe_id))
        .all(self.db)
        .await?
        .len() as i32
        + 1;

    self.snapshot(recipe_id, &recipe, next_number, Some(name.to_string()), parent_id)
        .await
}
```

- [ ] **Step 5: Run the test to confirm it passes**

```bash
cd src-tauri && cargo test test_save_named_always_creates_new_version -- --nocapture
```

Expected: PASS

- [ ] **Step 6: Run all tests**

```bash
cd src-tauri && cargo test
```

Expected: all tests pass.

- [ ] **Step 7: Commit**

```bash
git add src-tauri/src/repositories/recipe_version.rs
git commit -m "feat(backend): add save_named to RecipeVersionRepository"
```

---

## Task 5: Backend — update `create_or_reuse` to respect `branch_parent_id`

**Files:**
- Modify: `src-tauri/src/repositories/recipe_version.rs`

When `branch_parent_id` is set on the recipe, `create_or_reuse` must use it as the new version's parent and clear it afterwards. Otherwise the existing comparison/reuse logic applies.

- [ ] **Step 1: Write the failing test**

Add to the `#[cfg(test)]` module:

```rust
#[tokio::test]
async fn test_create_or_reuse_respects_branch_parent() {
    let db = setup_test_db().await;
    let recipe_id = make_recipe(&db).await;
    let repo = RecipeVersionRepository::new(&db);

    let v1 = repo.create_or_reuse(&recipe_id).await.unwrap();

    // Simulate branch_from_version having been called: set branch_parent_id = v1.id
    use sea_orm::{ActiveModelTrait, Set};
    use crate::entities::recipes;
    recipes::ActiveModel {
        id: Set(recipe_id.clone()),
        branch_parent_id: Set(Some(v1.id.clone())),
        ..Default::default()
    }
    .update(&db)
    .await
    .unwrap();

    // Add a fermentable so recipe is "changed" from v1 baseline
    FermentableRepository::new(&db)
        .create(
            &recipe_id,
            CreateFermentableAdditionInput {
                fermentable_id: None,
                name: "Crystal 60".into(),
                type_: "grain".into(),
                yield_pct: 70.0,
                color_lovibond: 60.0,
                amount_kg: 0.5,
                add_after_boil: None,
            },
        )
        .await
        .unwrap();

    let v2 = repo.create_or_reuse(&recipe_id).await.unwrap();

    assert_ne!(v1.id, v2.id);
    assert_eq!(v2.parent_version_id.as_deref(), Some(v1.id.as_str()));

    // branch_parent_id should be cleared on the recipe
    let recipe_row = crate::entities::recipes::Entity::find_by_id(&recipe_id)
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    assert!(recipe_row.branch_parent_id.is_none());
}
```

- [ ] **Step 2: Run the test to confirm it fails**

```bash
cd src-tauri && cargo test test_create_or_reuse_respects_branch_parent -- --nocapture
```

Expected: FAIL — parent_version_id is None or branch_parent_id not cleared.

- [ ] **Step 3: Update `create_or_reuse` to read and clear `branch_parent_id`**

Replace the current `create_or_reuse` method with:

```rust
pub async fn create_or_reuse(&self, recipe_id: &str) -> Result<RecipeVersionSummary, AppError> {
    let recipe = RecipeRepository::new(self.db).get(recipe_id).await?;

    let recipe_row = recipes::Entity::find_by_id(recipe_id)
        .one(self.db)
        .await?
        .ok_or(AppError::NotFound)?;

    let branch_parent_id = recipe_row.branch_parent_id.clone();

    // If a branch parent is set, always create a new snapshot branching from it
    if let Some(bp_id) = branch_parent_id {
        recipes::ActiveModel {
            id: Set(recipe_id.to_string()),
            branch_parent_id: Set(None),
            ..Default::default()
        }
        .update(self.db)
        .await?;

        let next_number = recipe_versions::Entity::find()
            .filter(recipe_versions::Column::RecipeId.eq(recipe_id))
            .all(self.db)
            .await?
            .len() as i32
            + 1;

        return self.snapshot(recipe_id, &recipe, next_number, None, Some(bp_id)).await;
    }

    let last = recipe_versions::Entity::find()
        .filter(recipe_versions::Column::RecipeId.eq(recipe_id))
        .order_by_desc(recipe_versions::Column::CreatedAt)
        .one(self.db)
        .await?;

    if let Some(last_version) = last {
        if self.matches_current(&last_version.id, &recipe).await? {
            return RecipeVersionSummary::try_from(last_version);
        }
        let next_number = last_version.version_number + 1;
        let parent_id = Some(last_version.id.clone());
        self.snapshot(recipe_id, &recipe, next_number, None, parent_id).await
    } else {
        self.snapshot(recipe_id, &recipe, 1, None, None).await
    }
}
```

Note: the previous `create_or_reuse` used `order_by_desc(version_number)` to find the last version. This is now changed to `order_by_desc(created_at)` for consistency with "most recent = highest created_at."

- [ ] **Step 4: Run the test to confirm it passes**

```bash
cd src-tauri && cargo test test_create_or_reuse_respects_branch_parent -- --nocapture
```

Expected: PASS

- [ ] **Step 5: Run all tests**

```bash
cd src-tauri && cargo test
```

Expected: all tests pass.

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/repositories/recipe_version.rs
git commit -m "feat(backend): create_or_reuse respects branch_parent_id for tree versioning"
```

---

## Task 6: Backend — `branch_from` repository method

**Files:**
- Modify: `src-tauri/src/repositories/recipe_version.rs`

`branch_from` restores a past version's full data to the live recipe (deletes current additions, re-inserts from version tables, updates recipe scalars) and sets `branch_parent_id`.

- [ ] **Step 1: Write the failing test**

Add to the `#[cfg(test)]` module:

```rust
#[tokio::test]
async fn test_branch_from_restores_version_data() {
    let db = setup_test_db().await;
    let recipe_id = make_recipe(&db).await;

    // Add a fermentable to create v1
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

    let repo = RecipeVersionRepository::new(&db);
    let v1 = repo.create_or_reuse(&recipe_id).await.unwrap();

    // Now change the live recipe (delete the fermentable, add different one)
    let fermentables = FermentableRepository::new(&db).list(&recipe_id).await.unwrap();
    FermentableRepository::new(&db).delete(&fermentables[0].id).await.unwrap();
    FermentableRepository::new(&db)
        .create(
            &recipe_id,
            CreateFermentableAdditionInput {
                fermentable_id: None,
                name: "Munich".into(),
                type_: "grain".into(),
                yield_pct: 80.0,
                color_lovibond: 8.0,
                amount_kg: 2.0,
                add_after_boil: None,
            },
        )
        .await
        .unwrap();

    // Branch back to v1
    repo.branch_from(&recipe_id, &v1.id).await.unwrap();

    // Live recipe should now have Pale Malt again
    let live_fermentables = FermentableRepository::new(&db).list(&recipe_id).await.unwrap();
    assert_eq!(live_fermentables.len(), 1);
    assert_eq!(live_fermentables[0].name, "Pale Malt");

    // branch_parent_id should be set on the recipe
    let recipe_row = crate::entities::recipes::Entity::find_by_id(&recipe_id)
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(recipe_row.branch_parent_id.as_deref(), Some(v1.id.as_str()));
}
```

- [ ] **Step 2: Run the test to confirm it fails**

```bash
cd src-tauri && cargo test test_branch_from_restores_version_data -- --nocapture
```

Expected: FAIL — `branch_from` not found.

- [ ] **Step 3: Add required imports for deletion**

Add to imports at the top of `recipe_version.rs`:

```rust
use sea_orm::DeleteMany;
use crate::entities::{
    mash_steps, mashes, recipe_addition_fermentables, recipe_addition_hops,
    recipe_addition_miscs, recipe_addition_waters, recipe_addition_yeasts,
    recipe_water_adjustments,
};
```

Merge these with the existing imports block — do not duplicate.

- [ ] **Step 4: Implement `branch_from`**

Add to `impl<'a> RecipeVersionRepository<'a>`:

```rust
pub async fn branch_from(
    &self,
    recipe_id: &str,
    version_id: &str,
) -> Result<(), AppError> {
    let full = self.get_full(version_id).await?;

    // --- Delete all existing additions on the live recipe ---
    recipe_addition_fermentables::Entity::delete_many()
        .filter(recipe_addition_fermentables::Column::RecipeId.eq(recipe_id))
        .exec(self.db)
        .await?;
    recipe_addition_hops::Entity::delete_many()
        .filter(recipe_addition_hops::Column::RecipeId.eq(recipe_id))
        .exec(self.db)
        .await?;
    recipe_addition_yeasts::Entity::delete_many()
        .filter(recipe_addition_yeasts::Column::RecipeId.eq(recipe_id))
        .exec(self.db)
        .await?;
    recipe_addition_miscs::Entity::delete_many()
        .filter(recipe_addition_miscs::Column::RecipeId.eq(recipe_id))
        .exec(self.db)
        .await?;
    recipe_addition_waters::Entity::delete_many()
        .filter(recipe_addition_waters::Column::RecipeId.eq(recipe_id))
        .exec(self.db)
        .await?;
    recipe_water_adjustments::Entity::delete_many()
        .filter(recipe_water_adjustments::Column::RecipeId.eq(recipe_id))
        .exec(self.db)
        .await?;
    // Delete mash steps then mash
    if let Some(mash_row) = mashes::Entity::find()
        .filter(mashes::Column::RecipeId.eq(recipe_id))
        .one(self.db)
        .await?
    {
        mash_steps::Entity::delete_many()
            .filter(mash_steps::Column::MashId.eq(&mash_row.id))
            .exec(self.db)
            .await?;
        mashes::Entity::delete_by_id(&mash_row.id)
            .exec(self.db)
            .await?;
    }

    // --- Re-insert from version snapshot ---
    for f in &full.fermentables {
        recipe_addition_fermentables::ActiveModel {
            id: Set(new_id()),
            recipe_id: Set(recipe_id.to_string()),
            fermentable_id: Set(f.fermentable_id.clone()),
            name: Set(f.name.clone()),
            r#type: Set(f.type_.clone()),
            yield_pct: Set(f.yield_pct),
            color_lovibond: Set(f.color_lovibond),
            amount_kg: Set(f.amount_kg),
            add_after_boil: Set(Some(f.add_after_boil as i32)),
            addition_order: Set(f.addition_order as i32),
        }
        .insert(self.db)
        .await?;
    }

    for h in &full.hops {
        recipe_addition_hops::ActiveModel {
            id: Set(new_id()),
            recipe_id: Set(recipe_id.to_string()),
            hop_id: Set(h.hop_id.clone()),
            name: Set(h.name.clone()),
            alpha_pct: Set(h.alpha_pct),
            form: Set(h.form.clone()),
            amount_kg: Set(h.amount_kg),
            r#use: Set(h.use_.clone()),
            time_min: Set(h.time_min),
            addition_order: Set(h.addition_order as i32),
            hopstand_temp_c: Set(h.hopstand_temp_c),
        }
        .insert(self.db)
        .await?;
    }

    for y in &full.yeasts {
        recipe_addition_yeasts::ActiveModel {
            id: Set(new_id()),
            recipe_id: Set(recipe_id.to_string()),
            yeast_id: Set(y.yeast_id.clone()),
            name: Set(y.name.clone()),
            r#type: Set(y.type_.clone()),
            form: Set(y.form.clone()),
            laboratory: Set(y.laboratory.clone()),
            product_id: Set(y.product_id.clone()),
            attenuation_pct: Set(y.attenuation_pct),
            amount: Set(y.amount),
            amount_is_weight: Set(Some(y.amount_is_weight as i32)),
            add_to_secondary: Set(Some(y.add_to_secondary as i32)),
            times_cultured: Set(Some(y.times_cultured as i32)),
        }
        .insert(self.db)
        .await?;
    }

    for m in &full.miscs {
        recipe_addition_miscs::ActiveModel {
            id: Set(new_id()),
            recipe_id: Set(recipe_id.to_string()),
            misc_id: Set(m.misc_id.clone()),
            name: Set(m.name.clone()),
            r#type: Set(m.type_.clone()),
            r#use: Set(m.use_.clone()),
            amount: Set(m.amount),
            amount_is_weight: Set(Some(m.amount_is_weight as i32)),
            time_min: Set(m.time_min),
            addition_order: Set(m.addition_order as i32),
        }
        .insert(self.db)
        .await?;
    }

    for w in &full.waters {
        recipe_addition_waters::ActiveModel {
            id: Set(new_id()),
            recipe_id: Set(recipe_id.to_string()),
            water_id: Set(w.water_id.clone()),
            name: Set(w.name.clone()),
            amount_l: Set(w.amount_l),
        }
        .insert(self.db)
        .await?;
    }

    for a in &full.water_adjustments {
        recipe_water_adjustments::ActiveModel {
            id: Set(new_id()),
            recipe_id: Set(recipe_id.to_string()),
            addition: Set(a.addition.to_string()),
            target: Set(a.target.to_string()),
            amount: Set(a.amount),
        }
        .insert(self.db)
        .await?;
    }

    if let Some(mash) = &full.mash {
        let mash_id = new_id();
        mashes::ActiveModel {
            id: Set(mash_id.clone()),
            recipe_id: Set(recipe_id.to_string()),
            name: Set(mash.name.clone()),
            grain_temp_c: Set(mash.grain_temp_c),
            tun_temp_c: Set(mash.tun_temp_c),
            sparge_temp_c: Set(mash.sparge_temp_c),
            ph: Set(mash.ph),
            notes: Set(mash.notes.clone()),
            ratio_l_per_kg: Set(mash.ratio_l_per_kg),
            tun_weight_kg: Set(mash.tun_weight_kg),
            tun_specific_heat: Set(mash.tun_specific_heat),
            equip_adjust: Set(Some(mash.equip_adjust as i32)),
        }
        .insert(self.db)
        .await?;

        for (i, step) in mash.steps.iter().enumerate() {
            mash_steps::ActiveModel {
                id: Set(new_id()),
                mash_id: Set(mash_id.clone()),
                name: Set(step.name.clone()),
                r#type: Set(step.type_.clone()),
                infuse_amount_l: Set(step.infuse_amount_l),
                step_temp_c: Set(step.step_temp_c),
                step_time_min: Set(step.step_time_min as i32),
                ramp_time_min: Set(step.ramp_time_min.map(|v| v as i32)),
                end_temp_c: Set(step.end_temp_c),
                step_order: Set(i as i32),
            }
            .insert(self.db)
            .await?;
        }
    }

    // --- Update recipe scalars ---
    let now = now_secs() as i32;
    recipes::ActiveModel {
        id: Set(recipe_id.to_string()),
        r#type: Set(full.type_.clone()),
        brewer: Set(full.brewer.clone()),
        asst_brewer: Set(full.asst_brewer.clone()),
        batch_size_l: Set(full.batch_size_l),
        boil_size_l: Set(full.boil_size_l),
        boil_time_min: Set(full.boil_time_min),
        efficiency_pct: Set(full.efficiency_pct),
        style_id: Set(full.style_id.clone()),
        equipment_profile_id: Set(full.equipment_profile_id.clone()),
        notes: Set(full.notes.clone()),
        og: Set(full.og),
        fg: Set(full.fg),
        primary_age_days: Set(full.primary_age_days),
        primary_temp_c: Set(full.primary_temp_c),
        secondary_age_days: Set(full.secondary_age_days),
        secondary_temp_c: Set(full.secondary_temp_c),
        tertiary_age_days: Set(full.tertiary_age_days),
        tertiary_temp_c: Set(full.tertiary_temp_c),
        age_days: Set(full.age_days),
        age_temp_c: Set(full.age_temp_c),
        carbonation_vols: Set(full.carbonation_vols),
        forced_carbonation: Set(Some(full.forced_carbonation as i32)),
        priming_sugar_name: Set(full.priming_sugar_name.clone()),
        carbonation_temp_c: Set(full.carbonation_temp_c),
        priming_sugar_equiv: Set(full.priming_sugar_equiv),
        keg_priming_factor: Set(full.keg_priming_factor),
        mash_water_id: Set(full.mash_water_id.clone()),
        sparge_water_id: Set(full.sparge_water_id.clone()),
        branch_parent_id: Set(Some(version_id.to_string())),
        updated_at: Set(now),
        ..Default::default()
    }
    .update(self.db)
    .await?;

    Ok(())
}
```

- [ ] **Step 5: Run the test to confirm it passes**

```bash
cd src-tauri && cargo test test_branch_from_restores_version_data -- --nocapture
```

Expected: PASS

- [ ] **Step 6: Run all tests**

```bash
cd src-tauri && cargo test
```

Expected: all tests pass.

- [ ] **Step 7: Commit**

```bash
git add src-tauri/src/repositories/recipe_version.rs
git commit -m "feat(backend): add branch_from to RecipeVersionRepository"
```

---

## Task 7: Wire new Tauri commands

**Files:**
- Modify: `src-tauri/src/commands/batches.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Add three new commands to `batches.rs`**

Add to the end of `src-tauri/src/commands/batches.rs`:

```rust
#[tauri::command]
pub async fn get_recipe_version(
    state: State<'_, AppState>,
    id: String,
) -> Result<Recipe, AppError> {
    RecipeVersionRepository::new(&state.db).get_full(&id).await
}

#[tauri::command]
pub async fn save_recipe_version(
    state: State<'_, AppState>,
    input: SaveRecipeVersionInput,
) -> Result<RecipeVersionSummary, AppError> {
    RecipeVersionRepository::new(&state.db)
        .save_named(&input.recipe_id, &input.name)
        .await
}

#[tauri::command]
pub async fn branch_from_version(
    state: State<'_, AppState>,
    recipe_id: String,
    version_id: String,
) -> Result<(), AppError> {
    RecipeVersionRepository::new(&state.db)
        .branch_from(&recipe_id, &version_id)
        .await
}
```

Add the required model imports at the top of the file (update the existing `use crate::models::*;` — it already imports everything via glob, so no change needed there). Add `Recipe` and `SaveRecipeVersionInput` if not already covered by `use crate::models::*;`.

- [ ] **Step 2: Register the three commands in `lib.rs`**

In `src-tauri/src/lib.rs`, add to the `tauri::generate_handler![` list (after `commands::batches::list_recipe_versions,`):

```rust
            commands::batches::get_recipe_version,
            commands::batches::save_recipe_version,
            commands::batches::branch_from_version,
```

- [ ] **Step 3: Build to confirm it compiles**

```bash
cd src-tauri && cargo build 2>&1 | head -30
```

Expected: builds without errors.

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/commands/batches.rs src-tauri/src/lib.rs
git commit -m "feat(backend): register get_recipe_version, save_recipe_version, branch_from_version commands"
```

---

## Task 8: Frontend — api.ts wrappers

**Files:**
- Modify: `src/lib/api.ts`

- [ ] **Step 1: Add imports and wrappers**

Add to `src/lib/api.ts` (after the existing `listRecipeVersions` line):

```typescript
export type SaveRecipeVersionInput = components["schemas"]["SaveRecipeVersionInput"];

export const getRecipeVersion = (id: string) =>
  invoke<Recipe>("get_recipe_version", { id });

export const saveRecipeVersion = (input: SaveRecipeVersionInput) =>
  invoke<RecipeVersionSummary>("save_recipe_version", { input });

export const branchFromVersion = (recipeId: string, versionId: string) =>
  invoke<void>("branch_from_version", { recipeId, versionId });
```

- [ ] **Step 2: Run TypeScript check**

```bash
bun run check
```

Expected: no type errors.

- [ ] **Step 3: Commit**

```bash
git add src/lib/api.ts
git commit -m "feat(frontend): add getRecipeVersion, saveRecipeVersion, branchFromVersion api wrappers"
```

---

## Task 9: Frontend — VersionHistoryPanel component

**Files:**
- Create: `src/lib/components/VersionHistoryPanel.svelte`

This panel shows all versions for a recipe as a flat list, most recent first, with indentation for child versions. It emits events for viewing a version or branching from one.

- [ ] **Step 1: Create the component**

Create `src/lib/components/VersionHistoryPanel.svelte`:

```svelte
<script lang="ts">
  import type { RecipeVersionSummary } from "$lib/api";

  let {
    versions,
    viewingVersionId,
    onview,
    onbranch,
    onclose,
  }: {
    versions: RecipeVersionSummary[];
    viewingVersionId: string | null;
    onview: (version: RecipeVersionSummary) => void;
    onbranch: (version: RecipeVersionSummary) => void;
    onclose: () => void;
  } = $props();

  function formatDate(ts: number): string {
    return new Date(ts * 1000).toLocaleDateString(undefined, {
      month: "short",
      day: "numeric",
      year: "numeric",
    });
  }

  function indentLevel(version: RecipeVersionSummary): number {
    if (!version.parent_version_id) return 0;
    const parent = versions.find((v) => v.id === version.parent_version_id);
    if (!parent) return 1;
    return indentLevel(parent) + 1;
  }
</script>

<div
  class="flex flex-col h-full border-l overflow-hidden"
  style="background: var(--color-bg-surface); border-color: var(--color-border); min-width: 220px; max-width: 260px;"
>
  <div
    class="flex items-center justify-between px-3 py-2 border-b flex-shrink-0"
    style="border-color: var(--color-border);"
  >
    <span class="text-xs font-semibold" style="color: var(--color-text-secondary);">
      VERSION HISTORY
    </span>
    <button
      onclick={onclose}
      class="text-xs px-1"
      style="color: var(--color-text-muted);"
    >✕</button>
  </div>

  <div class="flex-1 overflow-y-auto">
    {#each versions as version}
      {@const indent = Math.min(indentLevel(version), 3)}
      <button
        onclick={() => onview(version)}
        class="w-full text-left px-3 py-2 border-b transition-colors"
        style="
          padding-left: {0.75 + indent * 0.75}rem;
          border-color: var(--color-border);
          background: {viewingVersionId === version.id
            ? 'var(--color-bg-elevated)'
            : 'transparent'};
        "
      >
        <div class="flex items-center gap-1.5">
          {#if indent > 0}
            <span style="color: var(--color-text-muted); font-size: 0.6rem;">↳</span>
          {/if}
          <span class="text-xs font-mono" style="color: var(--color-accent);">
            v{version.version_number}
          </span>
          {#if version.name}
            <span class="text-xs truncate" style="color: var(--color-text-primary);">
              {version.name}
            </span>
          {/if}
        </div>
        <div class="text-xs mt-0.5" style="color: var(--color-text-muted);">
          {formatDate(version.created_at)}
        </div>
        {#if viewingVersionId === version.id}
          <button
            onclick={(e) => { e.stopPropagation(); onbranch(version); }}
            class="mt-1 text-xs px-2 py-0.5 rounded"
            style="background: var(--color-accent); color: #fff;"
          >
            Branch from here
          </button>
        {/if}
      </button>
    {/each}
  </div>
</div>
```

- [ ] **Step 2: Run TypeScript check**

```bash
bun run check
```

Expected: no type errors.

- [ ] **Step 3: Commit**

```bash
git add src/lib/components/VersionHistoryPanel.svelte
git commit -m "feat(frontend): add VersionHistoryPanel component"
```

---

## Task 10: Frontend — Recipe page integration

**Files:**
- Modify: `src/routes/recipe/[id]/+page.svelte`

Add: a "History" toggle button in the header, version panel alongside the tab content, read-only mode when viewing a past version, a banner with "Branch from here", and a "Save Version" button with a popover.

- [ ] **Step 1: Update the recipe page**

Replace the entire contents of `src/routes/recipe/[id]/+page.svelte`:

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import type { PageData } from "./$types";
  import {
    getRecipe,
    getRecipeStats,
    updateRecipe,
    listRecipeVersions,
    getRecipeVersion,
    saveRecipeVersion,
    branchFromVersion,
  } from "$lib/api";
  import type { Recipe, RecipeStats, RecipeVersionSummary } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import RecipeList from "$lib/components/RecipeList.svelte";
  import StatsSidebar from "$lib/components/StatsSidebar.svelte";
  import BrewingIcon from "$lib/components/BrewingIcon.svelte";
  import OverviewTab from "$lib/components/tabs/OverviewTab.svelte";
  import IngredientsTab from "$lib/components/tabs/IngredientsTab.svelte";
  import MashTab from "$lib/components/tabs/MashTab.svelte";
  import WaterTab from "$lib/components/tabs/WaterTab.svelte";
  import FermentationTab from "$lib/components/tabs/FermentationTab.svelte";
  import NotesTab from "$lib/components/tabs/NotesTab.svelte";
  import BatchesTab from "$lib/components/tabs/BatchesTab.svelte";
  import VersionHistoryPanel from "$lib/components/VersionHistoryPanel.svelte";
  import type { BrewingIconName } from "$lib/icons";

  let { data }: { data: PageData } = $props();

  let recipe = $state<Recipe | null>(null);
  let stats = $state<RecipeStats | null>(null);
  let activeTab = $state<"overview" | "ingredients" | "mash" | "water" | "fermentation" | "notes" | "batches">("overview");
  let saving = $state(false);

  // Version history state
  let showVersionPanel = $state(false);
  let versions = $state<RecipeVersionSummary[]>([]);
  let viewingVersion = $state<RecipeVersionSummary | null>(null);
  let viewingRecipe = $state<Recipe | null>(null); // recipe data for the viewed version

  // Save Version popover state
  let showSavePopover = $state(false);
  let saveVersionName = $state("");
  let savingVersion = $state(false);

  const TABS: { key: "overview" | "ingredients" | "mash" | "water" | "fermentation" | "notes" | "batches"; label: string; icon: BrewingIconName }[] = [
    { key: "overview", label: "Overview", icon: "overview" },
    { key: "ingredients", label: "Ingredients", icon: "ingredients" },
    { key: "mash", label: "Mash", icon: "mash" },
    { key: "water", label: "Water", icon: "water" },
    { key: "fermentation", label: "Fermentation", icon: "fermentation" },
    { key: "batches", label: "Batches", icon: "batches" },
    { key: "notes", label: "Notes", icon: "notes" },
  ] as const;

  async function loadRecipeById(id: string) {
    recipe = await ipc(getRecipe(id)) ?? null;
    if (recipe) {
      stats = await ipc(getRecipeStats(recipe.id)) ?? null;
    } else {
      stats = null;
    }
  }

  async function loadVersions(id: string) {
    const result = await ipc(listRecipeVersions(id));
    if (result) {
      versions = result.sort((a, b) => b.created_at - a.created_at);
    }
  }

  onMount(async () => {
    await loadRecipeById(data.id);
    await loadVersions(data.id);
  });

  $effect(() => {
    if (data?.id) {
      (async () => {
        await loadRecipeById(data.id);
        await loadVersions(data.id);
      })();
    }
  });

  async function refreshStats() {
    if (!recipe) return;
    stats = await ipc(getRecipeStats(recipe.id)) ?? null;
  }

  async function refreshRecipe() {
    await loadRecipeById(data.id);
    await loadVersions(data.id);
    viewingVersion = null;
    viewingRecipe = null;
  }

  async function handleNameBlur(e: FocusEvent) {
    const target = e.currentTarget as HTMLInputElement;
    if (!recipe || target.value === recipe.name) return;
    saving = true;
    recipe = await ipc(updateRecipe(recipe.id, { name: target.value })) ?? recipe;
    saving = false;
  }

  async function handleViewVersion(version: RecipeVersionSummary) {
    if (viewingVersion?.id === version.id) {
      viewingVersion = null;
      viewingRecipe = null;
      return;
    }
    viewingVersion = version;
    viewingRecipe = await ipc(getRecipeVersion(version.id)) ?? null;
  }

  async function handleBranchFromVersion(version: RecipeVersionSummary) {
    if (!recipe) return;
    const confirmed = confirm(
      `This will replace your current recipe with v${version.version_number}'s data. Continue?`
    );
    if (!confirmed) return;
    await ipc(branchFromVersion(recipe.id, version.id));
    await refreshRecipe();
  }

  async function handleSaveVersion() {
    if (!recipe || !saveVersionName.trim()) return;
    savingVersion = true;
    await ipc(saveRecipeVersion({ recipe_id: recipe.id, name: saveVersionName.trim() }));
    savingVersion = false;
    saveVersionName = "";
    showSavePopover = false;
    await loadVersions(data.id);
  }

  // The recipe data to display in tabs (either live or viewed version)
  const displayRecipe = $derived(viewingRecipe ?? recipe);
</script>

<RecipeList selectedId={data.id} />

{#if recipe}
  <div class="flex flex-1 flex-col overflow-hidden">
    <!-- Header -->
    <header
      class="flex items-center px-4 py-2 border-b gap-3 flex-shrink-0"
      style="background: var(--color-bg-surface); border-color: var(--color-border);"
    >
      <button
        onclick={() => goto("/")}
        class="text-xs px-2 py-1 rounded"
        style="color: var(--color-text-secondary); background: var(--color-bg-elevated);"
      >
        ← Recipes
      </button>
      <input
        value={recipe.name}
        onblur={handleNameBlur}
        disabled={viewingVersion !== null}
        class="flex-1 text-base font-semibold bg-transparent outline-none"
        style="color: var(--color-text-primary);"
      />
      {#if saving}
        <span class="text-xs" style="color: var(--color-text-muted);">Saving…</span>
      {/if}

      <!-- Save Version button -->
      <div class="relative">
        <button
          onclick={() => { showSavePopover = !showSavePopover; }}
          class="text-xs px-2 py-1 rounded"
          style="color: var(--color-text-secondary); background: var(--color-bg-elevated);"
        >
          Save Version
        </button>
        {#if showSavePopover}
          <div
            class="absolute right-0 top-full mt-1 p-3 rounded shadow-lg z-10 flex flex-col gap-2"
            style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); min-width: 200px;"
          >
            <input
              type="text"
              bind:value={saveVersionName}
              placeholder="Version name…"
              class="px-2 py-1 rounded text-sm outline-none"
              style="background: var(--color-bg-surface); color: var(--color-text-primary); border: 1px solid var(--color-border);"
              onkeydown={(e) => { if (e.key === "Enter") handleSaveVersion(); }}
            />
            <button
              onclick={handleSaveVersion}
              disabled={savingVersion || !saveVersionName.trim()}
              class="px-3 py-1 rounded text-sm"
              style="background: var(--color-accent); color: #fff;"
            >
              {savingVersion ? "Saving…" : "Save"}
            </button>
          </div>
        {/if}
      </div>

      <!-- Version history toggle -->
      <button
        onclick={() => { showVersionPanel = !showVersionPanel; }}
        class="text-xs px-2 py-1 rounded"
        style="
          color: {showVersionPanel ? '#fff' : 'var(--color-text-secondary)'};
          background: {showVersionPanel ? 'var(--color-accent)' : 'var(--color-bg-elevated)'};
        "
      >
        History ({versions.length})
      </button>
    </header>

    <!-- Read-only version banner -->
    {#if viewingVersion}
      <div
        class="flex items-center gap-3 px-4 py-2 text-sm flex-shrink-0"
        style="background: var(--color-bg-elevated); border-bottom: 1px solid var(--color-border);"
      >
        <span style="color: var(--color-text-secondary);">
          Viewing v{viewingVersion.version_number}
          {viewingVersion.name ? `· ${viewingVersion.name}` : ""}
          · {new Date(viewingVersion.created_at * 1000).toLocaleDateString()}
        </span>
        <button
          onclick={() => handleBranchFromVersion(viewingVersion!)}
          class="px-3 py-1 rounded text-sm"
          style="background: var(--color-accent); color: #fff;"
        >
          Branch from here
        </button>
        <button
          onclick={() => { viewingVersion = null; viewingRecipe = null; }}
          class="text-xs"
          style="color: var(--color-text-muted);"
        >
          Back to current
        </button>
      </div>
    {/if}

    <!-- Tab bar -->
    <nav
      class="flex border-b flex-shrink-0"
      style="background: var(--color-bg-surface); border-color: var(--color-border);"
    >
      {#each TABS as tab}
        <button
          onclick={() => activeTab = tab.key}
          class="px-4 py-2 text-sm border-b-2 transition-colors inline-flex items-center gap-2"
          style={activeTab === tab.key
            ? "border-color: var(--color-accent); color: var(--color-text-primary);"
            : "border-color: transparent; color: var(--color-text-secondary);"}
        >
          <BrewingIcon name={tab.icon} />
          {tab.label}
        </button>
      {/each}
    </nav>

    <!-- Tab content + stats sidebar + version panel -->
    <div class="flex flex-1 overflow-hidden">
      <div class="flex-1 overflow-y-auto p-4" style={viewingVersion ? "pointer-events: none; opacity: 0.85;" : ""}>
        {#if displayRecipe}
          {#if activeTab === "overview"}
            <OverviewTab recipe={displayRecipe} onchange={refreshRecipe} />
          {:else if activeTab === "ingredients"}
            <IngredientsTab recipe={displayRecipe} onchange={refreshRecipe} />
          {:else if activeTab === "mash"}
            <MashTab recipe={displayRecipe} {stats} onchange={refreshRecipe} />
          {:else if activeTab === "water"}
            <WaterTab recipe={displayRecipe} onchange={refreshRecipe} />
          {:else if activeTab === "fermentation"}
            <FermentationTab recipe={displayRecipe} onchange={refreshRecipe} />
          {:else if activeTab === "notes"}
            <NotesTab recipe={displayRecipe} onchange={refreshRecipe} />
          {:else if activeTab === "batches"}
            <BatchesTab recipeId={recipe.id} />
          {/if}
        {/if}
      </div>
      <StatsSidebar {stats} />
      {#if showVersionPanel}
        <VersionHistoryPanel
          {versions}
          viewingVersionId={viewingVersion?.id ?? null}
          onview={handleViewVersion}
          onbranch={handleBranchFromVersion}
          onclose={() => showVersionPanel = false}
        />
      {/if}
    </div>
  </div>
{:else}
  <div class="flex-1 flex items-center justify-center">
    <p class="text-sm" style="color: var(--color-text-muted);">Loading…</p>
  </div>
{/if}
```

- [ ] **Step 2: Run TypeScript check**

```bash
bun run check
```

Expected: no type errors.

- [ ] **Step 3: Commit**

```bash
git add src/routes/recipe/[id]/+page.svelte
git commit -m "feat(frontend): add version history panel, read-only mode, save version button to recipe page"
```

---

## Task 11: Frontend — BatchOverviewTab version link

**Files:**
- Modify: `src/lib/components/batch/BatchOverviewTab.svelte`

Add a small "Recipe v{n}" link in the batch overview that references the version used for this batch.

- [ ] **Step 1: Update BatchOverviewTab**

In `src/lib/components/batch/BatchOverviewTab.svelte`, update the script block to import `listRecipeVersions` and look up the version number:

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import type { Batch, UpdateBatchInput, RecipeVersionSummary } from "$lib/api";
  import { listRecipeVersions } from "$lib/api";
  import { ipc } from "$lib/stores/error";

  let { batch, onUpdate }: { batch: Batch; onUpdate: (input: UpdateBatchInput) => void } = $props();

  let batchVersion = $state<RecipeVersionSummary | null>(null);

  onMount(async () => {
    const versions = await ipc(listRecipeVersions(batch.recipe_id));
    if (versions) {
      batchVersion = versions.find((v) => v.id === batch.recipe_version_id) ?? null;
    }
  });

  const STATUSES = ["planned", "brewing", "fermenting", "packaged", "complete"] as const;

  function toDateInput(ts: number | null | undefined): string {
    if (!ts) return "";
    return new Date(ts * 1000).toISOString().slice(0, 10);
  }

  function fromDateInput(val: string): number | null {
    if (!val) return null;
    return Math.floor(new Date(val).getTime() / 1000);
  }
</script>
```

Then add the version link right before the `<!-- Status -->` section in the template:

```svelte
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
```

- [ ] **Step 2: Run TypeScript check**

```bash
bun run check
```

Expected: no type errors.

- [ ] **Step 3: Run all frontend tests**

```bash
bun run test
```

Expected: all tests pass.

- [ ] **Step 4: Commit**

```bash
git add src/lib/components/batch/BatchOverviewTab.svelte
git commit -m "feat(frontend): show recipe version link in batch overview"
```

---

## Self-Review Checklist

After completing all tasks, run the full test suite one final time:

```bash
just test
```

Expected: all Rust and frontend tests pass.

Verify spec coverage:
- [x] Migration adds `parent_version_id` and `branch_parent_id` — Task 1
- [x] `get_recipe_version` returns full recipe-shaped data from snapshot — Task 3, 7
- [x] `save_recipe_version` always creates a new named snapshot — Task 4, 7
- [x] `branch_from_version` restores version data and sets branch_parent_id — Task 6, 7
- [x] `create_or_reuse` respects branch_parent_id — Task 5
- [x] OpenAPI + codegen updated — Task 2
- [x] Version history panel — Task 9
- [x] Recipe page: panel toggle, read-only mode, banner, Save Version button — Task 10
- [x] BatchOverviewTab version link — Task 11
