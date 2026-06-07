# Equipment Profile Parity with Brewfather

**Date:** 2026-06-07
**Status:** Approved

## Goal

Add the fields present in Brewfather's equipment profile that Brewski currently lacks, and align the boil-off rate input to absolute volume/hr (matching Brewfather's more physically accurate approach).

## Decisions

- **Boil Off input**: switch from %/hr to absolute vol/hr (L/hr stored, gal/hr displayed in imperial). The percentage is shown parenthetically as a calculated label.
- **Mash-Tun Heat Capacity**: single "L equivalent water volume" field replaces the separate `tun_weight_kg` + `tun_specific_heat` fields in `equipment_profiles`. Those two fields remain in `mashes` and `recipe_version_mash` where they serve a BeerXML purpose.
- **Sparge water reminder**: out of scope (brew-session feature, not equipment config).
- **Unit conversion**: all new fields convert with the user's unit preference. Grain Absorption Rate and Water/Grain Ratio convert between L/kg and qt/lb.

## Database -- Migration 014

File: `src-tauri/migrations/014_equipment_profile_parity.sql`

### New columns on `equipment_profiles`

| Column | Type | Default | Notes |
|---|---|---|---|
| `evap_rate_l_hr` | REAL | (migrated) | Replaces `evap_rate_pct_hr` |
| `tun_heat_capacity_l` | REAL NOT NULL | 0.0 | Replaces `tun_weight_kg` + `tun_specific_heat` |
| `hopstand_temp_f` | REAL NOT NULL | 176.0 | Used when `calc_aroma_hop_utilization` is true |
| `grain_absorption_rate_l_per_kg` | REAL NOT NULL | 1.04 | ≈ 0.5 qt/lb |
| `water_grain_ratio_l_per_kg` | REAL NOT NULL | 3.12 | ≈ 1.5 qt/lb |
| `include_grain_volume_in_mash_limits` | INTEGER NOT NULL | 1 | Boolean |
| `overflow_target` | TEXT NOT NULL | 'mash' | Enum: mash / sparge / hlt |
| `hlt_water_limit_min_l` | REAL | NULL | Optional |
| `room_temp_f` | REAL NOT NULL | 68.0 | For strike water calculation |
| `grain_temp_f` | REAL NOT NULL | 68.0 | For strike water calculation |
| `sparge_temp_f` | REAL | NULL | Optional |

### Data migration for evap rate

```sql
-- Add with a safe default first, then overwrite with converted values
ALTER TABLE equipment_profiles ADD COLUMN evap_rate_l_hr REAL NOT NULL DEFAULT 3.8;
UPDATE equipment_profiles
SET evap_rate_l_hr = evap_rate_pct_hr / 100.0 * boil_size_l;
```

This converts existing percentage-per-hour values to absolute litres-per-hour using the stored pre-boil volume as the reference. The intermediate DEFAULT 3.8 (≈ 1 gal/hr) is overwritten for all existing rows by the UPDATE.

### Columns dropped from `equipment_profiles`

- `evap_rate_pct_hr` -- replaced by `evap_rate_l_hr`
- `tun_weight_kg` -- replaced by `tun_heat_capacity_l`
- `tun_specific_heat` -- replaced by `tun_heat_capacity_l`

These columns are retained in `mashes` and `recipe_version_mash` where they are part of the BeerXML mash profile standard.

## Backend (Rust)

### `brewing/volumes.rs`

Change `calculate_boil_volumes` signature: replace `evap_rate_pct_hr: f64` with `evap_rate_l_hr: f64`.

Update formula:
```
// Old
pre_boil = post_boil / (1 - evap_pct/100 * hours) + mash_loss

// New
pre_boil = post_boil + evap_rate_l_hr * hours + mash_loss
```

Update unit tests to pass absolute L/hr values instead of percentages (e.g. the existing "10%/hr on ~25L post-boil" test becomes "2.5 L/hr").

### `brewing/mod.rs`

Three places hard-code `evap_rate_pct_hr: 10.0` as default equipment for bare recipes. Change to `evap_rate_l_hr: 3.8` (≈ 1 gal/hr; physically equivalent to 10% of a typical 23 L batch).

### `repositories/water_chemistry.rs`

