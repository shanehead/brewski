# Batch Brew Day Recording — Design Spec

## Goal

Improve the batch overview so it actively helps during a brew day: show planned targets from the recipe alongside actual measurements, surface the single notes field at all stages, auto-fill stage dates on status advance, and add a Conditioning stage.

## Architecture

All changes are confined to the batch subsystem. The DB migration restructures the `batches` table (new `notes` and `conditioning_date` columns, drop three old notes columns, update status constraint). The Rust `get_batch` command is extended to join recipe version stats and return planned targets alongside actuals. `BatchOverviewTab.svelte` is updated to render the stage callout banner, contextual measurement highlighting, dates row, and notes field. Both desktop and mobile BatchView files are updated. No new routes or commands.

## Tech Stack

Rust (SQLite migration, SeaORM repository, Tauri command), OpenAPI YAML, openapi-typescript type regen, Svelte 5 runes, Tailwind + CSS vars.

---

## Data Model

### Migration `006_batch_brew_day.sql`

The `batches` table is recreated (SQLite requires a full table rebuild to change a CHECK constraint). Steps: create `batches_new`, copy all rows, drop `batches`, rename `batches_new` to `batches`.

Changes from current schema:

| Change | Detail |
|--------|--------|
| Add | `conditioning_date TIMESTAMP` nullable |
| Add | `notes TEXT` nullable — replaces the three stage-specific notes columns |
| Drop | `brew_day_notes`, `fermentation_notes`, `tasting_notes` |
| Status constraint | Replace `'complete'` with `'conditioning'`; final stage is `'packaged'` |

New full status set: `'planned'`, `'brewing'`, `'fermenting'`, `'conditioning'`, `'packaged'`

Existing rows with `status = 'complete'` are migrated to `'packaged'` in the `INSERT INTO batches_new SELECT …` step. Existing `brew_day_notes` content is copied into `notes`; `fermentation_notes` and `tasting_notes` are discarded (no production users yet).

### Batch API Response — Planned Targets

`get_batch` joins the linked recipe version's stats to populate five new nullable fields on the `Batch` response:

| Field | Source |
|-------|--------|
| `planned_og` | `recipe_version_stats.og` |
| `planned_fg` | `recipe_version_stats.fg` |
| `planned_pre_boil_gravity` | `recipe_version_stats.pre_boil_gravity` |
| `planned_post_boil_volume_l` | `recipe_version_stats.post_boil_volume_l` |
| `planned_batch_size_l` | equipment profile batch size on the recipe version |

All are `number | null` — null when no stats have been calculated for the recipe version yet. The frontend hides the callout banner when all five are null.

### OpenAPI Schema Updates

- `Batch.yaml`: add `conditioning_date`, `notes`, five `planned_*` fields; remove `brew_day_notes`, `fermentation_notes`, `tasting_notes`; update `status` description to list new values
- `BatchSummary.yaml`: update `status` description only
- Regenerate `src/lib/api.gen.ts` via `npx openapi-typescript`

---

## Stage Lifecycle

### Stages

```
planned → brewing → fermenting → conditioning → packaged
```

`packaged` is the terminal state. There is no "complete" or "archived" stage.

### Date Auto-fill

When the user advances status forward, the corresponding date is set to today's ISO date string if not already set. Going backwards never clears dates.

| Transition | Auto-fills |
|-----------|-----------|
| planned → brewing | `brew_date` |
| brewing → fermenting | `fermenter_date` |
| fermenting → conditioning | `conditioning_date` |
| conditioning → packaged | `packaging_date` |

Auto-fill is handled in the frontend `onStatusChange` handler before calling `onUpdate`. It reads the current date (`new Date().toISOString().slice(0, 10)`) and only sets the field if the current value is null/empty.

### Stage Callout Content

The callout banner above measurements shows planned targets relevant to the current stage. It is hidden if all planned values are null.

| Stage | Callout shows |
|-------|--------------|
| planned | Target OG · Target FG · Batch size |
| brewing | Target pre-boil gravity · Target OG · Target post-boil volume |
| fermenting | Actual OG (reference) · Target FG · Target ABV |
| conditioning | Actual OG · Actual FG · Actual ABV (all reference) |
| packaged | Actual OG · Actual FG · Actual ABV (final summary) |

Target ABV is derived: `((planned_og - planned_fg) * 131.25).toFixed(1)`. Actual ABV is derived: `((actual_og - actual_fg) * 131.25).toFixed(1)`. Neither is stored.

---

## UI — BatchOverviewTab

`src/lib/components/batch/BatchOverviewTab.svelte` is the shared component rendered by both desktop and mobile `BatchView.svelte`. All visual changes live here.

### Layout (top to bottom)

1. **Status control** — existing tab bar (desktop) / select (mobile), updated options
2. **Stage callout banner** — `{#if hasTargets}` block; indigo-tinted pill showing relevant targets for the stage
3. **Measurements grid** — 3-column grid (desktop), 2-column (mobile); cards highlighted for the current stage with indigo tint; non-relevant fields muted but still editable
4. **Dates row** — all four dates in a horizontal row; future/empty dates are muted; clicking a date opens a date input (existing pattern)
5. **Notes** — single `<textarea>` bound to `batch.notes`, full width, `min-height: 72px`, `resize: vertical`, always visible at all stages

### Measurement Highlighting

Each measurement card is styled with an indigo tint when it is "relevant" to the current stage. Relevance map:

| Stage | Highlighted fields |
|-------|--------------------|
| planned | _(none — no measurements taken yet)_ |
| brewing | `actual_pre_boil_gravity`, `actual_og`, `actual_post_boil_volume_l` |
| fermenting | `actual_og`, `actual_fg` |
| conditioning | `actual_fg`, `actual_batch_size_l` |
| packaged | `actual_og`, `actual_fg` |

All fields remain editable regardless of highlight state.

### Desktop & Mobile

`src/lib/desktop/BatchView.svelte` and `src/lib/mobile/BatchView.svelte` both render `BatchOverviewTab`. No changes are needed to these files — the shared component handles the full layout. If either BatchView currently passes status options as a prop, update the prop to use the new stage list.

---

## Testing

- **Rust**: update any existing batch tests that reference `brew_day_notes`, `fermentation_notes`, `tasting_notes`, or status `'complete'`; add a test that `get_batch` returns non-null `planned_og` when the recipe version has stats
- **Frontend**: `npm run check` passes with no errors; manual smoke test: create batch, advance through all five stages verifying date auto-fill, enter notes at each stage and confirm persistence, confirm callout shows correct targets per stage on both desktop and mobile
