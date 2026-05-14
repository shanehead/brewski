# Hopstand Temperature: Units-Aware Display

**Date:** 2026-05-13
**Status:** Approved

## Problem

Two gaps in the hopstand temperature UI:

1. The temperature input in `IngredientPicker` is hardcoded to `°C` — imperial users see and enter Celsius with no conversion.
2. The hops table rows show Use as plain text (e.g. `hopstand`) with no temperature — the per-hop temperature is invisible after adding.

## Design

### HopsTable — Use cell with inline temperature

For hopstand hops, render the Use cell as `hopstand (176°F)` or `hopstand (80°C)` depending on the user's units setting. For all other use types, render unchanged.

- `h.hopstand_temp_c` is the stored value (always °C, nullable)
- Convert with `cToF` from `$lib/units` when `units === 'imperial'`
- Format: `${h.use_} (${displayTemp}${tempLabel(units)})` — only when `h.use_ === 'hopstand' && h.hopstand_temp_c != null`

### IngredientPicker — unit-aware temperature input

The `hopstand_temp_c` state variable stays in °C internally. The input display and label are derived from the user's units:

- **Label:** `Temp (${tempLabel(units)})` using existing `tempLabel` utility
- **Displayed value:** `cToF(hopstand_temp_c)` when imperial, raw value when metric
- **On input change:** `fToC(v)` if imperial before storing in `hopstand_temp_c`, direct assignment if metric
- **Default on hop select:** `hopstand_temp_c = 80` (80°C = 176°F) — unchanged; display handles conversion

No changes to the API payload, backend, or stored data format. Temperature is always persisted in °C.

## Files Changed

- `src/lib/components/ingredients/HopsTable.svelte` — Use cell rendering
- `src/lib/components/ingredients/IngredientPicker.svelte` — temp input label, display value, and oninput handler

## Testing

Manual verification:
- Add a hopstand hop in metric → temp input shows `°C`, table shows `hopstand (80°C)`
- Add a hopstand hop in imperial → temp input shows `°F` with 176 default, table shows `hopstand (176°F)`
- Non-hopstand hops → Use cell unchanged, no temp input shown in picker
