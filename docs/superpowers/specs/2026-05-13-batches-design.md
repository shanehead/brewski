# Batches & Recipe Versioning Infrastructure — Design

**Date:** 2026-05-13  
**Status:** Approved

## Overview

Implement a Batches module that lets users record brew sessions against their recipes, track actual measurements (gravities, volumes, dates), log fermentation gravity readings over time, and capture tasting notes. Batches are backed by a recipe versioning infrastructure that snapshots the full recipe at brew time, giving each batch a stable, queryable reference to exactly what was brewed.

Recipe versioning UI (browsing history, diffing versions, restoring old versions) is explicitly out of scope for this feature. The versioning schema is built here so it can support that UI later.

---

## Data Model

### Recipe Versioning Infrastructure

When a batch is created, the current recipe state is compared against the most recent existing version. If nothing has changed, the new batch reuses that version. If anything differs, a new version is created (version_number increments by 1). This ensures version_number reflects meaningful recipe changes, not every brew.

**`recipe_versions`** — core recipe scalars at snapshot time:
```sql
CREATE TABLE recipe_versions (
  id                  TEXT PRIMARY KEY,
  recipe_id           TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
  version_number      INTEGER NOT NULL,
  name                TEXT,                    -- user-defined label (future versioning UI)
  type                TEXT NOT NULL,
  brewer              TEXT,
  batch_size_l        REAL NOT NULL,
  boil_size_l         REAL NOT NULL,
  boil_time_min       REAL NOT NULL,
  efficiency_pct      REAL,
  style_id            TEXT,
  mash_water_id       TEXT,
  sparge_water_id     TEXT,
  notes               TEXT,
  og                  REAL,
  fg                  REAL,
  primary_age_days    REAL,
  primary_temp_c      REAL,
  secondary_age_days  REAL,
  secondary_temp_c    REAL,
  carbonation_vols    REAL,
  created_at          INTEGER NOT NULL,
  UNIQUE(recipe_id, version_number)
);
```

**`recipe_version_fermentables`** — mirrors `recipe_addition_fermentables`:
```sql
CREATE TABLE recipe_version_fermentables (
  id                  TEXT PRIMARY KEY,
  recipe_version_id   TEXT NOT NULL REFERENCES recipe_versions(id) ON DELETE CASCADE,
  fermentable_id      TEXT REFERENCES fermentables(id),
  name                TEXT NOT NULL,
  type                TEXT NOT NULL,
  yield_pct           REAL NOT NULL,
  color_lovibond      REAL NOT NULL,
  amount_kg           REAL NOT NULL,
  add_after_boil      INTEGER DEFAULT 0,
  addition_order      INTEGER NOT NULL DEFAULT 0
);
```

**`recipe_version_hops`** — mirrors `recipe_addition_hops`:
```sql
CREATE TABLE recipe_version_hops (
  id                  TEXT PRIMARY KEY,
  recipe_version_id   TEXT NOT NULL REFERENCES recipe_versions(id) ON DELETE CASCADE,
  hop_id              TEXT REFERENCES hops(id),
  name                TEXT NOT NULL,
  alpha_pct           REAL NOT NULL,
  form                TEXT NOT NULL DEFAULT 'pellet',
  amount_kg           REAL NOT NULL,
  use                 TEXT NOT NULL,
  time_min            REAL NOT NULL,
  addition_order      INTEGER NOT NULL DEFAULT 0
);
```

**`recipe_version_yeasts`** — mirrors `recipe_addition_yeasts`:
```sql
CREATE TABLE recipe_version_yeasts (
  id                  TEXT PRIMARY KEY,
  recipe_version_id   TEXT NOT NULL REFERENCES recipe_versions(id) ON DELETE CASCADE,
  yeast_id            TEXT REFERENCES yeasts(id),
  name                TEXT NOT NULL,
  type                TEXT NOT NULL,
  form                TEXT NOT NULL,
  laboratory          TEXT,
  product_id          TEXT,
  attenuation_pct     REAL,
  amount              REAL,
  amount_is_weight    INTEGER DEFAULT 0,
  add_to_secondary    INTEGER DEFAULT 0,
  times_cultured      INTEGER DEFAULT 0
);
```

