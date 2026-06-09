# Water Volumes Display

**Date:** 2026-06-09

## Problem

Brewski does not show how much water a brewer needs — mash water, sparge water, top-up water, or total water. Brewfather shows this prominently; users miss it when switching to Brewski.

## Goal

Add a "Water Volumes" card to the top of the Water tab showing a full water breakdown, with a tun overflow warning when applicable.

## What We Show

| Row | Value | Visibility |
|---|---|---|
| Mash water | Calculated (see below) | Always |
| Sparge water | `total_water - mash_water` | Hidden when `sparge_method == "no_sparge"` |
| Top-up water | `equipment.top_up_water_l` | Always |
| **Total water** | `pre_boil_volume_l + grain_absorption * grain_kg + lauter_deadspace_l` | Always |
| Mash volume (water + grain) | `mash_water_l + grain_kg * GRAIN_DISPLACEMENT_L_PER_KG` | Always |

**Tun overflow warning:** shown when equipment has `tun_volume_l` set and `mash_volume_l > tun_volume_l`. Message: "Mash volume exceeds tun capacity by X gal/L."

Values respect the user's unit setting (gal vs L), following the existing `lToGal` / `volumeLabel` pattern.

## Calculation Details

### Total water

```
total_water_l = pre_boil_volume_l + grain_absorption_rate_l_per_kg * total_grain_kg + lauter_deadspace_l
```

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

### Mash volume

```
GRAIN_DISPLACEMENT_L_PER_KG = 0.67   // standard bulk grain density
mash_volume_l = mash_water_l + total_grain_kg * GRAIN_DISPLACEMENT_L_PER_KG
```

### Tun overflow

```
mash_volume_excess_l = if tun_volume_l.is_some() && mash_volume_l > tun_volume_l {
    Some(mash_volume_l - tun_volume_l)
} else {
    None
}
```

## Architecture

### RecipeStats (extend existing)

New fields added to `RecipeStats`:

| Field | Type | Notes |
|---|---|---|
| `mash_water_l` | `f64` | Required |
| `sparge_water_l` | `f64` | Required (0.0 when no-sparge) |
| `top_up_water_l` | `f64` | Required (0.0 when no equipment profile) |
| `total_water_l` | `f64` | Required |
| `mash_volume_l` | `f64` | Required |
| `mash_volume_excess_l` | `f64 \| null` | None when no overflow or no tun volume set |

### Rust changes

**`src-tauri/src/brewing/volumes.rs`** — new function:

```rust
pub fn calculate_water_volumes(
    pre_boil_volume_l: f64,
    grain_absorption_rate_l_per_kg: f64,
    lauter_deadspace_l: f64,
    total_grain_kg: f64,
    mash_infuse_l: Option<f64>,        // from mash step
    mash_ratio_l_per_kg: Option<f64>,  // from mash.ratio_l_per_kg
    top_up_water_l: f64,
    sparge_method: &str,
    tun_volume_l: Option<f64>,
) -> (f64, f64, f64, f64, f64, Option<f64>)
// returns (mash_water, sparge_water, top_up, total_water, mash_volume, mash_volume_excess)
```

**`src-tauri/src/brewing/mod.rs`** — call the new function after volumes are computed; populate the new fields on `RecipeStats`.

### Schema changes

**`docs/openapi/components/schemas/RecipeStats.yaml`** — add 6 new required/nullable fields following the existing pattern.

After editing, run the codegen pipeline to update `src-tauri/src/models.gen.rs` and `src/lib/api.gen.ts`.

### Frontend changes

**`src/lib/components/tabs/WaterTab.svelte`**
- Add `stats: RecipeStats | null` to the component props (same signature as `MashTab`)
- Add a "Water Volumes" card at the top of the tab body (before the Source Water card)
- Sparge row uses `{#if stats.sparge_water_l > 0}` — zero means no-sparge
- Tun warning uses `{#if stats.mash_volume_excess_l != null}`

**`src/lib/desktop/RecipeView.svelte`**, **`src/lib/desktop/BaselineRecipeView.svelte`**, **`src/lib/mobile/RecipeView.svelte`**, **`src/lib/mobile/BaselineRecipeView.svelte`** — each already passes `stats` to other tabs; add `{stats}` to the `WaterTab` call.

## Non-goals

- Adjusting mash water when it would exceed tun volume (Brewfather does this; we only warn)
- Displaying volumes anywhere other than the Water tab
- Adding a separate API command for water volumes
