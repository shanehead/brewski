# Mash Step Inline Editing

**Date:** 2026-05-13  
**Status:** Approved

## Overview

Allow users to edit existing mash steps by clicking on a step row, which transforms it in-place into editable inputs. Changes save on blur, consistent with the existing mash profile field pattern in `MashTab.svelte`.

## State

A single reactive variable `editingStepId: string | null` in `MashTab.svelte` tracks which step row (if any) is in edit mode. Only one step can be edited at a time. Clicking a different row while one is open replaces the active edit with the new one.

## Display Mode (default)

Step rows look identical to today: name on top, detail line below (temp · time · type · infuse). The row gains `cursor-pointer` and a subtle hover background to communicate it is clickable.

## Edit Mode

Clicking a row sets `editingStepId` to that step's id. The name/detail display is replaced with inline inputs using the same styling as the existing "Add Step" form:

| Field | Input type | Notes |
|-------|-----------|-------|
| Name | `text` | |
| Type | `select` | infusion / temperature / decoction |
| Temp | `number` | Unit-converted; same step/placeholder as add form |
| Time | `number` | Minutes |
| Infuse amount | `number` | Only shown when type = infusion; unit-converted |

The delete `×` button remains visible in edit mode.

## Saving

Each input calls `updateMashStep(step.id, { field: value })` on `blur`, then calls `onchange()`. No explicit Save button. Because every field saves independently on blur, there is no data loss risk when exiting edit mode.

## Exiting Edit Mode

- Clicking the same row again toggles it closed
- Clicking a different step row closes the current one and opens the new one
- Pressing Escape closes the active edit row (keydown handler on the edit container)

## API

`updateMashStep` already exists in `src/lib/api.ts:127`. `UpdateMashStepInput` fields in scope: `name`, `type_`, `step_temp_c`, `step_time_min`, `infuse_amount_l`.

## Files Changed

- `src/lib/components/tabs/MashTab.svelte` — only file that needs changes
