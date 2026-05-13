# BeerXML Fixture Tests for Calculation Validation

**Date:** 2026-05-13  
**Status:** Approved

## Overview

Add integration tests that validate `calculate_stats` against known recipe data. Three fixtures come from BrewDog's published DIY Dog BeerXML files; one is a hand-crafted BeerXML file built from a known Brewfather recipe (Nectaron Single Hop Hazy DIPA). Tests parse the fixtures, run our brewing calculations, and assert the output falls within tolerance of the expected values.

## Fixtures

Four XML files stored in `src-tauri/tests/fixtures/`:

| File | Recipe | Source | Why |
|------|--------|--------|-----|
| `punk_ipa_2007.xml` | Punk IPA 2007–2010 | diydog-beerxml repo | Simple single-malt grain bill; good baseline for OG/IBU |
| `alpha_dog.xml` | Alpha Dog | diydog-beerxml repo | Multi-grain; exercises color calculation |
| `jet_black_heart.xml` | Jet Black Heart | diydog-beerxml repo | Dark stout; pushes high-SRM Morey formula |
| `nectaron_hazy_dipa.xml` | Nectaron Single Hop Hazy DIPA | Hand-crafted from Brewfather screenshot | Real homebrewer recipe; includes hopstand addition |

The three diydog files are downloaded directly from https://github.com/stuartraetaylor/diydog-beerxml. Expected values come from the embedded `EST_OG`, `EST_FG`, `IBU`, `EST_COLOR` fields.

The Nectaron DIPA file is hand-crafted BeerXML from Brewfather data extracted from screenshots. Its ingredient data and expected stats are:

**Fermentables** (batch 5.5 gal / 20.82 L, 68% efficiency):

| Name | Amount | Yield | Color |
|------|--------|-------|-------|
| Pale Ale Golden Promise (Simpsons) | 6.804 kg | 80% | 2.4 °L |
| Oats, Flaked (Briess) | 0.907 kg | 70% | 1.6 °L |
| Dextrose (Briess) | 0.454 kg | 100% | 1.3 °L |

**Hops** (all Nectaron 10.1%):

| Amount | Use | Time |
|--------|-----|------|
| 28.35 g | Boil | 60 min |
| 28.35 g | Boil | 20 min |
| 28.35 g | Boil | 10 min |
| 56.70 g | Aroma (hopstand @ 82°C / 180°F) | 20 min |
| 141.75 g | Dry Hop | — |

**Yeast:** Imperial Yeast A04 Barbarian, 74% attenuation

**Expected stats (from Brewfather):** OG 1.084 · FG 1.018 · IBU 67 · SRM 5.6 · ABV 8.7%

**IBU note:** Brewfather's 67 IBU includes ~7.9 IBU from the hopstand addition. Our current `tinseth_ibu` has no hopstand temperature model (that is a separate spec). The boil-only additions (60 + 20 + 10 min) contribute ~59 IBU. The fixture test for this recipe therefore asserts against **59 IBU** (boil-only) with a comment explaining the discrepancy. The hopstand and dry-hop additions are passed with `use_ = "aroma"` / `use_ = "dry hop"` and contribute 0 IBU in the current model.

## Test-Only Parser

New file: `src-tauri/src/brewing/beerxml_fixture.rs`, compiled only under `#[cfg(test)]`.

### `ExpectedStats` struct

```rust
pub struct ExpectedStats {
    pub og: f64,
    pub fg: f64,
    pub ibu: f64,
    pub srm: f64,
}
```

### `load_fixture(filename: &str) -> (Recipe, ExpectedStats)`

Reads `tests/fixtures/<filename>` relative to the crate root (via `env!("CARGO_MANIFEST_DIR")`). Parses the XML using `quick_xml` (already a dependency). Builds a `Recipe` struct with:

- `batch_size_l`, `boil_time_min`, `efficiency_pct` from `<BATCH_SIZE>`, `<BOIL_TIME>`, `<EFFICIENCY>`
- Fermentables from `<FERMENTABLES>` → `RecipeAdditionFermentable` (name, yield_pct, color_lovibond, amount_kg, add_after_boil)
- Hops from `<HOPS>` → `RecipeAdditionHop` (alpha_pct, amount_kg, use_, time_min)
- Yeasts from `<YEASTS>` → `RecipeAdditionYeast` (attenuation_pct)
- All other `Recipe` fields set to sensible defaults (same pattern as `minimal_recipe()` in existing tests)

Extracts `EST_OG`, `EST_FG`, `IBU`, `EST_COLOR` into `ExpectedStats`. Panics with a descriptive message if a fixture file is missing or malformed (test failures, not silent skips).

The parser is separate from `parse_beerxml` in `import_export.rs` — that one targets the DB import path; this one targets the `Recipe` model directly. No production code changes.

## Tests

Four `#[test]` functions added to the existing `#[cfg(test)]` block in `src-tauri/src/brewing/mod.rs`:

```rust
#[test]
fn test_stats_punk_ipa_2007() { ... }

#[test]
fn test_stats_alpha_dog() { ... }

#[test]
fn test_stats_jet_black_heart() { ... }

#[test]
fn test_stats_nectaron_hazy_dipa() { ... }
```

Each test:
1. Calls `load_fixture("punk_ipa_2007.xml")` (etc.) → `(recipe, expected)`
2. Calls `calculate_stats(&recipe)`
3. Asserts with tolerances:

| Stat | Tolerance | Rationale |
|------|-----------|-----------|
| OG | ±0.003 | Minor rounding in extract calculations |
| FG | ±0.005 | Attenuation rounding compounds from OG |
| IBU | ±5.0 | Tinseth formula constants may differ slightly from BrewDog's software |
| SRM | ±1.5 | Morey formula is consistent but batch size rounding varies |

Assertion messages include the actual and expected values for easy diagnosis.

## Files Changed

| File | Change |
|------|--------|
| `src-tauri/tests/fixtures/punk_ipa_2007.xml` | New — downloaded from diydog-beerxml |
| `src-tauri/tests/fixtures/alpha_dog.xml` | New — downloaded from diydog-beerxml |
| `src-tauri/tests/fixtures/jet_black_heart.xml` | New — downloaded from diydog-beerxml |
| `src-tauri/tests/fixtures/nectaron_hazy_dipa.xml` | New — hand-crafted from Brewfather data |
| `src-tauri/src/brewing/beerxml_fixture.rs` | New — test-only parser |
| `src-tauri/src/brewing/mod.rs` | Add four fixture-based tests; add `mod beerxml_fixture` under `#[cfg(test)]` |
