# Misc Ingredients in Recipes — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Wire the existing Misc ingredient backend into the recipe UI so users can add spices, finings, herbs, and other miscellaneous ingredients to recipes.

**Architecture:** Add a `unit` column to `recipe_addition_miscs` and `recipe_version_miscs` via migration, update OpenAPI schemas + regenerate types, extend `IngredientPicker` (desktop + mobile) with a `misc` type, and create a `MiscTable` component following the existing `HopsTable` pattern. `amount_is_weight` is derived from `unit` in the repository — the frontend only sends `unit`.

**Tech Stack:** SvelteKit 5 (Svelte 5 runes), Tauri, Rust/SeaORM, SQLite, OpenAPI (Redocly), `just` task runner

---

## File Map

**Create:**
- `src-tauri/migrations/013_misc_unit.sql`
- `src/lib/components/ingredients/MiscTable.svelte`

**Modify:**
- `docs/openapi/components/schemas/RecipeAdditionMisc.yaml`
- `docs/openapi/components/schemas/CreateMiscAdditionInput.yaml`
- `docs/openapi/components/schemas/UpdateMiscAdditionInput.yaml`
- `src-tauri/src/models.rs`
- `src-tauri/src/repositories/misc.rs`
- `src-tauri/src/repositories/recipe_version.rs`
- `src/lib/icons.ts`
- `src/lib/desktop/IngredientPicker.svelte`
- `src/lib/mobile/IngredientPicker.svelte`
- `src/lib/components/tabs/IngredientsTab.svelte`

**Regenerated — never edit directly:**
- `src/lib/api.gen.ts` (via `just gen`)
- `src-tauri/src/models.gen.rs` (via `just gen`)
- `src-tauri/src/entities/recipe_addition_miscs.rs` (via `just gen-entities`)
- `src-tauri/src/entities/recipe_version_miscs.rs` (via `just gen-entities`)

---

## Task 1: DB migration — add `unit` column

**Files:**
- Create: `src-tauri/migrations/013_misc_unit.sql`

- [ ] **Step 1: Create the migration file**

```sql
-- src-tauri/migrations/013_misc_unit.sql
ALTER TABLE recipe_addition_miscs ADD COLUMN unit TEXT NOT NULL DEFAULT 'g';
ALTER TABLE recipe_version_miscs ADD COLUMN unit TEXT NOT NULL DEFAULT 'g';
```

- [ ] **Step 2: Apply the migration and regenerate entities**

Run from the project root:
```bash
just gen-entities
```

`gen-entities` runs `just migrate` first, then regenerates SeaORM entity files. Expected output ends with "Migrations applied" and "Generating entities".

- [ ] **Step 3: Verify entity files were updated**

Check that `src-tauri/src/entities/recipe_addition_miscs.rs` now contains `pub unit: String` in its `Model` struct. Same for `src-tauri/src/entities/recipe_version_miscs.rs`.

- [ ] **Step 4: Commit**

```bash
git add src-tauri/migrations/013_misc_unit.sql \
        src-tauri/src/entities/recipe_addition_miscs.rs \
        src-tauri/src/entities/recipe_version_miscs.rs
git commit -m "feat: add unit column to misc addition tables"
```

---

## Task 2: Update OpenAPI schemas and regenerate types

**Files:**
- Modify: `docs/openapi/components/schemas/RecipeAdditionMisc.yaml`
- Modify: `docs/openapi/components/schemas/CreateMiscAdditionInput.yaml`
- Modify: `docs/openapi/components/schemas/UpdateMiscAdditionInput.yaml`

- [ ] **Step 1: Add `unit` to `RecipeAdditionMisc.yaml`**

Add `unit` to both `required` and `properties`:

```yaml
type: object
required:
  - id
  - recipe_id
  - name
  - type_
  - use_
  - amount
  - amount_is_weight
  - time_min
  - addition_order
  - unit
properties:
  id:
    type: string
  recipe_id:
    type: string
  misc_id:
    type:
      - string
      - "null"
  name:
    type: string
  type_:
    type: string
  use_:
    type: string
  amount:
    type: number
  amount_is_weight:
    type: boolean
  time_min:
    type: number
  addition_order:
    type: integer
  unit:
    type: string
    description: "Display unit: g, oz, tsp, tbsp, or mL"
```

- [ ] **Step 2: Add `unit` to `CreateMiscAdditionInput.yaml`**

Add `unit` to `required` and `properties`:

```yaml
type: object
required:
  - name
  - type_
  - use_
  - amount
  - time_min
  - unit
properties:
  misc_id:
    type: string
  name:
    type: string
  type_:
    type: string
  use_:
    type: string
  amount:
    type: number
  amount_is_weight:
    type: boolean
  time_min:
    type: number
  unit:
    type: string
    description: "Display unit: g, oz, tsp, tbsp, or mL"
```

