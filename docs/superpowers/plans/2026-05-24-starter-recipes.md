# Starter Recipes Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Seed five read-only baseline recipes (Pliny the Elder, Heady Topper, Julius, Guinness Draught, Saison Dupont) into the database and surface them in a collapsible "Starter Recipes" section below the user's recipe list, with a "Clone to My Recipes" action.

**Architecture:** A `source` column (`'user'` | `'seeded'`) on the `recipes` table (matching the ingredient pattern from migration 004) separates baseline from user recipes. The backend filters `list_recipes` to user-only and exposes a new `list_baseline_recipes` command. The frontend adds a collapsible section to the recipe list on both platforms, a new read-only `BaselineRecipeView` component, and a new `/baseline-recipe/[id]` route.

**Tech Stack:** Rust / SeaORM / SQLite (backend), SvelteKit / Svelte 5 / TypeScript (frontend), OpenAPI 3.1 + codegen (`just gen`, `just gen-entities`)

---

## File Map

**Create:**
- `src-tauri/migrations/008_starter_recipes.sql`
- `docs/openapi/paths/commands/list_baseline_recipes.yaml`
- `src/lib/desktop/BaselineRecipeView.svelte`
- `src/lib/mobile/BaselineRecipeView.svelte`
- `src/routes/baseline-recipe/[id]/+page.ts`
- `src/routes/baseline-recipe/[id]/+page.svelte`

**Modify:**
- `docs/openapi/components/schemas/RecipeSummary.yaml`
- `docs/openapi/components/schemas/Recipe.yaml`
- `docs/openapi/openapi.yaml`
- `src-tauri/src/entities/recipes.rs` *(regenerated via `just gen-entities`)*
- `src-tauri/src/models.gen.rs` *(regenerated via `just gen`)*
- `src/lib/api.gen.ts` *(regenerated via `just gen`)*
- `src-tauri/src/repositories/recipe.rs`
- `src-tauri/src/commands/recipes.rs`
- `src-tauri/src/lib.rs`
- `src/lib/stores/settings.ts`
- `src/lib/stores/recipes.ts`
- `src/lib/components/RecipeList.svelte`
- `src/lib/mobile/RecipesHome.svelte`

---

## Task 1: Migration — add `source` column and seed baseline recipes

**Files:**
- Create: `src-tauri/migrations/008_starter_recipes.sql`

- [ ] **Step 1: Write the migration**

Create `src-tauri/migrations/008_starter_recipes.sql` with the full content below.

The timestamp `1748131200` is a fixed seed timestamp (2026-05-24 00:00:00 UTC). Recipe IDs use the `bm-recipe-` prefix, hop/fermentable/yeast addition IDs use `bm-r<N>-<ingredient>-<N>`.

