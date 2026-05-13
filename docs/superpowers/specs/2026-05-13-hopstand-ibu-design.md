# Hopstand, First Wort & Mash Hop IBU Calculation

**Date:** 2026-05-13
**Status:** Approved

## Problem

The current Tinseth IBU calculation in `src-tauri/src/brewing/ibu.rs` only special-cases dry hop additions (0 IBU). All other use types — including hopstand/whirlpool, first wort, and mash additions — are fed through the standard Tinseth formula using their stored `time_min`, producing incorrect results.

## Decisions

| Use type | IBU treatment | Rationale |
|---|---|---|
| `"Boil"` | Tinseth with `time_min` | Standard; existing behaviour |
| `"Aroma"` | Tinseth with `time_min` | Late boil addition; existing behaviour |
| `"Hopstand"` | Tinseth with Malowicki-adjusted time | Sub-boiling isomerization via Malowicki & Shellhammer (2005) |
| `"First Wort"` | Tinseth with recipe `boil_time_min` | Treated as full-boil addition; research-backed simplification |
| `"Mash"` | 0 IBU | Mash temps (65–68°C) produce negligible isomerization; grain absorption removes what little forms |
| `"Dry Hop"` | 0 IBU | Existing behaviour |

**Whirlpool temperature resolution for hopstand additions:**
1. Per-hop `whirlpool_temp_c` (optional override)
2. Recipe-level `whirlpool_temp_c` (optional)
3. Default: **80°C**

Resolution happens in `calculate_stats()` before constructing the input struct, so the IBU function always receives a concrete value.

## Data Model

### Database migration (new migration file)

```sql
ALTER TABLE recipes ADD COLUMN whirlpool_temp_c REAL;
ALTER TABLE recipe_addition_hops ADD COLUMN whirlpool_temp_c REAL;
```

Both columns are nullable. No default at the database level — defaults are applied in the calculation layer.

### Rust model changes

`Recipe` gains:
```rust
pub whirlpool_temp_c: Option<f64>,
```

`RecipeAdditionHop` gains:
```rust
pub whirlpool_temp_c: Option<f64>,
```

TypeScript types are regenerated from the updated schema via existing codegen.

## Calculation Design

### `HopIbuInput` struct

Replaces the current `(&f64, &f64, &f64, bool)` tuple:

```rust
pub struct HopIbuInput<'a> {
    pub alpha_pct: &'a f64,
    pub amount_kg: &'a f64,
    pub time_min: &'a f64,
    pub use_type: &'a str,
    pub whirlpool_temp_c: f64, // pre-resolved: per-hop → recipe → 80.0
}
```

### `malowicki_effective_time`

Standalone pub function in `ibu.rs`. Implements the Malowicki & Shellhammer (2005) isomerization rate model to convert actual hopstand minutes into boil-equivalent minutes:

```rust
// k1(T) = 7.9e11 * exp(-11858 / T), T in Kelvin
// Returns boil-equivalent minutes at the given sub-boiling temperature.
pub fn malowicki_effective_time(actual_min: f64, temp_c: f64) -> f64 {
    let t = temp_c + 273.15;
    let k1_t = 7.9e11 * f64::exp(-11858.0 / t);
    let k1_boil = 7.9e11 * f64::exp(-11858.0 / 373.15);
    actual_min * (k1_t / k1_boil)
}
```

At 80°C this produces ~17% of the boiling rate, so a 20-min stand ≈ 3.4 effective boil minutes.

### `tinseth_ibu` branching

The function gains a `boil_time_min: f64` parameter for first wort handling. Branching by `use_type`:

```
"Mash" | "Dry Hop"  → 0.0
"First Wort"        → Tinseth(boil_time_min)
"Hopstand"          → Tinseth(malowicki_effective_time(time_min, whirlpool_temp_c))
everything else     → Tinseth(time_min)
```

### `calculate_stats` changes

Builds `HopIbuInput` per hop addition, resolving `whirlpool_temp_c`:

```rust
let whirlpool_default = recipe.whirlpool_temp_c.unwrap_or(80.0);
let hop_inputs: Vec<HopIbuInput> = recipe.hops.iter().map(|h| HopIbuInput {
    alpha_pct: &h.alpha_pct,
    amount_kg: &h.amount_kg,
    time_min: &h.time_min,
    use_type: &h.use_,
    whirlpool_temp_c: h.whirlpool_temp_c.unwrap_or(whirlpool_default),
}).collect();

let ibu = ibu::tinseth_ibu(&hop_inputs, og, post_boil_volume_l, recipe.boil_time_min);
```

## Testing

### Unit tests for `malowicki_effective_time`

- At 100°C: effective time equals actual time (ratio = 1.0)
- At 80°C: effective time is ~17% of actual (spot-check the kinetics)
- At 0°C: effective time is essentially 0

### Unit tests for `tinseth_ibu`

- Mash addition → 0 IBU
- Hopstand at 80°C for 20 min → fewer IBUs than the same hop as a 20-min boil addition
- Hopstand at 100°C for 20 min → same IBUs as a 20-min boil addition (degenerate case)
- First wort → same IBUs as a full-boil addition (uses `boil_time_min`)
- Existing dry hop and boil tests updated to use `HopIbuInput` struct

### Fixture test updates

**`nectaron_hazy_dipa.xml`:** Change the `"Aroma"` addition (`TIME=20`) to `"Hopstand"`. This exercises the Malowicki path end-to-end. Recalculate expected IBU using Malowicki at 80°C default and update the fixture expected value. Remove the "boil-only IBU (hopstand excluded)" comment from the test.

No new fixture file is required.

## String Matching

Use type strings must be matched case-insensitively (e.g., `"Dry Hop"` vs `"dry hop"`). The implementation should normalize `use_type` to lowercase before branching, consistent with how the existing dry hop check should work. Any existing case-sensitive comparison in `mod.rs` should be fixed as part of this work.
