# Equipment Profile Enhancements Design

**Date:** 2026-05-17  
**Status:** Approved

## Overview

Expand equipment profiles to match Brewfather's field coverage and add a full edit/copy UI. Currently Brewski only supports create and delete — there is no way to edit a profile or duplicate one. The data model also lacks ~17 fields that affect volume, efficiency, hop, and mash/sparge calculations.

## Scope

Single comprehensive pass: one migration, updated Rust layer, new full-screen modal edit UI, and copy action — all in one implementation.

---

## Data Model

One new migration adds 17 columns to `equipment_profiles`. Existing rows get sensible defaults via `ALTER TABLE` statements.

| Column | Type | Default | Notes |
|---|---|---|---|
| `batch_volume_target` | TEXT | `'fermenter'` | enum: `'fermenter'` \| `'kettle'` |
| `mash_tun_loss_l` | REAL | `0` | volume left in tun after lauter |
| `hlt_deadspace_l` | REAL | NULL | optional |
| `cooling_shrinkage_pct` | REAL | `4.0` | boil expansion / cooling shrinkage |
| `calc_mash_efficiency` | INTEGER | `1` | boolean |
| `mash_efficiency_pct` | REAL | NULL | manual value when `calc_mash_efficiency` = 0 |
| `calc_aroma_hop_utilization` | INTEGER | `1` | boolean |
| `aroma_hop_utilization_pct` | REAL | `23` | editable when `calc_aroma_hop_utilization` = 0 |
| `whirlpool_time_min` | REAL | NULL | optional |
| `altitude_adjustment` | INTEGER | `0` | boolean |
| `boil_temp_f` | REAL | NULL | manual when `altitude_adjustment` = 0 |
| `sparge_method` | TEXT | `'no_sparge'` | enum: `'no_sparge'` \| `'batch_sparge'` \| `'fly_sparge'` |
| `mash_volume_min_l` | REAL | NULL | optional |
| `mash_volume_max_l` | REAL | NULL | optional |
| `sparge_volume_min_l` | REAL | NULL | optional |
| `sparge_volume_max_l` | REAL | NULL | optional |
| `calc_strike_water_temp` | INTEGER | `0` | boolean; calculation deferred |

The SeaORM entity (`entities/equipment_profiles.rs`), generated models (`models.gen.rs`), `CreateEquipmentProfileInput`, `UpdateEquipmentProfileInput`, repository, and all Tauri commands are updated to include all new fields.

---

## Backend

### Copy Command

New Tauri command: `copy_equipment_profile(id: String) -> Result<EquipmentProfile>`.

Implemented in `repositories/equipment.rs` — fetches the source profile and inserts a new row with all fields copied, appending `" (copy)"` to the name. No round-trip through the frontend required.

---

## UI

### Equipment List Page (`/equipment`)

Each profile row gains two icon actions alongside the existing delete:
- **Edit** — opens the full-screen modal pre-populated with that profile's values
- **Copy** — calls `copy_equipment_profile`, refreshes the list

### Equipment Profile Modal

Full-screen scrollable modal. Used for both creating new profiles and editing existing ones. Divided into five labeled sections:

#### 1. Header (always visible at top)
- Name (required)
- Boil Time (min)
- Description

#### 2. Volumes
- Batch Volume Target (dropdown: Fermenter / Kettle)
- Batch Volume (gal) — label updates dynamically: "Batch Volume (Fermenter)" or "Batch Volume (Kettle)" based on the target dropdown
- Calc boil volume (checkbox) → Pre-Boil Volume (calculated, read-only when checked)
- Boil Off (gal/hr) with live %-of-boil display
- Trub/Chiller Loss (gal)
- Mash-Tun Deadspace (gal)
- Mash-Tun Loss (gal)
- HLT Deadspace (gal, optional)
- Fermenter Loss (gal, optional)
- Fermenter Top-Up (gal, optional)
- Cooling Shrinkage % — single field here, not duplicated in Boil Temperature
- Calculated summary line: Post-Boil Kettle Volume · Bottling Volume · pre-boil hot volume note

#### 3. Efficiency
- Brewhouse Efficiency %
- Calc mash efficiency (checkbox) → Mash Efficiency % (calculated read-only when checked; editable when unchecked)

#### 4. Hops
- Hop Utilization Multiplier %
- Calc aroma hop utilization (checkbox) → Aroma Hop Utilization % (calculated read-only when checked; editable when unchecked)
- Whirlpool / No-Chill Time (min, optional)

#### 5. Boil Temperature
- Altitude adjustment (checkbox) — when checked, disables manual Boil Temperature and shows an altitude input that drives the calculation
- Boil Temperature °F (manual when altitude adjustment off)

#### 6. Mash / Sparge Water
- Grain Absorption Rate (qt/lb)
- Water/Grain Ratio (qt/lb)
- Sparge Method (dropdown: No Sparge / Batch Sparge / Fly Sparge)
- Mash Volume Limits: Min / Max (gal, optional)
- Sparge Volume Limits: Min / Max (gal, optional)
- Calc strike water temperature (checkbox) — stored, calculation deferred

#### Footer
- Cancel / Save buttons
- Notes field (existing)

---

## Calculation Changes (`brewing/mod.rs`)

The following fields are wired into recipe calculations in this pass:

| Field | Change |
|---|---|
| `batch_volume_target` | When `'kettle'`, treat `batch_size` as post-boil kettle volume; back-calculate fermenter and pre-boil volumes from there |
| `mash_tun_loss_l` | Added to pre-boil volume requirement alongside `lauter_deadspace_l` |
| `hlt_deadspace_l` | Factored into total water needed |
| `cooling_shrinkage_pct` | Replaces the implicit 4% constant used in hot-volume display |
| `aroma_hop_utilization_pct` | Applied to whirlpool/aroma hop additions (was hardcoded fallback) |
| `whirlpool_time_min` | Contributes to IBU calculation for whirlpool additions |

### Deferred (stored this pass, calculation logic in a follow-up)

| Field | Reason |
|---|---|
| `calc_mash_efficiency` / `mash_efficiency_pct` | Requires modeling grain absorption and lauter losses precisely |
| `boil_temp_f` / `altitude_adjustment` | Affects hop utilization curves; needs calibration work |
| `calc_strike_water_temp` | Needs tun thermal mass model; tun fields already exist |
| Mash/Sparge Volume Limits | UI warning display only, no calculation |

---

## Out of Scope

- Hopstand Temperature — captured on individual hop additions instead
- Sparge water reminder / time-left-of-mash notifications
- HLT Water Limit Min
- Overflow Target dropdown
