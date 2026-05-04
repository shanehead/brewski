# Brewski Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Scaffold the Tauri 2 + SvelteKit 5 project, set up SQLite with all migrations, define all Rust models, and implement fully-tested brewing calculation functions.

**Architecture:** All Rust logic lives in `src-tauri/src/`. The `brewing/` module holds pure functions with no DB access — OG, IBU (Tinseth), SRM (Morey), ABV, volumes. `models.rs` holds all shared structs. The DB pool is managed as Tauri app state.

**Tech Stack:** Rust, Tauri 2, SvelteKit 5, Svelte 5, Tailwind CSS v4, sqlx 0.8 (SQLite), uuid v1, serde, thiserror

---

### Task 1: Scaffold Tauri 2 + SvelteKit project

**Files:**
- Create: all project files via scaffold command

- [ ] **Step 1: Run scaffold inside the project directory**

```bash
cd /Users/shead/Documents/code/brewski
npm create tauri@2
```

When prompted, enter:
- Project name: `brewski`
- Choose which language for your frontend: `TypeScript / JavaScript`
- Choose your package manager: `npm`
- Choose your UI template: `Svelte` → then `SvelteKit`
- Add a pre-built Tauri plugin: skip (press Enter)

This generates: `src/`, `src-tauri/`, `package.json`, `vite.config.ts`, `svelte.config.js`, `tsconfig.json`.

- [ ] **Step 2: Install frontend dependencies**

```bash
npm install
```

Expected output: `added N packages`

- [ ] **Step 3: Verify the app builds**

```bash
npm run tauri dev
```

Expected: Tauri window opens showing the default Svelte welcome page. Close it.

- [ ] **Step 4: Commit**

```bash
git add -A
git commit -m "feat: scaffold Tauri 2 + SvelteKit project"
```

---

### Task 2: Configure Rust dependencies

**Files:**
- Modify: `src-tauri/Cargo.toml`

- [ ] **Step 1: Replace the `[dependencies]` section**

Open `src-tauri/Cargo.toml` and replace the `[dependencies]` block with:

```toml
[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-opener = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite", "migrate"] }
tokio = { version = "1", features = ["full"] }
uuid = { version = "1", features = ["v4"] }
thiserror = "2"

[dev-dependencies]
tokio = { version = "1", features = ["full", "test-util"] }
```

- [ ] **Step 2: Verify compilation**

```bash
cd src-tauri && cargo build
```

Expected: compiles without errors. First run downloads crates so may take a minute.

- [ ] **Step 3: Commit**

```bash
cd ..
git add src-tauri/Cargo.toml src-tauri/Cargo.lock
git commit -m "feat: add sqlx, uuid, thiserror dependencies"
```

---

### Task 3: Configure Tailwind CSS v4 + midnight theme

**Files:**
- Modify: `vite.config.ts`
- Modify: `src/app.css` (or create if absent)
- Create: `src/themes/midnight.css`

- [ ] **Step 1: Install Tailwind CSS v4 vite plugin**

```bash
npm install tailwindcss @tailwindcss/vite
```

- [ ] **Step 2: Update `vite.config.ts`**

```typescript
import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig(async () => ({
  plugins: [sveltekit(), tailwindcss()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
}));
```

- [ ] **Step 3: Create/replace `src/app.css`**

```css
@import "tailwindcss";
```

- [ ] **Step 4: Create `src/themes/midnight.css`**

```css
:root {
  --color-bg-base:        #0d0d12;
  --color-bg-surface:     #13131c;
  --color-bg-elevated:    #1a1a2a;
  --color-border:         #1e1e2e;
  --color-accent:         #5c5cff;
  --color-accent-hover:   #7c7cff;
  --color-text-primary:   #f0f0f5;
  --color-text-secondary: #888;
  --color-text-muted:     #444;
}
```

- [ ] **Step 5: Import theme in `src/app.html` or `src/routes/+layout.svelte`**

In `src/routes/+layout.svelte` (create if absent):

```svelte
<script lang="ts">
  import "../app.css";
  import "../themes/midnight.css";
</script>

<slot />
```

- [ ] **Step 6: Verify dev server starts cleanly**

```bash
npm run dev
```

Expected: starts on port 1420, no CSS errors.

- [ ] **Step 7: Commit**

```bash
git add -A
git commit -m "feat: configure Tailwind CSS v4 and midnight theme"
```

---

### Task 4: Write SQL migration

**Files:**
- Create: `src-tauri/src/db/migrations/001_initial.sql`
- Create: `src-tauri/src/db/mod.rs`

- [ ] **Step 1: Create the migrations directory**

```bash
mkdir -p src-tauri/src/db/migrations
```

- [ ] **Step 2: Create `src-tauri/src/db/migrations/001_initial.sql`**

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

CREATE TABLE settings (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
);

INSERT INTO settings (key, value) VALUES
  ('theme', 'midnight'),
  ('units', 'metric');
```

- [ ] **Step 3: Create `src-tauri/src/db/mod.rs`**

```rust
pub mod equipment;
pub mod library;
pub mod recipes;
pub mod additions;
pub mod mash;
pub mod settings;
```

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/db/
git commit -m "feat: add SQL migration with all 14 tables"
```