```sql
-- Add source column to recipes (matches 004_user_ingredients.sql pattern)
ALTER TABLE recipes ADD COLUMN source TEXT NOT NULL DEFAULT 'user'
  CHECK (source IN ('seeded', 'user'));

-- ─────────────────────────────────────────────────────────────
-- 1. Pliny the Elder (Russian River) — Double IPA, 5.5 gal
--    Reference: https://homebrewersassociation.org/homebrew-recipe/russian-river-pliny-the-elder-clone/
-- ─────────────────────────────────────────────────────────────
INSERT INTO recipes (id, name, type, batch_size_l, boil_size_l, boil_time_min,
  efficiency_pct, og, fg, notes, source, created_at, updated_at)
VALUES (
  'bm-recipe-pliny-the-elder',
  'Pliny the Elder',
  'All Grain',
  20.8,   -- 5.5 gal
  25.2,   -- 6.66 gal
  90,
  72.0,
  1.072,
  1.012,
  'Clone of Russian River Pliny the Elder Double IPA. Ferment at 68°F (20°C) for 2 weeks, dry hop in two additions. Reference: https://homebrewersassociation.org/homebrew-recipe/russian-river-pliny-the-elder-clone/',
  'seeded',
  1748131200,
  1748131200
);

INSERT INTO recipe_addition_fermentables
  (id, recipe_id, name, type, yield_pct, color_lovibond, amount_kg, addition_order)
VALUES
  ('bm-r1-f1', 'bm-recipe-pliny-the-elder', 'American 2-Row Pale Malt', 'grain', 78.0, 1.8, 6.01, 1),
  ('bm-r1-f2', 'bm-recipe-pliny-the-elder', 'Crystal 45L', 'grain', 74.0, 45.0, 0.272, 2),
  ('bm-r1-f3', 'bm-recipe-pliny-the-elder', 'Carapils / Dextrin', 'grain', 72.0, 1.5, 0.272, 3),
  ('bm-r1-f4', 'bm-recipe-pliny-the-elder', 'Corn Sugar (Dextrose)', 'sugar', 100.0, 0.0, 0.340, 4);

INSERT INTO recipe_addition_hops
  (id, recipe_id, name, alpha_pct, form, amount_kg, "use", time_min, addition_order)
VALUES
  ('bm-r1-h1', 'bm-recipe-pliny-the-elder', 'Columbus (Tomahawk)', 14.0, 'Pellet', 0.0992, 'Boil', 90, 1),
  ('bm-r1-h2', 'bm-recipe-pliny-the-elder', 'Columbus (Tomahawk)', 14.0, 'Pellet', 0.0213, 'Boil', 45, 2),
  ('bm-r1-h3', 'bm-recipe-pliny-the-elder', 'Simcoe', 13.0, 'Pellet', 0.0284, 'Boil', 30, 3),
  ('bm-r1-h4', 'bm-recipe-pliny-the-elder', 'Centennial', 10.0, 'Pellet', 0.0284, 'Aroma', 0, 4),
  ('bm-r1-h5', 'bm-recipe-pliny-the-elder', 'Simcoe', 13.0, 'Pellet', 0.0709, 'Aroma', 0, 5),
  ('bm-r1-h6', 'bm-recipe-pliny-the-elder', 'Columbus (Tomahawk)', 14.0, 'Pellet', 0.0567, 'Dry Hop', 7, 6),
  ('bm-r1-h7', 'bm-recipe-pliny-the-elder', 'Centennial', 10.0, 'Pellet', 0.0284, 'Dry Hop', 7, 7),
  ('bm-r1-h8', 'bm-recipe-pliny-the-elder', 'Simcoe', 13.0, 'Pellet', 0.0284, 'Dry Hop', 7, 8);

INSERT INTO recipe_addition_yeasts
  (id, recipe_id, name, type, form, laboratory, product_id, attenuation_pct)
VALUES
  ('bm-r1-y1', 'bm-recipe-pliny-the-elder', 'American Ale', 'ale', 'dry', 'Fermentis', 'US-05', 77.0);

INSERT INTO mashes (id, recipe_id, name, grain_temp_c)
VALUES ('bm-r1-mash', 'bm-recipe-pliny-the-elder', 'Single Infusion', 18.3);

INSERT INTO mash_steps (id, mash_id, name, type, step_temp_c, step_time_min, step_order)
VALUES ('bm-r1-ms1', 'bm-r1-mash', 'Saccharification Rest', 'Infusion', 66.7, 60, 1);

-- ─────────────────────────────────────────────────────────────
-- 2. Heady Topper (The Alchemist) — Double IPA, 5 gal
--    Reference: https://byo.com/recipes/alchemist-heady-topper-clone/
-- ─────────────────────────────────────────────────────────────
INSERT INTO recipes (id, name, type, batch_size_l, boil_size_l, boil_time_min,
  efficiency_pct, og, fg, notes, source, created_at, updated_at)
VALUES (
  'bm-recipe-heady-topper',
  'Heady Topper',
  'All Grain',
  18.9,   -- 5 gal
  22.7,   -- 6 gal
  90,
  72.0,
  1.077,
  1.017,
  'Clone of The Alchemist Heady Topper Double IPA. Key: use Vermont Ale (Conan) yeast for authentic character. Heavy dry hop in two split additions. Reference: https://byo.com/recipes/alchemist-heady-topper-clone/',
  'seeded',
  1748131200,
  1748131200
);

INSERT INTO recipe_addition_fermentables
  (id, recipe_id, name, type, yield_pct, color_lovibond, amount_kg, addition_order)
VALUES
  ('bm-r2-f1', 'bm-recipe-heady-topper', 'British 2-Row Pale Malt', 'grain', 78.0, 2.0, 6.80, 1),
  ('bm-r2-f2', 'bm-recipe-heady-topper', 'CaraVienne', 'grain', 74.0, 20.0, 0.170, 2),
  ('bm-r2-f3', 'bm-recipe-heady-topper', 'Corn Sugar (Dextrose)', 'sugar', 100.0, 0.0, 0.454, 3);

INSERT INTO recipe_addition_hops
  (id, recipe_id, name, alpha_pct, form, amount_kg, "use", time_min, addition_order)
VALUES
  ('bm-r2-h1', 'bm-recipe-heady-topper', 'Chinook', 13.0, 'Pellet', 0.0284, 'Boil', 60, 1),
  ('bm-r2-h2', 'bm-recipe-heady-topper', 'Columbus (Tomahawk)', 14.0, 'Pellet', 0.0142, 'Boil', 60, 2),
  ('bm-r2-h3', 'bm-recipe-heady-topper', 'Centennial', 10.0, 'Pellet', 0.0142, 'Boil', 30, 3),
  ('bm-r2-h4', 'bm-recipe-heady-topper', 'Simcoe', 13.0, 'Pellet', 0.0284, 'Boil', 10, 4),
  ('bm-r2-h5', 'bm-recipe-heady-topper', 'Centennial', 10.0, 'Pellet', 0.0284, 'Boil', 10, 5),
  ('bm-r2-h6', 'bm-recipe-heady-topper', 'Amarillo', 9.0, 'Pellet', 0.0567, 'Aroma', 0, 6),
  ('bm-r2-h7', 'bm-recipe-heady-topper', 'Simcoe', 13.0, 'Pellet', 0.0567, 'Dry Hop', 4, 7),
  ('bm-r2-h8', 'bm-recipe-heady-topper', 'Centennial', 10.0, 'Pellet', 0.0284, 'Dry Hop', 4, 8),
  ('bm-r2-h9', 'bm-recipe-heady-topper', 'Citra', 12.0, 'Pellet', 0.0284, 'Dry Hop', 4, 9),
  ('bm-r2-h10', 'bm-recipe-heady-topper', 'Columbus (Tomahawk)', 14.0, 'Pellet', 0.0142, 'Dry Hop', 4, 10);

INSERT INTO recipe_addition_yeasts
  (id, recipe_id, name, type, form, laboratory, product_id, attenuation_pct)
VALUES
  ('bm-r2-y1', 'bm-recipe-heady-topper', 'Vermont Ale (Conan)', 'ale', 'liquid', 'The Yeast Bay', 'Vermont Ale', 74.0);

INSERT INTO mashes (id, recipe_id, name, grain_temp_c)
VALUES ('bm-r2-mash', 'bm-recipe-heady-topper', 'Single Infusion', 18.3);

INSERT INTO mash_steps (id, mash_id, name, type, step_temp_c, step_time_min, step_order)
VALUES ('bm-r2-ms1', 'bm-r2-mash', 'Saccharification Rest', 'Infusion', 67.2, 60, 1);

-- ─────────────────────────────────────────────────────────────
-- 3. Julius (Tree House Brewing) — NEIPA, 5 gal
--    Reference: https://byo.com/recipe/tree-house-brewing-company-julius-clone/
--    Reference: https://hazyandhoppy.com/marshall-bishops-treehouse-julius-clone-recipe/
-- ─────────────────────────────────────────────────────────────
INSERT INTO recipes (id, name, type, batch_size_l, boil_size_l, boil_time_min,
  efficiency_pct, og, fg, notes, source, created_at, updated_at)
VALUES (
  'bm-recipe-julius',
  'Julius',
  'All Grain',
  18.9,   -- 5 gal
  22.7,   -- 6 gal
  60,
  72.0,
  1.065,
  1.014,
  'Clone of Tree House Brewing Julius New England IPA. Use London Ale III or similar hazy yeast. Low-sulfate, high-chloride water. Biotransformation dry hop on day 2 of active fermentation. References: https://byo.com/recipe/tree-house-brewing-company-julius-clone/ and https://hazyandhoppy.com/marshall-bishops-treehouse-julius-clone-recipe/',
  'seeded',
  1748131200,
  1748131200
);

INSERT INTO recipe_addition_fermentables
  (id, recipe_id, name, type, yield_pct, color_lovibond, amount_kg, addition_order)
VALUES
  ('bm-r3-f1', 'bm-recipe-julius', 'American 2-Row Pale Malt', 'grain', 78.0, 1.8, 4.54, 1),
  ('bm-r3-f2', 'bm-recipe-julius', 'Golden Promise Pale Malt', 'grain', 80.0, 2.5, 0.907, 2),
  ('bm-r3-f3', 'bm-recipe-julius', 'Carafoam (Carapils)', 'grain', 72.0, 1.5, 0.227, 3),
  ('bm-r3-f4', 'bm-recipe-julius', 'Flaked Oats', 'adjunct', 70.0, 1.0, 0.907, 4);

INSERT INTO recipe_addition_hops
  (id, recipe_id, name, alpha_pct, form, amount_kg, "use", time_min, addition_order)
VALUES
  ('bm-r3-h1', 'bm-recipe-julius', 'Citra', 12.0, 'Pellet', 0.0142, 'Boil', 60, 1),
  ('bm-r3-h2', 'bm-recipe-julius', 'Citra', 12.0, 'Pellet', 0.0567, 'Aroma', 0, 2),
  ('bm-r3-h3', 'bm-recipe-julius', 'Mosaic', 11.5, 'Pellet', 0.0567, 'Aroma', 0, 3),
  ('bm-r3-h4', 'bm-recipe-julius', 'Citra', 12.0, 'Pellet', 0.0851, 'Dry Hop', 4, 4),
  ('bm-r3-h5', 'bm-recipe-julius', 'Mosaic', 11.5, 'Pellet', 0.0567, 'Dry Hop', 4, 5);

INSERT INTO recipe_addition_yeasts
  (id, recipe_id, name, type, form, laboratory, product_id, attenuation_pct)
VALUES
  ('bm-r3-y1', 'bm-recipe-julius', 'London Ale III', 'ale', 'liquid', 'Omega Yeast', 'OYL-011', 74.0);

INSERT INTO mashes (id, recipe_id, name, grain_temp_c)
VALUES ('bm-r3-mash', 'bm-recipe-julius', 'Single Infusion', 18.3);

INSERT INTO mash_steps (id, mash_id, name, type, step_temp_c, step_time_min, step_order)
VALUES ('bm-r3-ms1', 'bm-r3-mash', 'Saccharification Rest', 'Infusion', 67.2, 60, 1);

-- ─────────────────────────────────────────────────────────────
-- 4. Guinness Draught — Irish Dry Stout, 5 gal
--    Reference: https://byo.com/recipe/guinness-draught-clone/
--    Reference: https://homebrewacademy.com/guinness-recipe/
-- ─────────────────────────────────────────────────────────────
INSERT INTO recipes (id, name, type, batch_size_l, boil_size_l, boil_time_min,
  efficiency_pct, og, fg, notes, source, created_at, updated_at)
VALUES (
  'bm-recipe-guinness-draught',
  'Guinness Draught',
  'All Grain',
  18.9,   -- 5 gal
  22.7,   -- 6 gal
  90,
  72.0,
  1.044,
  1.011,
  'Clone of Guinness Draught Irish Dry Stout. Key: use unmalted roasted barley (500L) for the characteristic dry, roasted finish. Ferment cool at 64°F (18°C). References: https://byo.com/recipe/guinness-draught-clone/ and https://homebrewacademy.com/guinness-recipe/',
  'seeded',
  1748131200,
  1748131200
);

INSERT INTO recipe_addition_fermentables
  (id, recipe_id, name, type, yield_pct, color_lovibond, amount_kg, addition_order)
VALUES
  ('bm-r4-f1', 'bm-recipe-guinness-draught', 'English 2-Row Pale Ale Malt', 'grain', 78.0, 3.0, 2.268, 1),
  ('bm-r4-f2', 'bm-recipe-guinness-draught', 'Flaked Barley', 'adjunct', 70.0, 1.5, 1.134, 2),
  ('bm-r4-f3', 'bm-recipe-guinness-draught', 'Roasted Barley (500L)', 'grain', 55.0, 500.0, 0.454, 3);

INSERT INTO recipe_addition_hops
  (id, recipe_id, name, alpha_pct, form, amount_kg, "use", time_min, addition_order)
VALUES
  ('bm-r4-h1', 'bm-recipe-guinness-draught', 'East Kent Goldings', 5.0, 'Pellet', 0.0425, 'Boil', 60, 1);

INSERT INTO recipe_addition_yeasts
  (id, recipe_id, name, type, form, laboratory, product_id, attenuation_pct)
VALUES
  ('bm-r4-y1', 'bm-recipe-guinness-draught', 'Irish Ale', 'ale', 'liquid', 'Wyeast', '1084', 72.0);

INSERT INTO mashes (id, recipe_id, name, grain_temp_c)
VALUES ('bm-r4-mash', 'bm-recipe-guinness-draught', 'Single Infusion', 18.3);

INSERT INTO mash_steps (id, mash_id, name, type, step_temp_c, step_time_min, step_order)
VALUES ('bm-r4-ms1', 'bm-r4-mash', 'Saccharification Rest', 'Infusion', 65.6, 60, 1);

-- ─────────────────────────────────────────────────────────────
-- 5. Saison Dupont — Belgian Saison, 5 gal
--    Reference: https://www.beerandbrewing.com/belgian-saison-in-the-style-of-saison-dupont-recipe
--    Reference: https://www.brewersfriend.com/homebrew/recipe/view/438817/saison-dupont-clone
-- ─────────────────────────────────────────────────────────────
INSERT INTO recipes (id, name, type, batch_size_l, boil_size_l, boil_time_min,
  efficiency_pct, og, fg, notes, source, created_at, updated_at)
VALUES (
  'bm-recipe-saison-dupont',
  'Saison Dupont',
  'All Grain',
  18.9,   -- 5 gal
  22.7,   -- 6 gal
  90,
  75.0,
  1.060,
  1.010,
  'Clone of Brasserie Dupont Saison Dupont. Low mash temp for a very dry finish. Use WY3724 or WLP565; ferment warm (80–90°F / 27–32°C) for full attenuation and classic spicy character. Soft, low-mineral water. References: https://www.beerandbrewing.com/belgian-saison-in-the-style-of-saison-dupont-recipe and https://www.brewersfriend.com/homebrew/recipe/view/438817/saison-dupont-clone',
  'seeded',
  1748131200,
  1748131200
);

INSERT INTO recipe_addition_fermentables
  (id, recipe_id, name, type, yield_pct, color_lovibond, amount_kg, addition_order)
VALUES
  ('bm-r5-f1', 'bm-recipe-saison-dupont', 'Belgian Pilsner Malt', 'grain', 80.0, 1.6, 4.990, 1),
  ('bm-r5-f2', 'bm-recipe-saison-dupont', 'Vienna Malt', 'grain', 78.0, 3.5, 0.227, 2),
  ('bm-r5-f3', 'bm-recipe-saison-dupont', 'Munich Malt', 'grain', 77.0, 6.0, 0.113, 3),
  ('bm-r5-f4', 'bm-recipe-saison-dupont', 'CaraMunich', 'grain', 74.0, 60.0, 0.227, 4),
  ('bm-r5-f5', 'bm-recipe-saison-dupont', 'Wheat Malt', 'grain', 80.0, 2.0, 0.227, 5);

INSERT INTO recipe_addition_hops
  (id, recipe_id, name, alpha_pct, form, amount_kg, "use", time_min, addition_order)
VALUES
  ('bm-r5-h1', 'bm-recipe-saison-dupont', 'Styrian Goldings', 5.5, 'Pellet', 0.0284, 'Boil', 90, 1),
  ('bm-r5-h2', 'bm-recipe-saison-dupont', 'East Kent Goldings', 5.0, 'Pellet', 0.0142, 'Boil', 10, 2);

INSERT INTO recipe_addition_yeasts
  (id, recipe_id, name, type, form, laboratory, product_id, attenuation_pct)
VALUES
  ('bm-r5-y1', 'bm-recipe-saison-dupont', 'Belgian Saison', 'ale', 'liquid', 'Wyeast', '3724', 78.0);

INSERT INTO mashes (id, recipe_id, name, grain_temp_c)
VALUES ('bm-r5-mash', 'bm-recipe-saison-dupont', 'Single Infusion', 18.3);

INSERT INTO mash_steps (id, mash_id, name, type, step_temp_c, step_time_min, step_order)
VALUES ('bm-r5-ms1', 'bm-r5-mash', 'Saccharification Rest', 'Infusion', 63.0, 90, 1);
```

