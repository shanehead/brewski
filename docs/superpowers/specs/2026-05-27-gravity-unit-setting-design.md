# Gravity Unit Setting — Design Spec

**Date:** 2026-05-27
**Status:** Approved

## Overview

Add a gravity unit preference to app settings (SG / Plato / Brix). All gravity displays in recipes and batches default to the chosen unit. Gravity input fields accept values in the preferred unit and convert to SG before saving. The Rust backend handles all conversions via the existing `convertGravity` IPC command.

## Data Model

Add one field to `AppSettings` in `src/lib/stores/settings.ts`:

```ts
gravity_unit?: "sg" | "plato" | "brix";
```

Default is `"sg"` when unset. `GravityUnit = "sg" | "plato" | "brix"` is already defined in `src/lib/api.ts` — no new types needed.

The setting is persisted via the existing `saveSetting` / `updateSetting` mechanism, the same as `units` and `theme`.

## Settings UI

A new dropdown is added to the **Units** section of `src/routes/settings/+page.svelte`, below the existing Measurement System row:

- **Label:** Gravity Unit
- **Options:** SG (1.050) | Plato (°P) | Brix (°Bx)
- Handler follows the same pattern as `handleUnitsChange`.

## Conversion Pattern

All gravity values are stored in and returned from the backend as specific gravity (SG). The existing IPC command `convertGravity(value, fromUnit)` in `src/lib/api.ts` returns `{ sg, plato, brix }`.

**Display (SG → preferred unit):**
Call `convertGravity(sgValue, "sg")` and read the field matching the preferred unit.

**Input (preferred unit → SG):**
Call `convertGravity(userValue, preferredUnit)` and read `.sg` before saving to the backend.

Components store converted display values in local `$state`, populated by `$effect` that re-runs whenever the raw SG value or `$settings.gravity_unit` changes.

**Formatting rules:**

| Unit  | Decimals | Suffix |
|-------|----------|--------|
| sg    | 3        | none   |
| plato | 1        | °P     |
| brix  | 1        | °Bx    |

Progress bar width calculations in `StatsSidebar` continue to use raw SG values (proportional math doesn't need conversion).

## Affected Components

### `src/lib/stores/settings.ts`
- Add `gravity_unit?: GravityUnit` to `AppSettings`.

### `src/routes/settings/+page.svelte`
- Add gravity unit `<select>` in the Units section.
- Add `handleGravityUnitChange` handler using `saveSetting("gravity_unit", ...)`.

### `src/lib/components/StatsSidebar.svelte`
- Add `$effect` blocks to convert `stats.og`, `stats.fg`, and `stats.pre_boil_gravity` when stats or `$settings.gravity_unit` changes.
- Display converted values with appropriate suffix.
- Keep SG values for progress bar `pct()` calls.

### `src/lib/components/batch/BatchGravityTab.svelte`
- **Display:** Convert each `r.gravity` (SG) to preferred unit via `convertGravity` for the table column.
- **Input:** Change placeholder text to reflect preferred unit (e.g. "Gravity (°Bx)"). On add, call `convertGravity(userValue, preferredUnit)` to get SG before passing to `addGravityReading`.
- Input `step` adapts: `0.001` for SG, `0.1` for Plato/Brix.

### `src/lib/components/batch/BatchOverviewTab.svelte`
- **Stage targets banner:** Convert OG, FG, pre-boil gravity values before building the `stageTargets` array.
- **Measurements grid:** For the three gravity fields (`actual_pre_boil_gravity`, `actual_og`, `actual_fg`), display values converted to preferred unit and accept input in preferred unit. On blur, convert back to SG before calling `onUpdate`.
- `decimals: 3` for gravity fields becomes dynamic based on unit.

### `src/lib/mobile/BatchView.svelte`
- Audit for any gravity display and apply the same conversion pattern used in desktop `BatchOverviewTab`.

## Out of Scope

- Per-recipe or per-batch gravity unit overrides (global setting only).
- ABV formula in `BatchOverviewTab` (existing calculation; no change).
- Gravity unit in list views (`RecipeList`, `BatchList`) — gravity is not displayed there.
- Backend schema changes — SG remains the storage format.
