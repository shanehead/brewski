# Hop IBU Contributions Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Display each hop addition's IBU contribution as a column in the Ingredients tab hops table.

**Architecture:** Extend `RecipeStats` with a `hop_stats` array of `HopStat` objects (each carrying a `hop_id` and `ibu`). The backend refactors `tinseth_ibu` to operate on a single hop, then `calculate_stats` maps over hops to produce `hop_stats`. The frontend threads `stats` down from `RecipeView` → `IngredientsTab` → `HopsTable`, where a `Map<string, number>` keyed by `hop_id` drives the new IBU column.

**Tech Stack:** Rust (Tauri backend), SvelteKit + TypeScript (frontend), OpenAPI YAML (schema source of truth), `just gen` for code generation, Vitest + Testing Library (frontend tests)

---

### Task 1: Add `HopStat` schema and extend `RecipeStats`

**Files:**
- Create: `docs/openapi/components/schemas/HopStat.yaml`
- Modify: `docs/openapi/components/schemas/RecipeStats.yaml`

- [ ] **Step 1: Create `HopStat.yaml`**

```yaml
# docs/openapi/components/schemas/HopStat.yaml
type: object
required:
  - hop_id
  - ibu
properties:
  hop_id:
    type: string
    description: The RecipeAdditionHop.id this stat corresponds to
  ibu:
    type: number
    description: IBU contribution from this hop addition
```

- [ ] **Step 2: Register `HopStat` in the OpenAPI index**

Open `docs/openapi/openapi.yaml` and add `HopStat` to the `schemas` map alongside the other entries:

```yaml
    HopStat:
      $ref: ./components/schemas/HopStat.yaml
```

- [ ] **Step 3: Extend `RecipeStats.yaml`**

Add `hop_stats` to both the `required` list and `properties`:

```yaml
type: object
required:
  - og
  - fg
  - abv_pct
  - ibu
  - srm
  - calories_per_355ml
  - bu_gu_ratio
  - pre_boil_gravity
  - pre_boil_volume_l
  - post_boil_volume_l
  - hop_stats
properties:
  og:
    type: number
    description: Calculated original gravity (specific gravity)
  fg:
    type: number
    description: Calculated final gravity (specific gravity)
  abv_pct:
    type: number
    description: Alcohol by volume percentage
  ibu:
    type: number
    description: International Bitterness Units
  srm:
    type: number
    description: Standard Reference Method color value
  calories_per_355ml:
    type: number
    description: Estimated calories per 355 ml (12 oz) serving
  bu_gu_ratio:
    type: number
    description: Bitterness/gravity ratio
  pre_boil_gravity:
    type: number
    description: Estimated pre-boil specific gravity
  pre_boil_volume_l:
    type: number
    description: Estimated pre-boil volume in litres
  post_boil_volume_l:
    type: number
    description: Estimated post-boil volume in litres
  strike_temp_c:
    type:
      - number
      - "null"
    description: Calculated strike water temperature in degrees Celsius
  hop_stats:
    type: array
    items:
      $ref: ./HopStat.yaml
    description: Per-hop IBU contributions, keyed by hop addition ID
```

- [ ] **Step 4: Commit**

```bash
git add docs/openapi/components/schemas/HopStat.yaml docs/openapi/components/schemas/RecipeStats.yaml docs/openapi/openapi.yaml
git commit -m "feat: add HopStat schema and extend RecipeStats with hop_stats"
```

---

### Task 2: Regenerate TypeScript and Rust types

**Files:**
- Modify (auto-generated): `src/lib/api.gen.ts`
- Modify (auto-generated): `src-tauri/src/models.gen.rs`
- Modify (fix test fixtures): `tests/MashTab.test.ts`
- Modify (fix test fixture): `tests/StatsSidebar.test.ts`

- [ ] **Step 1: Regenerate types**

```bash
just gen
```

This runs `redocly bundle` → `openapi-typescript` → `src/lib/api.gen.ts`, then `cargo typify` → `src-tauri/src/models.gen.rs`. Both now include `HopStat` and the updated `RecipeStats`.