- [ ] **Step 3: Add `unit` to `UpdateMiscAdditionInput.yaml`**

```yaml
type: object
properties:
  amount:
    type: number
  amount_is_weight:
    type: boolean
  use_:
    type: string
  time_min:
    type: number
  addition_order:
    type: integer
  unit:
    type: string
    description: "Display unit: g, oz, tsp, tbsp, or mL"
```

- [ ] **Step 4: Regenerate TypeScript and Rust types**

```bash
just gen
```

Expected: no errors. `src/lib/api.gen.ts` and `src-tauri/src/models.gen.rs` are updated.

- [ ] **Step 5: Verify the generated output**

Check that `api.gen.ts` now has `unit: string` in the `RecipeAdditionMisc` schema. Check that `models.gen.rs` has `pub unit: String` in `RecipeAdditionMisc` and `pub unit: String` in `CreateMiscAdditionInput`.

- [ ] **Step 6: Commit**

```bash
git add docs/openapi/components/schemas/RecipeAdditionMisc.yaml \
        docs/openapi/components/schemas/CreateMiscAdditionInput.yaml \
        docs/openapi/components/schemas/UpdateMiscAdditionInput.yaml \
        src/lib/api.gen.ts \
        src-tauri/src/models.gen.rs
git commit -m "feat: add unit field to misc addition OpenAPI schemas"
```

---

## Task 3: Update Rust — models and misc repository

**Files:**
- Modify: `src-tauri/src/models.rs`
- Modify: `src-tauri/src/repositories/misc.rs`

- [ ] **Step 1: Update `TryFrom` for `RecipeAdditionMisc` in `models.rs`**

Find the `impl TryFrom<entities::recipe_addition_miscs::Model> for RecipeAdditionMisc` block (around line 262) and add `unit`:

```rust
impl TryFrom<entities::recipe_addition_miscs::Model> for RecipeAdditionMisc {
    type Error = AppError;
    fn try_from(m: entities::recipe_addition_miscs::Model) -> Result<Self, AppError> {
        Ok(Self {
            id: m.id,
            recipe_id: m.recipe_id,
            misc_id: m.misc_id,
            name: m.name,
            type_: m.r#type,
            use_: m.r#use,
            amount: m.amount,
            amount_is_weight: m.amount_is_weight.unwrap_or(0) != 0,
            time_min: m.time_min,
            addition_order: m.addition_order as i64,
            unit: m.unit,
        })
    }
}
```

- [ ] **Step 2: Write a failing test for the `unit` field**

In `src-tauri/src/repositories/misc.rs`, inside the `#[cfg(test)]` module, add this test and update the existing `input()` helper:

```rust
fn input() -> CreateMiscAdditionInput {
    CreateMiscAdditionInput {
        misc_id: None,
        name: "Irish Moss".into(),
        type_: "fining".into(),
        use_: "Boil".into(),
        amount: 1.0,
        unit: "g".into(),
        time_min: 15.0,
        amount_is_weight: None,
    }
}

#[tokio::test]
async fn test_unit_roundtrips() {
    let db = setup_test_db().await;
    let recipe_id = make_recipe(&db).await;
    let repo = MiscRepository::new(&db);
    let created = repo
        .create(
            &recipe_id,
            CreateMiscAdditionInput {
                misc_id: None,
                name: "Coriander".into(),
                type_: "Spice".into(),
                use_: "Boil".into(),
                amount: 2.0,
                unit: "tsp".into(),
                time_min: 5.0,
                amount_is_weight: None,
            },
        )
        .await
        .unwrap();
    assert_eq!(created.unit, "tsp");
    assert!(!created.amount_is_weight); // tsp is volume
}
```

- [ ] **Step 3: Run tests to verify they fail**

```bash
cd src-tauri && cargo test repositories::misc::tests 2>&1 | tail -20
```

Expected: compile error because `CreateMiscAdditionInput` doesn't have `unit` yet in the impl (or the `ActiveModel` is missing `unit`).

- [ ] **Step 4: Update `MiscRepository::create` to set `unit` and derive `amount_is_weight`**

Replace the `create` method body in `src-tauri/src/repositories/misc.rs`:

```rust
pub async fn create(
    &self,
    recipe_id: &str,
    input: CreateMiscAdditionInput,
) -> Result<RecipeAdditionMisc, AppError> {
    let order = recipe_addition_miscs::Entity::find()
        .filter(recipe_addition_miscs::Column::RecipeId.eq(recipe_id))
        .count(self.db)
        .await? as i32;

    let amount_is_weight = if ["g", "oz"].contains(&input.unit.as_str()) {
        1i32
    } else {
        0i32
    };

    let id = new_id();
    recipe_addition_miscs::ActiveModel {
        id: Set(id.clone()),
        recipe_id: Set(recipe_id.to_string()),
        misc_id: Set(input.misc_id),
        name: Set(input.name),
        r#type: Set(input.type_),
        r#use: Set(input.use_),
        amount: Set(input.amount),
        amount_is_weight: Set(Some(amount_is_weight)),
        unit: Set(input.unit),
        time_min: Set(input.time_min),
        addition_order: Set(order),
    }
    .insert(self.db)
    .await?;

    recipe_addition_miscs::Entity::find_by_id(&id)
        .one(self.db)
        .await?
        .ok_or(AppError::NotFound)
        .and_then(RecipeAdditionMisc::try_from)
}
```

- [ ] **Step 5: Update `MiscRepository::update` to handle `unit`**

In the `update` method, add `unit` handling after the existing `if let Some(v) = input.addition_order` block:

```rust
if let Some(v) = input.unit {
    let is_weight = if ["g", "oz"].contains(&v.as_str()) {
        1i32
    } else {
        0i32
    };
    active.unit = Set(v);
    active.amount_is_weight = Set(Some(is_weight));
}
```

- [ ] **Step 6: Run tests to verify they pass**

```bash
cd src-tauri && cargo test repositories::misc::tests 2>&1 | tail -20
```

Expected: all 4 tests pass (`test_create_and_list`, `test_update`, `test_delete`, `test_unit_roundtrips`).

- [ ] **Step 7: Commit**

```bash
git add src-tauri/src/models.rs src-tauri/src/repositories/misc.rs
git commit -m "feat: wire unit field into misc repository and models"
```

---

## Task 4: Update recipe version snapshot and restore

**Files:**
- Modify: `src-tauri/src/repositories/recipe_version.rs`

- [ ] **Step 1: Update the misc snapshot block**

Find the "// Snapshot miscs" comment (around line 421). Add `unit` to the `ActiveModel`:

```rust
// Snapshot miscs
for m in &recipe.miscs {
    recipe_version_miscs::ActiveModel {
        id: Set(new_id()),
        recipe_version_id: Set(version_id.clone()),
        misc_id: Set(m.misc_id.clone()),
        name: Set(m.name.clone()),
        r#type: Set(m.type_.clone()),
        r#use: Set(m.use_.clone()),
        amount: Set(m.amount),
        amount_is_weight: Set(Some(m.amount_is_weight as i32)),
        unit: Set(m.unit.clone()),
        time_min: Set(m.time_min),
        addition_order: Set(m.addition_order as i32),
    }
    .insert(self.db)
    .await?;
}
```

- [ ] **Step 2: Update the misc restore block**

Find the misc restore block (around line 881 — the `.map(|m| RecipeAdditionMisc { ... })` call on `recipe_version_miscs`). Add `unit`:

```rust
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
        unit: m.unit,
        time_min: m.time_min,
        addition_order: m.addition_order as i64,
    })
    .collect();
```

- [ ] **Step 3: Run all Rust tests**

```bash
just test-rust
```

Expected: all tests pass.

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/repositories/recipe_version.rs
git commit -m "feat: include unit in recipe version misc snapshot and restore"
```

---

## Task 5: Add misc icon

**Files:**
- Modify: `src/lib/icons.ts`

- [ ] **Step 1: Add `misc` to `BrewingIconName` and `ICONS`**

In `src/lib/icons.ts`, add `"misc"` to the union type:

```ts
export type BrewingIconName =
  | "fermentable"
  | "hop"
  | "yeast"
  | "misc"
  | "overview"
  | "ingredients"
  | "mash"
  | "water"
  | "fermentation"
  | "notes"
  | "batches"
  | "recipes"
  | "tools"
  | "equipment"
  | "library"
  | "settings";
