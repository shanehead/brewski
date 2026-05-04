# Brewski — Design Spec

**Date:** 2026-05-03  
**Status:** Draft — awaiting user approval  
**Scope:** Walking skeleton v1 — Recipes module only

---

## Overview

Brewski is a free, open-source desktop homebrewing application built with Rust and Tauri. It targets homebrewers who want a native, offline-first alternative to web apps like Brewfather. v1 delivers a fully functional recipe editor. Batches, inventory, water profiles, and other modules follow in later iterations.

---

## Tech Stack

| Layer | Technology | Notes |
|---|---|---|
| Desktop shell | Tauri 2 | Cross-platform (macOS first, no one-way doors) |
| Backend language | Rust | All business logic and calculations |
| Database | SQLite via `sqlx` | Single local `.db` file, embedded |
| Frontend framework | Svelte 5 | Pure UI layer — no business logic |
| Styling | Tailwind CSS v4 | CSS custom properties for theming |
| Component primitives | bits-ui / melt-ui | Svelte-native headless components |

SpaceUI (`@spacedrive/primitives`) was evaluated but excluded — it has hard React peer dependencies. The Spacedrive dark aesthetic is replicated via Tailwind CSS and CSS custom properties.

---

## Architecture

**Principle:** all logic lives in Rust. Svelte calls `invoke()`, receives data, renders it. No brewing math, no validation, no business rules in TypeScript.

```
src-tauri/src/
  commands/         ← thin Tauri handlers; delegate to db/ or brewing/
    recipes.rs
    additions.rs
    mash.rs
    library.rs
    equipment.rs
    settings.rs
    import_export.rs
  db/               ← sqlx queries, one file per entity
    recipes.rs
    additions.rs
    mash.rs
    library.rs
    equipment.rs
    settings.rs
    migrations/
      001_initial.sql
  brewing/          ← pure calculation functions; no DB access; fully unit-testable
    og.rs           ← original gravity
    ibu.rs          ← bitterness (Tinseth formula)
    srm.rs          ← color (Morey formula)
    abv.rs          ← alcohol by volume
    volumes.rs      ← pre/post-boil volume calculations
    mod.rs          ← calculate_stats() orchestrates all of the above
  models.rs         ← all Rust structs with serde derives
  lib.rs
  main.rs

> **Future extraction path:** `brewing/` (pure functions) and `models.rs` are already suitable for a standalone `brewski-core` crate. When a second consumer (CLI, library) exists, convert to a Cargo workspace: `brewski-core` (models + calculations), `brewski-db` (sqlx queries), `brewski-tauri` (Tauri commands). No restructuring needed for v1.

src/                ← Svelte frontend
  lib/
    components/     ← UI components
    stores/         ← Svelte stores (recipe state, settings)
    api.ts          ← typed wrappers around invoke() calls
    theme.ts        ← theme loader
  routes/
    +page.svelte    ← recipe list
    recipe/
      [id]/
        +page.svelte  ← recipe editor
  themes/
    midnight.css    ← default dark theme
    amber.css       ← future warm theme
```

---

## Theming

Colors are CSS custom properties on `:root`. Switching themes at runtime requires no rebuild — Svelte injects the appropriate theme CSS file on load based on the `theme` setting read from SQLite.

```css
/* themes/midnight.css */
:root {
  --color-bg-base:      #0d0d12;
  --color-bg-surface:   #13131c;
  --color-bg-elevated:  #1a1a2a;
  --color-border:       #1e1e2e;
  --color-accent:       #5c5cff;
  --color-accent-hover: #7c7cff;
  --color-text-primary: #f0f0f5;
  --color-text-secondary: #888;
  --color-text-muted:   #444;
}
```

New themes require only a new CSS file — zero Rust changes.

---

## UI Layout

### App shell

