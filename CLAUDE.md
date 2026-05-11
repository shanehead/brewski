# Brewski

Homebrewing recipe manager — Tauri 2 desktop app with a SvelteKit frontend and SQLite backend.

## Tech stack

- **Frontend:** SvelteKit + Svelte 5 + TypeScript + TailwindCSS 4
- **Backend:** Rust (Tauri 2 command handlers), SQLite via sqlx
- **IPC:** Tauri `invoke()` — see `src/lib/api.ts` for all typed wrappers

## Package manager

Use **bun** — not npm or npx.

```bash
bun install        # install deps
bun run dev        # frontend only
bunx some-cli      # run a package binary
```

## Common commands (via Justfile)

```bash
just dev           # Tauri dev server (frontend + backend)
just dev-web       # frontend only
just build         # release build
just check         # TypeScript + OpenAPI lint
just lint-openapi  # Redocly lint of docs/openapi/openapi.yaml
just preview-docs  # render API docs in browser
```

## API spec

The Tauri IPC interface is documented as an OpenAPI 3.1 spec at `docs/openapi/openapi.yaml`.
Lint it with `just lint-openapi` before committing changes to it.

## Style guide

### Variable naming

Use full words for local variables — avoid abbreviations except where noted below.

**Always expand:**
| Avoid | Use instead |
|-------|-------------|
| `vol_gal`, `batch_gal` | `volume_gallons`, `batch_gallons` |
| `lbs`, `oz` | `pounds`, `ounces` |
| `boil_hrs` | `boil_hours` |
| `evap_fraction` | `evaporation_fraction` |
| `mcu` | `malt_color_units` |
| `ppg` | `points_per_pound_per_gallon` |
| `eff` | `efficiency` |
| `att` | `attenuation` |
| `gu` | `gravity_units` |
| `eq` (for equipment) | `equipment` |
| `fname`, `hname` | `fermentable_name`, `hop_name` |
| `abw` | `alcohol_by_weight` |
| `re` (real extract) | `real_extract` |
| `sg` | `specific_gravity` |
| `cal_per_ml` | `calories_per_ml` |

**Keep as-is (universal brewing acronyms and unit suffixes):**
- `og`, `fg`, `ibu`, `srm`, `abv` — standard brewing acronyms, expanding them hurts readability
- `_pct`, `_l`, `_kg`, `_c`, `_min`, `_ppm` — unit suffixes on struct fields that mirror the DB schema

### Domain terminology

**Use `addition`, not `ingredient`**, for recipe line items (e.g. `RecipeAdditionHop`, `recipe_addition_hops`).

In brewing, an *addition* captures a timed event — a hop addition at 60 minutes, a dry-hop addition at day 3 — not just a static ingredient list. The term encodes the temporal/process context that distinguishes a recipe's use of an ingredient from the ingredient itself in the library.
