# Water Volumes Display

**Date:** 2026-06-09

## Problem

Brewski does not show how much water a brewer needs — mash water, sparge water, top-up water, or total water. Brewfather shows this prominently; users miss it when switching to Brewski.

## Goal

Add a "Water Volumes" card to the top of the Water tab showing a full water breakdown, with an informational message when mash water is automatically adjusted to fit the tun, and an error when it cannot be resolved.

## What We Show

| Row | Value | Visibility |
|---|---|---|
| Mash water | Calculated (see below) | Always |
| Sparge water | `total_water - mash_water` | Hidden when `sparge_method == "no_sparge"` |
| Top-up water | Base equipment value + any overflow adjustment | Hidden when 0 |
| **Total water** | `pre_boil_volume_l + grain_absorption * grain_kg + mash_tun_deadspace_l` | Always |
| Mash volume (water + grain) | `mash_water_l + grain_kg * GRAIN_DISPLACEMENT_L_PER_KG` | Always |

**Auto-adjustment message (yellow):** shown when `top_up_overflow_l > 0`. Mash water was automatically reduced to fit within `mash_volume_max_l`; the excess was moved to top-up. Message: "Water amounts have been adjusted by X gal/L to not exceed mash volume limit."

**Unresolvable overflow error (red):** shown when `mash_volume_excess_l != null`. The mash volume exceeds the limit but cannot be automatically resolved (e.g. kettle batch_volume_target mode). Message: "Mash volume exceeds tun capacity by X gal/L."

Values respect the user's unit setting (gal vs L), following the existing `lToGal` / `volumeLabel` pattern.

## Calculation Details

### Total water

```
total_water_l = pre_boil_volume_l + grain_absorption_rate_l_per_kg * total_grain_kg + mash_tun_deadspace_l
```

`mash_tun_deadspace_l` (volume trapped below the false bottom that cannot drain to the kettle) is **excluded** when `sparge_method == "no_sparge"` — on a no-sparge system this volume drains fully into the kettle and is not a loss.

`pre_boil_volume_l` is already computed in `calculate_stats()`. The other inputs come from the equipment profile (falling back to 0 if no profile is set).

### Mash water

Resolution order:
1. Mash step `infuse_amount_l` — explicit volume from the mash schedule
2. `mash.ratio_l_per_kg * total_grain_kg` — ratio-based
3. `total_water_l` — fallback (implies no-sparge / full-volume mash)

### Sparge water

```
sparge_water_l = total_water_l - mash_water_l
```

Set to `0.0` when `sparge_method == "no_sparge"` or when no mash is defined (in which case the fallback above collapses mash_water = total_water anyway).

For sparge modes, if mash water would exceed the mash volume limit, `calculate_water_volumes` caps mash water at the limit and redirects the excess to sparge water. Total water is unchanged.

### Mash volume

```
GRAIN_DISPLACEMENT_L_PER_KG = 0.67   // standard bulk grain density
mash_volume_l = mash_water_l + total_grain_kg * GRAIN_DISPLACEMENT_L_PER_KG
```

### Mash volume limit

The overflow check uses `equipment.mash_volume_max_l` (Brewfather's "Mash Volume Limit"). This is the maximum permissible mash volume including grain displacement.

```
mash_volume_excess_l = if mash_volume_max_l.is_some() && mash_volume_l > mash_volume_max_l {
    Some(mash_volume_l - mash_volume_max_l)
} else {
    None
}
```

### No-sparge overflow redistribution

When `sparge_method == "no_sparge"` and `batch_volume_target != "kettle"`, a mash volume overflow cannot be fixed by redirecting water to sparge. Instead, `calculate_stats` runs a second pass:

1. Add `excess` to `top_up_water` → `adj_top_up`
2. Recalculate `pre_boil` with `adj_top_up` (higher top-up → lower post-boil requirement → lower pre-boil)
3. Recalculate water volumes with the new `pre_boil` — mash water decreases by the same amount, bringing `mash_volume_l` back to the limit

The amount moved is recorded in `top_up_overflow_l` so the UI can display the informational message. After redistribution, `mash_volume_excess_l` is `None` (the overflow was resolved).

**Kettle mode exclusion:** when `batch_volume_target == "kettle"`, `pre_boil` is fixed as `batch_size_l` and is not derived from fermenter + losses + top_up. Increasing `top_up_water` has no effect on `pre_boil`, so the redistribution math does not apply. In that mode, `mash_volume_excess_l` is surfaced directly as an unresolvable overflow.

## Architecture

### RecipeStats (extend existing)

Fields added to `RecipeStats`:

| Field | Type | Notes |
|---|---|---|
| `mash_water_l` | `f64` | Required |
| `sparge_water_l` | `f64` | Required (0.0 when no-sparge) |
| `top_up_water_l` | `f64` | Required; includes any overflow adjustment |
| `total_water_l` | `f64` | Required |
| `mash_volume_l` | `f64` | Required |
| `mash_volume_excess_l` | `f64 \| null` | Non-null only for unresolvable overflow |
| `top_up_overflow_l` | `f64 \| null` | Non-null when overflow was auto-resolved via top-up |

### Rust changes

**`src-tauri/src/brewing/volumes.rs`** — `calculate_water_volumes`:

```rust
pub fn calculate_water_volumes(
    pre_boil_volume_l: f64,
    grain_absorption_rate_l_per_kg: f64,
    mash_tun_deadspace_l: f64,
    total_grain_kg: f64,
    mash_infuse_l: Option<f64>,
    mash_ratio_l_per_kg: Option<f64>,
    sparge_method: &str,
    tun_volume_l: Option<f64>,  // mash_volume_max_l passed as effective_tun_l
) -> (f64, f64, f64, f64, Option<f64>)
// returns (mash_water, sparge_water, total_water, mash_volume, mash_volume_excess)
```

Sparge-mode overflow is handled inside this function (cap mash, redirect to sparge). No-sparge overflow is returned as `mash_volume_excess_l` for the caller to handle.

**`src-tauri/src/brewing/mod.rs`** — two-pass redistribution after the first `calculate_water_volumes` call (no-sparge + fermenter mode only); populates the new fields on `RecipeStats`.

### Schema changes

**`docs/openapi/components/schemas/RecipeStats.yaml`** — 7 new required/nullable fields.

After editing, run the codegen pipeline to update `src-tauri/src/models.gen.rs` and `src/lib/api.gen.ts`.

### Frontend changes

**`src/lib/components/tabs/WaterTab.svelte`**
- `stats: RecipeStats | null` prop added
- "Water Volumes" card at the top of the tab body (before the Source Water card)
- Sparge row: `{#if stats.sparge_water_l > 0}`
- Top-up row: `{#if stats.top_up_water_l > 0}`
- Yellow adjustment message: `{#if stats.top_up_overflow_l != null && stats.top_up_overflow_l > 0}`
- Red overflow error: `{#if stats.mash_volume_excess_l != null}`

**`src/lib/desktop/RecipeView.svelte`** and mobile equivalent — `{stats}` added to the `WaterTab` call.
