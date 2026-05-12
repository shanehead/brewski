# Water Chemistry — Layers 1 & 2

**Date:** 2026-05-10
**Status:** Approved

## Goal

Add water chemistry to recipe editing: a library of named source water profiles, per-recipe source water selection (separate mash and sparge), salt/acid addition tracking, and a live adjusted mineral profile summary.

Mash pH estimation (Layer 3) is out of scope and will be designed separately.

## Architecture

Three concerns:

1. **Water profile library** — named profiles (Distilled, RO, Pilsen, etc.) seeded in the existing `waters` table
2. **Per-recipe source water** — mash and sparge source profiles stored as FKs on `recipes`
3. **Per-recipe additions** — salt/acid additions in a new `recipe_water_adjustments` table

Adjusted profile calculation lives in Rust (consistent with existing brewing math). The frontend calls `calculate_water_profile` reactively on any change and renders the result.

## Schema

### Migration — two changes

**Add columns to `recipes`:**
```sql
ALTER TABLE recipes ADD COLUMN mash_water_id   TEXT REFERENCES waters(id);
ALTER TABLE recipes ADD COLUMN sparge_water_id TEXT REFERENCES waters(id);
```
If `sparge_water_id` is NULL, sparge uses the same profile as mash.

**New table:**
```sql
CREATE TABLE recipe_water_adjustments (
  id          TEXT PRIMARY KEY,
  recipe_id   TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
  addition    TEXT NOT NULL,
  target      TEXT NOT NULL,
  amount      REAL NOT NULL
);
```

`addition` values: `gypsum`, `calcium_chloride`, `epsom_salt`, `table_salt`, `baking_soda`, `chalk`, `lactic_acid`, `phosphoric_acid`

`target` values: `mash`, `sparge`

`amount` unit: grams for salts, ml for acids (implied by addition type — a well-known homebrewing convention)

The `waters` table is unchanged.

### Seed data

Seed these profiles into `001_initial.sql`:

| Name | Ca | Mg | Na | Cl | SO₄ | HCO₃ | pH |
|------|----|----|----|----|-----|------|----|
| Distilled | 0 | 0 | 0 | 0 | 0 | 0 | 7.0 |
| Reverse Osmosis | 3 | 1 | 2 | 2 | 3 | 10 | 6.5 |
| Pilsen | 7 | 3 | 2 | 5 | 5 | 25 | 7.1 |
| Munich | 77 | 17 | 4 | 8 | 18 | 295 | 7.7 |
| Vienna | 75 | 15 | 10 | 15 | 60 | 225 | 7.5 |
| London | 70 | 6 | 15 | 37 | 40 | 166 | 7.4 |
| Edinburgh | 120 | 25 | 55 | 65 | 140 | 285 | 7.2 |
| Dublin | 118 | 4 | 12 | 19 | 54 | 315 | 7.4 |
| Burton | 275 | 40 | 30 | 35 | 725 | 270 | 6.5 |

## Tauri Commands

Follow the schema-first workflow: update the OpenAPI spec first, then `just gen`, then implement.

| Command | Input | Output |
|---------|-------|--------|
| `set_recipe_water_sources` | `recipe_id`, `mash_water_id?`, `sparge_water_id?` | `Recipe` |
| `create_water_adjustment` | `recipe_id`, `addition`, `target`, `amount` | `RecipeWaterAdjustment` |
| `update_water_adjustment` | `id`, `addition?`, `target?`, `amount?` | `RecipeWaterAdjustment` |
| `delete_water_adjustment` | `id` | — |
| `calculate_water_profile` | `recipe_id` | `CalculatedWaterProfile` |

### `CalculatedWaterProfile` shape

```
{
  mash:     WaterProfile,
  sparge:   WaterProfile,
  combined: WaterProfile
}
```

Each `WaterProfile`:
```
{
  calcium_ppm, magnesium_ppm, sodium_ppm,
  chloride_ppm, sulfate_ppm, bicarbonate_ppm,
  cl_so4_ratio
}
```

### Ion contribution constants (Rust)

Stored as a constant in `src-tauri/src/brewing/mod.rs` alongside the existing strike temp math:

| Addition | Ca | Mg | Na | Cl | SO₄ | HCO₃ |
|----------|----|----|----|----|-----|------|
| Gypsum (CaSO₄·2H₂O) | 61.5 | — | — | — | 147.4 | — |
| Calcium Chloride (CaCl₂·2H₂O) | 72.0 | — | — | 127.5 | — | — |
| Epsom Salt (MgSO₄·7H₂O) | — | 26.1 | — | — | 103.0 | — |
| Table Salt (NaCl) | — | — | 104.0 | 160.6 | — | — |
| Baking Soda (NaHCO₃) | — | — | 75.3 | — | — | 190.7 |
| Chalk (CaCO₃) | 105.9 | — | — | — | — | 158.4 |

(ppm per gram per US gallon — the standard homebrewing unit. Rust implementation converts volume from liters: `ppm_increase = (constant * amount_g) / (volume_l / 3.785)`)

Acid additions (lactic, phosphoric) affect pH only — out of scope for Layer 2 adjusted profile; store the amount but omit from mineral math until Layer 3.

## Frontend — Water Tab

New `Water` tab on the recipe detail page.

**Source water section:**
- Mash Water dropdown (`list_waters`)
- Sparge Water dropdown — with "Same as mash" toggle that sets `sparge_water_id` to null
- Calls `set_recipe_water_sources` on change

**Additions section:**
- Two sub-tables: Mash and Sparge
- One row per addition type; hidden if amount is 0, revealed via "Add" button
- Inline number input; saves via `create_water_adjustment` / `update_water_adjustment` / `delete_water_adjustment`

**Adjusted profile summary:**
- Read-only mineral grid: Ca, Mg, Na, Cl, SO₄, HCO₃ — showing source ppm and adjusted ppm side by side
- Cl:SO₄ ratio with label: < 0.5 = "Hoppy", 0.5–1.5 = "Balanced", > 1.5 = "Malty"
- Recomputed via `calculate_water_profile` on any source or addition change

## Volume Derivation

Mash volume: `mashes.ratio_l_per_kg × total_grain_weight_kg` (grain weight summed from `recipe_addition_fermentables`)

Sparge volume: `recipes.batch_size_l + boil_evaporation - mash_volume` (boil evaporation from equipment profile if linked, otherwise a reasonable default)

If volumes cannot be derived (no mash profile, no fermentables), the calculation returns the source water profile unchanged.

## Out of Scope

- Mash pH estimation (Layer 3 — separate design)
- Water blending (mixing two sources at a ratio)
- Acid effect on mineral profile