- [ ] **Step 2: Fix TypeScript type errors in MashTab test**

`hop_stats` is now required on `RecipeStats`. Open `tests/MashTab.test.ts` and add it to `makeStats`:

```typescript
function makeStats(overrides: Partial<RecipeStats> = {}): RecipeStats {
  return {
    og: 1.050,
    fg: 1.012,
    abv_pct: 5.0,
    ibu: 20,
    srm: 5,
    calories_per_355ml: 150,
    bu_gu_ratio: 0.4,
    pre_boil_gravity: 1.040,
    pre_boil_volume_l: 27,
    post_boil_volume_l: 23,
    strike_temp_c: null,
    hop_stats: [],
    ...overrides,
  };
}
```

- [ ] **Step 3: Fix TypeScript type errors in StatsSidebar test**

Open `tests/StatsSidebar.test.ts` and update `makeStats` to include `hop_stats`:

```typescript
function makeStats(og = 1.054, fg = 1.010): RecipeStats {
  return {
    og, fg,
    abv_pct: 5.8,
    ibu: 45,
    srm: 8,
    bu_gu_ratio: 0.83,
    calories_per_355ml: 180,
    pre_boil_gravity: 1.040,
    pre_boil_volume_l: 26,
    post_boil_volume_l: 23,
    strike_temp_c: null,
    hop_stats: [],
  } as RecipeStats;
}
```

- [ ] **Step 4: Verify no TypeScript errors**

```bash
bun run check
```

Expected: no errors.

- [ ] **Step 5: Commit**

```bash
git add src/lib/api.gen.ts src-tauri/src/models.gen.rs tests/MashTab.test.ts tests/StatsSidebar.test.ts
git commit -m "feat: regenerate types with HopStat and hop_stats"
```

---

### Task 3: Refactor `tinseth_ibu` to operate on a single hop

**Files:**
- Modify: `src-tauri/src/brewing/ibu.rs`

The function currently takes a `&[HopIbuInput]` slice and returns their sum. Refactor it to accept a single `&HopIbuInput` and return that addition's contribution. All existing tests use single-element vecs and are updated to call the function directly.

- [ ] **Step 1: Replace the function body and update all tests**

Replace the entire contents of `src-tauri/src/brewing/ibu.rs` with:

```rust
pub struct HopIbuInput<'a> {
    pub alpha_pct: &'a f64,
    pub amount_kg: &'a f64,
    pub time_min: &'a f64,
    pub use_type: &'a str,
    pub form: &'a str,
    /// Pre-resolved: per-hop override → recipe default → 80.0
    pub hopstand_temp_c: f64,
    /// Extra whirlpool time added to hopstand additions, in minutes
    pub whirlpool_time_min: f64,
    /// When Some, use this flat utilization fraction instead of the Malowicki model
    pub aroma_utilization_override: Option<f64>,
}

/// Malowicki & Shellhammer (2005) isomerization rate model.
/// Returns the boil-equivalent minutes for actual_min at temp_c.
/// k1(T) = 7.9e11 * exp(-11858 / T), T in Kelvin.
pub fn malowicki_effective_time(actual_min: f64, temp_c: f64) -> f64 {
    let t = temp_c + 273.15;
    let k1_t = 7.9e11_f64 * f64::exp(-11858.0 / t);
    let k1_boil = 7.9e11_f64 * f64::exp(-11858.0 / 373.15);
    actual_min * (k1_t / k1_boil)
}

fn form_utilization(form_lower: &str) -> f64 {
    match form_lower {
        "leaf" | "plug" => 0.85,
        _ => 1.0,
    }
}

pub fn tinseth_ibu(hop: &HopIbuInput, og: f64, post_boil_volume_l: f64, boil_time_min: f64) -> f64 {
    let volume_gallons = post_boil_volume_l * 0.264172;
    // Tinseth bigness factor: accounts for wort gravity suppressing utilization.
    // Constants 1.65 and 0.000125 are empirically derived by Glenn Tinseth.
    let bigness = 1.65 * 0.000125f64.powf(og - 1.0);

    let use_lower = hop.use_type.to_lowercase();
    // Mash and dry hop never contribute IBUs regardless of form.
    if matches!(use_lower.as_str(), "mash" | "dry hop") {
        return 0.0;
    }
    let ounces = *hop.amount_kg * 35.274;
    let alpha_fraction = *hop.alpha_pct / 100.0;
    let form_lower = hop.form.to_lowercase();
    // CO2 extract: fully isomerized — full utilization regardless of boil time.
    // Bigness still applies (gravity suppresses utilization even for extracts).
    if form_lower == "co2 extract" {
        // Pre-isomerized: max Tinseth utilization regardless of boil time.
        // Time factor saturates at 1/4.15; extract gets that ceiling unconditionally.
        return (bigness * (1.0 / 4.15) * alpha_fraction * ounces * 7490.0) / volume_gallons;
    }
    let effective_time = match use_lower.as_str() {
        "first wort" => boil_time_min,
        "hopstand" => {
            if let Some(flat_util) = hop.aroma_utilization_override {
                return (flat_util
                    * form_utilization(&form_lower)
                    * alpha_fraction
                    * ounces
                    * 7490.0)
                    / volume_gallons;
            }
            malowicki_effective_time(*hop.time_min + hop.whirlpool_time_min, hop.hopstand_temp_c)
        }
        _ => *hop.time_min,
    };
    if effective_time <= 0.0 {
        return 0.0;
    }
    // Tinseth time factor: models the exponential approach to maximum utilization.
    // -0.04 is the time decay constant; 4.15 normalises to a 0–1 range.
    let time_factor = (1.0 - f64::exp(-0.04 * effective_time)) / 4.15;
    let utilization = bigness * time_factor * form_utilization(&form_lower);
    // 7490 converts (utilization × AAU × oz / gal) to IBUs.
    // Derived from Tinseth's original formula constants.
    (utilization * alpha_fraction * ounces * 7490.0) / volume_gallons
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_malowicki_at_boiling_returns_actual_time() {
        let effective = malowicki_effective_time(20.0, 100.0);
        assert!((effective - 20.0).abs() < 0.01, "got {effective}");
    }

    #[test]
    fn test_malowicki_at_80c_reduces_time() {
        let effective = malowicki_effective_time(20.0, 80.0);
        assert!(effective > 2.0 && effective < 5.0, "got {effective}");
    }

    #[test]
    fn test_malowicki_at_0c_is_near_zero() {
        let effective = malowicki_effective_time(60.0, 0.0);
        assert!(effective < 0.001, "got {effective}");
    }

    #[test]
    fn test_ibu_single_addition() {
        // 28g (0.028 kg) of 10% AA hops, 60 min, OG 1.047, 23L → ~29 IBU
        let hop = HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &60.0f64,
            use_type: "Boil",
            form: "Pellet",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        };
        let ibu = tinseth_ibu(&hop, 1.047, 23.0, 60.0);
        assert!((ibu - 29.0).abs() < 3.0, "IBU was {ibu:.1}, expected ~29");
    }

    #[test]
    fn test_dry_hop_contributes_zero_ibu() {
        let hop = HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &0.0f64,
            use_type: "Dry Hop",
            form: "Pellet",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        };
        let ibu = tinseth_ibu(&hop, 1.047, 23.0, 60.0);
        assert_eq!(ibu, 0.0);
    }

    #[test]
    fn test_mash_hop_contributes_zero_ibu() {
        let hop = HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &60.0f64,
            use_type: "Mash",
            form: "Pellet",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        };
        let ibu = tinseth_ibu(&hop, 1.047, 23.0, 60.0);
        assert_eq!(ibu, 0.0);
    }

    #[test]
    fn test_hopstand_contributes_less_than_boil() {
        let boil_hop = HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &20.0f64,
            use_type: "Boil",
            form: "Pellet",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        };
        let hopstand_hop = HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &20.0f64,
            use_type: "Hopstand",
            form: "Pellet",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        };
        let boil_ibu = tinseth_ibu(&boil_hop, 1.047, 23.0, 60.0);
        let hopstand_ibu = tinseth_ibu(&hopstand_hop, 1.047, 23.0, 60.0);
        assert!(
            hopstand_ibu < boil_ibu,
            "hopstand {hopstand_ibu} should be < boil {boil_ibu}"
        );
        assert!(hopstand_ibu > 0.0, "hopstand ibu should be > 0");
    }

    #[test]
    fn test_hopstand_at_boiling_equals_boil() {
        let boil_hop = HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &20.0f64,
            use_type: "Boil",
            form: "Pellet",
            hopstand_temp_c: 100.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        };
        let hopstand_hop = HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &20.0f64,
            use_type: "Hopstand",
            form: "Pellet",
            hopstand_temp_c: 100.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        };
        let boil_ibu = tinseth_ibu(&boil_hop, 1.047, 23.0, 60.0);
        let hopstand_ibu = tinseth_ibu(&hopstand_hop, 1.047, 23.0, 60.0);
        assert!(
            (hopstand_ibu - boil_ibu).abs() < 0.01,
            "at 100°C: hopstand={hopstand_ibu}, boil={boil_ibu}"
        );
    }

    #[test]
    fn test_first_wort_uses_boil_time() {
        let first_wort = HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &0.0f64,
            use_type: "First Wort",
            form: "Pellet",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        };
        let boil_60 = HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &60.0f64,
            use_type: "Boil",
            form: "Pellet",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        };
        let fw_ibu = tinseth_ibu(&first_wort, 1.047, 23.0, 60.0);
        let boil_ibu = tinseth_ibu(&boil_60, 1.047, 23.0, 60.0);
        assert!(
            (fw_ibu - boil_ibu).abs() < 0.01,
            "FWH={fw_ibu}, Boil60={boil_ibu}"
        );
    }

    #[test]
    fn test_leaf_hop_reduces_ibu_by_15_percent() {
        let pellet = HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &60.0f64,
            use_type: "Boil",
            form: "Pellet",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        };
        let leaf = HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &60.0f64,
            use_type: "Boil",
            form: "Leaf",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        };
        let pellet_ibu = tinseth_ibu(&pellet, 1.047, 23.0, 60.0);
        let leaf_ibu = tinseth_ibu(&leaf, 1.047, 23.0, 60.0);
        let ratio = leaf_ibu / pellet_ibu;
        assert!(
            (ratio - 0.85).abs() < 0.01,
            "leaf/pellet ratio {ratio:.4}, expected 0.85"
        );
    }

    #[test]
    fn test_plug_hop_reduces_ibu_by_15_percent() {
        let pellet = HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &60.0f64,
            use_type: "Boil",
            form: "Pellet",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        };
        let plug = HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &60.0f64,
            use_type: "Boil",
            form: "Plug",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        };
        let pellet_ibu = tinseth_ibu(&pellet, 1.047, 23.0, 60.0);
        let plug_ibu = tinseth_ibu(&plug, 1.047, 23.0, 60.0);
        let ratio = plug_ibu / pellet_ibu;
        assert!(
            (ratio - 0.85).abs() < 0.01,
            "plug/pellet ratio {ratio:.4}, expected 0.85"
        );
    }

    #[test]
    fn test_cryo_hop_same_ibu_as_pellet() {
        let pellet = HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &60.0f64,
            use_type: "Boil",
            form: "Pellet",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        };
        let cryo = HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &60.0f64,
            use_type: "Boil",
            form: "Cryo",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        };
        let pellet_ibu = tinseth_ibu(&pellet, 1.047, 23.0, 60.0);
        let cryo_ibu = tinseth_ibu(&cryo, 1.047, 23.0, 60.0);
        assert!(
            (cryo_ibu - pellet_ibu).abs() < 0.01,
            "cryo IBU {cryo_ibu:.2} should equal pellet IBU {pellet_ibu:.2}"
        );
    }

    #[test]
    fn test_co2_extract_ignores_boil_time() {
        let short = HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &1.0f64,
            use_type: "Boil",
            form: "CO2 Extract",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        };
        let long = HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &60.0f64,
            use_type: "Boil",
            form: "CO2 Extract",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        };
        let short_ibu = tinseth_ibu(&short, 1.047, 23.0, 60.0);
        let long_ibu = tinseth_ibu(&long, 1.047, 23.0, 60.0);
        assert!(
            (short_ibu - long_ibu).abs() < 0.01,
            "CO2 extract IBU should not depend on boil time: 1min={short_ibu:.2}, 60min={long_ibu:.2}"
        );
    }

    #[test]
    fn test_co2_extract_dry_hop_contributes_zero_ibu() {
        let hop = HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &0.0f64,
            use_type: "Dry Hop",
            form: "CO2 Extract",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        };
        let ibu = tinseth_ibu(&hop, 1.047, 23.0, 60.0);
        assert_eq!(ibu, 0.0);
    }

    #[test]
    fn test_leaf_hopstand_with_utilization_override_reduces_ibu() {
        let pellet = HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &20.0f64,
            use_type: "Hopstand",
            form: "Pellet",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: Some(0.23),
        };
        let leaf = HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &20.0f64,
            use_type: "Hopstand",
            form: "Leaf",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: Some(0.23),
        };
        let pellet_ibu = tinseth_ibu(&pellet, 1.047, 23.0, 60.0);
        let leaf_ibu = tinseth_ibu(&leaf, 1.047, 23.0, 60.0);
        let ratio = leaf_ibu / pellet_ibu;
        assert!(
            (ratio - 0.85).abs() < 0.01,
            "leaf/pellet hopstand override ratio {ratio:.4}, expected 0.85"
        );
    }

    #[test]
    fn test_co2_extract_higher_utilization_than_pellet_at_60min() {
        let co2 = HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &1.0f64,
            use_type: "Boil",
            form: "CO2 Extract",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        };
        let pellet = HopIbuInput {
            alpha_pct: &10.0f64,
            amount_kg: &0.028f64,
            time_min: &60.0f64,
            use_type: "Boil",
            form: "Pellet",
            hopstand_temp_c: 80.0,
            whirlpool_time_min: 0.0,
            aroma_utilization_override: None,
        };
        let co2_ibu = tinseth_ibu(&co2, 1.047, 23.0, 60.0);
        let pellet_ibu = tinseth_ibu(&pellet, 1.047, 23.0, 60.0);
        assert!(
            co2_ibu > pellet_ibu,
            "CO2 extract (saturated utilization ~0.241) {co2_ibu:.2} should exceed pellet 60min time_factor ~0.219 {pellet_ibu:.2}"
        );
    }
}
```