```

Add the `misc` SVG to the `ICONS` record (place it after `yeast`):

```ts
misc: `
  <path d="M12 21C12 21 11 16 12 12C13 8 16 5 16 5C16 5 20 8 19 13C18 17 15 19 12 21Z" fill="#10b981"/>
  <path d="M12 21C12 21 13 16 12 12C11 8 8 5 8 5C8 5 4 8 5 13C6 17 9 19 12 21Z" fill="#059669"/>
  <rect x="11.25" y="12" width="1.5" height="9" rx="0.75" fill="#065f46"/>
`,
```

- [ ] **Step 2: Commit**

```bash
git add src/lib/icons.ts
git commit -m "feat: add misc brewing icon"
```

---

## Task 6: Create MiscTable component

**Files:**
- Create: `src/lib/components/ingredients/MiscTable.svelte`

- [ ] **Step 1: Create the component**

```svelte
<!-- src/lib/components/ingredients/MiscTable.svelte -->
<script lang="ts">
  import type { Recipe } from "$lib/api";
  import { createRecipeMisc, deleteRecipeMisc } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import BrewingIcon from "$lib/components/BrewingIcon.svelte";
  import IngredientPicker, { type AddPayload } from "$platform/IngredientPicker.svelte";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  let adding = $state(false);

  async function handlePickerAdd(payload: AddPayload) {
    if (payload.type !== "misc") return;
    const result = await ipc(
      createRecipeMisc(recipe.id, {
        misc_id: payload.item.id,
        name: payload.item.name,
        type_: payload.item.type_,
        use_: payload.use_,
        amount: payload.amount,
        unit: payload.unit,
        time_min: payload.time_min,
      })
    );
    if (result === undefined) return;
    adding = false;
    onchange();
  }

  async function handleDelete(id: string) {
    await ipc(deleteRecipeMisc(id));
    onchange();
  }
</script>

<div class="flex flex-col gap-2">
  <div class="flex items-center justify-between">
    <h3 class="text-sm font-semibold flex items-center gap-2" style="color: var(--color-text-primary);">
      <BrewingIcon name="misc" />
      Misc
    </h3>
    <button
      onclick={() => (adding = true)}
      class="text-xs px-2 py-1 rounded"
      style="background: var(--color-accent); color: #fff;"
    >+ Add</button>
  </div>

  <IngredientPicker
    type="misc"
    open={adding}
    onclose={() => (adding = false)}
    onadd={handlePickerAdd}
  />

  {#if recipe.miscs.length > 0}
    <table class="w-full text-sm">
      <thead>
        <tr style="color: var(--color-text-secondary);">
          <th class="text-left py-1 font-medium text-sm">Name</th>
          <th class="text-left py-1 font-medium text-sm">Type</th>
          <th class="text-right py-1 font-medium text-sm">Amount</th>
          <th class="text-right py-1 font-medium text-sm">Use</th>
          <th class="text-right py-1 font-medium text-sm">Time</th>
          <th class="w-6"></th>
        </tr>
      </thead>
      <tbody>
        {#each recipe.miscs as m (m.id)}
          <tr class="border-t" style="border-color: var(--color-border);">
            <td class="py-1.5" style="color: var(--color-text-primary);">{m.name}</td>
            <td class="py-1.5 text-xs" style="color: var(--color-text-secondary);">{m.type_}</td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);"
              >{m.amount} {m.unit}</td
            >
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">{m.use_}</td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);"
              >{m.time_min}min</td
            >
            <td class="pl-1">
              <button
                onclick={() => handleDelete(m.id)}
                class="text-xs opacity-40 hover:opacity-100"
                style="color: var(--color-text-secondary);">×</button
              >
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>
```

- [ ] **Step 2: Run the TypeScript check**

```bash
just check-ts
```

Expected: no errors. (The misc picker type isn't wired yet — that comes next. If there are type errors about `'misc'` not being in the type union, that's expected and will be fixed in Task 7.)

- [ ] **Step 3: Commit**

```bash
git add src/lib/components/ingredients/MiscTable.svelte
git commit -m "feat: add MiscTable component"
```

---

## Task 7: Extend desktop IngredientPicker with misc type

**Files:**
- Modify: `src/lib/desktop/IngredientPicker.svelte`

The desktop picker is ~600 lines. Each change below is a targeted edit. Read the file first, then apply each step.

- [ ] **Step 1: Update imports and `AddPayload` type**

Change the import block at the top of the `<script>` section. Add `Misc` to the type imports and `listMiscLibrary` to the API imports:

```ts
import type { Hop, Fermentable, Yeast, Misc } from '$lib/api';
import type {
  CreateHopInput, CreateFermentableInput, CreateYeastInput,
} from '$lib/api';
import {
  listHopLibrary, listFermentableLibrary, listYeastLibrary, listMiscLibrary,
  createHop, createFermentable, createYeast,
} from '$lib/api';
```

Change the `AddPayload` export type to add the misc variant:

```ts
export type AddPayload =
  | { type: 'hop'; item: Hop; form: string; amount_kg: number; use_: string; time_min: number; hopstand_temp_c: number | null }
  | { type: 'fermentable'; item: Fermentable; amount_kg: number }
  | { type: 'yeast'; item: Yeast; amount: number }
  | { type: 'misc'; item: Misc; amount: number; unit: string; use_: string; time_min: number };
```

- [ ] **Step 2: Update the `type` prop and state**

Change the `type` prop union:

```ts
let {
  type,
  open,
  onclose,
  onadd,
}: {
  type: 'hop' | 'fermentable' | 'yeast' | 'misc';
  open: boolean;
  onclose: () => void;
  onadd: (payload: AddPayload) => void;
} = $props();
```

Change the `library` and `selected` state types:

```ts
let library = $state<(Hop | Fermentable | Yeast | Misc)[]>([]);
let selected = $state<Hop | Fermentable | Yeast | Misc | null>(null);
```

Add misc state variables and constants (place alongside the existing `HOP_USES`/`HOP_FORMS` constants):

```ts
const MISC_USES = ['Boil', 'Mash', 'Primary', 'Secondary', 'Bottling'] as const;
const MISC_UNITS = ['g', 'oz', 'tsp', 'tbsp', 'mL'] as const;

let miscUse = $state('Boil');
let miscUnit = $state('g');
let miscTime = $state(15);
```

- [ ] **Step 3: Update `loadLibrary`, `reloadLibrary`, and the amount reset `$effect`**

In `loadLibrary`, change the final `else` to include `yeast` explicitly and add `misc`:

```ts
async function loadLibrary() {
  if (libraryLoaded) return;
  if (type === 'hop') library = (await ipc(listHopLibrary())) ?? [];
  else if (type === 'fermentable') library = (await ipc(listFermentableLibrary())) ?? [];
  else if (type === 'yeast') library = (await ipc(listYeastLibrary())) ?? [];
  else library = (await ipc(listMiscLibrary())) ?? [];
  libraryLoaded = true;
}
```

Apply the same change to `reloadLibrary`.

In the `$effect` that resets `amount` when `selected` changes, add a misc case:

```ts
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
  else if (type === 'yeast') { amount = 1; }
  else { amount = 1; miscUse = 'Boil'; miscUnit = 'g'; miscTime = 15; }
});
```

- [ ] **Step 4: Update `handleAdd`, `rowSubtext`, `headerIcon`, `headerTitle`**

In `handleAdd`, change the final `else` (yeast) to explicit `else if` and add misc:

```ts
function handleAdd() {
  if (!selected || amount <= 0) return;
  if (type === 'hop') {
    onadd({ type: 'hop', item: selected as Hop, form: hopForm, amount_kg: amount, use_, time_min: time, hopstand_temp_c: use_ === 'hopstand' ? hopstand_temp_c : null });
  } else if (type === 'fermentable') {
    onadd({ type: 'fermentable', item: selected as Fermentable, amount_kg: amount });
  } else if (type === 'yeast') {
    onadd({ type: 'yeast', item: selected as Yeast, amount });
  } else {
    onadd({ type: 'misc', item: selected as Misc, amount, unit: miscUnit, use_: miscUse, time_min: miscTime });
  }
}
```

Update `rowSubtext` to accept `Misc` and add the misc case:

```ts
function rowSubtext(item: Hop | Fermentable | Yeast | Misc): string {
  if (type === 'hop') return `${(item as Hop).alpha_pct}% AA`;
  if (type === 'fermentable') {
    const f = item as Fermentable;
    return `${f.yield_pct.toFixed(0)}% · ${f.color_lovibond}°L`;
  }
  if (type === 'yeast') {
    const y = item as Yeast;
    return y.laboratory ?? y.form;
  }
  return (item as Misc).type_;
}
```

Update `headerIcon` and `headerTitle`:

```ts
const headerIcon = $derived<BrewingIconName>(
  type === "hop" ? "hop" : type === "fermentable" ? "fermentable" : type === "yeast" ? "yeast" : "misc"
);