- [ ] **Step 2: Apply the migration**

```bash
just migrate
```

Expected: completes without error. The `recipes` table now has a `source` column, and 5 seeded recipes are in the DB.

- [ ] **Step 3: Verify the seeded data**

```bash
sqlite3 dev.db "SELECT id, name, source FROM recipes ORDER BY source;"
```

Expected output:
```
bm-recipe-pliny-the-elder|Pliny the Elder|seeded
bm-recipe-heady-topper|Heady Topper|seeded
bm-recipe-julius|Julius|seeded
bm-recipe-guinness-draught|Guinness Draught|seeded
bm-recipe-saison-dupont|Saison Dupont|seeded
```
(No `user` rows yet if running on a fresh DB.)

- [ ] **Step 4: Commit**

```bash
git add src-tauri/migrations/008_starter_recipes.sql
git commit -m "feat(db): add source column and seed 5 baseline recipes"
```

---

## Task 2: OpenAPI schema and path changes

**Files:**
- Modify: `docs/openapi/components/schemas/RecipeSummary.yaml`
- Modify: `docs/openapi/components/schemas/Recipe.yaml`
- Create: `docs/openapi/paths/commands/list_baseline_recipes.yaml`
- Modify: `docs/openapi/openapi.yaml`

- [ ] **Step 1: Add `source` to RecipeSummary**