```
┌──────────────────────────────────────────────────────┐
│ Rail │ Recipe List (220px) │ Editor (flex: 1)        │
│  48  │                     │                         │
│  px  │  Search             │  ← Recipes  Recipe Name │
│      │  + New Recipe       │  ─────────────────────  │
│  ●   │                     │  Overview │ Ingredients │
│  ○   │  Midnight Oat Stout │  Mash │ Fermentation    │
│  ○   │  Pacific Haze IPA   │  Notes                  │
│      │  Belgian Tripel     │                         │
│      │  ...                │  [content]   │  Stats   │
│  ○   │                     │              │  sidebar │
│  ⚙   │                     │              │  160px   │
└──────────────────────────────────────────────────────┘
```

- **Icon rail (48px):** Recipes icon (active in v1), Settings icon at bottom. Future modules (Batches, Inventory) add icons here.
- **Recipe list panel (220px):** Search bar, New Recipe button, scrollable list of `RecipeSummary` rows.
- **Recipe editor (full-screen):** Clicking a recipe navigates to the full-screen editor. Back button returns to list.
- **Tab bar:** Overview · Ingredients · Mash · Fermentation · Notes
- **Stats sidebar (160px):** OG, FG, ABV, IBU, SRM, BU:GU, Calories — persists across all tabs, recalculated on every ingredient change.

### Recipe editor tabs

| Tab | Content |
|---|---|
| Overview | Name, style, type (all-grain/extract/partial mash), batch size, equipment profile, brewer, dates |
| Ingredients | Fermentables table, Hops table, Yeasts table, Miscs table, Waters table — each with Add button |
| Mash | Mash profile name, grain temp, sparge temp, pH, step list with drag-to-reorder |
| Fermentation | Primary/secondary/tertiary age + temp, carbonation, packaging |
| Notes | Recipe notes (textarea), taste notes, taste rating |

---

## Data Model

All values stored in SI units with unit suffixes: `_l` (litres), `_kg` (kilograms), `_c` (Celsius), `_pct` (percent), `_min` (minutes), `_ppm` (parts per million). Schema is BeerXML 1.0 aligned.

### Styles
```sql
CREATE TABLE styles (
  id               TEXT PRIMARY KEY,
  name             TEXT NOT NULL,
  category         TEXT NOT NULL,
  category_number  TEXT NOT NULL,
  style_letter     TEXT NOT NULL,
  style_guide      TEXT NOT NULL,
  type             TEXT NOT NULL,
  og_min           REAL NOT NULL,
  og_max           REAL NOT NULL,
  fg_min           REAL NOT NULL,
  fg_max           REAL NOT NULL,
  ibu_min          REAL NOT NULL,
  ibu_max          REAL NOT NULL,
  color_min_srm    REAL NOT NULL,
  color_max_srm    REAL NOT NULL,
  carb_min_vols    REAL,
  carb_max_vols    REAL,
  abv_min_pct      REAL,
  abv_max_pct      REAL,
  notes            TEXT,
  profile          TEXT,
  ingredients      TEXT,
  examples         TEXT
);
```

### Equipment profiles

`boil_size_l` and `boil_time_min` appear on both `equipment_profiles` (defaults for the kit) and `recipes` (the actual values used for that recipe). When a new recipe is created with an equipment profile, these fields are pre-filled from the profile but are then independently editable on the recipe. Recipe values always take precedence for all calculations.

```sql
CREATE TABLE equipment_profiles (
  id                    TEXT PRIMARY KEY,
  name                  TEXT NOT NULL,
  notes                 TEXT,
  boil_size_l           REAL NOT NULL,
  batch_size_l          REAL NOT NULL,
  calc_boil_volume      INTEGER NOT NULL DEFAULT 1,
  tun_volume_l          REAL,
  tun_weight_kg         REAL,
  tun_specific_heat     REAL,
  lauter_deadspace_l    REAL DEFAULT 0,
  top_up_kettle_l       REAL DEFAULT 0,
  trub_chiller_loss_l   REAL DEFAULT 0,
  evap_rate_pct_hr      REAL DEFAULT 10,
  boil_time_min         REAL NOT NULL DEFAULT 60,
  top_up_water_l        REAL DEFAULT 0,
  fermenter_loss_l      REAL DEFAULT 0,
  hop_utilization_pct   REAL DEFAULT 100,
  efficiency_pct        REAL NOT NULL DEFAULT 72,
  created_at            INTEGER NOT NULL,
  updated_at            INTEGER NOT NULL
);
```