const headerTitle = $derived(
  type === "hop" ? "Add Hop" : type === "fermentable" ? "Add Fermentable" : type === "yeast" ? "Add Yeast" : "Add Misc"
);
```

Update the search placeholder (in the `<input>` element in the left panel):

```svelte
placeholder="Search {type === 'hop' ? 'hops' : type === 'fermentable' ? 'fermentables' : type === 'yeast' ? 'yeasts' : 'misc'}…"
```

- [ ] **Step 5: Add misc detail panel and bottom bar to the right panel**

In the right panel, the current structure ends with `{:else}` for yeast and `{/if}`. Change the yeast block from `{:else}` to `{:else if type === 'yeast'}` and add the misc block before `{/if}`:

```svelte
{:else if type === 'yeast'}
  <!-- existing yeast content unchanged -->

{:else if type === 'misc'}
  {@const misc = selected as Misc}
  <div style="flex: 1; overflow-y: auto; padding: 16px; display: flex; flex-direction: column; gap: 10px;">
    <div>
      <h2 style="font-size: 18px; font-weight: 700; margin: 0;">{misc.name}</h2>
      <div style="display: flex; gap: 6px; margin-top: 6px; flex-wrap: wrap;">
        <span style="background: var(--color-bg-elevated); color: var(--color-text-secondary); padding: 2px 8px; border-radius: 99px; font-size: 11px;">{misc.type_}</span>
        {#if misc.source === 'user'}
          <span style="background: color-mix(in srgb, var(--color-accent) 15%, transparent); color: var(--color-accent); padding: 2px 8px; border-radius: 99px; font-size: 11px; border: 1px solid color-mix(in srgb, var(--color-accent) 40%, transparent);">custom</span>
        {/if}
      </div>
    </div>
    {#if misc.use_for}
      <p style="font-size: 12px; color: var(--color-text-secondary); line-height: 1.5; margin: 0;">{misc.use_for}</p>
    {/if}
    {#if misc.notes}
      <p style="font-size: 12px; color: var(--color-text-muted); line-height: 1.5; margin: 0;">{misc.notes}</p>
    {/if}
  </div>
  <div style="border-top: 1px solid var(--color-border); padding: 12px 16px; display: flex; gap: 10px; align-items: flex-end; background: var(--color-bg-surface); flex-shrink: 0;">
    <div>
      <div style="font-size: 11px; color: var(--color-text-secondary); margin-bottom: 4px;">Amount</div>
      <input type="number" inputmode="decimal" step="0.1" bind:value={amount} min="0.001"
        style="width: 70px; background: var(--color-bg-elevated); border: 1px solid var(--color-border); border-radius: 5px; padding: 5px 8px; color: var(--color-text-primary); font-size: 13px;" />
    </div>
    <div>
      <div style="font-size: 11px; color: var(--color-text-secondary); margin-bottom: 4px;">Unit</div>
      <select bind:value={miscUnit} style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); border-radius: 5px; padding: 5px 8px; color: var(--color-text-primary); font-size: 13px;">
        {#each MISC_UNITS as u}<option value={u}>{u}</option>{/each}
      </select>
    </div>
    <div>
      <div style="font-size: 11px; color: var(--color-text-secondary); margin-bottom: 4px;">Use</div>
      <select bind:value={miscUse} style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); border-radius: 5px; padding: 5px 8px; color: var(--color-text-primary); font-size: 13px;">
        {#each MISC_USES as u}<option value={u}>{u}</option>{/each}
      </select>
    </div>
    <div>
      <div style="font-size: 11px; color: var(--color-text-secondary); margin-bottom: 4px;">Time (min)</div>
      <input type="number" inputmode="decimal" step="1" bind:value={miscTime} min="0"
        style="width: 65px; background: var(--color-bg-elevated); border: 1px solid var(--color-border); border-radius: 5px; padding: 5px 8px; color: var(--color-text-primary); font-size: 13px;" />
    </div>
    <button onclick={handleAdd} disabled={!canAdd}
      style="margin-left: auto; background: {canAdd ? 'var(--color-accent)' : 'var(--color-bg-elevated)'}; color: {canAdd ? '#fff' : 'var(--color-text-muted)'}; border: none; border-radius: 6px; padding: 8px 18px; font-size: 13px; font-weight: 600; cursor: {canAdd ? 'pointer' : 'default'};">
      Add to Recipe
    </button>
  </div>
{/if}
```

- [ ] **Step 6: Verify types**

```bash
just check-ts
```

Expected: no errors.

- [ ] **Step 7: Commit**

```bash
git add src/lib/desktop/IngredientPicker.svelte
git commit -m "feat: add misc type to desktop IngredientPicker"
```

---

## Task 8: Extend mobile IngredientPicker with misc type

**Files:**
- Modify: `src/lib/mobile/IngredientPicker.svelte`

The mobile picker mirrors the desktop one but uses a different layout (scrollable sheet instead of a two-panel dialog). Apply the same logical changes as Task 7, adapted to the mobile file's structure.

- [ ] **Step 1: Update imports, `AddPayload`, prop type, state, and constants**

```ts
import type { Hop, Fermentable, Yeast, Misc } from '$lib/api';
import type {
  CreateHopInput, CreateFermentableInput, CreateYeastInput,
} from '$lib/api';
import {
  listHopLibrary, listFermentableLibrary, listYeastLibrary, listMiscLibrary,
  createHop, createFermentable, createYeast,
} from '$lib/api';

export type AddPayload =
  | { type: 'hop'; item: Hop; form: string; amount_kg: number; use_: string; time_min: number; hopstand_temp_c: number | null }
  | { type: 'fermentable'; item: Fermentable; amount_kg: number }
  | { type: 'yeast'; item: Yeast; amount: number }
  | { type: 'misc'; item: Misc; amount: number; unit: string; use_: string; time_min: number };

let {
  type,
  open,
  onclose,
  onadd,
}: {
  type: 'hop' | 'fermentable' | 'yeast' | 'misc';
  open: boolean;
  onclose: () => void;
  onadd: (payload: AddPayload) => void;
} = $props();

let library = $state<(Hop | Fermentable | Yeast | Misc)[]>([]);
let selected = $state<Hop | Fermentable | Yeast | Misc | null>(null);

const MISC_USES = ['Boil', 'Mash', 'Primary', 'Secondary', 'Bottling'] as const;
const MISC_UNITS = ['g', 'oz', 'tsp', 'tbsp', 'mL'] as const;

let miscUse = $state('Boil');
let miscUnit = $state('g');
let miscTime = $state(15);
```

- [ ] **Step 2: Update `loadLibrary`, `reloadLibrary`, amount reset `$effect`, `handleAdd`, `rowSubtext`, `headerIcon`, `headerTitle`, search placeholder**

```ts
async function loadLibrary() {
  if (libraryLoaded) return;
  if (type === 'hop') library = (await ipc(listHopLibrary())) ?? [];
  else if (type === 'fermentable') library = (await ipc(listFermentableLibrary())) ?? [];
  else if (type === 'yeast') library = (await ipc(listYeastLibrary())) ?? [];
  else library = (await ipc(listMiscLibrary())) ?? [];
  libraryLoaded = true;
}
// Apply the same pattern to reloadLibrary.

// Amount reset $effect — add misc case:
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
  else if (type === 'yeast') { amount = 1; }
  else { amount = 1; miscUse = 'Boil'; miscUnit = 'g'; miscTime = 15; }
});

function handleAdd() {
  if (!selected || amount <= 0) return;
  if (type === 'hop') {
    onadd({ type: 'hop', item: selected as Hop, form: hopForm, amount_kg: amount, use_, time_min: time, hopstand_temp_c: use_ === 'hopstand' ? hopstand_temp_c : null });
  } else if (type === 'fermentable') {
    onadd({ type: 'fermentable', item: selected as Fermentable, amount_kg: amount });
  } else if (type === 'yeast') {
    onadd({ type: 'yeast', item: selected as Yeast, amount });
  } else {
    onadd({ type: 'misc', item: selected as Misc, amount, unit: miscUnit, use_: miscUse, time_min: miscTime });
  }
}

function rowSubtext(item: Hop | Fermentable | Yeast | Misc): string {
  if (type === 'hop') return `${(item as Hop).alpha_pct}% AA`;
  if (type === 'fermentable') {
    const f = item as Fermentable;
    return `${f.yield_pct.toFixed(0)}% · ${f.color_lovibond}°L`;
  }
  if (type === 'yeast') {
    const y = item as Yeast;
    return y.laboratory ?? y.form;
  }
  return (item as Misc).type_;
}

const headerIcon = $derived<BrewingIconName>(
  type === "hop" ? "hop" : type === "fermentable" ? "fermentable" : type === "yeast" ? "yeast" : "misc"
);

const headerTitle = $derived(
  type === "hop" ? "Add Hop" : type === "fermentable" ? "Add Fermentable" : type === "yeast" ? "Add Yeast" : "Add Misc"
);
```

Search input placeholder (in the template):
```svelte
placeholder="Search {type === 'hop' ? 'hops' : type === 'fermentable' ? 'fermentables' : type === 'yeast' ? 'yeasts' : 'misc'}…"
```

- [ ] **Step 3: Add misc detail and bottom controls to the mobile right/detail panel**

The mobile picker's detail section (below the list) uses the same conditional pattern. Change the yeast `{:else}` to `{:else if type === 'yeast'}` and add:

```svelte
{:else if type === 'misc'}
  {@const misc = selected as Misc}
  <div style="padding: 16px; display: flex; flex-direction: column; gap: 10px;">
    <div>
      <h2 style="font-size: 17px; font-weight: 700; margin: 0;">{misc.name}</h2>
      <div style="display: flex; gap: 6px; margin-top: 6px; flex-wrap: wrap;">
        <span style="background: var(--color-bg-elevated); color: var(--color-text-secondary); padding: 2px 8px; border-radius: 99px; font-size: 11px;">{misc.type_}</span>
        {#if misc.source === 'user'}
          <span style="background: color-mix(in srgb, var(--color-accent) 15%, transparent); color: var(--color-accent); padding: 2px 8px; border-radius: 99px; font-size: 11px; border: 1px solid color-mix(in srgb, var(--color-accent) 40%, transparent);">custom</span>
        {/if}
      </div>
    </div>
    {#if misc.use_for}
      <p style="font-size: 13px; color: var(--color-text-secondary); line-height: 1.5; margin: 0;">{misc.use_for}</p>
    {/if}
    {#if misc.notes}
      <p style="font-size: 12px; color: var(--color-text-muted); line-height: 1.5; margin: 0;">{misc.notes}</p>
    {/if}
    <div style="display: flex; flex-wrap: wrap; gap: 10px; align-items: flex-end; margin-top: 4px;">
      <div>
        <div style="font-size: 11px; color: var(--color-text-secondary); margin-bottom: 4px;">Amount</div>
        <input type="number" inputmode="decimal" step="0.1" bind:value={amount} min="0.001"
          style="width: 70px; background: var(--color-bg-elevated); border: 1px solid var(--color-border); border-radius: 5px; padding: 7px 8px; color: var(--color-text-primary); font-size: 14px;" />
      </div>
      <div>
        <div style="font-size: 11px; color: var(--color-text-secondary); margin-bottom: 4px;">Unit</div>
        <select bind:value={miscUnit} style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); border-radius: 5px; padding: 7px 8px; color: var(--color-text-primary); font-size: 14px;">
          {#each MISC_UNITS as u}<option value={u}>{u}</option>{/each}
        </select>
      </div>
      <div>
        <div style="font-size: 11px; color: var(--color-text-secondary); margin-bottom: 4px;">Use</div>
        <select bind:value={miscUse} style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); border-radius: 5px; padding: 7px 8px; color: var(--color-text-primary); font-size: 14px;">
          {#each MISC_USES as u}<option value={u}>{u}</option>{/each}
        </select>
      </div>
      <div>
        <div style="font-size: 11px; color: var(--color-text-secondary); margin-bottom: 4px;">Time (min)</div>
        <input type="number" inputmode="decimal" step="1" bind:value={miscTime} min="0"
          style="width: 65px; background: var(--color-bg-elevated); border: 1px solid var(--color-border); border-radius: 5px; padding: 7px 8px; color: var(--color-text-primary); font-size: 14px;" />
      </div>
    </div>
    <button onclick={handleAdd} disabled={!canAdd}
      style="width: 100%; background: {canAdd ? 'var(--color-accent)' : 'var(--color-bg-elevated)'}; color: {canAdd ? '#fff' : 'var(--color-text-muted)'}; border: none; border-radius: 8px; padding: 12px; font-size: 15px; font-weight: 600; cursor: {canAdd ? 'pointer' : 'default'}; margin-top: 4px;">
      Add to Recipe
    </button>
  </div>
{/if}
```

- [ ] **Step 4: Verify types and commit**

```bash
just check-ts
git add src/lib/mobile/IngredientPicker.svelte
git commit -m "feat: add misc type to mobile IngredientPicker"
```

---

## Task 9: Wire MiscTable into IngredientsTab and verify

**Files:**
- Modify: `src/lib/components/tabs/IngredientsTab.svelte`

- [ ] **Step 1: Add MiscTable to IngredientsTab**

`src/lib/components/tabs/IngredientsTab.svelte` currently renders three cards. Add the import and a fourth card:

```svelte
<script lang="ts">
  import type { Recipe } from "$lib/api";
  import FermentablesTable from "$lib/components/ingredients/FermentablesTable.svelte";
  import HopsTable from "$lib/components/ingredients/HopsTable.svelte";
  import YeastsTable from "$lib/components/ingredients/YeastsTable.svelte";
  import MiscTable from "$lib/components/ingredients/MiscTable.svelte";
  import Card from "$lib/components/Card.svelte";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();
</script>

<div class="flex flex-col gap-4">
  <Card title="Fermentables">
    <FermentablesTable {recipe} {onchange} />
  </Card>

  <Card title="Hops">
    <HopsTable {recipe} {onchange} />
  </Card>

  <Card title="Yeast">
    <YeastsTable {recipe} {onchange} />
  </Card>

  <Card title="Misc">
    <MiscTable {recipe} {onchange} />
  </Card>
</div>
```

- [ ] **Step 2: Run all checks**

```bash
just check-ts
just test-rust
```

Expected: no TypeScript errors, all Rust tests pass.

- [ ] **Step 3: Commit**

```bash
git add src/lib/components/tabs/IngredientsTab.svelte
git commit -m "feat: add Misc card to IngredientsTab"
```

- [ ] **Step 4: Manual smoke test**

Start the app with `just dev` (or the equivalent dev command). Open a recipe, go to the Ingredients tab. Verify:
1. A "Misc" card appears below Yeast
2. Clicking "+ Add" opens the ingredient picker with a misc search list
3. Selecting a misc item shows its detail panel with type badge and notes
4. Entering an amount, choosing a unit (e.g. "tsp"), use, and time, then clicking "Add to Recipe" adds a row to the table
5. The row shows Name, Type, Amount + unit, Use, Time
6. Clicking × removes the row

Then open a recipe on mobile layout and repeat steps 2–6.
