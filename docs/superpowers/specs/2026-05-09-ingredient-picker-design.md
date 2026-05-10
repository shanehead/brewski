# Ingredient Picker Dialog — Design Spec

## Summary

Replace the inline add forms in `HopsTable`, `FermentablesTable`, and `YeastsTable` with a master/detail modal dialog that lets the user search the full BeerMaverick library and view rich ingredient data before adding to the recipe.

---

## Data Layer

The `Hop` and `Fermentable` TypeScript interfaces in `src/lib/api.ts` are currently thin. Both must be expanded to expose all BeerMaverick columns:

**Hop** — add: `beta_pct`, `type_` (purpose), `origin`, `notes`, `substitutes`, `hsi_pct`, `humulene_pct`, `caryophyllene_pct`, `cohumulone_pct`, `myrcene_pct`

**Fermentable** — add: `origin`, `supplier`, `notes` (subcategory), `add_after_boil`, `diastatic_power_lintner`, `max_in_batch_pct`

**Yeast** — already complete, no changes needed.

The corresponding Rust command handlers (`list_hop_library`, `list_fermentable_library`) must SELECT all columns. The Rust structs must include the new fields.

---

## Component Architecture

A single shared `IngredientPicker.svelte` component handles all three ingredient types, configured by a `type` prop (`'hop' | 'fermentable' | 'yeast'`).

Each ingredient table (`HopsTable`, `FermentablesTable`, `YeastsTable`) replaces its current inline "adding" form with a `<IngredientPicker>` element gated by a `open: boolean` state. On confirm, the picker dispatches a callback with the selected item + recipe-specific inputs; the table calls the existing `createRecipe*` API.

The detail panel renders type-specific content via conditional blocks inside the picker.

---

## Dialog UX & Behavior

**Size:** `80vw × 75vh`, capped at `max-width: 960px`, with a sensible `min-width` floor. The list/detail split is ~38%/62% of the dialog width.

**Opening:** Triggered by the existing "+ Add" button in each ingredient table. The inline form is removed entirely.

**Closing:** Escape key or clicking the backdrop.

**Layout — left panel (list):**
- Search input at top, filters in real time client-side (full library already loaded on mount)
- Scrollable list of rows; each row shows:
  - Hops: name + alpha %
  - Fermentables: name + yield %
  - Yeasts: name + lab name + attenuation range
- Selected row highlighted with accent color

**Layout — right panel (detail):**

Shown when a row is selected. Content is type-specific:

*Hops:* Name, badges (origin, purpose, form), stat grid (alpha, beta, cohumulone, myrcene, humulene, caryophyllene), characteristics text, substitutes. Recipe inputs: amount, use, time.

*Fermentables:* Name, badges (type, subcategory), stat grid (yield, color with swatch, diastatic power, max in batch). Recipe inputs: amount.

*Yeasts:* Name, lab · product ID, badges (fermentation type, form, species), stat grid (attenuation range, temperature range, flocculation, alcohol tolerance, Phenolic, Diastaticus). Flavor profile text, styles, comparables. Recipe inputs: amount.

Note: POF+ is labelled **"Phenolic"**, STA1+ is labelled **"Diastaticus"**.

**Unselected state:** Right panel shows a placeholder: *"Select an ingredient to see details."*

**Add to Recipe button:** Disabled until an item is selected and amount > 0. On click: calls the existing `createRecipe*` API, closes the dialog.

---

## Edge Cases

- Empty search: list shows *"No results for '…'"*, detail panel stays at placeholder
- Library load failure: falls back to existing `ipc()` error toast, no special handling needed
- Amount ≤ 0: "Add to Recipe" button remains disabled