### Ingredient library (reusable named entities)
```sql
CREATE TABLE fermentables (
  id                        TEXT PRIMARY KEY,
  name                      TEXT NOT NULL,
  type                      TEXT NOT NULL,
  yield_pct                 REAL NOT NULL,
  color_lovibond            REAL NOT NULL,
  origin                    TEXT,
  supplier                  TEXT,
  notes                     TEXT,
  add_after_boil            INTEGER DEFAULT 0,
  coarse_fine_diff_pct      REAL,
  moisture_pct              REAL,
  diastatic_power_lintner   REAL,
  protein_pct               REAL,
  max_in_batch_pct          REAL,
  recommend_mash            INTEGER,
  ibu_gal_per_lb            REAL
);

CREATE TABLE hops (
  id                  TEXT PRIMARY KEY,
  name                TEXT NOT NULL,
  alpha_pct           REAL NOT NULL,
  beta_pct            REAL,
  form                TEXT NOT NULL DEFAULT 'pellet',
  type                TEXT,
  origin              TEXT,
  year                TEXT,
  notes               TEXT,
  substitutes         TEXT,
  hsi_pct             REAL,
  humulene_pct        REAL,
  caryophyllene_pct   REAL,
  cohumulone_pct      REAL,
  myrcene_pct         REAL
);

CREATE TABLE yeasts (
  id                TEXT PRIMARY KEY,
  name              TEXT NOT NULL,
  type              TEXT NOT NULL,
  form              TEXT NOT NULL,
  laboratory        TEXT,
  product_id        TEXT,
  min_temperature_c REAL,
  max_temperature_c REAL,
  flocculation      TEXT,
  attenuation_pct   REAL,
  notes             TEXT,
  best_for          TEXT,
  max_reuse         INTEGER,
  add_to_secondary  INTEGER DEFAULT 0
);

-- BeerXML requires use and time_min on the library record as suggested defaults.
-- recipe_addition_miscs holds the actual use/time_min values for a specific recipe.
CREATE TABLE miscs (
  id                TEXT PRIMARY KEY,
  name              TEXT NOT NULL,
  type              TEXT NOT NULL,
  use               TEXT NOT NULL,
  time_min          REAL NOT NULL,
  notes             TEXT,
  use_for           TEXT,
  amount_is_weight  INTEGER DEFAULT 0
);

CREATE TABLE waters (
  id              TEXT PRIMARY KEY,
  name            TEXT NOT NULL,
  calcium_ppm     REAL NOT NULL,
  bicarbonate_ppm REAL NOT NULL,
  sulfate_ppm     REAL NOT NULL,
  chloride_ppm    REAL NOT NULL,
  sodium_ppm      REAL NOT NULL,
  magnesium_ppm   REAL NOT NULL,
  ph              REAL,
  notes           TEXT
);
```

### Recipes
```sql
CREATE TABLE recipes (
  id                    TEXT PRIMARY KEY,
  name                  TEXT NOT NULL,
  type                  TEXT NOT NULL DEFAULT 'all_grain',
  brewer                TEXT,
  asst_brewer           TEXT,
  batch_size_l          REAL NOT NULL,
  boil_size_l           REAL NOT NULL,
  boil_time_min         REAL NOT NULL DEFAULT 60,
  efficiency_pct        REAL,
  equipment_profile_id  TEXT REFERENCES equipment_profiles(id),
  style_id              TEXT REFERENCES styles(id),
  notes                 TEXT,
  taste_notes           TEXT,
  taste_rating          REAL,
  og                    REAL,
  fg                    REAL,
  fermentation_stages   INTEGER DEFAULT 1,
  primary_age_days      REAL,
  primary_temp_c        REAL,
  secondary_age_days    REAL,
  secondary_temp_c      REAL,
  tertiary_age_days     REAL,
  tertiary_temp_c       REAL,
  age_days              REAL,
  age_temp_c            REAL,
  carbonation_vols      REAL,
  forced_carbonation    INTEGER DEFAULT 0,
  priming_sugar_name    TEXT,
  carbonation_temp_c    REAL,
  priming_sugar_equiv   REAL,
  keg_priming_factor    REAL,
  date                  TEXT,
  created_at            INTEGER NOT NULL,
  updated_at            INTEGER NOT NULL
);
```

