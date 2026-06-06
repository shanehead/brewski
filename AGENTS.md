# Brewski

Homebrewing recipe manager — Tauri 2 app targeting **desktop, iOS, and Android**, with a SvelteKit frontend and SQLite backend.

## Tech stack

- **Frontend:** SvelteKit + Svelte 5 + TypeScript + TailwindCSS 4
- **Backend:** Rust (Tauri 2 command handlers), SQLite via SeaORM
- **IPC:** Tauri `invoke()` — types in `src/lib/api.gen.ts` (generated — do not edit by hand)
- **Platforms:** macOS/Windows/Linux (desktop), iOS, Android

## Package manager

Use **bun** — not npm or npx.

```bash
bun install        # install deps
bun run dev        # frontend only
bunx some-cli      # run a package binary
```

## Platform support

Brewski runs on **desktop (macOS/Windows/Linux), iOS, and Android**. Every feature must work on all three platforms.

### Platform-specific UI

The frontend uses a build-time `$platform` alias (resolved in `vite.config.ts` and `svelte.config.js` via `TAURI_ENV_PLATFORM`) to swap between platform-specific implementations:

- `src/lib/desktop/` — desktop components (icon rail nav, multi-column layouts)
- `src/lib/mobile/` — mobile components (bottom tab bar, single-column scroll)

Route pages are thin wrappers that import from `$platform`. When adding a new page or major UI section, **create implementations in both `src/lib/desktop/` and `src/lib/mobile/`**.

**Any UI design — including mockups, plans, and implementation — must cover both desktop and mobile.** Never design or propose UI for one platform without addressing the other.

Mobile layout rules:
- Use `height: 100dvh` (not `100vh`) to handle iOS URL bar resizing
- Apply `env(safe-area-inset-top/bottom, 0px)` padding for notch/home-indicator clearance
- Touch targets must be at least 44px tall
- No horizontal overflow — layouts must fit in a single column

### Dev commands

```bash
just dev            # desktop (Tauri)
just dev-ios        # iOS simulator
just dev-android    # Android emulator
just dev-web        # frontend only (no Tauri)
```

## Screenshots

Screenshots are committed PNG assets captured with Playwright against the `just dev-web` frontend server (port 1420). There are two sets:

| Set | Output directory | Used in |
|---|---|---|
| README | `docs/screenshots/` | `README.md` |
| Docs (getting-started) | `docs/site/public/screenshots/` | `docs/site/getting-started/*.md` |

**To regenerate:**
1. Start `just dev-web`
2. Run `just screenshots`
3. Review the updated PNGs
4. Commit the changed images alongside the UI change

**Update when:** a UI change affects any captured screen. Check the table below to know which pages are affected.

### Docs screenshot inventory

| File | Page | What it shows |
|---|---|---|
| `docs-gs-overview.png` | `what-is-brewski.md` | Recipe list — main app shell |
| `docs-gs-recipe-new.png` | `first-recipe.md` | Recipe list sidebar with New Recipe button |
| `docs-gs-recipe-ingredients.png` | `first-recipe.md` | Ingredients tab, stats sidebar |
| `docs-gs-recipe-mash.png` | `first-recipe.md` | Mash tab with infusion step |
| `docs-gs-batch-overview.png` | `first-batch.md` | Batches tab, Brew this Recipe button |
| `docs-gs-batch-gravity.png` | `first-batch.md` | Gravity log with fermentation readings |
| `docs-gs-batch-carbonation.png` | `first-batch.md` | Carbonation section, priming sugar |
| `docs-gs-import.png` | `importing.md` | Recipe list with Import BeerXML button |

### README screenshot inventory

Screens currently captured: `recipes`, `tools`, `tools-abv`, `tools-carbonation`, `library`, `tools-mobile`, `tools-abv-mobile`, `recipes-mobile`.

To regenerate: start `just dev-web`, then run `just screenshots`. The script writes PNGs to `docs/screenshots/`. Requires `playwright` as a dev dependency (`bun add -d playwright`) and `bunx playwright install chromium` on first run.

## Common commands (via Justfile)

```bash
just dev           # Tauri dev server (frontend + backend)
just dev-web       # frontend only
just build         # release build
just check         # TypeScript + OpenAPI lint
just test          # run all tests (Rust + frontend)
just lint-openapi  # Redocly lint of docs/openapi/openapi.yaml
just preview-docs  # render API docs in browser
```

