# BJCP 2021 Style Seeding

**Date:** 2026-05-10
**Status:** Approved

## Goal

Populate the `styles` table with all BJCP 2021 style guide entries, overwriting any existing styles seed data.

## Data Source

Use `bjcp-2021-styles.json` — a local copy of the canonical BJCP 2021 machine-readable JSON (from `christinedraper/bjcp-guidelines` on GitHub), committed to the repo root alongside the existing ingredient data files (`fermentables.json`, `hops.json`, `yeasts.json`).

## Transform Script

A one-off bun script at `scripts/seed-styles.mjs`:

1. Reads `bjcp-2021-styles.json` from the repo root
2. Maps each sub-style entry to the `styles` table schema (see field mapping below)
3. Writes a `INSERT INTO styles (...)` block into `src-tauri/src/migration/sql/001_initial.sql`, replacing any existing styles INSERT block

The script is idempotent: re-running it produces the same SQL output.

## Field Mapping

| SQL column         | BJCP JSON field                       | Notes                                      |
|--------------------|---------------------------------------|--------------------------------------------|
| `id`               | `"{category_number}{style_letter}"`   | e.g. `"1A"` — unique per sub-style         |
| `name`             | sub-style `name`                      |                                            |
| `category`         | parent category `name`                |                                            |
| `category_number`  | parent category `number` (as string)  | e.g. `"1"`                                 |
| `style_letter`     | sub-style `letter`                    | e.g. `"A"`                                 |
| `style_guide`      | `"BJCP 2021"`                         | constant                                   |
| `type_`            | sub-style `type` or `"Beer"`          | fallback to `"Beer"` if absent             |
| `og_min/og_max`    | `stats.og.low` / `stats.og.high`      | SG float, e.g. `1.048`; `0` if absent     |
| `fg_min/fg_max`    | `stats.fg.low` / `stats.fg.high`      | SG float; `0` if absent                   |
| `ibu_min/max`      | `stats.ibu.low` / `stats.ibu.high`    | `0` if absent                              |
| `color_min_srm`    | `stats.srm.low`                       | `0` if absent                              |
| `color_max_srm`    | `stats.srm.high`                      | `0` if absent                              |
| `abv_min_pct`      | `stats.abv.low`                       | nullable                                   |
| `abv_max_pct`      | `stats.abv.high`                      | nullable                                   |
| `carb_min_vols`    | `stats.co2.low`                       | nullable                                   |
| `carb_max_vols`    | `stats.co2.high`                      | nullable                                   |
| `notes`            | `overall_impression`                  | nullable                                   |
| `profile`          | `aroma` + `appearance` + `flavor`     | concatenated as `"Aroma: ...\nAppearance: ...\nFlavor: ..."`, nullable |
| `ingredients`      | `characteristic_ingredients`          | nullable                                   |
| `examples`         | `commercial_examples`                 | nullable                                   |

String values are SQL-escaped (single quotes doubled).

## Migration Strategy

The INSERT statements go into `001_initial.sql` — no new migration file is needed. The `styles` table is currently empty, so this is the initial seed. Existing dev databases pick up the data on the next `just migrate`.

## Justfile Integration

Add a `seed-styles` recipe to the Justfile that runs the transform script and applies migrations:

```
seed-styles:
    bun scripts/seed-styles.mjs
    just migrate
```

## Out of Scope

- UI changes for browsing or selecting styles (existing `list_styles` command already serves this)
- Adding new columns to the styles schema
- Runtime import command (styles are reference data, not user data)
