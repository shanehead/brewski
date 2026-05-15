# Equipment Rail Icon — Design Spec

**Date:** 2026-05-15

## Overview

Move the Equipment Profiles section out of Settings and give it a dedicated icon in the navigation rail, positioned as a first-class section alongside Recipes, Batches, and Tools.

## Motivation

Equipment profiles define a brewer's setup (batch size, boil volume, efficiency) and are referenced throughout the app. Burying them inside Settings underrepresents their importance and makes them harder to discover.

## Changes

### 1. New route: `/equipment`

Create `src/routes/equipment/+page.svelte` containing:

- A "Default Profile" selector (`<select>`) bound to the `default_equipment_profile_id` setting, populated from the profiles list.
- A list of existing profiles, each showing name, batch size (L), and efficiency (%), with a Delete button per row.
- An "Add" form: text input for profile name + Add button. New profiles default to 27L boil / 23L batch / 72% efficiency (same defaults as today).

All API calls (`listEquipmentProfiles`, `createEquipmentProfile`, `deleteEquipmentProfile`) and settings calls (`loadSettings`, `saveSetting`) are identical to the existing implementation — just relocated.

No `+page.ts` loader is needed; data is fetched in `onMount` as in the current settings page.

### 2. Rail update: `AppShell.svelte`

Updated rail order (top to bottom):

1. Recipes (`/`)
2. Batches (`/batches`)
3. Tools (`/tools`)
4. **Equipment** (`/equipment`) ← new
5. `flex-1` spacer
6. Settings (`/settings`)

The Equipment icon uses an inline SVG brewing kettle/vessel, matching the existing inline SVG style (18×18 or 22×22 px, `stroke="currentColor" stroke-width="2"`). A new `isEquipment` derived store tracks `$page.url.pathname.startsWith('/equipment')` for active-state highlighting.

### 3. Settings cleanup: `settings/+page.svelte`

Remove:
- The entire "Equipment Profiles" section (list, add form, default selector).
- `listEquipmentProfiles`, `createEquipmentProfile`, `deleteEquipmentProfile` imports.
- `profiles` state, `newProfileName` state.
- `handleAddProfile`, `handleDeleteProfile`, `handleDefaultEquipChange` handlers.
- The `default_equipment_profile_id` saveSetting call.

Retain: Appearance (theme selector) and Units (measurement system selector).

## No Backend Changes

All API endpoints are unchanged. This is a pure frontend relocation.

## Files Changed

| File | Change |
|------|--------|
| `src/routes/equipment/+page.svelte` | Create — new equipment management page |
| `src/lib/components/AppShell.svelte` | Add Equipment rail icon and `isEquipment` derived |
| `src/routes/settings/+page.svelte` | Remove equipment section and related imports/state/handlers |
