# Equipment Database

**Date:** 2026-05-10
**Status:** Approved

## Goal

Populate the equipment library with pre-built profiles for popular homebrewing systems, and add an Equipment section to the app UI for browsing and managing profiles.

Equipment selection at the recipe level is deferred — profiles will be chosen at the Batch level (future work). This feature delivers the library and seed data that Batch will reference.

## What's Already Built

The `equipment_profiles` table and all four CRUD Tauri commands are fully implemented:
- `list_equipment_profiles`
- `create_equipment_profile`
- `update_equipment_profile`
- `delete_equipment_profile`

No schema changes, new commands, or Rust work are needed.

## Seed Data

### Source file

Add `data/equipment.json` as the canonical source for seed values, following the same pattern as `data/fermentables.json`, `data/hops.json`, etc.

Add `scripts/seed-equipment.mjs` to transform `data/equipment.json` into `INSERT OR IGNORE INTO equipment_profiles (...)` statements in `001_initial.sql`, replacing any existing equipment seed block.

Add a `seed-equipment` recipe to the Justfile:
```
seed-equipment:
    bun scripts/seed-equipment.mjs
    just migrate
```

### Profiles to seed

IDs use the pattern `eq-<slug>` for stability across re-migrations.

| ID | Name | Batch (L) | Boil (L) | Boil (min) | Evap (%/hr) | Trub+Chill (L) | Fermenter loss (L) | Efficiency (%) | Notes |
|----|------|-----------|----------|------------|-------------|----------------|--------------------|----------------|-------|
| `eq-default` | Standard 5 Gallon | 19.0 | 23.0 | 60 | 10.0 | 1.5 | 1.0 | 72 | Generic 5 gal kettle (existing) |
| `eq-biab-generic` | Generic BIAB | 19.0 | 26.5 | 60 | 10.0 | 0.5 | 1.0 | 80 | Bag-in-kettle, higher efficiency |
| `eq-grainfather-g30` | Grainfather G30 | 23.0 | 27.0 | 60 | 8.5 | 2.0 | 1.0 | 72 | |
| `eq-grainfather-g70` | Grainfather G70 | 57.0 | 65.0 | 60 | 7.0 | 3.5 | 2.0 | 72 | |
| `eq-brewzilla-35` | Brewzilla 35L | 23.0 | 30.0 | 60 | 10.0 | 1.5 | 1.0 | 72 | Gen 4 |
| `eq-brewzilla-65` | Brewzilla 65L | 50.0 | 57.0 | 60 | 10.0 | 2.5 | 2.0 | 72 | Gen 4 |
| `eq-spike-solo` | Spike Solo | 19.0 | 26.5 | 60 | 10.0 | 1.5 | 1.0 | 72 | 10 gal kettle |
| `eq-spike-trio-10` | Spike Trio 10 Gal | 19.0 | 26.5 | 60 | 10.0 | 1.5 | 1.0 | 72 | 3-vessel |
| `eq-clawhammer-biab` | Clawhammer BIAB | 19.0 | 26.5 | 60 | 10.0 | 0.5 | 1.0 | 80 | 10 gal electric BIAB |
| `eq-blichmann-breweasy` | Blichmann BrewEasy | 19.0 | 26.5 | 60 | 10.0 | 1.5 | 1.0 | 72 | Tippy dump 2-vessel |
| `eq-anvil-foundry-6` | Anvil Foundry 6.5 Gal | 19.0 | 24.6 | 60 | 9.5 | 1.0 | 1.0 | 80 | All-in-one BIAB |
| `eq-anvil-foundry-10` | Anvil Foundry 10.5 Gal | 30.0 | 38.0 | 60 | 9.5 | 1.5 | 1.0 | 80 | All-in-one BIAB |
| `eq-brewtools-b40` | Brewtools B40 Pro | 30.0 | 36.0 | 60 | 8.0 | 1.5 | 1.0 | 75 | Recirculating |
| `eq-brewtools-b80` | Brewtools B80 Pro | 65.0 | 76.0 | 60 | 7.5 | 2.5 | 2.0 | 75 | Recirculating |
| `eq-ss-svbs` | SS Brewtech SVBS | 19.0 | 26.5 | 60 | 10.0 | 1.5 | 1.0 | 72 | Single vessel brewing system |
| `eq-braumeister-20` | Speidel Braumeister 20L | 15.0 | 20.0 | 60 | 8.0 | 1.0 | 0.5 | 78 | |
| `eq-braumeister-50` | Speidel Braumeister 50L | 38.0 | 50.0 | 60 | 7.5 | 2.0 | 1.5 | 78 | |
| `eq-unibrau` | Unibrau | 23.0 | 30.0 | 60 | 9.0 | 1.5 | 1.0 | 75 | All-in-one recirculating |

All profiles: `hop_utilization_pct = 100`, `created_at = 0`, `updated_at = 0`.

Exact manufacturer specs should be verified from official documentation before committing seed data — values above are representative starting points.

## Frontend — Equipment Library

New **Equipment** entry in the app sidebar/navigation, alongside Fermentables, Hops, Yeasts, etc.

**List view:**
- Table/card list of all profiles, sorted by name
- Columns: Name, Batch Size, Boil Size, Efficiency, Boil Time
- "New Profile" button → opens create form

**Create / Edit form (modal or inline panel):**
- Fields matching the `equipment_profiles` schema: name, batch size, boil size, boil time, evap rate, efficiency, trub/chiller loss, fermenter loss, hop utilization, lauter deadspace, tun volume/weight/specific heat, notes
- Save calls `create_equipment_profile` or `update_equipment_profile`

**Delete:**
- Delete button per row with confirmation prompt
- Calls `delete_equipment_profile`

The UI pattern follows the existing ingredient library screens.

## Out of Scope

- Equipment selection on recipes (deferred — chosen at Batch level)
- Import/export of equipment profiles
- Equipment-specific brewing calculations (already handled by existing strike temp and related commands)