- [ ] **Step 2: Run the Rust tests**

```bash
cd src-tauri && cargo test brewing::ibu
```

Expected: all ibu tests pass.

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/brewing/ibu.rs
git commit -m "refactor: tinseth_ibu acts on a single hop addition"
```

---

### Task 4: Update `calculate_stats` to produce `hop_stats`

**Files:**
- Modify: `src-tauri/src/brewing/mod.rs`

- [ ] **Step 1: Update the import in `mod.rs`**

At the top of `src-tauri/src/brewing/mod.rs`, change:

```rust
use crate::models::{Recipe, RecipeStats};
```

to:

```rust
use crate::models::{HopStat, Recipe, RecipeStats};
```

- [ ] **Step 2: Replace the IBU calculation in `calculate_stats`**

Find and replace the two lines that build `hop_inputs` and compute `ibu`:

```rust
    let ibu = ibu::tinseth_ibu(&hop_inputs, og, post_boil_volume_l, recipe.boil_time_min);
```

Replace with:

```rust
    let hop_stats: Vec<HopStat> = recipe
        .hops
        .iter()
        .zip(hop_inputs.iter())
        .map(|(h, input)| HopStat {
            hop_id: h.id.clone(),
            ibu: ibu::tinseth_ibu(input, og, post_boil_volume_l, recipe.boil_time_min),
        })
        .collect();

    let ibu: f64 = hop_stats.iter().map(|s| s.ibu).sum();
