# Hop Format Support

**Date:** 2026-05-20  
**Status:** Approved

## Overview

Extend hop form support from `{Pellet, Plug, Leaf}` to `{Pellet, Plug, Leaf, Cryo, CO2 Extract}`. Each form affects IBU utilization differently. Utilization multipliers are hardcoded (not user-configurable). Implementation follows Option A: extend in-place, no new columns or migrations.

## Data Model

No schema migration needed. The `form` field is already `TEXT` on three tables:

- `hops` (library)
- `recipe_addition_hops` (recipe additions)
- `recipe_version_hops` (version snapshots)

Valid form values expand to: `Pellet`, `Plug`, `Leaf`, `Cryo`, `CO2 Extract`.

The seeded hop library (231 hops) remains unchanged — all seeded hops are `Pellet`, which is correct.

## IBU Calculation

**File:** `src-tauri/src/brewing/ibu.rs`

### Changes to `HopIbuInput`

Add `form: &str` field. The `brewing/mod.rs` caller already has access to `form` on the hop addition entity and passes it through.

### New `form_utilization` function

```rust
fn form_utilization(form: &str) -> f64 {
    match form.to_lowercase().as_str() {
        "pellet" | "cryo" => 1.0,
        "leaf" | "plug"   => 0.85,
        _                 => 1.0,
    }
}
```

Cryo returns `1.0` because its concentration is already reflected in the alpha acid percentage — users enter the AA% from the package label, which is ~2× higher than equivalent pellets. No separate multiplier needed.

### CO2 Extract code path

Before the main Tinseth logic, check for CO2 Extract. If use type is `dry hop` or `mash`, return `0.0` as normal. Otherwise, skip the time factor and apply bigness only:

```
IBU = (bigness × alpha_fraction × ounces × 7490) / volume_gallons
```

Bigness is still applied (wort gravity suppresses utilization regardless of form). There is no time factor — full isomerization is assumed regardless of boil duration.

### Standard form multiplier

For all other forms, multiply the final Tinseth IBU result by `form_utilization(form)`.

### Utilization reference table

| Form        | Utilization vs Pellet | IBU calculation         |
|-------------|----------------------|-------------------------|
| Pellet      | 1.0×                 | Standard Tinseth        |
| Cryo        | 1.0×                 | Standard Tinseth (AA% accounts for concentration) |
| Leaf        | 0.85×                | Tinseth × 0.85          |
| Plug        | 0.85×                | Tinseth × 0.85          |
| CO2 Extract | full utilization     | Bigness only, no time factor |

## Backend

No new Tauri commands, repositories, or entities required. The only backend change is `ibu.rs` and the `HopIbuInput` construction in `brewing/mod.rs`.

OpenAPI spec (`docs/openapi/paths/commands/`) updated: the `form` field description for hop-related schemas changes from `Pellet, Plug, Leaf` to `Pellet, Plug, Leaf, Cryo, CO2 Extract`.

Generated TypeScript types (`src/lib/api.gen.ts`) regenerated after spec update.

## UI

### Ingredient picker — `IngredientPicker.svelte`

Add a `Form` dropdown between hop name and alpha acid fields. Styled consistently with other fields in the picker. Defaults to `Pellet`. When a hop is selected from the library, the form field prepopulates from the library hop's `form` value.

### Library editor — `IngredientEditModal.svelte`

Add `Cryo` and `CO2 Extract` to the existing form dropdown. Current options are `Pellet`, `Plug`, `Leaf`.

### Hops table — `HopsTable.svelte`

Show a small badge next to the hop name for non-Pellet forms only. Badge is omitted when form is `Pellet` to keep all-pellet recipes uncluttered.

Badge colors:
- `Cryo` → green
- `CO2 Extract` → purple  
- `Leaf` / `Plug` → neutral gray

## Out of Scope

- User-configurable utilization multipliers (hardcoded by design)
- Pre-isomerized extracts (Iso-extract, Tetra-hop) — requires a completely different IBU contribution path; tiny user base
- Updating seeded hop library with cryo-specific entries
- Inventory tracking of hop formats
