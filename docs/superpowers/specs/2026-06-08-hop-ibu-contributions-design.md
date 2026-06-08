# Hop IBU Contributions

**Date:** 2026-06-08

## Goal

Show each hop addition's IBU contribution in the Ingredients tab hops table.

## Schema

Add a new `HopStat` type to the OpenAPI spec:

```yaml
# docs/openapi/components/schemas/HopStat.yaml
type: object
required: [hop_id, ibu]
properties:
  hop_id:
    type: string
    description: The RecipeAdditionHop.id this stat corresponds to
  ibu:
    type: number
    description: IBU contribution from this hop addition
```

Add `hop_stats` to `RecipeStats.yaml`:

```yaml
hop_stats:
  type: array
  items:
    $ref: ./HopStat.yaml
  description: Per-hop IBU contributions, keyed by hop addition ID
```

Run `just gen` to regenerate `models.gen.rs` and `api.gen.ts`.

## Backend

### `ibu.rs`

Refactor `tinseth_ibu` to act on a single `HopIbuInput` rather than a slice. It returns the IBU contribution for that one addition.

Before (current shape):
```rust
pub fn tinseth_ibu(hops: &[HopIbuInput], og: f64, post_boil_volume_l: f64, boil_time_min: f64) -> f64
```

After:
```rust
pub fn tinseth_ibu(hop: &HopIbuInput, og: f64, post_boil_volume_l: f64, boil_time_min: f64) -> f64
```

The internal logic is unchanged -- just de-sliced. Existing unit tests that pass single-element `Vec`s call it directly on the element instead.

### `mod.rs`

`calculate_stats` maps over `recipe.hops`, calling `tinseth_ibu` for each. It keeps individual values for `hop_stats` and sums them for the existing `ibu` total:

```rust
let hop_stats: Vec<HopStat> = recipe.hops.iter().zip(hop_inputs.iter())
    .map(|(h, input)| HopStat {
        hop_id: h.id.clone(),
        ibu: ibu::tinseth_ibu(input, og, post_boil_volume_l, recipe.boil_time_min),
    })
    .collect();

let ibu: f64 = hop_stats.iter().map(|s| s.ibu).sum();
```

`RecipeStats` is populated with both `ibu` and `hop_stats`.

## Frontend

### `IngredientsTab.svelte`

Add `stats: RecipeStats | null` prop. Pass it through to `HopsTable`.

### Desktop and mobile `RecipeView.svelte`

Both already hold `stats`. Pass it to `IngredientsTab` where `IngredientsTab` is rendered.

### `HopsTable.svelte`

Add `stats: RecipeStats | null` prop. Build a lookup map:

```ts
const hopIbus = $derived(
  new Map(stats?.hop_stats?.map(s => [s.hop_id, s.ibu]) ?? [])
);
```

Add an IBU column (right-aligned). Each row renders:

```ts
const ibu = hopIbus.get(h.id);
// display: ibu > 0 ? ibu.toFixed(0) : "—"
```

Resulting table layout:

```
Name          AA%    Amount    Use         Time    IBU
──────────────────────────────────────────────────────
Magnum        12%    28g       Boil        60min    35
Citra          5%    28g       Boil        10min     8
Citra          5%    56g       Hopstand    20min     3
Citra          5%    56g       Dry Hop     5days     —
```

## Out of scope

- IBU per hop on the Overview tab or mobile stats summary
- Utilization percentage or any other per-hop stat beyond IBU