```

- [ ] **Step 3: Add `hop_stats` to the returned `RecipeStats`**

Find the `RecipeStats { ... }` struct literal at the end of `calculate_stats` and add `hop_stats`:

```rust
    RecipeStats {
        og,
        fg,
        abv_pct,
        ibu,
        srm,
        calories_per_355ml: calories,
        bu_gu_ratio,
        pre_boil_gravity,
        pre_boil_volume_l,
        post_boil_volume_l,
        strike_temp_c,
        hop_stats,
    }
```

- [ ] **Step 4: Extend the existing `test_stats_with_hops` test**

In the `tests` module inside `mod.rs`, find `test_stats_with_hops` and add assertions after the existing ones:

```rust
    assert_eq!(stats.hop_stats.len(), 1);
    assert_eq!(stats.hop_stats[0].hop_id, "h1");
    assert!(stats.hop_stats[0].ibu > 0.0);
    // Individual hop sum must equal reported total.
    let sum: f64 = stats.hop_stats.iter().map(|s| s.ibu).sum();
    assert!((sum - stats.ibu).abs() < 0.001);
```

- [ ] **Step 5: Run all Rust tests**

```bash
cd src-tauri && cargo test
```

Expected: all tests pass including the fixture-based stats tests.

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/brewing/mod.rs
git commit -m "feat: calculate_stats produces per-hop hop_stats"
```