### Recipe additions (library item + recipe-specific use/amount)
```sql
CREATE TABLE recipe_addition_fermentables (
  id              TEXT PRIMARY KEY,
  recipe_id       TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
  fermentable_id  TEXT REFERENCES fermentables(id),
  name            TEXT NOT NULL,
  type            TEXT NOT NULL,
  yield_pct       REAL NOT NULL,
  color_lovibond  REAL NOT NULL,
  amount_kg       REAL NOT NULL,
  add_after_boil  INTEGER DEFAULT 0,
  addition_order  INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE recipe_addition_hops (
  id             TEXT PRIMARY KEY,
  recipe_id      TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
  hop_id         TEXT REFERENCES hops(id),
  name           TEXT NOT NULL,
  alpha_pct      REAL NOT NULL,
  form           TEXT NOT NULL DEFAULT 'pellet',
  amount_kg      REAL NOT NULL,
  use            TEXT NOT NULL,
  time_min       REAL NOT NULL,
  addition_order INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE recipe_addition_yeasts (
  id               TEXT PRIMARY KEY,
  recipe_id        TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
  yeast_id         TEXT REFERENCES yeasts(id),
  name             TEXT NOT NULL,
  type             TEXT NOT NULL,
  form             TEXT NOT NULL,
  laboratory       TEXT,
  product_id       TEXT,
  attenuation_pct  REAL,
  amount           REAL,
  amount_is_weight INTEGER DEFAULT 0,
  add_to_secondary INTEGER DEFAULT 0,
  times_cultured   INTEGER DEFAULT 0
);

CREATE TABLE recipe_addition_miscs (
  id               TEXT PRIMARY KEY,
  recipe_id        TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
  misc_id          TEXT REFERENCES miscs(id),
  name             TEXT NOT NULL,
  type             TEXT NOT NULL,
  use              TEXT NOT NULL,
  amount           REAL NOT NULL,
  amount_is_weight INTEGER DEFAULT 0,
  time_min         REAL NOT NULL,
  addition_order   INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE recipe_addition_waters (
  id        TEXT PRIMARY KEY,
  recipe_id TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
  water_id  TEXT REFERENCES waters(id),
  name      TEXT NOT NULL,
  amount_l  REAL NOT NULL
);
```

### Mash
```sql
CREATE TABLE mashes (
  id                TEXT PRIMARY KEY,
  recipe_id         TEXT NOT NULL UNIQUE REFERENCES recipes(id) ON DELETE CASCADE,
  name              TEXT NOT NULL DEFAULT 'Single Infusion',
  grain_temp_c      REAL NOT NULL DEFAULT 21,
  tun_temp_c        REAL,
  sparge_temp_c     REAL,
  ph                REAL,
  tun_weight_kg     REAL,
  tun_specific_heat REAL,
  equip_adjust      INTEGER DEFAULT 0,
  notes             TEXT
);

CREATE TABLE mash_steps (
  id              TEXT PRIMARY KEY,
  mash_id         TEXT NOT NULL REFERENCES mashes(id) ON DELETE CASCADE,
  name            TEXT NOT NULL,
  type            TEXT NOT NULL DEFAULT 'infusion',
  infuse_amount_l REAL,
  step_temp_c     REAL NOT NULL,
  step_time_min   INTEGER NOT NULL,
  ramp_time_min   INTEGER,
  end_temp_c      REAL,
  step_order      INTEGER NOT NULL
);
```

### Settings
```sql
CREATE TABLE settings (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
  -- Keys: theme, units (metric|imperial), default_equipment_profile_id
);
```

---

## Tauri Command API

Five verbs throughout: `list_`, `get_`, `create_`, `update_`, `delete_`. Non-CRUD reads use `get_`. Creation with a `source_id` input field handles duplication.