In `docs/openapi/components/schemas/RecipeSummary.yaml`, add `source` to `required` and add the property:

```yaml
type: object
required:
  - id
  - name
  - type_
  - batch_size_l
  - created_at
  - updated_at
  - source
properties:
  id:
    type: string
  name:
    type: string
  style_name:
    type:
      - string
      - "null"
  type_:
    type: string
    description: Recipe type (e.g. All Grain, Extract, Partial Mash)
  batch_size_l:
    type: number
    description: Target batch volume in litres
  created_at:
    type: integer
    format: int64
    description: Unix timestamp (milliseconds)
  updated_at:
    type: integer
    format: int64
    description: Unix timestamp (milliseconds)
  source:
    type: string
    enum: [user, seeded]
    description: "'user' = created by the user; 'seeded' = built-in starter recipe"
```

- [ ] **Step 2: Add `source` to Recipe**

In `docs/openapi/components/schemas/Recipe.yaml`, add `source` to `required` and add the same property block shown in Step 1 (same `type`, `enum`, `description`).

- [ ] **Step 3: Create list_baseline_recipes path**

Create `docs/openapi/paths/commands/list_baseline_recipes.yaml`:

```yaml
post:
  operationId: listBaselineRecipes
  summary: List all built-in starter recipes
  tags:
    - Recipes
  responses:
    "200":
      description: Array of seeded recipe summaries
      content:
        application/json:
          schema:
            type: array
            items:
              $ref: "../../components/schemas/RecipeSummary.yaml"
    "500":
      $ref: "../../components/responses/Error.yaml"
```

- [ ] **Step 4: Register the new path in openapi.yaml**

In `docs/openapi/openapi.yaml`, add after the `list_recipes` path entry:

```yaml
  /commands/list_baseline_recipes:
    $ref: ./paths/commands/list_baseline_recipes.yaml
```

- [ ] **Step 5: Lint the spec**

```bash
just lint-openapi
```

Expected: no errors.

- [ ] **Step 6: Commit**

```bash
git add docs/openapi/
git commit -m "feat(openapi): add source field and list_baseline_recipes command"
```

---

## Task 3: Codegen — regenerate Rust models, TypeScript types, and SeaORM entities

**Files:**
- Modify: `src-tauri/src/models.gen.rs` (regenerated)
- Modify: `src/lib/api.gen.ts` (regenerated)
- Modify: `src-tauri/src/entities/recipes.rs` (regenerated)

- [ ] **Step 1: Regenerate TypeScript types and Rust models**

```bash
just gen
```

Expected: `src/lib/api.gen.ts` and `src-tauri/src/models.gen.rs` are updated. `RecipeSummary` and `Recipe` now include a `source` field. `listBaselineRecipes` function is present in `api.gen.ts`.

- [ ] **Step 2: Regenerate SeaORM entities**

```bash
just gen-entities
```

Expected: `src-tauri/src/entities/recipes.rs` now has `pub source: String`.

- [ ] **Step 3: Verify compilation**

```bash
cargo build --manifest-path src-tauri/Cargo.toml
```

Expected: build fails only on the repository changes not yet made (missing `source` field in `RecipeSummary` construction). That's expected — proceed.

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/models.gen.rs src/lib/api.gen.ts src-tauri/src/entities/recipes.rs
git commit -m "chore(codegen): regenerate after source field addition"
```

---

## Task 4: Repository — filter `list()` and add `list_baseline()`

**Files:**
- Modify: `src-tauri/src/repositories/recipe.rs`

- [ ] **Step 1: Write failing tests**

In `src-tauri/src/repositories/recipe.rs`, add to the `#[cfg(test)] mod tests` block:

```rust
#[tokio::test]
async fn test_list_excludes_seeded_recipes() {
    let db = setup_test_db().await;
    let repo = RecipeRepository::new(&db);

    // Create a user recipe (default source = 'user')
    repo.create(basic_input()).await.unwrap();

    // Directly insert a seeded recipe
    recipes::ActiveModel {
        id: Set("bm-test-seeded".to_string()),
        name: Set("Seeded Recipe".to_string()),
        r#type: Set("All Grain".to_string()),
        batch_size_l: Set(19.0),
        boil_size_l: Set(23.0),
        boil_time_min: Set(60.0),
        source: Set("seeded".to_string()),
        created_at: Set(0),
        updated_at: Set(0),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    let list = repo.list().await.unwrap();
    assert!(list.iter().all(|r| r.source == "user"));
    assert_eq!(list.len(), 1);
}

#[tokio::test]
async fn test_list_baseline_returns_only_seeded() {
    let db = setup_test_db().await;
    let repo = RecipeRepository::new(&db);

    repo.create(basic_input()).await.unwrap();

    recipes::ActiveModel {
        id: Set("bm-test-seeded".to_string()),
        name: Set("Seeded Recipe".to_string()),
        r#type: Set("All Grain".to_string()),
        batch_size_l: Set(19.0),
        boil_size_l: Set(23.0),
        boil_time_min: Set(60.0),
        source: Set("seeded".to_string()),
        created_at: Set(0),
        updated_at: Set(0),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    let baselines = repo.list_baseline().await.unwrap();
    assert_eq!(baselines.len(), 1);
    assert_eq!(baselines[0].source, "seeded");
}
```

- [ ] **Step 2: Run tests to confirm they fail**

```bash
cargo test --manifest-path src-tauri/Cargo.toml test_list_excludes_seeded_recipes test_list_baseline_returns_only_seeded -- --nocapture 2>&1 | tail -20
```

Expected: compile errors (missing `source` in `RecipeSummary` construction and `list_baseline` not found).

- [ ] **Step 3: Update `list()` and add `list_baseline()`**

In `src-tauri/src/repositories/recipe.rs`, update the `list` method and add `list_baseline`. The `use` imports at the top already include `QueryOrder`; add `QueryFilter` and `ColumnTrait`:

```rust
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};
```

Replace the `list` method:

```rust
pub async fn list(&self) -> Result<Vec<RecipeSummary>, AppError> {
    let results = recipes::Entity::find()
        .filter(recipes::Column::Source.eq("user"))
        .find_also_related(styles::Entity)
        .order_by_desc(recipes::Column::UpdatedAt)
        .all(self.db)
        .await?;

    results
        .into_iter()
        .map(|(r, s)| {
            Ok(RecipeSummary {
                id: r.id,
                name: r.name,
                style_name: s.map(|st| st.name),
                type_: r.r#type,
                batch_size_l: r.batch_size_l,
                source: r.source,
                created_at: r.created_at as i64,
                updated_at: r.updated_at as i64,
            })
        })
        .collect()
}

pub async fn list_baseline(&self) -> Result<Vec<RecipeSummary>, AppError> {
    let results = recipes::Entity::find()
        .filter(recipes::Column::Source.eq("seeded"))
        .find_also_related(styles::Entity)
        .order_by_asc(recipes::Column::Name)
        .all(self.db)
        .await?;

    results
        .into_iter()
        .map(|(r, s)| {
            Ok(RecipeSummary {
                id: r.id,
                name: r.name,
                style_name: s.map(|st| st.name),
                type_: r.r#type,
                batch_size_l: r.batch_size_l,
                source: r.source,
                created_at: r.created_at as i64,
                updated_at: r.updated_at as i64,
            })
        })
        .collect()
}
```

Also find all other `RecipeSummary { ... }` constructors in this file and add `source: r.source,` to each (search for `RecipeSummary {` — there may be one in a version restore path).

- [ ] **Step 4: Run tests to confirm they pass**

```bash
cargo test --manifest-path src-tauri/Cargo.toml test_list_excludes_seeded_recipes test_list_baseline_returns_only_seeded -- --nocapture 2>&1 | tail -10
```

Expected: both tests pass.

- [ ] **Step 5: Run full test suite**

```bash
just test
```

Expected: all tests pass.

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/repositories/recipe.rs
git commit -m "feat(recipes): filter list() to user recipes, add list_baseline()"
```

---

## Task 5: Tauri command + registration

**Files:**
- Modify: `src-tauri/src/commands/recipes.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Add the command**

In `src-tauri/src/commands/recipes.rs`, add after `list_recipes`:

```rust
#[tauri::command]
pub async fn list_baseline_recipes(
    state: State<'_, AppState>,
) -> Result<Vec<RecipeSummary>, AppError> {
    RecipeRepository::new(&state.db).list_baseline().await
}
```

- [ ] **Step 2: Register in lib.rs**

In `src-tauri/src/lib.rs`, add `commands::recipes::list_baseline_recipes` to the `tauri::generate_handler![]` macro, directly after `commands::recipes::list_recipes`.

- [ ] **Step 3: Verify compilation**

```bash
cargo build --manifest-path src-tauri/Cargo.toml
```

Expected: clean build.

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/commands/recipes.rs src-tauri/src/lib.rs
git commit -m "feat(commands): add list_baseline_recipes Tauri command"
```

---

## Task 6: Frontend — settings store and recipe stores

**Files:**
- Modify: `src/lib/stores/settings.ts`
- Modify: `src/lib/stores/recipes.ts`

- [ ] **Step 1: Add `starters_collapsed` to AppSettings**

In `src/lib/stores/settings.ts`, add the new field:

```typescript
export interface AppSettings {
  units?: "metric" | "imperial";
  theme?: string;
  default_equipment_profile_id?: string;
  last_route?: string;
  starters_collapsed?: boolean;
}
```

- [ ] **Step 2: Add baseline recipe store**

In `src/lib/stores/recipes.ts`:

```typescript
import { writable } from "svelte/store";
import type { RecipeSummary } from "$lib/api";
import { listRecipes, listBaselineRecipes } from "$lib/api";

export const recipeList = writable<RecipeSummary[]>([]);
export const baselineRecipeList = writable<RecipeSummary[]>([]);

export async function refreshRecipeList() {
  const list = await listRecipes();
  recipeList.set(list);
}

export async function refreshBaselineRecipeList() {
  const list = await listBaselineRecipes();
  baselineRecipeList.set(list);
}
```

- [ ] **Step 3: Verify TypeScript**

```bash
just check
```

Expected: no type errors.

- [ ] **Step 4: Commit**

```bash
git add src/lib/stores/settings.ts src/lib/stores/recipes.ts
git commit -m "feat(stores): add starters_collapsed setting and baseline recipe store"
```

---

## Task 7: Desktop — collapsible Starter Recipes section in RecipeList

**Files:**
- Modify: `src/lib/components/RecipeList.svelte`

- [ ] **Step 1: Write a component test for the collapse toggle**

In `src/lib/components/RecipeList.test.ts` (create if it doesn't exist, following patterns in existing `*.test.ts` files):

```typescript
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, fireEvent } from "@testing-library/svelte";
import RecipeList from "./RecipeList.svelte";

