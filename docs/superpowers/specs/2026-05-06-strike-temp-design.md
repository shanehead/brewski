# Strike Temperature Calculator — Design Spec

## Overview

Add strike water temperature to the Mash tab. The calculation lives entirely in Rust (`brewing/strike.rs`), the result surfaces in `RecipeStats`, and MashTab displays it as a read-only value. A stored fallback ratio (`ratio_l_per_kg` on the `mashes` table) lets the calculation work even when it can't be auto-derived.

## Formula

```
strike_temp_c = (0.2 / ratio_l_per_kg) × (target_temp_c − grain_temp_c) + target_temp_c
```

- `grain_temp_c` — from `mash.grain_temp_c` (already stored)
- `target_temp_c` — `step_temp_c` of the first infusion mash step
- `ratio_l_per_kg` — derived or stored (see below)

## Data Model

### DB migration (`src-tauri/src/migration/sql/003_strike_temp.sql`)

```sql
ALTER TABLE mashes ADD COLUMN ratio_l_per_kg REAL;
```

### Rust model changes

**`Mash`** — add field:
```rust
pub ratio_l_per_kg: Option<f64>,
```

**`UpdateMashInput`** — add field:
```rust
pub ratio_l_per_kg: Option<f64>,
```

**`RecipeStats`** — add field:
```rust
pub strike_temp_c: Option<f64>,
```

## Calculation Logic (`brewing/mod.rs`)

`calculate_stats` derives the ratio and calls the new function after the existing stat calculations:

1. **Total grain weight** — sum of `fermentable.amount_kg` across all fermentables
2. **First infusion step** — first `MashStep` where `infuse_amount_l.is_some()`
3. **Derived ratio** — `infuse_amount_l / total_grain_kg` when both are non-zero
4. **Fallback** — use `mash.ratio_l_per_kg` when derivation fails
5. **Result** — `strike_temp_c = Some(calculate_strike_temp(...))` if ratio is available; `None` otherwise

`strike_temp_c` is `None` when:
- Recipe has no mash
- Ratio cannot be derived AND no stored fallback ratio
- Total grain weight is zero

## New File: `brewing/strike.rs`

```rust
pub fn calculate_strike_temp(grain_temp_c: f64, target_temp_c: f64, ratio_l_per_kg: f64) -> f64 {
    (0.2 / ratio_l_per_kg) * (target_temp_c - grain_temp_c) + target_temp_c
}
```

Unit tested with a known reference point:
- grain 20°C, target 67°C, ratio 3.0 L/kg → ~71.4°C

## Frontend

### `api.ts`

Changes:

- `Mash`: add `ratio_l_per_kg: number | null`
- `RecipeStats`: add `strike_temp_c: number | null`
- Add new `UpdateMashInput` interface (currently `updateMash` accepts `object`; this types it properly):
  ```ts
  export interface UpdateMashInput {
    name?: string;
    grain_temp_c?: number;
    tun_temp_c?: number;
    sparge_temp_c?: number;
    ph?: number;
    notes?: string;
    ratio_l_per_kg?: number;
  }
  ```
- Update `updateMash` signature: `(recipeId: string, input: UpdateMashInput) => invoke<Mash>(...)`

### `+page.svelte`

Pass `stats` to MashTab:
```svelte
<MashTab {recipe} {stats} onchange={refreshRecipe} />
```

### `MashTab.svelte`

Prop signature change:
```ts
let { recipe, stats, onchange }: { recipe: Recipe; stats: RecipeStats | null; onchange: () => void }
```

**Strike temp display** — read-only, placed in the existing 2-column grid alongside grain temp. Shown only when `stats?.strike_temp_c != null`. Value converted per units setting (°C or °F).

**Ratio input** — shown only when auto-derive is unavailable: no fermentables with weight OR no infusion step with `infuse_amount_l`. Label is "Water:Grain Ratio (L/kg)" in metric, "Water:Grain Ratio (qt/lb)" in imperial. The DB always stores L/kg; imperial display converts using `1 qt/lb = 2.0864 L/kg`. Saves via `handleMashField("ratio_l_per_kg", value_in_l_per_kg)`.

No calculation logic in the frontend — only conditional renders and unit conversion of the display value.

## Testing

- `brewing/strike.rs` — unit test for the pure function with a reference value
- Existing `brewing/mod.rs` integration tests cover the stats pipeline; add one test case with a mash that has `infuse_amount_l` set to verify `strike_temp_c` is populated
- `just check` passes (TypeScript + OpenAPI lint)

## Files Changed

| File | Change |
|------|--------|
| `src-tauri/src/migration/sql/003_strike_temp.sql` | New — adds `ratio_l_per_kg` column |
| `src-tauri/src/models.rs` | Add field to `Mash`, `UpdateMashInput`, `RecipeStats` |
| `src-tauri/src/brewing/strike.rs` | New — pure calculation function |
| `src-tauri/src/brewing/mod.rs` | Wire strike temp into `calculate_stats` |
| `src/lib/api.ts` | Add fields to `Mash`, `UpdateMashInput`, `RecipeStats` |
| `src/routes/recipe/[id]/+page.svelte` | Pass `stats` to MashTab |
| `src/lib/components/tabs/MashTab.svelte` | Accept `stats` prop, display strike temp, show ratio field |
