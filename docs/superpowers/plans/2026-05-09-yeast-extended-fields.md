# Yeast Extended Fields Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add 9 BeerMaverick yeast fields to the DB schema and propagate them through entity, model, OpenAPI spec, TypeScript types, and UI — keeping BeerXML fields intact.

**Architecture:** A single SQLite migration (m004) adds 9 nullable columns to the `yeasts` table and deletes the 9 placeholder seed yeasts. The SeaORM entity, Rust model, OpenAPI spec, TypeScript interface, and YeastsTable component are each updated in sequence. No changes to BeerXML import/export.

**Tech Stack:** SQLite (via sqlx), Sea-ORM 1.x, Rust (Tauri 2), OpenAPI 3.1 (Redocly lint), SvelteKit + Svelte 5, TypeScript, Bun

---

## File Map

| File | Action | Responsibility |
|---|---|---|
| `src-tauri/src/migration/sql/004_yeast_extended_fields.sql` | Create | Raw SQL: ALTER TABLE + DELETE seed rows |
| `src-tauri/src/migration/m004_yeast_extended_fields.rs` | Create | Rust migration wrapper |
| `src-tauri/src/migration/mod.rs` | Modify | Register m004 |
| `src-tauri/src/entities/yeasts.rs` | Modify | SeaORM model for yeasts table |
| `src-tauri/src/models.rs` | Modify | `Yeast` struct + `TryFrom` conversion |
| `docs/openapi.yaml` | Modify | `Yeast` schema in IPC API spec |
| `src/lib/api.ts` | Modify | TypeScript `Yeast` interface |
| `src/lib/components/ingredients/YeastsTable.svelte` | Modify | Display new fields in library list |

---

## Task 1: SQL Migration

**Files:**
- Create: `src-tauri/src/migration/sql/004_yeast_extended_fields.sql`

- [ ] **Step 1: Write the SQL file**

```sql
ALTER TABLE yeasts ADD COLUMN min_attenuation_pct REAL;
ALTER TABLE yeasts ADD COLUMN max_attenuation_pct REAL;
ALTER TABLE yeasts ADD COLUMN alcohol_tolerance TEXT;
ALTER TABLE yeasts ADD COLUMN flavor_profile TEXT;
ALTER TABLE yeasts ADD COLUMN styles TEXT;
ALTER TABLE yeasts ADD COLUMN substitutes TEXT;
ALTER TABLE yeasts ADD COLUMN species TEXT;
ALTER TABLE yeasts ADD COLUMN pof_positive INTEGER;
ALTER TABLE yeasts ADD COLUMN sta1_positive INTEGER;

DELETE FROM yeasts WHERE id IN (
    'y-us05', 'y-1056', 'y-wlp001', 'y-s04', 'y-1084',
    'y-wlp300', 'y-t58', 'y-w34-70', 'y-s189'
);
```

- [ ] **Step 2: Commit**

```bash
git add src-tauri/src/migration/sql/004_yeast_extended_fields.sql
git commit -m "feat: add migration SQL for yeast extended fields"
```

---

## Task 2: Rust Migration Wrapper

**Files:**
- Create: `src-tauri/src/migration/m004_yeast_extended_fields.rs`
- Modify: `src-tauri/src/migration/mod.rs`

- [ ] **Step 1: Write the migration Rust file**

Create `src-tauri/src/migration/m004_yeast_extended_fields.rs`:

```rust
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str { "m004_yeast_extended_fields" }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(include_str!("sql/004_yeast_extended_fields.sql"))
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();
        for col in &[
            "min_attenuation_pct", "max_attenuation_pct", "alcohol_tolerance",
            "flavor_profile", "styles", "substitutes", "species",
            "pof_positive", "sta1_positive",
        ] {
            conn.execute_unprepared(&format!("ALTER TABLE yeasts DROP COLUMN {col}"))
                .await?;
        }
        Ok(())
    }
}
```

- [ ] **Step 2: Register the migration in `mod.rs`**

Current `src-tauri/src/migration/mod.rs`:
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

Replace with:
```rust
use sea_orm_migration::prelude::*;

mod m001_initial;
mod m002_seed_data;
mod m003_strike_temp;
mod m004_yeast_extended_fields;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m001_initial::Migration),
            Box::new(m002_seed_data::Migration),
            Box::new(m003_strike_temp::Migration),
            Box::new(m004_yeast_extended_fields::Migration),
        ]
    }
}
```

- [ ] **Step 3: Verify compilation**