**`recipe_version_miscs`** — mirrors `recipe_addition_miscs`:
```sql
CREATE TABLE recipe_version_miscs (
  id                  TEXT PRIMARY KEY,
  recipe_version_id   TEXT NOT NULL REFERENCES recipe_versions(id) ON DELETE CASCADE,
  misc_id             TEXT REFERENCES miscs(id),
  name                TEXT NOT NULL,
  type                TEXT NOT NULL,
  use                 TEXT NOT NULL,
  amount              REAL NOT NULL,
  amount_is_weight    INTEGER DEFAULT 0,
  time_min            REAL NOT NULL,
  addition_order      INTEGER NOT NULL DEFAULT 0
);
```

**`recipe_version_waters`** — mirrors `recipe_addition_waters`:
```sql
CREATE TABLE recipe_version_waters (
  id                  TEXT PRIMARY KEY,
  recipe_version_id   TEXT NOT NULL REFERENCES recipe_versions(id) ON DELETE CASCADE,
  water_id            TEXT REFERENCES waters(id),
  name                TEXT NOT NULL,
  amount_l            REAL NOT NULL
);
```

**`recipe_version_water_adjustments`** — mirrors `recipe_water_adjustments`:
```sql
CREATE TABLE recipe_version_water_adjustments (
  id                  TEXT PRIMARY KEY,
  recipe_version_id   TEXT NOT NULL REFERENCES recipe_versions(id) ON DELETE CASCADE,
  addition            TEXT NOT NULL,
  target              TEXT NOT NULL,
  amount              REAL NOT NULL
);
```

**`recipe_version_mash`** — mirrors `mashes`:
```sql
CREATE TABLE recipe_version_mash (
  id                  TEXT PRIMARY KEY,
  recipe_version_id   TEXT NOT NULL UNIQUE REFERENCES recipe_versions(id) ON DELETE CASCADE,
  name                TEXT NOT NULL DEFAULT 'Single Infusion',
  grain_temp_c        REAL NOT NULL DEFAULT 21,
  tun_temp_c          REAL,
  sparge_temp_c       REAL,
  ph                  REAL,
  notes               TEXT,
  ratio_l_per_kg      REAL
);
```

**`recipe_version_mash_steps`** — mirrors `mash_steps`:
```sql
CREATE TABLE recipe_version_mash_steps (
  id                      TEXT PRIMARY KEY,
  recipe_version_mash_id  TEXT NOT NULL REFERENCES recipe_version_mash(id) ON DELETE CASCADE,
  name                    TEXT NOT NULL,
  type                    TEXT NOT NULL DEFAULT 'infusion',
  infuse_amount_l         REAL,
  step_temp_c             REAL NOT NULL,
  step_time_min           INTEGER NOT NULL,
  ramp_time_min           INTEGER,
  end_temp_c              REAL,
  step_order              INTEGER NOT NULL
);
```

### Batches

```sql
CREATE TABLE batches (
  id                        TEXT PRIMARY KEY,
  recipe_id                 TEXT NOT NULL REFERENCES recipes(id),
  recipe_version_id         TEXT NOT NULL REFERENCES recipe_versions(id),
  name                      TEXT,
  status                    TEXT NOT NULL DEFAULT 'planned',
  brew_date                 INTEGER,
  fermenter_date            INTEGER,
  packaging_date            INTEGER,
  actual_pre_boil_volume_l  REAL,
  actual_post_boil_volume_l REAL,
  actual_batch_size_l       REAL,
  actual_pre_boil_gravity   REAL,
  actual_og                 REAL,
  actual_fg                 REAL,
  brew_day_notes            TEXT,
  fermentation_notes        TEXT,
  tasting_notes             TEXT,
  rating                    INTEGER,    -- 1–10
  created_at                INTEGER NOT NULL,
  updated_at                INTEGER NOT NULL
);

CREATE TABLE batch_gravity_readings (
  id          TEXT PRIMARY KEY,
  batch_id    TEXT NOT NULL REFERENCES batches(id) ON DELETE CASCADE,
  recorded_at INTEGER NOT NULL,
  gravity     REAL NOT NULL,
  temp_c      REAL,
  notes       TEXT
);
```

**Batch status lifecycle:** `planned → brewing → fermenting → packaged → complete`

Status is set manually. No automatic transitions.

---

## Recipe Version Deduplication

`repositories/recipe_version.rs` exposes a single entry point:

```
create_or_reuse_version(recipe_id, pool) -> Result<RecipeVersion>
```