```rust
// Recipes
list_recipes()                                      -> Result<Vec<RecipeSummary>>
get_recipe(id)                                      -> Result<Recipe>
create_recipe(input: CreateRecipeInput)             -> Result<Recipe>
update_recipe(id, input: UpdateRecipeInput)         -> Result<Recipe>
delete_recipe(id)                                   -> Result<()>

// Recipe additions
create_recipe_fermentable(recipe_id, input)         -> Result<RecipeAdditionFermentable>
update_recipe_fermentable(id, input)                -> Result<RecipeAdditionFermentable>
delete_recipe_fermentable(id)                       -> Result<()>

create_recipe_hop(recipe_id, input)                 -> Result<RecipeAdditionHop>
update_recipe_hop(id, input)                        -> Result<RecipeAdditionHop>
delete_recipe_hop(id)                               -> Result<()>

create_recipe_yeast(recipe_id, input)               -> Result<RecipeAdditionYeast>
update_recipe_yeast(id, input)                      -> Result<RecipeAdditionYeast>
delete_recipe_yeast(id)                             -> Result<()>

create_recipe_misc(recipe_id, input)                -> Result<RecipeAdditionMisc>
update_recipe_misc(id, input)                       -> Result<RecipeAdditionMisc>
delete_recipe_misc(id)                              -> Result<()>

create_recipe_water(recipe_id, input)               -> Result<RecipeAdditionWater>
update_recipe_water(id, input)                      -> Result<RecipeAdditionWater>
delete_recipe_water(id)                             -> Result<()>

// Mash
get_mash(recipe_id)                                 -> Result<Mash>
update_mash(recipe_id, input: UpdateMashInput)      -> Result<Mash>
create_mash_step(mash_id, input)                    -> Result<MashStep>
update_mash_step(id, input)                         -> Result<MashStep>
delete_mash_step(id)                                -> Result<()>
update_mash_step_order(mash_id, ordered_ids)        -> Result<()>

// Computed stats (pure Rust, not persisted)
get_recipe_stats(recipe_id)                         -> Result<RecipeStats>

// Equipment profiles
list_equipment_profiles()                           -> Result<Vec<EquipmentProfile>>
create_equipment_profile(input)                     -> Result<EquipmentProfile>
update_equipment_profile(id, input)                 -> Result<EquipmentProfile>
delete_equipment_profile(id)                        -> Result<()>

// Styles
list_styles()                                       -> Result<Vec<Style>>

// Ingredient library
list_fermentable_library()                          -> Result<Vec<Fermentable>>
list_hop_library()                                  -> Result<Vec<Hop>>
list_yeast_library()                                -> Result<Vec<Yeast>>
list_misc_library()                                 -> Result<Vec<Misc>>
list_water_library()                                -> Result<Vec<Water>>

// Settings
get_settings()                                      -> Result<HashMap<String, String>>
update_setting(key, value)                          -> Result<()>

// Import / export
get_recipe_beerxml(recipe_id)                       -> Result<String>
create_recipes_from_beerxml(xml)                    -> Result<Vec<RecipeSummary>>
```

### Key structs