---

### Task 5: Set up DB connection pool in Tauri app state

**Files:**
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/src/main.rs`
- Create: placeholder files for db submodules (so lib.rs compiles)

- [ ] **Step 1: Create stub files for each db submodule**

Create these files, each with only a comment for now (real content comes in later tasks):

`src-tauri/src/db/equipment.rs`:
```rust
use sqlx::SqlitePool;
use crate::error::AppError;
```

`src-tauri/src/db/library.rs`:
```rust
use sqlx::SqlitePool;
use crate::error::AppError;
```

`src-tauri/src/db/recipes.rs`:
```rust
use sqlx::SqlitePool;
use crate::error::AppError;
```

`src-tauri/src/db/additions.rs`:
```rust
use sqlx::SqlitePool;
use crate::error::AppError;
```

`src-tauri/src/db/mash.rs`:
```rust
use sqlx::SqlitePool;
use crate::error::AppError;
```

`src-tauri/src/db/settings.rs`:
```rust
use sqlx::SqlitePool;
use crate::error::AppError;
```

- [ ] **Step 2: Create `src-tauri/src/error.rs`**

```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    Other(String),
}

impl serde::Serialize for AppError {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}
```

- [ ] **Step 3: Replace `src-tauri/src/lib.rs`**

```rust
mod db;
mod error;
pub mod models;
pub mod brewing;

use sqlx::SqlitePool;
use tauri::Manager;

pub struct AppState {
    pub db: SqlitePool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_dir)?;
            let db_url = format!("sqlite:{}", app_dir.join("brewski.db").display());

            let pool = tauri::async_runtime::block_on(
                SqlitePool::connect(&db_url)
            )?;

            tauri::async_runtime::block_on(
                sqlx::migrate!("src/db/migrations").run(&pool)
            )?;

            app.manage(AppState { db: pool });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 4: Create stub `src-tauri/src/models.rs` and `src-tauri/src/brewing/mod.rs`**

`src-tauri/src/models.rs`:
```rust
// Populated in Task 6
```

`src-tauri/src/brewing/mod.rs`:
```rust
// Populated in Task 12
```

- [ ] **Step 5: Verify it compiles**

```bash
cd src-tauri && cargo build
```

Expected: compiles. The migration will also run on first launch.

- [ ] **Step 6: Commit**

```bash
cd ..
git add src-tauri/src/
git commit -m "feat: set up sqlx pool and Tauri app state"
```

---

### Task 6: Write Rust models

**Files:**
- Modify: `src-tauri/src/models.rs`