Logic:
1. Load the most recent version for this recipe (by `version_number DESC`).
2. Load the current live recipe scalars and all additions/mash.
3. Deep-compare the two. If identical in every field, return the existing version.
4. Otherwise, insert a new `recipe_versions` row (`version_number = previous + 1`), copy all addition and mash rows into the version-scoped tables, and return the new version.

The comparison covers: recipe scalars, fermentables, hops, yeasts, miscs, waters, mash profile, and mash steps. Order of additions is included in the comparison (via `addition_order`).

---

## Tauri Command API

New module: `src-tauri/src/commands/batches.rs`

### Batch commands
| Command | Description |
|---|---|
| `create_batch(recipe_id, name?)` | Runs dedup snapshot, creates batch with status `planned`, returns `Batch` |
| `list_batches()` | All batches sorted by `brew_date DESC`, nulls first |
| `list_batches_for_recipe(recipe_id)` | Filtered batch list for recipe's Batches tab |
| `get_batch(id)` | Full batch including gravity readings |
| `update_batch(id, updates)` | Partial update: status, dates, volumes, gravities, notes, rating |
| `delete_batch(id)` | Cascades to gravity readings |

### Gravity reading commands
| Command | Description |
|---|---|
| `add_gravity_reading(batch_id, recorded_at, gravity, temp_c?, notes?)` | Appends a reading |
| `delete_gravity_reading(id)` | Removes a reading |

### Recipe version commands
| Command | Description |
|---|---|
| `list_recipe_versions(recipe_id)` | Version list with `version_number`, `name`, `created_at` |
| `get_recipe_version(id)` | Full snapshot with all ingredients and mash steps |

---

## UI Structure

### Navigation changes
- Add a **Batches icon** to the icon rail between Recipes and Tools.
- Add a **"Batches" tab** as the 6th tab in the recipe editor tab bar.

### Top-level Batches section (`/batches`)
- Flat list of all batches, sorted by `brew_date DESC` (planned/null brew_date at top).
- Each row: recipe name, batch name or auto-label ("Batch #3"), brew date, status badge, OG → FG if available.
- "New Batch" button opens a recipe picker modal.
- Architecture: flat list now; search/filter/grouping can be layered on without restructuring.

### Recipe editor — Batches tab
- Filtered list of this recipe's batches (same row component as above).
- "Brew this Recipe" button: runs `create_batch`, then navigates to the new batch detail.
- Summary header: total batch count, average rating.

### Batch detail (`/batches/[id]`) — 4 tabs

**Overview tab**
- Status selector (segmented control or dropdown).
- Key dates: brew date, into-fermenter date, packaging date.
- Actual vs. target stat pairs: pre-boil volume, post-boil volume, batch size, pre-boil gravity, OG, FG, calculated actual ABV.
- Fields are visually emphasized based on current status (OG prominent during `brewing`, FG during `fermenting`).

**Gravity Log tab**
- Table of readings: date/time, gravity, temp, notes. Sorted by `recorded_at`.
- Inline "Add Reading" form at the bottom.
- Future: sparkline chart above the table.

**Notes tab**
- Two labelled text areas: **Brew Day Notes** and **Fermentation Notes**.
- Auto-saves on blur (consistent with recipe notes tab behavior).

**Tasting tab**
- Tasting notes text area.
- 1–10 numeric rating input.

---

## Frontend File Structure

```
src/routes/batches/
  +page.svelte              # Batch list (top-level)
  +page.ts
  [id]/
    +page.svelte            # Batch detail (4 tabs)
    +page.ts

src/lib/components/
  BatchList.svelte          # Shared list used in /batches and recipe Batches tab
  batch/
    BatchOverviewTab.svelte
    BatchGravityTab.svelte
    BatchNotesTab.svelte
    BatchTastingTab.svelte

src/lib/stores/
  batches.ts                # Batch list store (mirrors recipes.ts)
```

All Tauri calls go through `src/lib/api.ts` — no component calls `invoke()` directly.

---

## Testing

- Integration tests in `src-tauri/src/commands/batches.rs` using in-memory SQLite (matching existing test patterns).
- Cover: create batch (new version created), create second batch with unchanged recipe (version reused), create batch after recipe change (new version created), update batch fields, add/delete gravity readings, delete batch cascades readings.
