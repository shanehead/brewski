# Yeast Extended Fields Design

**Date:** 2026-05-09
**Status:** Approved

## Goal

Add BeerMaverick yeast fields to the database schema and expose them through the full stack (DB → entity → model → OpenAPI → TypeScript → UI). The existing BeerXML-aligned fields stay intact; new fields are additive.

## New Fields

All new columns are nullable. Sourced from [BeerMaverick's yeast database](https://beermaverick.com/yeasts/) comparison tool.

| Column | Type | Description |
|---|---|---|
| `min_attenuation_pct` | REAL | Attenuation range lower bound (BeerXML has single `attenuation_pct`) |
| `max_attenuation_pct` | REAL | Attenuation range upper bound |
| `alcohol_tolerance` | TEXT | Categorical: "low", "medium", "high", "very_high" |
| `flavor_profile` | TEXT | Free-text flavor/aroma description |
| `styles` | TEXT | Suitable beer styles (e.g. "IPA, Pale Ale") |
| `substitutes` | TEXT | Substitute yeast strains (e.g. "OYL-061, LalBrew Voss") |
| `species` | TEXT | Biological species (e.g. "Saccharomyces cerevisiae") |
| `pof_positive` | INTEGER | Phenolic Off-Flavor gene present (0/1 boolean) |
| `sta1_positive` | INTEGER | STA-1 dextrin-fermenting gene present (0/1 boolean) |

## Architecture

### 1. Migration (`m004_yeast_extended_fields`)

Single migration with two responsibilities:
- `ALTER TABLE yeasts ADD COLUMN` for each of the 9 new nullable fields
- `DELETE` the 9 existing placeholder seed yeasts — they will be replaced by researched BeerMaverick data in a future task

Three files touched: `sql/004_yeast_extended_fields.sql`, `m004_yeast_extended_fields.rs`, and `migration/mod.rs` to register it.

Note: migrations will be collapsed before the first release.

### 2. SeaORM Entity (`src-tauri/src/entities/yeasts.rs`)

Add 9 new `Option<>` fields matching the new DB columns.

### 3. Model (`src-tauri/src/models.rs`)

Add 9 fields to the `Yeast` struct. A comment marks the BeerXML/BeerMaverick boundary:

```rust
// BeerXML fields
pub attenuation_pct: Option<f64>, // BeerXML single value; min/max range below

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
```

Update `TryFrom<entities::yeasts::Model>` to map all new fields. BeerXML import/export is unchanged.

### 4. OpenAPI Spec (`docs/openapi.yaml`)

Add the 9 new optional fields to the `Yeast` schema definition. Validate with `just lint-openapi`.

### 5. Frontend Types (`src/lib/api.ts`)

Add 9 new optional fields to the `Yeast` TypeScript interface.

### 6. Frontend Component (`src/lib/components/ingredients/YeastsTable.svelte`)

Surface the new fields in the yeast list display: flavor profile, alcohol tolerance, styles, and substitutes are the most useful for the library view.

## Out of Scope

- Populating BeerMaverick data for existing or new yeast strains (separate task)
- BeerXML import/export changes
- Any UI for creating/editing yeast library entries