```rust
pub struct RecipeSummary {
    pub id: String,
    pub name: String,
    pub style_name: Option<String>,
    pub type_: String,
    pub batch_size_l: f64,
    pub created_at: i64,
    pub updated_at: i64,
}

pub struct Recipe {
    pub id: String,
    pub name: String,
    pub type_: String,
    pub brewer: Option<String>,
    pub asst_brewer: Option<String>,
    pub batch_size_l: f64,
    pub boil_size_l: f64,
    pub boil_time_min: f64,
    pub efficiency_pct: Option<f64>,   // None = inherit from equipment profile
    pub style_id: Option<String>,
    pub equipment_profile_id: Option<String>,
    pub notes: Option<String>,
    pub taste_notes: Option<String>,
    pub taste_rating: Option<f64>,
    pub og: Option<f64>,
    pub fg: Option<f64>,
    pub fermentation_stages: i32,
    pub primary_age_days: Option<f64>,
    pub primary_temp_c: Option<f64>,
    pub secondary_age_days: Option<f64>,
    pub secondary_temp_c: Option<f64>,
    pub tertiary_age_days: Option<f64>,
    pub tertiary_temp_c: Option<f64>,
    pub age_days: Option<f64>,
    pub age_temp_c: Option<f64>,
    pub carbonation_vols: Option<f64>,
    pub forced_carbonation: bool,
    pub priming_sugar_name: Option<String>,
    pub carbonation_temp_c: Option<f64>,
    pub priming_sugar_equiv: Option<f64>,
    pub keg_priming_factor: Option<f64>,
    pub date: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    // Resolved relations
    pub equipment_profile: Option<EquipmentProfile>,
    pub style: Option<Style>,
    pub fermentables: Vec<RecipeAdditionFermentable>,
    pub hops: Vec<RecipeAdditionHop>,
    pub yeasts: Vec<RecipeAdditionYeast>,
    pub miscs: Vec<RecipeAdditionMisc>,
    pub waters: Vec<RecipeAdditionWater>,
    pub mash: Option<Mash>,  // includes mash.steps
}

pub struct RecipeStats {
    pub og: f64,
    pub fg: f64,
    pub abv_pct: f64,
    pub ibu: f64,
    pub srm: f64,
    pub calories_per_12oz: f64,
    pub bu_gu_ratio: f64,
    pub pre_boil_gravity: f64,
    pub pre_boil_volume_l: f64,
    pub post_boil_volume_l: f64,
}
```

---

## Brewing Calculations

All implemented in `src-tauri/src/brewing/`. Pure functions with no DB access — fully unit-testable.

| Stat | Formula |
|---|---|
| OG | Tinseth / standard points-per-gallon from `yield_pct` and grain bill |
| FG | `OG × (1 - attenuation_pct)` |
| ABV | `(OG - FG) × 131.25` |
| IBU | Tinseth formula using `alpha_pct`, boil time, volume |
| SRM | Morey formula: `1.4922 × (MCU ^ 0.6859)` where MCU = (color × lbs) / gallons |
| Calories | Standard ASBC formula |
| BU:GU | `IBU / ((OG - 1) × 1000)` |

---

## Data Conventions

- **All storage in SI units** with unit suffixes (`_l`, `_kg`, `_c`, `_pct`, `_min`, `_ppm`)
- **Unit conversion** happens at the Tauri command boundary — Svelte receives values in user-preferred units
- **Snapshot pattern** on recipe additions — key fields (`alpha_pct`, `yield_pct`, `color_lovibond`, etc.) are copied from the library at addition time; editing the library never silently changes existing recipes
- **IDs** are UUIDs (v4), generated in Rust
- **Timestamps** are Unix epoch integers (seconds)
- **Booleans** stored as `INTEGER` (0/1) per SQLite convention

---

## Walking Skeleton Scope (v1)

**In scope:**
- Recipe CRUD (create, list, edit, delete, duplicate via source_id)
- All five ingredient types on a recipe (fermentables, hops, yeasts, miscs, waters)
- Mash profile with ordered steps
- Fermentation schedule and carbonation fields
- Live stats sidebar (OG, FG, ABV, IBU, SRM)
- Equipment profiles (CRUD, linked to recipes)
- Style list (pre-seeded BJCP data, read-only in v1)
- Ingredient library (pre-seeded common ingredients, read-only in v1)
- BeerXML import and export
- Settings: theme, units (metric/imperial), default equipment profile
- Themeable UI via CSS custom properties (Midnight dark theme shipped)

**Explicitly out of scope for v1:**
- Batches / brew log
- Inventory tracking
- Water chemistry calculations
- Custom style creation
- Custom library ingredient creation
- Cloud sync

---

## Cross-Platform Considerations

macOS is the primary development target. All Tauri APIs used must be cross-platform (no macOS-specific native integrations). File picker, notifications, and window management use Tauri's built-in cross-platform abstractions. This preserves a straight path to Windows and Linux builds.