## Schema-first development

Types flow from the OpenAPI spec — never edit generated files by hand.

```
docs/openapi/openapi.yaml   ← source of truth
       ↓
just gen-ts   →  src/lib/api.gen.ts          (TypeScript types + fetch wrappers)
just gen-rust →  src-tauri/src/models.gen.rs (Rust structs via cargo-typify)
```

**Workflow for adding or changing a type:**

1. Edit the OpenAPI spec under `docs/openapi/`
2. Run `just lint-openapi` — fix any spec errors
3. Run `just gen` — regenerates both `api.gen.ts` and `models.gen.rs`
4. Implement the Tauri command in Rust using the generated model types
5. Run `just check` and `just test` before committing

## Database (SeaORM)

Migrations live in `src-tauri/src/migration/`. Entities are generated from the DB schema and committed.

```bash
just migrate       # apply migrations to dev.db
just gen-entities  # regenerate SeaORM entities from dev.db (runs migrate first)
```

Entities in `src-tauri/src/entities/` are generated by `sea-orm-cli` — do not edit by hand.

## API spec

The Tauri IPC interface is documented as an OpenAPI 3.1 spec under `docs/openapi/` (split across multiple files, bundled at lint/gen time).

Lint with `just lint-openapi` before committing changes.

## Quality gates — required before every commit

Run both of these and ensure they pass (zero errors) before committing:

```bash
just check   # TypeScript type-check + OpenAPI lint
just test    # Rust + frontend unit tests
```

Do not commit with failing type errors, test failures, or OpenAPI lint errors. Warnings are acceptable; errors are not.

## Pre-push checklist

Before pushing, review the changes and ask whether they affect user-facing content. Not every push requires both updates — use judgment.

**1. README**

Does this change affect anything shown in `README.md` — feature descriptions, setup steps, or screenshots? If so, update the copy and/or regenerate affected screenshots (see [README screenshots](#readme-screenshots) above).

**2. Docs site (GitHub Pages)**

Does this change affect anything covered in the docs under `docs/site/`? If so, update the relevant pages. Screenshots in the docs follow the same regeneration process as README screenshots.

## Documentation writing style

Docs under `docs/site/` are drafted by AI and reviewed/edited by the project author. Write for a human voice the author can ship with minimal edits.

### Do

- Write directly to the brewer: use "you" and "your"
- Use contractions freely: "you're", "it's", "don't", "here's"
- Bold UI element names: **Add Hop**, **Save**, **Mash** tab
- Use `→` for navigation paths: Settings → Units → Gravity Unit
- Explain the *why*, not just the *how* — "Alpha % is how bitter your hops are, so don't leave it blank"
- Use analogies where they help — brewing is physical, keep explanations grounded
- Keep sentences short. If a sentence needs a comma and a semicolon, split it into two.

### Don't

- No em-dashes (—). Use a comma, a period, or a new sentence instead.
- No "It is important to note that…"
- No "Navigate to the X section in order to…"
- No "This feature allows users to…"
- No passive voice where active works
- No excessive hedging ("you may want to consider")
- No marketing language: "powerful", "seamless", "intuitive", "robust", "easy". Show what the feature does; let the reader decide.
- Don't write to convince — write to inform. The reader already installed the app; they don't need a sales pitch.
- No negation lists ("no X, no Y, no Z"). They read as defensive. State what's true positively instead.

### Tone target

The docs should read like a knowledgeable friend walking you through the app, not a technical manual or a product brochure. If a sentence sounds like it belongs on a landing page, rewrite it.

## Git commits

Use [Conventional Commits](https://www.conventionalcommits.org/) style: `type(scope): description`.

Common types: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`.

## Design

Brand guidelines, color tokens, iconography rules, and a clickable UI kit
live in [`design/`](./design/). When working on user-facing UI, follow
[`design/SKILL.md`](./design/SKILL.md).

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

## Rust guidelines

When making Rust design decisions — API shape, error handling, naming, trait design, unsafe code, FFI, or library ergonomics — fetch and consult the Microsoft Pragmatic Rust Guidelines before proposing or implementing:

**URL:** https://microsoft.github.io/rust-guidelines/agents/all.txt

The document is formatted for agent consumption (structured IDs, `<why>` tags, runnable code examples). Fetch it on demand rather than loading it into every session.