```bash
cd src-tauri && cargo check 2>&1
```

Expected: no errors.

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/migration/m004_yeast_extended_fields.rs src-tauri/src/migration/mod.rs
git commit -m "feat: register m004 yeast extended fields migration"
```

---

## Task 3: SeaORM Entity

**Files:**
- Modify: `src-tauri/src/entities/yeasts.rs`

- [ ] **Step 1: Add the 9 new fields to the entity**

Current `src-tauri/src/entities/yeasts.rs`:
```rust
//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.20

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "yeasts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub id: String,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub r#type: String,
    #[sea_orm(column_type = "Text")]
    pub form: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub laboratory: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub product_id: Option<String>,
    pub min_temperature_c: Option<f64>,
    pub max_temperature_c: Option<f64>,
    #[sea_orm(column_type = "Text", nullable)]
    pub flocculation: Option<String>,
    pub attenuation_pct: Option<f64>,
    #[sea_orm(column_type = "Text", nullable)]
    pub notes: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub best_for: Option<String>,
    pub max_reuse: Option<i32>,
    pub add_to_secondary: Option<i32>,
}
```

Replace with:
```rust
//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.20

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "yeasts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub id: String,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub r#type: String,
    #[sea_orm(column_type = "Text")]
    pub form: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub laboratory: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub product_id: Option<String>,
    pub min_temperature_c: Option<f64>,
    pub max_temperature_c: Option<f64>,
    #[sea_orm(column_type = "Text", nullable)]
    pub flocculation: Option<String>,
    pub attenuation_pct: Option<f64>,
    #[sea_orm(column_type = "Text", nullable)]
    pub notes: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub best_for: Option<String>,
    pub max_reuse: Option<i32>,
    pub add_to_secondary: Option<i32>,
    // BeerMaverick extended fields (m004)
    pub min_attenuation_pct: Option<f64>,
    pub max_attenuation_pct: Option<f64>,
    #[sea_orm(column_type = "Text", nullable)]
    pub alcohol_tolerance: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub flavor_profile: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub styles: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub substitutes: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub species: Option<String>,
    pub pof_positive: Option<i32>,
    pub sta1_positive: Option<i32>,
}
```

- [ ] **Step 2: Verify compilation and tests pass**

```bash
cd src-tauri && cargo test 2>&1
```

Expected: all tests pass. The migration runs in `setup_test_db` which now applies m004 cleanly.

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/entities/yeasts.rs
git commit -m "feat: add BeerMaverick extended fields to yeasts entity"
```

---

## Task 4: Rust Model

**Files:**
- Modify: `src-tauri/src/models.rs` (lines ~202–240)

- [ ] **Step 1: Update the `Yeast` struct**

Find the `Yeast` struct in `src-tauri/src/models.rs` (around line 202). Replace it with:

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Yeast {
    // BeerXML fields
    pub id: String,
    pub name: String,
    pub type_: String,
    pub form: String,
    pub laboratory: Option<String>,
    pub product_id: Option<String>,
    pub min_temperature_c: Option<f64>,
    pub max_temperature_c: Option<f64>,
    pub flocculation: Option<String>,
    /// BeerXML single attenuation value; see min_attenuation_pct / max_attenuation_pct for range
    pub attenuation_pct: Option<f64>,
    pub notes: Option<String>,
    pub best_for: Option<String>,
    pub max_reuse: Option<i64>,
    pub add_to_secondary: bool,
    // BeerMaverick extended fields
    pub min_attenuation_pct: Option<f64>,
    pub max_attenuation_pct: Option<f64>,
    pub alcohol_tolerance: Option<String>,
    pub flavor_profile: Option<String>,
    pub styles: Option<String>,
    pub substitutes: Option<String>,
    pub species: Option<String>,
    pub pof_positive: Option<bool>,
    pub sta1_positive: Option<bool>,
}
```

- [ ] **Step 2: Update the `TryFrom` conversion**

Find `impl TryFrom<entities::yeasts::Model> for Yeast` (around line 220). Replace it with:

```rust
impl TryFrom<entities::yeasts::Model> for Yeast {
    type Error = AppError;
    fn try_from(m: entities::yeasts::Model) -> Result<Self, AppError> {
        Ok(Self {
            id: m.id,
            name: m.name,
            type_: m.r#type,
            form: m.form,
            laboratory: m.laboratory,
            product_id: m.product_id,
            min_temperature_c: m.min_temperature_c,
            max_temperature_c: m.max_temperature_c,
            flocculation: m.flocculation,
            attenuation_pct: m.attenuation_pct,
            notes: m.notes,
            best_for: m.best_for,
            max_reuse: m.max_reuse.map(|v| v as i64),
            add_to_secondary: m.add_to_secondary.unwrap_or(0) != 0,
            min_attenuation_pct: m.min_attenuation_pct,
            max_attenuation_pct: m.max_attenuation_pct,
            alcohol_tolerance: m.alcohol_tolerance,
            flavor_profile: m.flavor_profile,
            styles: m.styles,
            substitutes: m.substitutes,
            species: m.species,
            pof_positive: m.pof_positive.map(|v| v != 0),
            sta1_positive: m.sta1_positive.map(|v| v != 0),
        })
    }
}
```

- [ ] **Step 3: Run tests**

```bash
cd src-tauri && cargo test 2>&1
```

Expected: all tests pass.

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/models.rs
git commit -m "feat: add BeerMaverick extended fields to Yeast model"
```

