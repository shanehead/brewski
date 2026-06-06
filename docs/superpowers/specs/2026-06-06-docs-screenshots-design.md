# Design: Docs Site Screenshots

**Date:** 2026-06-06
**Scope:** Add screenshots to the getting-started docs pages; establish a repeatable capture workflow as the app evolves.

---

## Goal

Embed real app screenshots into the getting-started section of the docs site so new users can orient themselves visually before and during their first use. Provide a single command to regenerate all screenshots when the UI changes.

---

## Approach

Extend the existing Playwright capture script (`scripts/capture-screenshots.mjs`) to also output docs-specific screenshots. One script, one `just screenshots` command, two output directories. Docs shots are desktop-only (1280×800) since the docs site is read on desktop.

This matches the existing README screenshot pattern and keeps the toolchain minimal.

---

## File Locations

| Asset | Path |
|---|---|
| Docs screenshots | `docs/site/public/screenshots/` |
| README screenshots | `docs/screenshots/` (unchanged) |
| Capture script | `scripts/capture-screenshots.mjs` (extended) |

VitePress serves `docs/site/public/` as static assets. Images reference as `/screenshots/filename.png` in markdown — no path prefix gymnastics needed.

---

## Naming Convention

Docs screenshots use the prefix `docs-gs-` (getting-started):

```
docs-gs-overview.png
docs-gs-recipe-new.png
docs-gs-recipe-ingredients.png
docs-gs-recipe-mash.png
docs-gs-batch-overview.png
docs-gs-batch-gravity.png
docs-gs-batch-carbonation.png
docs-gs-import.png
```

README screenshots keep their existing names in `docs/screenshots/`.

---

## Screenshots Per Page

### `what-is-brewski.md` — 1 shot

| File | What it shows | Placement |
|---|---|---|
| `docs-gs-overview.png` | Recipe list — desktop view of the main app screen | Below the opening paragraph |

### `installation.md` — 0 shots

Install steps involve native OS dialogs that cannot be mocked. Text is clear enough without screenshots.

### `first-recipe.md` — 3 shots

| File | What it shows | Placement |
|---|---|---|
| `docs-gs-recipe-new.png` | Recipe list sidebar with the **New Recipe** button visible | After Step 1 |
| `docs-gs-recipe-ingredients.png` | Ingredients tab with fermentables, hops, and yeast filled in; live stats sidebar showing OG/IBU/SRM | After Step 2 |
| `docs-gs-recipe-mash.png` | Mash tab with a single-infusion step configured | After Step 3 |

### `first-batch.md` — 3 shots

| File | What it shows | Placement |
|---|---|---|
| `docs-gs-batch-overview.png` | Batches tab on a recipe showing the **Brew this Recipe** button | After "Starting a batch" |
| `docs-gs-batch-gravity.png` | Gravity Log tab with a few readings entered | After "Fermentation and gravity readings" |
| `docs-gs-batch-carbonation.png` | Carbonation section with priming sugar calculator filled in | After "Packaging and carbonation" |

### `importing.md` — 1 shot

| File | What it shows | Placement |
|---|---|---|
| `docs-gs-import.png` | Recipe list sidebar showing the **Import BeerXML** button | After "How to import" |

---

## Capture Script Changes

Add a `// ── Docs: Getting Started (1280×800) ──` section to `scripts/capture-screenshots.mjs` that:

1. Uses the same `TAURI_MOCK` already defined in the script (no new mock data needed for most shots; batch and gravity shots need batch/reading mock data added)
2. Outputs to `path.join(ROOT, "docs", "site", "public", "screenshots")`
3. Uses `interact` callbacks where needed to navigate to tabs (Ingredients, Mash, Batches, Gravity Log)

New mock commands to add to `TAURI_MOCK`:
- `list_batches` — return one batch with brew day measurements filled in
- `list_gravity_readings` — return 3-4 readings spanning typical fermentation
- `get_recipe` — return a recipe with all fields populated (needed for recipe editor views)

---

## Justfile

Add under the `# ── Dev ──` section:

```just
# Capture README and docs screenshots (requires: just dev-web running on :1420)
screenshots:
    bun scripts/capture-screenshots.mjs
```

---

## AGENTS.md Update

Expand the existing "README screenshots" section to cover both output directories:

- Clarify that `just screenshots` captures both README (`docs/screenshots/`) and docs (`docs/site/public/screenshots/`) shots
- List the docs-gs-* filenames and which page each belongs to
- Keep the existing regeneration instructions (start `just dev-web`, run `just screenshots`, commit)

---

## Update Workflow

When a UI change affects a captured screen:

1. Start `just dev-web`
2. Run `just screenshots`
3. Review the updated PNGs
4. Commit the changed images with the UI change

No automation. Screenshots are committed assets — they update when you update the UI, same as the README shots today.

---

## Out of Scope

- Guide, reference, and concept pages (added later as separate work)
- Mobile screenshots for docs (docs site is desktop-read)
- CI auto-capture (can be added later if the manual workflow proves too easy to forget)
- Screenshot diffing or visual regression testing