One formula derives total evaporation volume from `evap_rate_pct_hr`. Update it to use `evap_rate_l_hr * boil_time_min / 60.0` directly.

### Entity, models, repository

- `entities/equipment_profiles.rs`: regenerate via sea-orm-codegen after migration (removes old fields, adds new ones).
- `models.gen.rs`: regenerate via the project's schema-first codegen script.
- `repositories/equipment.rs`: update `create` and `update` handlers to include the new fields; remove the dropped fields; add sensible defaults in the create path for any NOT NULL fields not provided by the caller.
- `brewing/beerxml_fixture.rs`: update the hard-coded equipment struct literals that set `tun_weight_kg`, `tun_specific_heat`, and `evap_rate_pct_hr` to use the new field names.
- `commands/import_export.rs`: update BeerXML import/export equipment mapping for the same field renames.

## Frontend (`EquipmentProfileModal.svelte`)

### Volumes section

- State variable renamed from `evapRatePctHr` to `evapRateLHr`.
- Input and display use the standard `volIn` / `volDisp` helpers (L ↔ gal conversion).
- Label: `Boil Off ({calcEvapPct}%) {volumeLabel}/hr` where `calcEvapPct` is derived as `(evapRateLHr * boilHours / preBoilColdL) * 100` (total evaporation as % of pre-boil volume).
- Pre-boil derived formula updated: `preBoilColdL = postBoilColdL + evapRateLHr * boilHours + mashTunLossL`.

### Hops section

Add **Hopstand Temperature** field:
- Shown only when `calcAromaHopUtilization` is true (same conditional block as the existing "calculated" display).
- Stored as °F, displayed with `tempDisp` / `tempIn` helpers.
- Default: 176°F (80°C), matching Brewfather.
- Positioned alongside the Aroma Hop Utilization field.

### Mash / Sparge Water section

Removals:
- **Tun Weight** input removed (field dropped from equipment_profiles).

Additions and reorganization:

1. **Mash-Tun Heat Capacity** (`tun_heat_capacity_l`) -- volume input (L ↔ gal), label "Mash-Tun Heat Capacity" with unit suffix. Replaces Tun Weight in the same grid slot.
2. **Grain Absorption Rate** (`grain_absorption_rate_l_per_kg`) -- volume-per-weight ratio, converted between L/kg and qt/lb.
3. **Water/Grain Ratio** (`water_grain_ratio_l_per_kg`) -- volume-per-weight ratio, same conversion.
4. **Include grain volume in mash limits** checkbox (`includeGrainVolumeInMashLimits`) -- placed immediately before the Mash Volume Limits subsection.
5. **Overflow Target** select (`overflowTarget`) -- options: Mash / Sparge / HLT, placed after Sparge Volume Limits.
6. **HLT Water Limit Min** (`hltWaterLimitMinL`) -- optional volume input, placed after HLT Deadspace.
7. **Strike Water Temperature subsection** -- when `calcStrikeWaterTemp` is true, reveal:
   - **Room Temperature** (`roomTempF`) -- temp input with °F/°C conversion, default 68°F.
   - **Grain Temperature** (`grainTempF`) -- temp input with °F/°C conversion, default 68°F.
   - Helper text: "Set heat capacity to 0 if your mash tun is pre-heated."
8. **Sparge Temperature** (`spargeTempF`) -- optional temp input, placed in its own row at the bottom of the section.

### Unit conversion helpers for volume-per-weight ratios

Add two small helpers alongside the existing `volIn` / `weightIn` family:

```ts
function ratioDisp(lPerKg: number): string {
  // L/kg → qt/lb: 1 L/kg = 0.4796 qt/lb
  return (units === "imperial" ? lPerKg * 0.4796 : lPerKg).toFixed(2);
}
function ratioIn(e: Event): number {
  const v = parseFloat((e.target as HTMLInputElement).value) || 0;
  return units === "imperial" ? v / 0.4796 : v;
}
```

## Out of Scope

- Sparge water reminder / time-left-of-mash (brew session feature).
- Using the new equipment profile fields to auto-populate new recipe mash defaults (separate feature).
- Any changes to the `mashes` or `recipe_version_mash` tables.