- [ ] **Step 1: Write all structs in `src-tauri/src/models.rs`**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Style {
    pub id: String,
    pub name: String,
    pub category: String,
    pub category_number: String,
    pub style_letter: String,
    pub style_guide: String,
    #[sqlx(rename = "type")]
    pub type_: String,
    pub og_min: f64,
    pub og_max: f64,
    pub fg_min: f64,
    pub fg_max: f64,
    pub ibu_min: f64,
    pub ibu_max: f64,
    pub color_min_srm: f64,
    pub color_max_srm: f64,
    pub carb_min_vols: Option<f64>,
    pub carb_max_vols: Option<f64>,
    pub abv_min_pct: Option<f64>,
    pub abv_max_pct: Option<f64>,
    pub notes: Option<String>,
    pub profile: Option<String>,
    pub ingredients: Option<String>,
    pub examples: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct EquipmentProfile {
    pub id: String,
    pub name: String,
    pub notes: Option<String>,
    pub boil_size_l: f64,
    pub batch_size_l: f64,
    pub calc_boil_volume: bool,
    pub tun_volume_l: Option<f64>,
    pub tun_weight_kg: Option<f64>,
    pub tun_specific_heat: Option<f64>,
    pub lauter_deadspace_l: f64,
    pub top_up_kettle_l: f64,
    pub trub_chiller_loss_l: f64,
    pub evap_rate_pct_hr: f64,
    pub boil_time_min: f64,
    pub top_up_water_l: f64,
    pub fermenter_loss_l: f64,
    pub hop_utilization_pct: f64,
    pub efficiency_pct: f64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Fermentable {
    pub id: String,
    pub name: String,
    #[sqlx(rename = "type")]
    pub type_: String,
    pub yield_pct: f64,
    pub color_lovibond: f64,
    pub origin: Option<String>,
    pub supplier: Option<String>,
    pub notes: Option<String>,
    pub add_after_boil: bool,
    pub coarse_fine_diff_pct: Option<f64>,
    pub moisture_pct: Option<f64>,
    pub diastatic_power_lintner: Option<f64>,
    pub protein_pct: Option<f64>,
    pub max_in_batch_pct: Option<f64>,
    pub recommend_mash: Option<bool>,
    pub ibu_gal_per_lb: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Hop {
    pub id: String,
    pub name: String,
    pub alpha_pct: f64,
    pub beta_pct: Option<f64>,
    pub form: String,
    #[sqlx(rename = "type")]
    pub type_: Option<String>,
    pub origin: Option<String>,
    pub year: Option<String>,
    pub notes: Option<String>,
    pub substitutes: Option<String>,
    pub hsi_pct: Option<f64>,
    pub humulene_pct: Option<f64>,
    pub caryophyllene_pct: Option<f64>,
    pub cohumulone_pct: Option<f64>,
    pub myrcene_pct: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Yeast {
    pub id: String,
    pub name: String,
    #[sqlx(rename = "type")]
    pub type_: String,
    pub form: String,
    pub laboratory: Option<String>,
    pub product_id: Option<String>,
    pub min_temperature_c: Option<f64>,
    pub max_temperature_c: Option<f64>,
    pub flocculation: Option<String>,
    pub attenuation_pct: Option<f64>,
    pub notes: Option<String>,
    pub best_for: Option<String>,
    pub max_reuse: Option<i64>,
    pub add_to_secondary: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Misc {
    pub id: String,
    pub name: String,
    #[sqlx(rename = "type")]
    pub type_: String,
    #[sqlx(rename = "use")]
    pub use_: String,
    pub time_min: f64,
    pub notes: Option<String>,
    pub use_for: Option<String>,
    pub amount_is_weight: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Water {
    pub id: String,
    pub name: String,
    pub calcium_ppm: f64,
    pub bicarbonate_ppm: f64,
    pub sulfate_ppm: f64,
    pub chloride_ppm: f64,
    pub sodium_ppm: f64,
    pub magnesium_ppm: f64,
    pub ph: Option<f64>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct RecipeSummary {
    pub id: String,
    pub name: String,
    pub style_name: Option<String>,
    #[sqlx(rename = "type")]
    pub type_: String,
    pub batch_size_l: f64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub type_: String,
    pub brewer: Option<String>,
    pub asst_brewer: Option<String>,
    pub batch_size_l: f64,
    pub boil_size_l: f64,
    pub boil_time_min: f64,
    pub efficiency_pct: Option<f64>,
    pub style_id: Option<String>,
    pub equipment_profile_id: Option<String>,
    pub notes: Option<String>,
    pub taste_notes: Option<String>,
    pub taste_rating: Option<f64>,
    pub og: Option<f64>,
    pub fg: Option<f64>,
    pub fermentation_stages: i64,
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
    pub equipment_profile: Option<EquipmentProfile>,
    pub style: Option<Style>,
    pub fermentables: Vec<RecipeAdditionFermentable>,
    pub hops: Vec<RecipeAdditionHop>,
    pub yeasts: Vec<RecipeAdditionYeast>,
    pub miscs: Vec<RecipeAdditionMisc>,
    pub waters: Vec<RecipeAdditionWater>,
    pub mash: Option<Mash>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct RecipeRow {
    pub id: String,
    pub name: String,
    #[sqlx(rename = "type")]
    pub type_: String,
    pub brewer: Option<String>,
    pub asst_brewer: Option<String>,
    pub batch_size_l: f64,
    pub boil_size_l: f64,
    pub boil_time_min: f64,
    pub efficiency_pct: Option<f64>,
    pub style_id: Option<String>,
    pub equipment_profile_id: Option<String>,
    pub notes: Option<String>,
    pub taste_notes: Option<String>,
    pub taste_rating: Option<f64>,
    pub og: Option<f64>,
    pub fg: Option<f64>,
    pub fermentation_stages: i64,
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
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct RecipeAdditionFermentable {
    pub id: String,
    pub recipe_id: String,
    pub fermentable_id: Option<String>,
    pub name: String,
    #[sqlx(rename = "type")]
    pub type_: String,
    pub yield_pct: f64,
    pub color_lovibond: f64,
    pub amount_kg: f64,
    pub add_after_boil: bool,
    pub addition_order: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct RecipeAdditionHop {
    pub id: String,
    pub recipe_id: String,
    pub hop_id: Option<String>,
    pub name: String,
    pub alpha_pct: f64,
    pub form: String,
    pub amount_kg: f64,
    #[sqlx(rename = "use")]
    pub use_: String,
    pub time_min: f64,
    pub addition_order: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct RecipeAdditionYeast {
    pub id: String,
    pub recipe_id: String,
    pub yeast_id: Option<String>,
    pub name: String,
    #[sqlx(rename = "type")]
    pub type_: String,
    pub form: String,
    pub laboratory: Option<String>,
    pub product_id: Option<String>,
    pub attenuation_pct: Option<f64>,
    pub amount: Option<f64>,
    pub amount_is_weight: bool,
    pub add_to_secondary: bool,
    pub times_cultured: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct RecipeAdditionMisc {
    pub id: String,
    pub recipe_id: String,
    pub misc_id: Option<String>,
    pub name: String,
    #[sqlx(rename = "type")]
    pub type_: String,
    #[sqlx(rename = "use")]
    pub use_: String,
    pub amount: f64,
    pub amount_is_weight: bool,
    pub time_min: f64,
    pub addition_order: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct RecipeAdditionWater {
    pub id: String,
    pub recipe_id: String,
    pub water_id: Option<String>,
    pub name: String,
    pub amount_l: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub notes: Option<String>,
    pub steps: Vec<MashStep>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct MashRow {
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
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct MashStep {
    pub id: String,
    pub mash_id: String,
    pub name: String,
    #[sqlx(rename = "type")]
    pub type_: String,
    pub infuse_amount_l: Option<f64>,
    pub step_temp_c: f64,
    pub step_time_min: i64,
    pub ramp_time_min: Option<i64>,
    pub end_temp_c: Option<f64>,
    pub step_order: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
}

// --- Input types for create/update commands ---

#[derive(Debug, Deserialize)]
pub struct CreateRecipeInput {
    pub name: String,
    pub type_: Option<String>,
    pub batch_size_l: Option<f64>,
    pub boil_size_l: Option<f64>,
    pub boil_time_min: Option<f64>,
    pub equipment_profile_id: Option<String>,
    pub source_id: Option<String>,  // if set, duplicates source recipe
}

#[derive(Debug, Deserialize)]
pub struct UpdateRecipeInput {
    pub name: Option<String>,
    pub type_: Option<String>,
    pub brewer: Option<String>,
    pub asst_brewer: Option<String>,
    pub batch_size_l: Option<f64>,
    pub boil_size_l: Option<f64>,
    pub boil_time_min: Option<f64>,
    pub efficiency_pct: Option<f64>,
    pub style_id: Option<String>,
    pub equipment_profile_id: Option<String>,
    pub notes: Option<String>,
    pub taste_notes: Option<String>,
    pub taste_rating: Option<f64>,
    pub fermentation_stages: Option<i64>,
    pub primary_age_days: Option<f64>,
    pub primary_temp_c: Option<f64>,
    pub secondary_age_days: Option<f64>,
    pub secondary_temp_c: Option<f64>,
    pub tertiary_age_days: Option<f64>,
    pub tertiary_temp_c: Option<f64>,
    pub age_days: Option<f64>,
    pub age_temp_c: Option<f64>,
    pub carbonation_vols: Option<f64>,
    pub forced_carbonation: Option<bool>,
    pub priming_sugar_name: Option<String>,
    pub carbonation_temp_c: Option<f64>,
    pub date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFermentableAdditionInput {
    pub fermentable_id: Option<String>,
    pub name: String,
    pub type_: String,
    pub yield_pct: f64,
    pub color_lovibond: f64,
    pub amount_kg: f64,
    pub add_after_boil: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFermentableAdditionInput {
    pub amount_kg: Option<f64>,
    pub add_after_boil: Option<bool>,
    pub addition_order: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateHopAdditionInput {
    pub hop_id: Option<String>,
    pub name: String,
    pub alpha_pct: f64,
    pub form: Option<String>,
    pub amount_kg: f64,
    pub use_: String,
    pub time_min: f64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateHopAdditionInput {
    pub amount_kg: Option<f64>,
    pub use_: Option<String>,
    pub time_min: Option<f64>,
    pub addition_order: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateYeastAdditionInput {
    pub yeast_id: Option<String>,
    pub name: String,
    pub type_: String,
    pub form: String,
    pub laboratory: Option<String>,
    pub product_id: Option<String>,
    pub attenuation_pct: Option<f64>,
    pub amount: Option<f64>,
    pub amount_is_weight: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateYeastAdditionInput {
    pub attenuation_pct: Option<f64>,
    pub amount: Option<f64>,
    pub amount_is_weight: Option<bool>,
    pub add_to_secondary: Option<bool>,
    pub times_cultured: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMiscAdditionInput {
    pub misc_id: Option<String>,
    pub name: String,
    pub type_: String,
    pub use_: String,
    pub amount: f64,
    pub amount_is_weight: Option<bool>,
    pub time_min: f64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMiscAdditionInput {
    pub amount: Option<f64>,
    pub amount_is_weight: Option<bool>,
    pub use_: Option<String>,
    pub time_min: Option<f64>,
    pub addition_order: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateWaterAdditionInput {
    pub water_id: Option<String>,
    pub name: String,
    pub amount_l: f64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWaterAdditionInput {
    pub amount_l: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMashInput {
    pub name: Option<String>,
    pub grain_temp_c: Option<f64>,
    pub tun_temp_c: Option<f64>,
    pub sparge_temp_c: Option<f64>,
    pub ph: Option<f64>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMashStepInput {
    pub name: String,
    pub type_: Option<String>,
    pub infuse_amount_l: Option<f64>,
    pub step_temp_c: f64,
    pub step_time_min: i64,
    pub ramp_time_min: Option<i64>,
    pub end_temp_c: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMashStepInput {
    pub name: Option<String>,
    pub type_: Option<String>,
    pub infuse_amount_l: Option<f64>,
    pub step_temp_c: Option<f64>,
    pub step_time_min: Option<i64>,
    pub ramp_time_min: Option<i64>,
    pub end_temp_c: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEquipmentProfileInput {
    pub name: String,
    pub notes: Option<String>,
    pub boil_size_l: f64,
    pub batch_size_l: f64,
    pub boil_time_min: Option<f64>,
    pub evap_rate_pct_hr: Option<f64>,
    pub trub_chiller_loss_l: Option<f64>,
    pub fermenter_loss_l: Option<f64>,
    pub efficiency_pct: f64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEquipmentProfileInput {
    pub name: Option<String>,
    pub notes: Option<String>,
    pub boil_size_l: Option<f64>,
    pub batch_size_l: Option<f64>,
    pub boil_time_min: Option<f64>,
    pub evap_rate_pct_hr: Option<f64>,
    pub trub_chiller_loss_l: Option<f64>,
    pub fermenter_loss_l: Option<f64>,
    pub efficiency_pct: Option<f64>,
}
```

- [ ] **Step 2: Verify models compile**

```bash
cd src-tauri && cargo build 2>&1 | head -30
```

Expected: compiles. Fix any `use` keyword conflicts (they're handled via `#[sqlx(rename)]`).

- [ ] **Step 3: Commit**

```bash
cd ..
git add src-tauri/src/models.rs
git commit -m "feat: define all Rust models"
```

---

### Task 7: OG calculation

**Files:**
- Create: `src-tauri/src/brewing/og.rs`
- Modify: `src-tauri/src/brewing/mod.rs`

- [ ] **Step 1: Write the failing test in `src-tauri/src/brewing/og.rs`**

```rust
pub fn calculate_og(
    fermentables: &[(&f64, &f64, bool)],  // (yield_pct, amount_kg, add_after_boil)
    batch_size_l: f64,
    efficiency_pct: f64,
) -> f64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_og_pale_ale() {
        // 5 kg pale malt (75% yield), 23L batch, 75% efficiency
        let fermentables = vec![(&75.0f64, &5.0f64, false)];
        let og = calculate_og(&fermentables, 23.0, 75.0);
        // Expected ~1.047 (within ±0.002 tolerance)
        assert!((og - 1.047).abs() < 0.002, "OG was {og:.4}, expected ~1.047");
    }

    #[test]
    fn test_og_late_addition_gets_full_efficiency() {
        let normal = vec![(&75.0f64, &2.5f64, false)];
        let late = vec![(&75.0f64, &2.5f64, true)];
        let og_normal = calculate_og(&normal, 23.0, 75.0);
        let og_late = calculate_og(&late, 23.0, 75.0);
        // Late additions bypass mash efficiency, so og_late > og_normal
        assert!(og_late > og_normal);
    }

    #[test]
    fn test_og_empty_grain_bill() {
        let og = calculate_og(&[], 23.0, 75.0);
        assert_eq!(og, 1.0);
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

```bash
cd src-tauri && cargo test brewing::og
```

Expected: FAIL with `not yet implemented` panic.

- [ ] **Step 3: Implement `calculate_og`**

```rust
pub fn calculate_og(
    fermentables: &[(&f64, &f64, bool)],  // (yield_pct, amount_kg, add_after_boil)
    batch_size_l: f64,
    efficiency_pct: f64,
) -> f64 {
    let batch_gal = batch_size_l * 0.264172;
    let total_points: f64 = fermentables.iter().map(|(yield_pct, amount_kg, add_after_boil)| {
        let eff = if *add_after_boil { 100.0 } else { efficiency_pct };
        let lbs = *amount_kg * 2.20462;
        let ppg = *yield_pct / 100.0 * 46.0;
        lbs * ppg * (eff / 100.0)
    }).sum();
    1.0 + (total_points / batch_gal) / 1000.0
}
```

- [ ] **Step 4: Run tests to verify they pass**

```bash
cd src-tauri && cargo test brewing::og
```

Expected: 3 tests pass.

- [ ] **Step 5: Add og module to `src-tauri/src/brewing/mod.rs`**

```rust
pub mod og;
```

- [ ] **Step 6: Commit**

```bash
cd ..
git add src-tauri/src/brewing/
git commit -m "feat: OG calculation with tests"
```

---

### Task 8: FG, ABV, and calorie calculations

**Files:**
- Create: `src-tauri/src/brewing/abv.rs`
- Modify: `src-tauri/src/brewing/mod.rs`

- [ ] **Step 1: Write failing tests in `src-tauri/src/brewing/abv.rs`**

```rust
pub fn calculate_fg(og: f64, attenuation_pct: f64) -> f64 {
    todo!()
}

pub fn calculate_abv(og: f64, fg: f64) -> f64 {
    todo!()
}

pub fn calculate_calories_per_355ml(og: f64, fg: f64) -> f64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fg_from_attenuation() {
        // OG 1.052, 75% attenuation → FG 1.013
        let fg = calculate_fg(1.052, 75.0);
        assert!((fg - 1.013).abs() < 0.001, "FG was {fg:.4}, expected ~1.013");
    }

    #[test]
    fn test_abv_standard() {
        // OG 1.052, FG 1.013 → ~5.1% ABV
        let abv = calculate_abv(1.052, 1.013);
        assert!((abv - 5.1).abs() < 0.2, "ABV was {abv:.2}, expected ~5.1");
    }

    #[test]
    fn test_calories_reasonable_range() {
        // 355ml of 5% beer should be ~150 kcal
        let cal = calculate_calories_per_355ml(1.052, 1.013);
        assert!(cal > 130.0 && cal < 175.0, "Calories was {cal:.1}");
    }
}
```

- [ ] **Step 2: Run to verify failure**

```bash
cd src-tauri && cargo test brewing::abv
```

Expected: FAIL with `not yet implemented`.

- [ ] **Step 3: Implement all three functions**

```rust
pub fn calculate_fg(og: f64, attenuation_pct: f64) -> f64 {
    1.0 + (og - 1.0) * (1.0 - attenuation_pct / 100.0)
}

pub fn calculate_abv(og: f64, fg: f64) -> f64 {
    (og - fg) * 131.25
}

// ASBC formula: kcal per 355 mL (12 oz)
pub fn calculate_calories_per_355ml(og: f64, fg: f64) -> f64 {
    let abw = (og - fg) * 105.0;  // alcohol by weight
    let re = 0.1808 * og_to_plato(og) + 0.8192 * og_to_plato(fg);
    let cal_per_ml = (6.9 * abw + 4.0 * (re - 0.1)) * fg * 10.0 / 1000.0;
    cal_per_ml * 355.0
}

fn og_to_plato(sg: f64) -> f64 {
    (-1.0 * 616.868) + (1111.14 * sg) - (630.272 * sg * sg) + (135.997 * sg * sg * sg)
}
```

- [ ] **Step 4: Run to verify tests pass**

```bash
cd src-tauri && cargo test brewing::abv
```

Expected: 3 tests pass.

- [ ] **Step 5: Add to `src-tauri/src/brewing/mod.rs`**

```rust
pub mod abv;
pub mod og;
```

- [ ] **Step 6: Commit**

```bash
cd ..
git add src-tauri/src/brewing/
git commit -m "feat: FG, ABV, and calorie calculations with tests"
```

---

### Task 9: IBU calculation (Tinseth)

**Files:**
- Create: `src-tauri/src/brewing/ibu.rs`
- Modify: `src-tauri/src/brewing/mod.rs`

- [ ] **Step 1: Write failing tests in `src-tauri/src/brewing/ibu.rs`**

```rust
pub fn tinseth_ibu(
    hops: &[(&f64, &f64, &f64, bool)],  // (alpha_pct, amount_kg, time_min, is_dry_hop)
    og: f64,
    post_boil_volume_l: f64,
) -> f64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ibu_single_addition() {
        // 28g (0.028 kg) of 10% AA hops, 60 min, OG 1.047, 23L → ~29 IBU
        let hops = vec![(&10.0f64, &0.028f64, &60.0f64, false)];
        let ibu = tinseth_ibu(&hops, 1.047, 23.0);
        assert!((ibu - 29.0).abs() < 3.0, "IBU was {ibu:.1}, expected ~29");
    }

    #[test]
    fn test_dry_hop_contributes_zero_ibu() {
        let dry_hop = vec![(&10.0f64, &0.028f64, &0.0f64, true)];
        let ibu = tinseth_ibu(&dry_hop, 1.047, 23.0);
        assert_eq!(ibu, 0.0);
    }

    #[test]
    fn test_ibu_zero_with_no_hops() {
        let ibu = tinseth_ibu(&[], 1.047, 23.0);
        assert_eq!(ibu, 0.0);
    }
}
```

- [ ] **Step 2: Run to verify failure**

```bash
cd src-tauri && cargo test brewing::ibu
```

Expected: FAIL.

- [ ] **Step 3: Implement**

```rust
pub fn tinseth_ibu(
    hops: &[(&f64, &f64, &f64, bool)],  // (alpha_pct, amount_kg, time_min, is_dry_hop)
    og: f64,
    post_boil_volume_l: f64,
) -> f64 {
    let vol_gal = post_boil_volume_l * 0.264172;
    hops.iter().map(|(alpha_pct, amount_kg, time_min, is_dry_hop)| {
        if *is_dry_hop || **time_min <= 0.0 {
            return 0.0;
        }
        let bigness = 1.65 * 0.000125f64.powf(og - 1.0);
        let time_factor = (1.0 - f64::exp(-0.04 * *time_min)) / 4.15;
        let utilization = bigness * time_factor;
        let oz = *amount_kg * 35.274;
        let alpha = *alpha_pct / 100.0;
        (utilization * alpha * oz * 7490.0) / vol_gal
    }).sum()
}
```

- [ ] **Step 4: Run to verify tests pass**

```bash
cd src-tauri && cargo test brewing::ibu
```

Expected: 3 tests pass.

- [ ] **Step 5: Add to `src-tauri/src/brewing/mod.rs`**

```rust
pub mod abv;
pub mod ibu;
pub mod og;
```

- [ ] **Step 6: Commit**

```bash
cd ..
git add src-tauri/src/brewing/
git commit -m "feat: IBU Tinseth calculation with tests"
```

---

### Task 10: SRM color calculation (Morey)

**Files:**
- Create: `src-tauri/src/brewing/srm.rs`
- Modify: `src-tauri/src/brewing/mod.rs`

- [ ] **Step 1: Write failing tests in `src-tauri/src/brewing/srm.rs`**

```rust
pub fn morey_srm(
    fermentables: &[(&f64, &f64)],  // (color_lovibond, amount_kg)
    batch_size_l: f64,
) -> f64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_srm_pale_ale() {
        // 5 kg pale malt at 1.5 lovibond, 23L → light golden, ~3 SRM
        let fermentables = vec![(&1.5f64, &5.0f64)];
        let srm = morey_srm(&fermentables, 23.0);
        assert!(srm > 2.0 && srm < 5.0, "SRM was {srm:.2}, expected ~3");
    }

    #[test]
    fn test_srm_stout() {
        // Mix of pale + roasted for a stout should give dark color
        let fermentables = vec![
            (&3.5f64, &5.0f64),   // pale malt
            (&300.0f64, &0.5f64), // roasted barley
        ];
        let srm = morey_srm(&fermentables, 23.0);
        assert!(srm > 30.0, "SRM was {srm:.1}, expected dark (>30)");
    }

    #[test]
    fn test_srm_empty() {
        let srm = morey_srm(&[], 23.0);
        assert_eq!(srm, 0.0);
    }
}
```

- [ ] **Step 2: Run to verify failure**

```bash
cd src-tauri && cargo test brewing::srm
```

Expected: FAIL.

- [ ] **Step 3: Implement**

```rust
pub fn morey_srm(
    fermentables: &[(&f64, &f64)],  // (color_lovibond, amount_kg)
    batch_size_l: f64,
) -> f64 {
    if fermentables.is_empty() {
        return 0.0;
    }
    let batch_gal = batch_size_l * 0.264172;
    let mcu: f64 = fermentables.iter().map(|(color_lovibond, amount_kg)| {
        let lbs = *amount_kg * 2.20462;
        (*color_lovibond * lbs) / batch_gal
    }).sum();
    1.4922 * mcu.powf(0.6859)
}
```

- [ ] **Step 4: Run to verify tests pass**

```bash
cd src-tauri && cargo test brewing::srm
```

Expected: 3 tests pass.

- [ ] **Step 5: Add to `src-tauri/src/brewing/mod.rs`**

```rust
pub mod abv;
pub mod ibu;
pub mod og;
pub mod srm;
```

- [ ] **Step 6: Commit**

```bash
cd ..
git add src-tauri/src/brewing/
git commit -m "feat: SRM Morey color calculation with tests"
```

---

### Task 11: Volume calculations

**Files:**
- Create: `src-tauri/src/brewing/volumes.rs`
- Modify: `src-tauri/src/brewing/mod.rs`

- [ ] **Step 1: Write failing tests in `src-tauri/src/brewing/volumes.rs`**

```rust
/// Returns (pre_boil_volume_l, post_boil_volume_l)
pub fn calculate_boil_volumes(
    batch_size_l: f64,
    boil_time_min: f64,
    evap_rate_pct_hr: f64,
    trub_chiller_loss_l: f64,
    fermenter_loss_l: f64,
    top_up_water_l: f64,
) -> (f64, f64) {
    todo!()
}

pub fn calculate_pre_boil_gravity(og: f64, post_boil_volume_l: f64, pre_boil_volume_l: f64) -> f64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boil_volumes_standard() {
        // Batch 23L, 60 min boil, 10%/hr evap, 1L trub loss, 1L fermenter loss, 0 top-up
        let (pre, post) = calculate_boil_volumes(23.0, 60.0, 10.0, 1.0, 1.0, 0.0);
        // post_boil = batch + trub + fermenter - top_up = 23 + 1 + 1 = 25L
        assert!((post - 25.0).abs() < 0.5, "post_boil was {post:.2}L, expected ~25L");
        // pre_boil = post_boil / (1 - evap_rate * time_hr) = 25 / 0.9 ≈ 27.8L
        assert!((pre - 27.8).abs() < 0.5, "pre_boil was {pre:.2}L, expected ~27.8L");
    }

    #[test]
    fn test_pre_boil_gravity() {
        // OG 1.050, 25L post-boil, 27.8L pre-boil → pre-boil gravity lower
        let pbg = calculate_pre_boil_gravity(1.050, 25.0, 27.8);
        assert!(pbg < 1.050, "Pre-boil gravity {pbg:.4} should be less than OG 1.050");
        assert!((pbg - 1.045).abs() < 0.003, "pbg was {pbg:.4}, expected ~1.045");
    }
}
```

- [ ] **Step 2: Run to verify failure**

```bash
cd src-tauri && cargo test brewing::volumes
```

Expected: FAIL.

- [ ] **Step 3: Implement**

```rust
pub fn calculate_boil_volumes(
    batch_size_l: f64,
    boil_time_min: f64,
    evap_rate_pct_hr: f64,
    trub_chiller_loss_l: f64,
    fermenter_loss_l: f64,
    top_up_water_l: f64,
) -> (f64, f64) {
    let post_boil = batch_size_l + trub_chiller_loss_l + fermenter_loss_l - top_up_water_l;
    let boil_hrs = boil_time_min / 60.0;
    let evap_fraction = evap_rate_pct_hr / 100.0 * boil_hrs;
    let pre_boil = post_boil / (1.0 - evap_fraction);
    (pre_boil, post_boil)
}

pub fn calculate_pre_boil_gravity(og: f64, post_boil_volume_l: f64, pre_boil_volume_l: f64) -> f64 {
    let og_points = (og - 1.0) * 1000.0;
    let pbg_points = og_points * post_boil_volume_l / pre_boil_volume_l;
    1.0 + pbg_points / 1000.0
}
```

- [ ] **Step 4: Run to verify tests pass**

```bash
cd src-tauri && cargo test brewing::volumes
```

Expected: 2 tests pass.

- [ ] **Step 5: Add to `src-tauri/src/brewing/mod.rs`**

```rust
pub mod abv;
pub mod ibu;
pub mod og;
pub mod srm;
pub mod volumes;
```

- [ ] **Step 6: Commit**

```bash
cd ..
git add src-tauri/src/brewing/
git commit -m "feat: volume calculations with tests"
```

---

### Task 12: RecipeStats orchestration

**Files:**
- Modify: `src-tauri/src/brewing/mod.rs`

- [ ] **Step 1: Replace `src-tauri/src/brewing/mod.rs` with the orchestrator**

```rust
pub mod abv;
pub mod ibu;
pub mod og;
pub mod srm;
pub mod volumes;

use crate::models::{
    EquipmentProfile, Recipe, RecipeAdditionFermentable, RecipeAdditionHop,
    RecipeAdditionYeast, RecipeStats,
};

pub fn calculate_stats(recipe: &Recipe) -> RecipeStats {
    let efficiency = recipe.efficiency_pct
        .or_else(|| recipe.equipment_profile.as_ref().map(|e| e.efficiency_pct))
        .unwrap_or(72.0);

    let fermentable_inputs: Vec<(&f64, &f64, bool)> = recipe.fermentables.iter()
        .map(|f| (&f.yield_pct, &f.amount_kg, f.add_after_boil))
        .collect();

    let og = og::calculate_og(&fermentable_inputs, recipe.batch_size_l, efficiency);

    let fg = recipe.yeasts.iter()
        .filter_map(|y| y.attenuation_pct)
        .next()
        .map(|att| abv::calculate_fg(og, att))
        .unwrap_or_else(|| abv::calculate_fg(og, 75.0));

    let abv_pct = abv::calculate_abv(og, fg);
    let calories = abv::calculate_calories_per_355ml(og, fg);

    let eq = recipe.equipment_profile.as_ref();
    let evap_rate = eq.map(|e| e.evap_rate_pct_hr).unwrap_or(10.0);
    let trub_loss = eq.map(|e| e.trub_chiller_loss_l).unwrap_or(1.0);
    let fermenter_loss = eq.map(|e| e.fermenter_loss_l).unwrap_or(1.0);
    let top_up = eq.map(|e| e.top_up_water_l).unwrap_or(0.0);

    let (pre_boil_volume_l, post_boil_volume_l) = volumes::calculate_boil_volumes(
        recipe.batch_size_l,
        recipe.boil_time_min,
        evap_rate,
        trub_loss,
        fermenter_loss,
        top_up,
    );

    let pre_boil_gravity = volumes::calculate_pre_boil_gravity(og, post_boil_volume_l, pre_boil_volume_l);

    let hop_inputs: Vec<(&f64, &f64, &f64, bool)> = recipe.hops.iter()
        .map(|h| (&h.alpha_pct, &h.amount_kg, &h.time_min, h.use_ == "dry hop"))
        .collect();

    let ibu = ibu::tinseth_ibu(&hop_inputs, og, post_boil_volume_l);

    let srm_inputs: Vec<(&f64, &f64)> = recipe.fermentables.iter()
        .map(|f| (&f.color_lovibond, &f.amount_kg))
        .collect();

    let srm = srm::morey_srm(&srm_inputs, recipe.batch_size_l);

    let gu = (og - 1.0) * 1000.0;
    let bu_gu_ratio = if gu > 0.0 { ibu / gu } else { 0.0 };

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
    }
}
```

- [ ] **Step 2: Verify compilation**

```bash
cd src-tauri && cargo build
```

Expected: compiles cleanly.

- [ ] **Step 3: Run all brewing tests together**

```bash
cd src-tauri && cargo test brewing
```

Expected: all 11 tests pass.

- [ ] **Step 4: Commit**

```bash
cd ..
git add src-tauri/src/brewing/
git commit -m "feat: RecipeStats orchestration in brewing::mod"
```

---

## Foundation complete

At this point:
- Tauri 2 + SvelteKit project scaffolded and building
- All 14 tables in SQLite migration
- All Rust models defined
- All brewing calculations implemented and tested (11 unit tests)
- `calculate_stats()` orchestrates OG → FG → ABV → IBU → SRM → volumes

Continue with `2026-05-03-brewski-backend.md` to implement all Tauri commands.
