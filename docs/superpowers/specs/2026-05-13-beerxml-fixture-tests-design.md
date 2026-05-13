# BeerXML Fixture Tests for Calculation Validation

**Date:** 2026-05-13  
**Status:** Approved

## Overview

Add integration tests that validate `calculate_stats` against known recipe data from BrewDog's published DIY Dog BeerXML files. Tests parse real recipe fixtures, run our brewing calculations, and assert the output falls within tolerance of the expected values embedded in the XML.

## Fixtures

Three XML files downloaded from https://github.com/stuartraetaylor/diydog-beerxml and stored in `src-tauri/tests/fixtures/`:

| File | Recipe | Why |
|------|--------|-----|
| `punk_ipa_2007.xml` | Punk IPA 2007–2010 | Simple single-malt grain bill; good baseline for OG/IBU |
| `alpha_dog.xml` | Alpha Dog | Multi-grain; exercises color calculation |
| `jet_black_heart.xml` | Jet Black Heart | Dark stout; pushes high-SRM Morey formula |

Expected values present in each XML: `EST_OG`, `EST_FG`, `IBU`, `EST_COLOR`.

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

Three `#[test]` functions added to the existing `#[cfg(test)]` block in `src-tauri/src/brewing/mod.rs`:

```rust
#[test]
fn test_stats_punk_ipa_2007() { ... }

#[test]
fn test_stats_alpha_dog() { ... }

#[test]
fn test_stats_jet_black_heart() { ... }
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
| `src-tauri/tests/fixtures/punk_ipa_2007.xml` | New — downloaded fixture |
| `src-tauri/tests/fixtures/alpha_dog.xml` | New — downloaded fixture |
| `src-tauri/tests/fixtures/jet_black_heart.xml` | New — downloaded fixture |
| `src-tauri/src/brewing/beerxml_fixture.rs` | New — test-only parser |
| `src-tauri/src/brewing/mod.rs` | Add three fixture-based tests; add `mod beerxml_fixture` under `#[cfg(test)]` |
