# Batch Carbonation Integration — Design Spec

> Date: 2026-05-20

## Overview

Add a carbonation section to the batch Overview tab that appears when the batch reaches **Conditioning** or **Packaged** status. It shows both bottle priming sugar and keg serving pressure, pre-populated from batch and recipe data, and saves the carbonation plan back to the batch record.

A standalone carbonation calculator already exists at `/tools/carbonation`. This feature integrates that same calculation into the batch workflow so brewers don't have to manually re-enter data.

---

## Placement

The carbonation section renders at the bottom of `BatchOverviewTab` conditionally:

```
batch.status === "conditioning" || batch.status === "packaged"
```

No new tab is added. The section is hidden during earlier stages (planned, brewing, fermenting) to avoid clutter.

---

## Inputs

| Input | Source | Editable? | Persisted? |
|---|---|---|---|
| Target CO₂ (vols) | `recipe.carbonation_vols` | No | No — lives on recipe |
| Packaging temp (°C) | Pre-filled from `recipe.primary_temp_c` | Yes | Yes — `batch.packaging_temp_c` |
| Sugar type | Defaults to `"corn_sugar"` | Yes | Yes — `batch.carbonation_sugar_type` |

The target CO₂ vols is read-only in the batch context. To change the target, the brewer updates the recipe.

---

## Outputs

Both results are always shown simultaneously — the brewer uses whichever is relevant (bottles or keg):

| Output | Display | Persisted field |
|---|---|---|
| Priming sugar | Xg of [sugar type] | `batch.priming_sugar_g` |
| Serving pressure | X kPa / X PSI | `batch.serving_pressure_kpa` |

Results update live as inputs change (reactive `$effect`). The four persisted fields are written to the batch via `update_batch` on blur of the temp input and on change of the sugar type select — matching the save pattern used elsewhere in `BatchOverviewTab`.

---

## New Batch Schema Fields

Four nullable fields added to `Batch`, `UpdateBatchInput`, and the DB migration:

```
packaging_temp_c         f64?   — actual temp at packaging time
carbonation_sugar_type   str?   — "corn_sugar" | "table_sugar" | "dry_malt_extract"
priming_sugar_g          f64?   — calculated, saved for historical record
serving_pressure_kpa     f64?   — calculated, saved for historical record
```

The `packaging_temp_c` pre-fill logic runs client-side on first render: if `batch.packaging_temp_c` is null and `recipe.primary_temp_c` is set, use `recipe.primary_temp_c` as the initial input value (but only save once the user triggers a calculation).

---

## Component Structure

Extract a `BatchCarbonationSection` Svelte component:

```
src/lib/components/batch/BatchCarbonationSection.svelte
```

Props:
- `batch: Batch`
- `recipePrimaryTempC: number | null`
- `recipeCarbonationVols: number | null`
- `onUpdate: (input: UpdateBatchInput) => void`

`BatchOverviewTab` conditionally renders it when status is conditioning/packaged, passing batch data and the recipe's carbonation fields. `BatchOverviewTab` currently fetches recipe versions but not the full recipe — it needs to also call `get_recipe(batch.recipe_id)` on mount to retrieve `primary_temp_c` and `carbonation_vols`. These are passed as props to `BatchCarbonationSection`.

---

## Calculation

Reuses the existing Tauri commands:
- `calculate_priming_sugar(target_vols, batch_size_l, temp_c, sugar_type)` — returns grams
- `calculate_co2_pressure(target_vols, temp_c)` — returns kPa

Batch size used: `batch.actual_batch_size_l ?? batch.planned_batch_size_l`. If neither is set, show a placeholder prompting the brewer to enter batch size in Measurements.

PSI conversion in the frontend: `kPa × 0.145038`.

---

## OpenAPI Schema Updates

Add to `Batch.yaml`, `UpdateBatchInput.yaml`:
- `packaging_temp_c: [number, "null"]`
- `carbonation_sugar_type: [string, "null"]`
- `priming_sugar_g: [number, "null"]`
- `serving_pressure_kpa: [number, "null"]`

---

## DB Migration

New SQLx migration adds four nullable columns to the `batches` table:

```sql
ALTER TABLE batches ADD COLUMN packaging_temp_c REAL;
ALTER TABLE batches ADD COLUMN carbonation_sugar_type TEXT;
ALTER TABLE batches ADD COLUMN priming_sugar_g REAL;
ALTER TABLE batches ADD COLUMN serving_pressure_kpa REAL;
```

---

## Testing

- Unit tests for `BatchCarbonationSection`: renders when status is conditioning/packaged, hidden otherwise; pre-fills temp from recipe; calls `onUpdate` with all four fields on change.
- Existing `carbonation.rs` calculation tests are sufficient for the backend math.
- Integration test: `update_batch` with new fields round-trips correctly.
