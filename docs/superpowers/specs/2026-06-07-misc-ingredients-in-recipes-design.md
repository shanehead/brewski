# Misc Ingredients in Recipes

**Date:** 2026-06-07
**Status:** Approved

## Summary

Wire the existing Misc ingredient backend into the recipe UI. The backend (DB schema, repository, API commands) is fully implemented. This feature adds the UI to add, view, and remove misc ingredient additions (spices, finings, herbs, water agents, etc.) from a recipe.

## Scope

- Wire Misc into recipes only
- Library page already supports creating/editing/deleting custom Misc ingredients — no changes there
- No inline "create from scratch" in the picker (go to Library first)

Out of scope: new ingredient types, picker refactoring, custom yeast creation flows.

## Data Layer

### DB Migration

Add a `unit` column to `recipe_addition_miscs`:

```sql
ALTER TABLE recipe_addition_miscs ADD COLUMN unit TEXT NOT NULL DEFAULT 'g';
```

The default `'g'` gives existing rows a sensible value without a backfill.

No other tables change.

### Model Updates

**`RecipeAdditionMisc`** — add `unit: String`

**`CreateMiscAdditionInput`** — add `unit: String`

**`UpdateMiscAdditionInput`** — add `unit: Option<String>`

OpenAPI spec and generated frontend types (`api.gen.ts`) updated accordingly.

### Unit Set

Five supported units: `g`, `oz`, `tsp`, `tbsp`, `mL`

The existing `amount_is_weight` boolean is derived from the chosen unit at save time:
- `g`, `oz` → `amount_is_weight = true`
- `tsp`, `tbsp`, `mL` → `amount_is_weight = false`

The raw `amount` number is stored as entered — no conversion to a canonical base unit. "2 tsp" round-trips as "2 tsp".

## Frontend Components

### `IngredientPicker.svelte` (two files)

Extend the existing picker with a `'misc'` type. The same changes apply to both platform variants:
- `src/lib/desktop/IngredientPicker.svelte`
- `src/lib/mobile/IngredientPicker.svelte`

**Props:** `type` union gains `'misc'`

**Library loading:** `listMiscLibrary()` when `type === 'misc'`

**`AddPayload` union** gains:
```ts
| { type: 'misc'; item: Misc; amount: number; unit: string; use_: string; time_min: number }
```

**Detail panel:** Name, type badge, source badge (`built-in` / `custom`), notes, use_for

**Bottom bar controls:**
- Amount: number input
- Unit: dropdown — `g`, `oz`, `tsp`, `tbsp`, `mL`
- Use: dropdown — `Boil`, `Mash`, `Primary`, `Secondary`, `Bottling`
- Time (min): number input
- "Add to Recipe" button (disabled until selection + amount > 0)

**No fork button** in the picker — seeded misc ingredients can be duplicated from the Library page, same as the existing flow for hops and fermentables.

**Header:** icon `"misc"`, title "Add Misc"

### `MiscTable.svelte` (new)

Location: `src/lib/components/ingredients/MiscTable.svelte`

Follows `HopsTable.svelte` structure exactly.

**Header row:** BrewingIcon `"misc"` + "Misc" label + "+ Add" button

**Table columns:** Name | Type | Amount (with unit) | Use | Time | ×

**Behavior:**
- "+ Add" opens `IngredientPicker` with `type="misc"`
- `handlePickerAdd` calls `createRecipeMisc`, then `onchange()`
- × calls `deleteRecipeMisc(id)`, then `onchange()`
- No inline editing (consistent with Hops, Fermentables, Yeast tables)
- Table only renders when `recipe.miscs.length > 0` (empty state: nothing shown)

### `IngredientsTab.svelte`

`src/lib/components/tabs/IngredientsTab.svelte` is a single shared component (not platform-split). Add a Misc card below the Yeast card:

```svelte
<Card title="Misc">
  <MiscTable {recipe} {onchange} />
</Card>
```

## Error Handling

All API calls go through the existing `ipc()` wrapper. No new error paths.

## Testing

**Rust:** Update misc repository tests to include `unit` in `CreateMiscAdditionInput`. Add a round-trip test for a non-default unit (e.g. `"tsp"`).

**Frontend:** No new component-level tests — consistent with existing ingredient table components.
