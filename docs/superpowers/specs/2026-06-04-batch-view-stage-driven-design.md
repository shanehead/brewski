# Batch View: Stage-Driven Redesign

**Date:** 2026-06-04

## Overview

Simplify the batch view by collapsing two stacked tab bars into one stage-driven tab bar, moving gravity readings into the Fermenting stage, placing tasting in the Packaged stage, surfacing attachments via a persistent header button, and removing the Conditioning status by merging it into Packaged.

## Current Structure

The desktop batch view has two stacked tab bars:

1. **Outer tab bar** (content navigation): Overview | Gravity Log | Tasting | Attachments
2. **Status tab bar** (inside Overview): Planned | Brewing | Fermenting | Conditioning | Packaged

This creates visual noise and disconnects contextually related content — gravity readings are in their own top-level tab rather than in the Fermenting stage where they belong.

## Proposed Structure

### Single tab bar: stage navigation

Replace both tab bars with one stage tab bar that also controls batch status:

```
Planned | Brewing | Fermenting | Packaged
```

Clicking a tab both navigates to that stage's content and updates the batch status (same behaviour as the current status TabBar). The "Conditioning" status is removed — its dates, carbonation content, and logic are absorbed into "Packaged".

### Stage content

**Planned**
- Stage callout (planned OG, FG, batch size)
- Measurements (planned OG, FG, pre-boil gravity, batch size)
- Dates (Brew Date)
- Notes

**Brewing**
- Stage callout (planned pre-boil gravity, OG, post-boil volume)
- Measurements (actual pre-boil gravity, actual OG, pre/post-boil volumes) — highlighted fields per existing HIGHLIGHTED map
- Dates (Brew Date, Into Fermenter)
- Notes

**Fermenting**
- Stage callout (actual OG, target FG, target ABV)
- Measurements (actual OG, actual FG) — highlighted
- Gravity Log (full `BatchGravityTab` content embedded here — table of readings + Add Reading form)
- Dates (Into Fermenter, Packaging)
- Notes

**Packaged** (merges former Conditioning + Packaged)
- Stage callout (actual OG, FG, ABV)
- Measurements (actual FG, batch size)
- Carbonation section (existing `BatchCarbonationSection`, shown unconditionally in this stage)
- Dates (Packaging Date) — remove separate Conditioning Date field from this section since the status no longer exists
- Tasting (existing `BatchTastingTab` content embedded here — rating input)
- Notes

### Attachments

A persistent "Attachments" button (paperclip icon + count badge) sits in the batch header row, to the right of the batch name/version info. Clicking it opens a full-screen overlay (same style as the existing recipe picker modal) rendering the existing `BatchAttachmentsTab` content. Closing the overlay returns to the current stage. This keeps attachments accessible from any stage without consuming a tab slot.

## Status / Database Changes

- Remove `"conditioning"` from the status enum in the API and database migration.
- The `conditioning_date` field on the batch remains (it holds a valid date that may have been set) but is no longer surfaced in the UI as a separate date input — it is not needed in the Packaged stage date row.
- Any existing batch with `status = "conditioning"` should be migrated to `status = "packaged"` in a database migration.
- Remove `conditioning_date` from the Dates section in the Packaged stage view. The packaging date is sufficient.
- Update `BatchList.svelte` STATUS_LABELS and STATUS_COLORS to remove "conditioning".
- Update `BatchOverviewTab.svelte` STATUSES array, HIGHLIGHTED map, stageTargets switch, and onStatusChange handler.

## Files Affected

| File | Change |
|------|--------|
| `src/lib/desktop/BatchView.svelte` | Remove outer tab bar and tab state; restructure to render active stage component; add Attachments header button |
| `src/lib/mobile/BatchView.svelte` | Remove Gravity Log section from its own section; embed it under the Fermenting section; move Tasting under Packaged; update status list |
| `src/lib/components/batch/BatchOverviewTab.svelte` | Becomes the new stage-driven view: absorbs BatchGravityTab into Fermenting section, BatchTastingTab into Packaged section; removes Conditioning from STATUSES |
| `src/lib/components/batch/BatchGravityTab.svelte` | No structural change; rendered inline inside Fermenting section |
| `src/lib/components/batch/BatchTastingTab.svelte` | No structural change; rendered inline inside Packaged section |
| `src/lib/components/batch/BatchAttachmentsTab.svelte` | No structural change; opened from header button |
| `src/lib/components/BatchList.svelte` | Remove "conditioning" from STATUS_LABELS and STATUS_COLORS |
| `src/lib/api.gen.ts` | Update BatchStatus type (conditioning removed) — auto-generated, updated via spec change |
| OpenAPI spec / backend | Remove conditioning from BatchStatus enum; add DB migration to move conditioning → packaged |

## Out of Scope

- Changing the gravity log data model or API
- Redesigning the attachments panel/modal interaction beyond surfacing the existing component
- Mobile layout changes beyond status list update and section reordering