---

## Task 5: OpenAPI Spec

**Files:**
- Modify: `docs/openapi.yaml` (around line 1450)

- [ ] **Step 1: Update the `Yeast` schema**

Find the `Yeast:` schema block in `docs/openapi.yaml` (around line 1450). Replace it with:

```yaml
    Yeast:
      type: object
      required: [id, name, type_, form, add_to_secondary]
      properties:
        id:
          type: string
        name:
          type: string
        type_:
          type: string
          description: "Ale, Lager, Wheat, Wine, Champagne"
        form:
          type: string
          description: "Liquid, Dry, Slant, Culture"
        laboratory:
          type: [string, "null"]
        product_id:
          type: [string, "null"]
        min_temperature_c:
          type: [number, "null"]
        max_temperature_c:
          type: [number, "null"]
        flocculation:
          type: [string, "null"]
          description: "Low, Medium, High, Very High"
        attenuation_pct:
          type: [number, "null"]
          description: "BeerXML single attenuation value; see min/max fields for range"
        notes:
          type: [string, "null"]
        best_for:
          type: [string, "null"]
        max_reuse:
          type: [integer, "null"]
        add_to_secondary:
          type: boolean
        min_attenuation_pct:
          type: [number, "null"]
          description: "BeerMaverick attenuation range lower bound"
        max_attenuation_pct:
          type: [number, "null"]
          description: "BeerMaverick attenuation range upper bound"
        alcohol_tolerance:
          type: [string, "null"]
          description: "low, medium, high, very_high"
        flavor_profile:
          type: [string, "null"]
        styles:
          type: [string, "null"]
          description: "Suitable beer styles, comma-separated"
        substitutes:
          type: [string, "null"]
          description: "Substitute yeast strains, comma-separated"
        species:
          type: [string, "null"]
          description: "e.g. Saccharomyces cerevisiae"
        pof_positive:
          type: [boolean, "null"]
          description: "Phenolic Off-Flavor gene present"
        sta1_positive:
          type: [boolean, "null"]
          description: "STA-1 dextrin-fermenting gene present"
```

- [ ] **Step 2: Lint the spec**

```bash
just lint-openapi 2>&1
```

Expected: no errors or warnings.

- [ ] **Step 3: Commit**

```bash
git add docs/openapi.yaml
git commit -m "feat: add BeerMaverick extended fields to Yeast OpenAPI schema"
```

---

## Task 6: TypeScript Interface

**Files:**
- Modify: `src/lib/api.ts` (around line 228)

- [ ] **Step 1: Update the `Yeast` interface**

Find the `export interface Yeast` block in `src/lib/api.ts` (around line 228). Replace it with:

```typescript
export interface Yeast {
  // BeerXML fields
  id: string;
  name: string;
  type_: string;
  form: string;
  laboratory: string | null;
  product_id: string | null;
  min_temperature_c: number | null;
  max_temperature_c: number | null;
  flocculation: string | null;
  /** BeerXML single attenuation value; see min/max fields for range */
  attenuation_pct: number | null;
  notes: string | null;
  best_for: string | null;
  max_reuse: number | null;
  add_to_secondary: boolean;
  // BeerMaverick extended fields
  min_attenuation_pct: number | null;
  max_attenuation_pct: number | null;
  alcohol_tolerance: string | null;
  flavor_profile: string | null;
  styles: string | null;
  substitutes: string | null;
  species: string | null;
  pof_positive: boolean | null;
  sta1_positive: boolean | null;
}
```

- [ ] **Step 2: Run type check**