// Mock the API and stores as needed per the existing test patterns in the project
```

*Note:* Check existing component tests (e.g. in `src/lib/components/`) for the correct mock pattern for Svelte 5 + Vitest before writing this test. The test should verify:
1. When `starters_collapsed` is `false`, baseline recipe rows are visible.
2. Clicking the toggle calls `saveSetting('starters_collapsed', 'true')` and hides the rows.

- [ ] **Step 2: Implement the collapsible section**

Replace `src/lib/components/RecipeList.svelte` with:

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { recipeList, refreshRecipeList, baselineRecipeList, refreshBaselineRecipeList } from "$lib/stores/recipes";
  import { createRecipe, deleteRecipe, createRecipesFromBeerxml } from "$lib/api";
  import type { RecipeSummary } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { settings, saveSetting } from "$lib/stores/settings";
  import { type Units, lToGal, volumeLabel } from "$lib/units";

  let { selectedId = $bindable<string | null>(null) } = $props();
  let search = $state("");
  let fileInput: HTMLInputElement;

  const units = $derived<Units>($settings.units === "imperial" ? "imperial" : "metric");
  const startersCollapsed = $derived($settings.starters_collapsed ?? false);

  const filtered = $derived(
    search.trim()
      ? $recipeList.filter((r) => r.name.toLowerCase().includes(search.toLowerCase()))
      : $recipeList
  );

  onMount(() => {
    ipc(refreshRecipeList());
    ipc(refreshBaselineRecipeList());
  });

  async function handleNew() {
    const recipe = await ipc(createRecipe({ name: "New Recipe" }));
    if (!recipe) return;
    await ipc(refreshRecipeList());
    goto(`/recipe/${recipe.id}`);
  }

  async function handleDelete(id: string) {
    await ipc(deleteRecipe(id));
    await ipc(refreshRecipeList());
    if (selectedId === id) goto("/");
  }

  async function handleImport(event: Event) {
    const file = (event.target as HTMLInputElement).files?.[0];
    if (!file) return;
    const xml = await file.text();
    const imported = await ipc(createRecipesFromBeerxml(xml));
    if (!imported) return;
    await ipc(refreshRecipeList());
    fileInput.value = "";
  }

  function toggleStarters() {
    saveSetting("starters_collapsed", startersCollapsed ? "false" : "true");
  }
</script>

<aside class="w-56 flex flex-col flex-shrink-0 border-r overflow-hidden"
       style="background: var(--color-bg-surface); border-color: var(--color-border);">
  <!-- Search + New -->
  <div class="p-2 flex flex-col gap-1.5 border-b" style="border-color: var(--color-border);">
    <div class="relative">
      <svg class="absolute left-2 top-1/2 -translate-y-1/2 pointer-events-none" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" style="color: var(--color-text-muted);">
        <circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
      </svg>
      <input
        type="search"
        placeholder="Search recipes…"
        bind:value={search}
        class="w-full pl-7 pr-2.5 py-1.5 rounded text-sm outline-none"
        style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);"
      />
    </div>
    <button
      onclick={handleNew}
      class="w-full py-1.5 rounded text-sm font-medium transition-colors"
      style="background: var(--color-accent); color: #fff;"
    >
      + New Recipe
    </button>
    <input
      type="file"
      accept=".xml,.beerxml,text/xml,application/xml"
      bind:this={fileInput}
      onchange={handleImport}
      class="hidden"
    />
    <button
      onclick={() => fileInput.click()}
      class="w-full py-1.5 rounded text-sm font-medium transition-colors"
      style="border: 1px solid var(--color-accent); color: var(--color-accent); background: transparent;"
    >
      Import BeerXML
    </button>
  </div>

  <!-- User recipe list -->
  <ul class="flex-1 overflow-y-auto py-1">
    {#each filtered as recipe (recipe.id)}
      <li class="group relative">
        <a
          href="/recipe/{recipe.id}"
          class="flex flex-col px-3 py-2 pr-7 cursor-pointer transition-colors hover:bg-[var(--color-bg-elevated)]"
          style={selectedId === recipe.id
            ? "background: var(--color-bg-elevated); border-left: 2px solid var(--color-accent); padding-left: calc(0.75rem - 2px);"
            : "color: var(--color-text-primary); border-left: 2px solid transparent; padding-left: calc(0.75rem - 2px);"}
        >
          <span class="text-sm font-medium truncate" style="color: var(--color-text-primary);">{recipe.name}</span>
          <span class="text-xs truncate mt-0.5" style="color: var(--color-text-secondary);">
            {recipe.style_name ?? recipe.type_} · {(units === "imperial" ? lToGal(recipe.batch_size_l) : recipe.batch_size_l).toFixed(1)}{volumeLabel(units)}
          </span>
        </a>
        <button
          onclick={() => handleDelete(recipe.id)}
          aria-label="Delete recipe"
          class="absolute right-2 top-1/2 -translate-y-1/2 opacity-0 group-hover:opacity-100 transition-opacity text-sm leading-none"
          style="color: var(--color-text-muted);"
        >×</button>
      </li>
    {:else}
      <li class="px-3 py-6 text-center text-sm" style="color: var(--color-text-muted);">
        {search ? "No matches" : "No recipes yet"}
      </li>
    {/each}

    <!-- Starter Recipes section -->
    {#if $baselineRecipeList.length > 0}
      <li>
        <button
          onclick={toggleStarters}
          class="w-full flex items-center justify-between px-3 py-1.5 text-left"
          style="background: var(--color-bg-base); border-top: 1px solid var(--color-border); border-bottom: 1px solid var(--color-border);"
        >
          <span class="text-xs font-semibold uppercase tracking-wider" style="color: var(--color-text-muted);">
            Starter Recipes
          </span>
          <span class="text-xs" style="color: var(--color-text-muted);">
            {startersCollapsed ? "▸" : "▾"}
          </span>
        </button>
      </li>
      {#if !startersCollapsed}
        {#each $baselineRecipeList as recipe (recipe.id)}
          <li>
            <a
              href="/baseline-recipe/{recipe.id}"
              class="flex flex-col px-3 py-2 cursor-pointer transition-colors hover:bg-[var(--color-bg-elevated)]"
              style="border-left: 2px solid transparent; padding-left: calc(0.75rem - 2px); color: var(--color-text-secondary);"
            >
              <span class="text-sm font-medium truncate">{recipe.name}</span>
              <span class="text-xs truncate mt-0.5" style="color: var(--color-text-muted);">
                {recipe.style_name ?? recipe.type_} · {(units === "imperial" ? lToGal(recipe.batch_size_l) : recipe.batch_size_l).toFixed(1)}{volumeLabel(units)}
              </span>
            </a>
          </li>
        {/each}
      {/if}
    {/if}
  </ul>
</aside>
```

- [ ] **Step 3: Verify TypeScript**

```bash
just check
```

Expected: no errors.

- [ ] **Step 4: Commit**

```bash
git add src/lib/components/RecipeList.svelte
git commit -m "feat(desktop): add collapsible Starter Recipes section to recipe list"
```

---

## Task 8: Mobile — collapsible Starter Recipes section in RecipesHome

**Files:**
- Modify: `src/lib/mobile/RecipesHome.svelte`

- [ ] **Step 1: Implement the collapsible section**

Replace `src/lib/mobile/RecipesHome.svelte` with:

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { recipeList, refreshRecipeList, baselineRecipeList, refreshBaselineRecipeList } from "$lib/stores/recipes";
  import { createRecipe, createRecipesFromBeerxml } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { settings, saveSetting } from "$lib/stores/settings";
  import { type Units, lToGal, volumeLabel } from "$lib/units";

  let fileInput: HTMLInputElement;

  const units = $derived<Units>($settings.units === "imperial" ? "imperial" : "metric");
  const startersCollapsed = $derived($settings.starters_collapsed ?? false);

  onMount(() => {
    ipc(refreshRecipeList());
    ipc(refreshBaselineRecipeList());
  });

  async function handleNew() {
    const recipe = await ipc(createRecipe({ name: "New Recipe" }));
    if (!recipe) return;
    await ipc(refreshRecipeList());
    goto(`/recipe/${recipe.id}`);
  }

  async function handleImport(event: Event) {
    const file = (event.target as HTMLInputElement).files?.[0];
    if (!file) return;
    const xml = await file.text();
    const imported = await ipc(createRecipesFromBeerxml(xml));
    if (!imported) return;
    await ipc(refreshRecipeList());
    fileInput.value = "";
  }

  function toggleStarters() {
    saveSetting("starters_collapsed", startersCollapsed ? "false" : "true");
  }
</script>

