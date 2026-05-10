# Tools Section Design

## Overview

Add a Tools section to the Brewski sidebar exposing standalone brewing calculators. Tools are standalone numeric calculators — no recipe context required — backed by Tauri Rust commands that reuse and extend the existing `brewing/` module.

## Tools Included

| Tool | Description |
|---|---|
| ABV / Attenuation / Calories | Calculate ABV, attenuation %, and calories per 12oz from OG + FG |
| Hydrometer Temperature Correction | Correct a hydrometer reading for wort temperature vs calibration temp |
| Refractometer / Brix | Convert Brix to SG pre-fermentation; Terrill formula correction post-fermentation |
| Carbonation | Priming sugar grams or CO₂ pressure (kPa) for a target carbonation level |
| Gravity Conversions | Convert between SG, Plato, and Brix |
| Unit Conversions | Volume (L ↔ gal), weight (kg ↔ lb), temperature (°C ↔ °F) — pure frontend |
| Yeast Pitch Rate | Required cell count and starter volume from OG, batch size, and pitch rate |
| Color Conversion | Convert between SRM, EBC, and Lovibond |

## Navigation

A wrench icon is added to the icon rail in `AppShell.svelte` between Recipes and Settings. It navigates to `/tools`, which redirects to `/tools/abv-calories`. The active icon highlights using the same pattern as the existing Recipes and Settings icons.

## Layout

Layout A: persistent tool list on the left, active tool detail on the right — matching the Settings page pattern.

- Left panel: `+layout.svelte` renders a list of all 8 tools. Active item is highlighted with an accent left-border, derived from `$page.url.pathname`.
- Right panel: `{@render children()}` — each tool's `+page.svelte` fills this area.

## Route Structure

```
src/routes/tools/
  +layout.svelte         ← tool list sidebar + {@render children()}
  +page.svelte           ← redirect to /tools/abv-calories
  abv-calories/
    +page.svelte
  hydrometer-temp/
    +page.svelte
  refractometer/
    +page.svelte
  carbonation/
    +page.svelte
  gravity-conversions/
    +page.svelte
  unit-conversions/
    +page.svelte
  pitch-rate/
    +page.svelte
  color-conversion/
    +page.svelte
```

Each tool page is self-contained: local `$state` for inputs, reactive results via `$derived` or `oninput` calling the relevant `api.ts` wrapper, results displayed inline. No shared store needed.

## Rust Backend

### New brewing modules

Each new module lives in `src-tauri/src/brewing/` and is added to `brewing/mod.rs`.

**`hydro.rs`**
```rust
pub fn correct_hydrometer_temp(measured_sg: f64, measured_temp_c: f64, calibration_temp_c: f64) -> f64
```
Standard polynomial correction formula (ASBC / Palmer).

**`refractometer.rs`**
```rust
pub fn brix_to_sg(brix: f64, wort_correction_factor: f64) -> f64
pub fn correct_refractometer_fg(og_brix: f64, fg_brix: f64, wort_correction_factor: f64) -> f64
```
Pre-fermentation: SG = 1 + (brix / (258.6 - 0.4 * brix / 258.2)). Post-fermentation: Terrill formula to correct for alcohol's effect on refractometer readings. Default wort correction factor is 1.04.

**`carbonation.rs`**
```rust
pub enum SugarType { TableSugar, CornSugar, DryMaltExtract }
pub fn priming_sugar_grams(target_vols: f64, batch_size_l: f64, temp_c: f64, sugar_type: SugarType) -> f64
pub fn co2_pressure_kpa(target_vols: f64, temp_c: f64) -> f64
```
Priming sugar uses the residual CO₂ table (volumes dissolved at temp) to find additional CO₂ needed, then converts to grams by sugar type. Pressure uses the standard carbonation chart formula.

**`gravity.rs`**
```rust
pub fn sg_to_plato(sg: f64) -> f64
pub fn plato_to_sg(plato: f64) -> f64
pub fn sg_to_brix(sg: f64) -> f64
pub fn brix_to_sg_simple(brix: f64) -> f64
```
Makes `og_to_plato` in `abv.rs` `pub(crate)` so `gravity.rs` can import it. Plato → SG is the inverse cubic approximation.

**`pitch.rs`**
```rust
pub fn required_cell_count(og: f64, batch_size_l: f64, pitch_rate_m_per_ml_per_plato: f64) -> f64
pub fn starter_volume_l(required_cells: f64, yeast_pack_cells: f64, viability_pct: f64) -> f64
```
`required_cell_count` = pitch_rate × batch_size_ml × plato. `starter_volume_l` uses the Jamil/MrMalty propagation model (1 billion cells/gram/liter of DME).

**`color.rs`**
```rust
pub fn srm_to_ebc(srm: f64) -> f64      // EBC = SRM × 1.97
pub fn ebc_to_srm(ebc: f64) -> f64      // SRM = EBC / 1.97
pub fn srm_to_lovibond(srm: f64) -> f64 // Lovibond = (SRM + 0.76) / 1.3546
pub fn lovibond_to_srm(lovibond: f64) -> f64
```

### New command file

`src-tauri/src/commands/tools.rs` — one Tauri command per tool, pure functions with no DB or `AppState` access:

| Command | Inputs | Return type |
|---|---|---|
| `calculate_abv_calories` | `og: f64, fg: f64` | `AbvCaloriesResult { abv_pct, attenuation_pct, calories_per_355ml }` |
| `correct_hydrometer_temp` | `measured_sg, measured_temp_c, calibration_temp_c: f64` | `f64` |
| `calculate_refractometer` | `brix, wort_correction_factor: f64` | `RefractometerResult { sg }` |
| `correct_refractometer_fg` | `og_brix, fg_brix, wort_correction_factor: f64` | `RefractometerFgResult { fg_sg }` |
| `calculate_priming_sugar` | `target_vols, batch_size_l, temp_c: f64, sugar_type: String` | `f64` (grams) |
| `calculate_co2_pressure` | `target_vols, temp_c: f64` | `f64` (kPa) |
| `convert_gravity` | `value: f64, from_unit: String` | `GravityConversionResult { sg, plato, brix }` |
| `calculate_pitch_rate` | `og, batch_size_l, pitch_rate, yeast_pack_cells, viability_pct: f64` | `PitchRateResult { required_cells, starter_volume_l }` |
| `convert_color` | `value: f64, from_unit: String` | `ColorConversionResult { srm, ebc, lovibond }` |

Commands are registered in `lib.rs` alongside existing commands.

## TypeScript API Layer

New result interfaces and `invoke()` wrappers added to `src/lib/api.ts`, following the existing pattern. One typed wrapper per command.

Unit conversions (volume, weight, temperature) remain as pure TypeScript functions in `src/lib/units.ts` — they are already implemented there and require no IPC.

## Frontend Component Pattern

Each tool page uses the same structure:
- Labelled inputs bound to local `$state` variables
- Results computed reactively: either `$derived` (for synchronous unit conversions) or an `async` call to the Tauri command triggered by `oninput`/`onchange`
- Results displayed in a result card styled consistently with the mockup (accent-bordered box, large result values)
- All inputs and labels respect the user's unit setting from the settings store

## Unit Awareness

Tools that involve volume, weight, or temperature display values in the user's preferred units (metric/imperial), using existing `units.ts` helpers and label functions. Inputs accept the display unit and convert to SI before calling Tauri commands.