```bash
just check 2>&1
```

Expected: no TypeScript errors.

- [ ] **Step 3: Commit**

```bash
git add src/lib/api.ts
git commit -m "feat: add BeerMaverick extended fields to Yeast TypeScript interface"
```

---

## Task 7: YeastsTable Component

**Files:**
- Modify: `src/lib/components/ingredients/YeastsTable.svelte`

**Note:** `recipe.yeasts` is `RecipeAdditionYeast[]`, not `Yeast[]`. The new BeerMaverick fields are on the library `Yeast` type only. The recipe list display stays unchanged; the library dropdown option label gets a flavor profile hint when available.

- [ ] **Step 1: Update the library dropdown to show flavor profile**

Replace the entire component body with:

```svelte
<script lang="ts">
  import type { Recipe, Yeast } from "$lib/api";
  import { listYeastLibrary, createRecipeYeast, deleteRecipeYeast } from "$lib/api";
  import { onMount } from "svelte";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  let library = $state<Yeast[]>([]);
  let adding = $state(false);
  let selectedLibId = $state("");

  onMount(async () => { library = await listYeastLibrary(); });

  const selectedLib = $derived(library.find((y) => y.id === selectedLibId));

  function libraryOptionLabel(y: Yeast): string {
    const base = `${y.name} (${y.laboratory ?? y.form})`;
    const attenuation = y.min_attenuation_pct != null && y.max_attenuation_pct != null
      ? `${y.min_attenuation_pct}–${y.max_attenuation_pct}%`
      : y.attenuation_pct != null ? `${y.attenuation_pct}%` : null;
    return attenuation ? `${base} · ${attenuation}` : base;
  }

  async function handleAdd() {
    if (!selectedLib) return;
    await createRecipeYeast(recipe.id, {
      yeast_id: selectedLib.id,
      name: selectedLib.name,
      type_: selectedLib.type_,
      form: selectedLib.form,
      laboratory: selectedLib.laboratory,
      product_id: selectedLib.product_id,
      attenuation_pct: selectedLib.attenuation_pct,
      amount: 1,
    });
    adding = false;
    selectedLibId = "";
    onchange();
  }

  async function handleDelete(id: string) {
    await deleteRecipeYeast(id);
    onchange();
  }
</script>

<div class="flex flex-col gap-2">
  <div class="flex items-center justify-between">
    <h3 class="text-sm font-semibold" style="color: var(--color-text-primary);">Yeast</h3>
    <button onclick={() => adding = !adding} class="text-xs px-2 py-1 rounded"
            style="background: var(--color-accent); color: #fff;">+ Add</button>
  </div>

  {#if adding}
    <div class="flex gap-2 items-end p-2 rounded" style="background: var(--color-bg-elevated);">
      <div class="flex-1">
        <label for="yeast-select" class="text-xs mb-1 block" style="color: var(--color-text-secondary);">Yeast</label>
        <select id="yeast-select" bind:value={selectedLibId} class="w-full px-2 py-1.5 rounded text-sm"
                style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);">
          <option value="">Choose…</option>
          {#each library as y}
            <option value={y.id}>{libraryOptionLabel(y)}</option>
          {/each}
        </select>
      </div>
      <button onclick={handleAdd} class="text-xs px-3 py-1.5 rounded"
              style="background: var(--color-accent); color: #fff;">Add</button>
      <button onclick={() => adding = false} class="text-xs px-2 py-1.5 rounded"
              style="color: var(--color-text-secondary);">Cancel</button>
    </div>
  {/if}

  {#each recipe.yeasts as y (y.id)}
    <div class="flex items-center justify-between py-1.5 border-t" style="border-color: var(--color-border);">
      <div>
        <p class="text-sm" style="color: var(--color-text-primary);">{y.name}</p>
        <p class="text-xs" style="color: var(--color-text-secondary);">
          {y.laboratory ?? ""} {y.product_id ?? ""} · {y.attenuation_pct ?? "?"}% attenuation
        </p>
      </div>
      <button onclick={() => handleDelete(y.id)} class="text-xs opacity-40 hover:opacity-100"
              style="color: var(--color-text-secondary);">×</button>
    </div>
  {/each}
</div>
```

- [ ] **Step 2: Run type check**

```bash
just check 2>&1
```

Expected: no TypeScript errors.

- [ ] **Step 3: Commit**

```bash
git add src/lib/components/ingredients/YeastsTable.svelte
git commit -m "feat: display BeerMaverick extended fields in YeastsTable"
```