<div class="flex flex-col h-full overflow-hidden" style="background: var(--color-bg-surface);">
  <div class="p-3 border-b flex flex-col gap-2" style="border-color: var(--color-border);">
    <button
      onclick={handleNew}
      class="w-full py-3 rounded text-sm font-medium"
      style="background: var(--color-accent); color: #fff;"
    >+ New Recipe</button>
    <input
      type="file"
      accept=".xml,.beerxml,text/xml,application/xml"
      bind:this={fileInput}
      onchange={handleImport}
      class="hidden"
    />
    <button
      onclick={() => fileInput.click()}
      class="w-full py-3 rounded text-sm font-medium"
      style="border: 1px solid var(--color-accent); color: var(--color-accent); background: transparent;"
    >Import BeerXML</button>
  </div>

  <div class="flex-1 overflow-y-auto">
    <!-- User recipes -->
    {#each $recipeList as recipe (recipe.id)}
      <a
        href="/recipe/{recipe.id}"
        class="flex items-center justify-between px-4 py-3 border-b text-sm"
        style="border-color: var(--color-border); color: var(--color-text-primary);"
      >
        <span class="truncate">{recipe.name}</span>
        <span style="color: var(--color-text-muted);">›</span>
      </a>
    {:else}
      <p class="p-4 text-sm" style="color: var(--color-text-muted);">No recipes yet. Tap + to create one.</p>
    {/each}

    <!-- Starter Recipes section -->
    {#if $baselineRecipeList.length > 0}
      <button
        onclick={toggleStarters}
        class="w-full flex items-center justify-between px-4 py-2 border-b"
        style="background: var(--color-bg-base); border-color: var(--color-border);"
      >
        <span class="text-xs font-semibold uppercase tracking-wider" style="color: var(--color-text-muted);">
          Starter Recipes
        </span>
        <span class="text-xs" style="color: var(--color-text-muted);">
          {startersCollapsed ? "▸" : "▾"}
        </span>
      </button>
      {#if !startersCollapsed}
        {#each $baselineRecipeList as recipe (recipe.id)}
          <a
            href="/baseline-recipe/{recipe.id}"
            class="flex items-center justify-between px-4 py-3 border-b text-sm"
            style="border-color: var(--color-border); color: var(--color-text-secondary);"
          >
            <span class="truncate">{recipe.name}</span>
            <span style="color: var(--color-text-muted);">›</span>
          </a>
        {/each}
      {/if}
    {/if}
  </div>
</div>
```

- [ ] **Step 2: Verify TypeScript**

```bash
just check
```

Expected: no errors.

- [ ] **Step 3: Commit**

```bash
git add src/lib/mobile/RecipesHome.svelte
git commit -m "feat(mobile): add collapsible Starter Recipes section to recipes screen"
```

---

## Task 9: Desktop — BaselineRecipeView component

**Files:**
- Create: `src/lib/desktop/BaselineRecipeView.svelte`

- [ ] **Step 1: Create the component**

The desktop `BaselineRecipeView` mirrors the structure of `RecipeView.svelte` but strips editing: no inline field editing, no version history, no Batches tab, no delete. It adds a "Clone to My Recipes" button that calls `create_recipe` with `source_id`.

Create `src/lib/desktop/BaselineRecipeView.svelte`:

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { getRecipe, getRecipeStats, createRecipe } from "$lib/api";
  import type { Recipe, RecipeStats } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import RecipeList from "$lib/components/RecipeList.svelte";
  import StatsSidebar from "$lib/components/StatsSidebar.svelte";
  import OverviewTab from "$lib/components/tabs/OverviewTab.svelte";
  import IngredientsTab from "$lib/components/tabs/IngredientsTab.svelte";
  import MashTab from "$lib/components/tabs/MashTab.svelte";
  import WaterTab from "$lib/components/tabs/WaterTab.svelte";
  import FermentationTab from "$lib/components/tabs/FermentationTab.svelte";
  import NotesTab from "$lib/components/tabs/NotesTab.svelte";
  import TabBar from "$lib/components/TabBar.svelte";
  import type { BrewingIconName } from "$lib/icons";

  let { id }: { id: string } = $props();

  let recipe = $state<Recipe | null>(null);
  let stats = $state<RecipeStats | null>(null);
  let activeTab = $state<"overview" | "ingredients" | "mash" | "water" | "fermentation" | "notes">("overview");
  let cloning = $state(false);

  onMount(async () => {
    recipe = await ipc(getRecipe(id)) ?? null;
    if (recipe) stats = await ipc(getRecipeStats(id)) ?? null;
  });

  async function handleClone() {
    if (!recipe || cloning) return;
    cloning = true;
    const cloned = await ipc(createRecipe({
      name: recipe.name,
      source_id: recipe.id,
      type_: recipe.type_,
      batch_size_l: recipe.batch_size_l,
      boil_size_l: recipe.boil_size_l,
      boil_time_min: recipe.boil_time_min,
      efficiency_pct: recipe.efficiency_pct ?? undefined,
      notes: recipe.notes ?? undefined,
    }));
    cloning = false;
    if (!cloned) return;
    goto(`/recipe/${cloned.id}`);
  }

  const tabs: { id: typeof activeTab; label: string; icon: BrewingIconName }[] = [
    { id: "overview", label: "Overview", icon: "recipe" },
    { id: "ingredients", label: "Ingredients", icon: "fermentable" },
    { id: "mash", label: "Mash", icon: "mash" },
    { id: "water", label: "Water", icon: "water" },
    { id: "fermentation", label: "Fermentation", icon: "yeast" },
    { id: "notes", label: "Notes", icon: "notes" },
  ];
</script>

<RecipeList />

{#if recipe}
  <div class="flex flex-1 overflow-hidden">
    <!-- Main content -->
    <div class="flex flex-col flex-1 overflow-hidden">
      <!-- Header -->
      <div class="flex items-center justify-between px-4 py-3 border-b flex-shrink-0"
           style="border-color: var(--color-border); background: var(--color-bg-surface);">
        <div>
          <h1 class="text-lg font-semibold" style="color: var(--color-text-primary);">{recipe.name}</h1>
          <p class="text-xs mt-0.5" style="color: var(--color-text-muted);">Starter Recipe — read only</p>
        </div>
        <button
          onclick={handleClone}
          disabled={cloning}
          class="px-4 py-1.5 rounded text-sm font-semibold transition-colors disabled:opacity-50"
          style="background: var(--color-accent); color: #fff;"
        >
          {cloning ? "Cloning…" : "Clone to My Recipes"}
        </button>
      </div>

      <!-- Tabs -->
      <TabBar {tabs} bind:activeTab />

      <!-- Tab content -->
      <div class="flex-1 overflow-y-auto p-4">
        {#if activeTab === "overview"}
          <OverviewTab {recipe} {stats} readonly />
        {:else if activeTab === "ingredients"}
          <IngredientsTab {recipe} {stats} readonly />
        {:else if activeTab === "mash"}
          <MashTab {recipe} readonly />
        {:else if activeTab === "water"}
          <WaterTab {recipe} readonly />
        {:else if activeTab === "fermentation"}
          <FermentationTab {recipe} readonly />
        {:else if activeTab === "notes"}
          <NotesTab {recipe} readonly />
        {/if}
      </div>
    </div>

    <!-- Stats sidebar -->
    {#if stats}
      <StatsSidebar {recipe} {stats} />
    {/if}
  </div>
{:else}
  <div class="flex-1 flex items-center justify-center" style="color: var(--color-text-muted);">
    <p class="text-sm">Loading…</p>
  </div>
{/if}
```

> **Note:** The tab components (OverviewTab, IngredientsTab, etc.) likely do not currently accept a `readonly` prop — they will ignore unknown props harmlessly in Svelte 5. If any tab shows edit controls that should be hidden, add a `readonly` prop to those components in a follow-up pass. Do not block this task on that.

- [ ] **Step 2: Verify TypeScript**

```bash
just check
```

Expected: no errors (or only warnings about unused `readonly` props if tab components don't declare it yet).

- [ ] **Step 3: Commit**

```bash
git add src/lib/desktop/BaselineRecipeView.svelte
git commit -m "feat(desktop): add read-only BaselineRecipeView with clone action"
```

---

## Task 10: Mobile — BaselineRecipeView component

**Files:**
- Create: `src/lib/mobile/BaselineRecipeView.svelte`

- [ ] **Step 1: Create the component**

Check how `src/lib/mobile/RecipeView.svelte` is structured (single-column scroll, tab bar at bottom or top, no sidebar) and mirror it for read-only display. Create `src/lib/mobile/BaselineRecipeView.svelte`:

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { getRecipe, getRecipeStats, createRecipe } from "$lib/api";
  import type { Recipe, RecipeStats } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import OverviewTab from "$lib/components/tabs/OverviewTab.svelte";
  import IngredientsTab from "$lib/components/tabs/IngredientsTab.svelte";
  import MashTab from "$lib/components/tabs/MashTab.svelte";
  import WaterTab from "$lib/components/tabs/WaterTab.svelte";
  import FermentationTab from "$lib/components/tabs/FermentationTab.svelte";
  import NotesTab from "$lib/components/tabs/NotesTab.svelte";
  import TabBar from "$lib/components/TabBar.svelte";
  import type { BrewingIconName } from "$lib/icons";

  let { id }: { id: string } = $props();

  let recipe = $state<Recipe | null>(null);
  let stats = $state<RecipeStats | null>(null);
  let activeTab = $state<"overview" | "ingredients" | "mash" | "water" | "fermentation" | "notes">("overview");
  let cloning = $state(false);

  onMount(async () => {
    recipe = await ipc(getRecipe(id)) ?? null;
    if (recipe) stats = await ipc(getRecipeStats(id)) ?? null;
  });

  async function handleClone() {
    if (!recipe || cloning) return;
    cloning = true;
    const cloned = await ipc(createRecipe({
      name: recipe.name,
      source_id: recipe.id,
      type_: recipe.type_,
      batch_size_l: recipe.batch_size_l,
      boil_size_l: recipe.boil_size_l,
      boil_time_min: recipe.boil_time_min,
      efficiency_pct: recipe.efficiency_pct ?? undefined,
      notes: recipe.notes ?? undefined,
    }));
    cloning = false;
    if (!cloned) return;
    goto(`/recipe/${cloned.id}`);
  }

  const tabs: { id: typeof activeTab; label: string; icon: BrewingIconName }[] = [
    { id: "overview", label: "Overview", icon: "recipe" },
    { id: "ingredients", label: "Ingredients", icon: "fermentable" },
    { id: "mash", label: "Mash", icon: "mash" },
    { id: "water", label: "Water", icon: "water" },
    { id: "fermentation", label: "Fermentation", icon: "yeast" },
    { id: "notes", label: "Notes", icon: "notes" },
  ];
</script>

<div class="flex flex-col h-full overflow-hidden" style="background: var(--color-bg-base);">
  {#if recipe}
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b flex-shrink-0"
         style="border-color: var(--color-border); background: var(--color-bg-surface);">
      <div class="flex-1 min-w-0 mr-3">
        <h1 class="text-base font-semibold truncate" style="color: var(--color-text-primary);">{recipe.name}</h1>
        <p class="text-xs" style="color: var(--color-text-muted);">Starter Recipe</p>
      </div>
      <button
        onclick={handleClone}
        disabled={cloning}
        class="px-3 py-2 rounded text-sm font-semibold flex-shrink-0 disabled:opacity-50"
        style="background: var(--color-accent); color: #fff; min-height: 44px;"
      >
        {cloning ? "Cloning…" : "Clone"}
      </button>
    </div>

    <!-- Tabs -->
    <TabBar {tabs} bind:activeTab />

    <!-- Tab content -->
    <div class="flex-1 overflow-y-auto p-4">
      {#if activeTab === "overview"}
        <OverviewTab {recipe} {stats} readonly />
      {:else if activeTab === "ingredients"}
        <IngredientsTab {recipe} {stats} readonly />
      {:else if activeTab === "mash"}
        <MashTab {recipe} readonly />
      {:else if activeTab === "water"}
        <WaterTab {recipe} readonly />
      {:else if activeTab === "fermentation"}
        <FermentationTab {recipe} readonly />
      {:else if activeTab === "notes"}
        <NotesTab {recipe} readonly />
      {/if}
    </div>
  {:else}
    <div class="flex-1 flex items-center justify-center" style="color: var(--color-text-muted);">
      <p class="text-sm">Loading…</p>
    </div>
  {/if}
</div>
```

- [ ] **Step 2: Verify TypeScript**

```bash
just check
```

Expected: no errors.

- [ ] **Step 3: Commit**

```bash
git add src/lib/mobile/BaselineRecipeView.svelte
git commit -m "feat(mobile): add read-only BaselineRecipeView with clone action"
```

---

## Task 11: Route — /baseline-recipe/[id]

**Files:**
- Create: `src/routes/baseline-recipe/[id]/+page.ts`
- Create: `src/routes/baseline-recipe/[id]/+page.svelte`

- [ ] **Step 1: Create the load function**

Create `src/routes/baseline-recipe/[id]/+page.ts`:

```typescript
import type { PageLoad } from "./$types";

export const load: PageLoad = ({ params }) => {
  return { id: params.id };
};
```

- [ ] **Step 2: Create the page**

Create `src/routes/baseline-recipe/[id]/+page.svelte`:

```svelte
<script lang="ts">
  import type { PageData } from "./$types";
  import BaselineRecipeView from "$platform/BaselineRecipeView.svelte";

  let { data }: { data: PageData } = $props();
</script>

<BaselineRecipeView id={data.id} />
```

- [ ] **Step 3: Verify TypeScript and run tests**

```bash
just check && just test
```

Expected: all pass.

- [ ] **Step 4: Commit**

```bash
git add src/routes/baseline-recipe/
git commit -m "feat(routes): add /baseline-recipe/[id] route"
```

---

## Task 12: Smoke test the full feature

- [ ] **Step 1: Start the dev server**

```bash
just dev-web
```

Open http://localhost:1420.

- [ ] **Step 2: Verify starter recipes appear**

Navigate to the Recipes screen. The "Starter Recipes" section should appear below any user recipes (or the empty state message) with all 5 starters: Pliny the Elder, Heady Topper, Julius, Guinness Draught, Saison Dupont.

- [ ] **Step 3: Verify collapse persists**

Click the "Starter Recipes" toggle to collapse it. Reload the page. The section should still be collapsed.

- [ ] **Step 4: Verify baseline recipe view**

Click any starter recipe. Confirm:
- The recipe loads with correct name, style, tabs, and stats.
- No edit controls are visible.
- "Clone to My Recipes" button is present in the header.

- [ ] **Step 5: Verify clone**

Click "Clone to My Recipes". Confirm:
- You are redirected to `/recipe/<new-id>`.
- The new recipe appears in the user's recipe list with the same name.
- The starter recipe still appears in the Starter Recipes section.

- [ ] **Step 6: Final commit**

```bash
git add -A
git commit -m "feat(starter-recipes): smoke test verified — feature complete"
```