---

### Task 5: Thread `stats` through `IngredientsTab` and both `RecipeView` components

**Files:**
- Modify: `src/lib/components/tabs/IngredientsTab.svelte`
- Modify: `src/lib/desktop/RecipeView.svelte`
- Modify: `src/lib/mobile/RecipeView.svelte`

- [ ] **Step 1: Add `stats` prop to `IngredientsTab.svelte`**

Open `src/lib/components/tabs/IngredientsTab.svelte`. Update the script block import and prop:

```svelte
<script lang="ts">
  import type { Recipe, RecipeStats } from "$lib/api";
  import FermentablesTable from "$lib/components/ingredients/FermentablesTable.svelte";
  import HopsTable from "$lib/components/ingredients/HopsTable.svelte";
  import YeastsTable from "$lib/components/ingredients/YeastsTable.svelte";
  import MiscTable from "$lib/components/ingredients/MiscTable.svelte";
  import Card from "$lib/components/Card.svelte";

  let { recipe, stats, onchange }: { recipe: Recipe; stats: RecipeStats | null; onchange: () => void } = $props();
</script>
```

Then pass `{stats}` to `HopsTable` in the template:

```svelte
  <Card title="Hops">
    <HopsTable {recipe} {stats} {onchange} />
  </Card>
```

- [ ] **Step 2: Pass `stats` from desktop `RecipeView.svelte`**

Open `src/lib/desktop/RecipeView.svelte`. Find line 410 and update the `IngredientsTab` call:

```svelte
            <IngredientsTab recipe={displayRecipe} {stats} onchange={refreshRecipe} />
```

- [ ] **Step 3: Pass `stats` from mobile `RecipeView.svelte`**

Open `src/lib/mobile/RecipeView.svelte`. Find line 171 and update the `IngredientsTab` call:

```svelte
          <IngredientsTab {recipe} {stats} onchange={load} />
```

- [ ] **Step 4: Type-check**

```bash
bun run check
```

Expected: no errors.

- [ ] **Step 5: Commit**

```bash
git add src/lib/components/tabs/IngredientsTab.svelte src/lib/desktop/RecipeView.svelte src/lib/mobile/RecipeView.svelte
git commit -m "feat: thread stats through IngredientsTab to HopsTable"
```

---

### Task 6: Add IBU column to `HopsTable` and write tests

**Files:**
- Create: `tests/HopsTable.test.ts`
- Modify: `src/lib/components/ingredients/HopsTable.svelte`

- [ ] **Step 1: Write the failing test**

Create `tests/HopsTable.test.ts`:

```typescript
import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";
import type { Recipe, RecipeStats, RecipeAdditionHop } from "$lib/api";
import HopsTable from "$lib/components/ingredients/HopsTable.svelte";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));

function makeHop(overrides: Partial<RecipeAdditionHop> = {}): RecipeAdditionHop {
  return {
    id: "h1",
    recipe_id: "r1",
    hop_id: null,
    name: "Cascade",
    alpha_pct: 5.5,
    form: "Pellet",
    amount_kg: 0.05,
    use_: "Boil",
    time_min: 60,
    addition_order: 0,
    hopstand_temp_c: null,
    ...overrides,
  };
}

function makeRecipe(hops: RecipeAdditionHop[]): Recipe {
  return {
    id: "r1",
    hops,
    fermentables: [],
    yeasts: [],
    miscs: [],
    waters: [],
    water_adjustments: [],
  } as unknown as Recipe;
}

function makeStats(hopStats: Array<{ hop_id: string; ibu: number }>): RecipeStats {
  return {
    og: 1.050,
    fg: 1.012,
    abv_pct: 5.0,
    ibu: hopStats.reduce((sum, s) => sum + s.ibu, 0),
    srm: 5,
    calories_per_355ml: 150,
    bu_gu_ratio: 0.4,
    pre_boil_gravity: 1.040,
    pre_boil_volume_l: 27,
    post_boil_volume_l: 23,
    strike_temp_c: null,
    hop_stats: hopStats,
  } as RecipeStats;
}

describe("HopsTable IBU column", () => {
  it("shows rounded IBU for a boil hop with a non-zero contribution", () => {
    const hop = makeHop({ id: "h1" });
    render(HopsTable, {
      recipe: makeRecipe([hop]),
      stats: makeStats([{ hop_id: "h1", ibu: 29.3 }]),
      onchange: vi.fn(),
    });
    expect(screen.getByText("29")).toBeInTheDocument();
  });

  it("shows — for a dry hop addition (zero IBU)", () => {
    const hop = makeHop({ id: "h1", use_: "Dry Hop" });
    render(HopsTable, {
      recipe: makeRecipe([hop]),
      stats: makeStats([{ hop_id: "h1", ibu: 0 }]),
      onchange: vi.fn(),
    });
    const dashes = screen.getAllByText("—");
    expect(dashes.length).toBeGreaterThan(0);
  });

  it("shows — for all hops when stats is null", () => {
    const hop = makeHop({ id: "h1" });
    render(HopsTable, {
      recipe: makeRecipe([hop]),
      stats: null,
      onchange: vi.fn(),
    });
    const dashes = screen.getAllByText("—");
    expect(dashes.length).toBeGreaterThan(0);
  });
});
```

- [ ] **Step 2: Run the test to confirm it fails**

```bash
bun run test -- HopsTable
```

Expected: FAIL — `stats` prop not accepted by `HopsTable`.

- [ ] **Step 3: Update `HopsTable.svelte`**

Replace the entire contents of `src/lib/components/ingredients/HopsTable.svelte` with:

```svelte
<script lang="ts">
  import type { Recipe, RecipeStats } from "$lib/api";
  import { createRecipeHop, deleteRecipeHop } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { settings } from "$lib/stores/settings";
  import { type Units, kgToHopDisplay, hopWeightLabel, cToF, tempLabel } from "$lib/units";
  import BrewingIcon from "$lib/components/BrewingIcon.svelte";
  import IngredientPicker, { type AddPayload } from "$platform/IngredientPicker.svelte";
  import Tooltip from "$lib/components/Tooltip.svelte";
  import DocLink from "$lib/components/DocLink.svelte";
  import { DOCS } from "$lib/docs-urls";

  let { recipe, stats, onchange }: { recipe: Recipe; stats: RecipeStats | null; onchange: () => void } = $props();

  let adding = $state(false);
  const units = $derived<Units>($settings.units === "imperial" ? "imperial" : "metric");

  const hopIbus = $derived(
    new Map(stats?.hop_stats?.map(s => [s.hop_id, s.ibu]) ?? [])
  );

  async function handlePickerAdd(payload: AddPayload) {
    if (payload.type !== "hop") return;
    const result = await ipc(createRecipeHop(recipe.id, {
      hop_id: payload.item.id,
      name: payload.item.name,
      alpha_pct: payload.item.alpha_pct,
      form: payload.form,
      amount_kg: payload.amount_kg,
      use_: payload.use_,
      time_min: payload.time_min,
      hopstand_temp_c: payload.hopstand_temp_c ?? undefined,
    }));
    if (result === undefined) return;
    adding = false;
    onchange();
  }

  async function handleDelete(id: string) {
    await ipc(deleteRecipeHop(id));
    onchange();
  }
</script>

<div class="flex flex-col gap-2">
  <div class="flex items-center justify-between">
    <h3 class="text-sm font-semibold flex items-center gap-2" style="color: var(--color-text-primary);">
      <BrewingIcon name="hop" />
      Hops
    </h3>
    <div class="flex items-center gap-2">
      <DocLink label="Hops guide" url={DOCS.hops} />
      <button onclick={() => adding = true} class="text-xs px-2 py-1 rounded"
              style="background: var(--color-accent); color: #fff;">+ Add</button>
    </div>
  </div>

  <IngredientPicker
    type="hop"
    open={adding}
    onclose={() => adding = false}
    onadd={handlePickerAdd}
  />

  {#if recipe.hops.length > 0}
    <table class="w-full text-sm">
      <thead>
        <tr style="color: var(--color-text-secondary);">
          <th class="text-left py-1 font-medium text-sm">Name</th>
          <th class="text-right py-1 font-medium text-sm">
            <span class="inline-flex items-center gap-1">AA% <Tooltip text="Alpha acid percentage. This drives bitterness. Higher alpha means fewer grams to hit your IBU target." /></span>
          </th>
          <th class="text-right py-1 font-medium text-sm">{hopWeightLabel(units)}</th>
          <th class="text-right py-1 font-medium text-sm">
            <span class="inline-flex items-center gap-1">Use <Tooltip text="When the hop is added. Boil adds bitterness. Whirlpool and Hopstand add flavor and aroma. Dry Hop adds aroma only." /></span>
          </th>
          <th class="text-right py-1 font-medium text-sm">Time</th>
          <th class="text-right py-1 font-medium text-sm">IBU</th>
          <th class="w-6"></th>
        </tr>
      </thead>
      <tbody>
        {#each recipe.hops as h (h.id)}
          {@const ibu = hopIbus.get(h.id)}
          <tr class="border-t" style="border-color: var(--color-border);">
            <td class="py-1.5" style="color: var(--color-text-primary);">
              {h.name}
              {#if h.form !== 'Pellet'}
                {@const badgeColor =
                  h.form === 'Cryo' ? 'background: #d1fae5; color: #065f46;' :
                  h.form === 'CO2 Extract' ? 'background: #ede9fe; color: #5b21b6;' :
                  'background: var(--color-bg-elevated); color: var(--color-text-secondary);'}
                <span style="font-size: 10px; padding: 1px 5px; border-radius: 4px; font-weight: 600; margin-left: 4px; {badgeColor}">{h.form}</span>
              {/if}
            </td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">{h.alpha_pct}%</td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">{kgToHopDisplay(h.amount_kg, units).toFixed(units === "imperial" ? 2 : 0)}{hopWeightLabel(units)}</td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">
              {#if h.use_ === 'hopstand' && h.hopstand_temp_c != null}
                {h.use_} ({units === 'imperial' ? cToF(h.hopstand_temp_c).toFixed(0) : h.hopstand_temp_c.toFixed(0)}{tempLabel(units)})
              {:else}
                {h.use_}
              {/if}
            </td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">{h.time_min}min</td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">
              {ibu != null && ibu > 0 ? ibu.toFixed(0) : '—'}
            </td>
            <td class="pl-1">
              <button onclick={() => handleDelete(h.id)} class="text-xs opacity-40 hover:opacity-100"
                      style="color: var(--color-text-secondary);">×</button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>
```

- [ ] **Step 4: Run the test to confirm it passes**

```bash
bun run test -- HopsTable
```

Expected: all 3 tests pass.

- [ ] **Step 5: Run all frontend tests**

```bash
bun run test
```

Expected: all tests pass.

- [ ] **Step 6: Commit**

```bash
git add tests/HopsTable.test.ts src/lib/components/ingredients/HopsTable.svelte
git commit -m "feat: show per-hop IBU contributions in hops table"
```
