# Starter Recipes Design

**Date:** 2026-05-24
**Status:** Approved

## Overview

Pre-seeded "starter" recipes (e.g. Pliny the Elder, Heady Topper, Tree House Julius, and a selection of non-IPA styles) that users can browse and clone into their own library. Starters never appear in the user's recipe list — they live in a collapsible section below it. Users can view a starter in full read-only detail and clone it to their own library with one tap.

---

## Data Model

### Migration 008: `source` column on `recipes`

Follows the exact same pattern as `004_user_ingredients.sql`:

```sql
ALTER TABLE recipes ADD COLUMN source TEXT NOT NULL DEFAULT 'user'
  CHECK (source IN ('seeded', 'user'));
```

- Existing rows default to `'user'` — no data migration needed.
- Baseline recipes are seeded in the same migration with `source = 'seeded'`.
- IDs use the existing `bm-` prefix convention (e.g. `bm-recipe-pliny-the-elder`).

### Seeded recipes (initial set)

IPAs / DIPAs:
- Pliny the Elder (Russian River) — DIPA
- Heady Topper (The Alchemist) — DIPA
- Tree House Julius — NEIPA

Non-IPA:
- Guinness Draught (Irish Stout)
- Saison Dupont (Belgian Saison)

All seeded recipes use ingredient IDs from the existing seeded ingredient library where possible. Any gaps are handled by embedding ingredient data directly (name-only, no library FK required).

---

## Backend

### OpenAPI / type changes

- Add `source: 'user' | 'seeded'` field to the `Recipe` and `RecipeSummary` schemas in `docs/openapi/`.
- Add `list_baseline_recipes` command returning `Vec<RecipeSummary>` filtered to `source = 'seeded'`.
- `list_recipes` gains `WHERE source = 'user'` — baselines are never returned to the user's recipe list.
- No changes to `create_recipe` — the existing `source_id` field on `CreateRecipeInput` handles cloning by copying all additions into the new recipe. The new recipe defaults to `source = 'user'`.

### Repository changes

- `RecipeRepository::list()` — add `.filter(recipes::Column::Source.eq("user"))`.
- `RecipeRepository::list_baseline()` — new method, same as `list()` but filters `source = 'seeded'`.

### Tauri command

```rust
#[tauri::command]
pub async fn list_baseline_recipes(state: State<'_, AppState>) -> Result<Vec<RecipeSummary>, AppError> {
    RecipeRepository::new(&state.db).list_baseline().await
}
```

---

## Settings

Add `starters_collapsed?: boolean` to `AppSettings` in `src/lib/stores/settings.ts`. Persisted via `saveSetting('starters_collapsed', 'true' | 'false')` when the user toggles the section. Restored from `$settings.starters_collapsed` on mount. Defaults to `false` (expanded) when not yet set.

---

## Frontend

### Recipes list — desktop (`RecipeList.svelte`)

Below the user's recipe list, add:

1. A sticky section-divider row — "Starter Recipes" label left, collapse toggle (▾/▸) right.
2. When expanded: baseline recipe rows rendered below in the same row style as user recipes, but visually muted (secondary text color). No delete button. Clicking navigates to the read-only baseline viewer (`/baseline-recipe/:id`).
3. Toggle saves `starters_collapsed` to settings; initial state reads from `$settings.starters_collapsed`.

Baseline rows do **not** appear in search results — the search filter applies only to user recipes.

### Recipes screen — mobile (`RecipesHome.svelte`)

Same section divider + collapsible group at the bottom of the scrollable list. Same `starters_collapsed` persistence.

### Baseline recipe viewer

Two new platform-specific components — `src/lib/desktop/BaselineRecipeView.svelte` and `src/lib/mobile/BaselineRecipeView.svelte` — following the same split as `RecipeView.svelte`. Both reuse existing shared tab components (Overview, Ingredients, Mash, Water, Fermentation, Notes) and `StatsSidebar`. Differences from the editable view:

- All fields are display-only — no inline editing.
- No version history panel.
- No Batches tab.
- No delete action.
- Header shows a prominent **"Clone to My Recipes"** button (accent style). On click:
  1. Calls `create_recipe({ name: recipe.name, source_id: recipe.id, ... })`.
  2. Navigates to the new user recipe at `/recipe/:newId`.

### Routes

- Desktop: `/baseline-recipe/:id` → `RecipeView` wrapper that loads from `get_recipe` and renders `BaselineRecipeView`.
- Mobile: same route, mobile-specific wrapper.

---

## What Is Not In Scope

- User-curated or community-contributed starters.
- The ability to update or delete seeded recipes after shipping.
- Syncing seeded recipes via cloud sync — baselines are local-only, seeded at DB init.
- Attribution / licensing display in the UI (seeded recipes are homebrewer approximations, not official).
