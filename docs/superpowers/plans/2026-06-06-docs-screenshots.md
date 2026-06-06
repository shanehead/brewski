# Docs Screenshots Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add 8 Playwright-captured screenshots to the getting-started docs pages and a `just screenshots` command to regenerate them.

**Architecture:** Extend `scripts/capture-screenshots.mjs` with a second output directory (`docs/site/public/screenshots/`) and 8 new captures. VitePress serves `public/` as static assets, so images reference as `/screenshots/filename.png` in markdown. Batch screenshots require two mock batch IDs: `b1` (status `fermenting`, with gravity readings) and `b2` (status `packaged`, with carbonation data).

**Tech Stack:** Playwright (already installed), Node.js ESM, SvelteKit routes, VitePress, Just

---

### Task 1: Scaffold output directory and update capture function

**Files:**
- Create: `docs/site/public/screenshots/.gitkeep`
- Modify: `scripts/capture-screenshots.mjs` (lines 19-21, 129-142)

- [ ] **Step 1: Create the output directory**

```bash
mkdir -p /Users/shead/Documents/code/brewski/docs/site/public/screenshots
touch /Users/shead/Documents/code/brewski/docs/site/public/screenshots/.gitkeep
```

- [ ] **Step 2: Add `DOCS_OUT` constant and update `capture()` to accept `outDir`**

Replace the `OUT` constant and `capture` function in `scripts/capture-screenshots.mjs`:

```js
// Replace these two lines:
const OUT = path.join(ROOT, "docs", "screenshots");
// with:
const OUT = path.join(ROOT, "docs", "screenshots");
const DOCS_OUT = path.join(ROOT, "docs", "site", "public", "screenshots");
```

Then update the `capture` function signature from:
```js
async function capture(width, height, url, name, interact) {
```
to:
```js
async function capture(width, height, url, name, interact, outDir = OUT) {
```

And update the screenshot line inside `capture` from:
```js
  await pg.screenshot({ path: path.join(OUT, `${name}.png`) });
```
to:
```js
  await pg.screenshot({ path: path.join(outDir, `${name}.png`) });
```

- [ ] **Step 3: Verify existing captures still work**

The existing calls use the default `outDir = OUT`, so they need no changes. Confirm the calls at the bottom of the file still look like:
```js
await capture(1280, 800, "/recipe/r1", "recipes");
await capture(1280, 800, "/tools", "tools");
// ...etc
```
No changes needed there.

- [ ] **Step 4: Commit**

```bash
git add docs/site/public/screenshots/.gitkeep scripts/capture-screenshots.mjs
git commit -m "chore: add docs screenshots output dir and outDir param to capture()"
```

---

### Task 2: Add batch and gravity mock data to TAURI_MOCK

**Files:**
- Modify: `scripts/capture-screenshots.mjs` (the `TAURI_MOCK` string, after the `list_water_library` block)

The batch screenshots need two mock scenarios: a fermenting batch (`b1`) with gravity readings, and a packaged batch (`b2`) with carbonation data. The mock checks `args?.id` to return the right one.

- [ ] **Step 1: Add mock commands inside the TAURI_MOCK string**

The existing TAURI_MOCK already has `if (cmd === 'list_batches_for_recipe') return [];` — **replace that line** with the new combined handler below, and add the remaining commands **immediately before** the `return null;` line:

```js
      if (cmd === 'list_batches' || cmd === 'list_batches_for_recipe') return [
        { id: 'b1', recipe_id: 'r1', recipe_name: 'Pliny the Elder Clone', recipe_version_id: 'v1', name: 'Batch 1', status: 'fermenting', brew_date: 1716000000000, actual_og: 1.071, actual_fg: null, rating: null, created_at: 1716000000000, updated_at: 1716100000000 },
        { id: 'b2', recipe_id: 'r1', recipe_name: 'Pliny the Elder Clone', recipe_version_id: 'v1', name: 'Batch 2', status: 'packaged', brew_date: 1715000000000, actual_og: 1.072, actual_fg: 1.010, rating: 8, created_at: 1715000000000, updated_at: 1715500000000 },
      ];
      if (cmd === 'get_batch') {
        const fermenting = {
          id: 'b1', recipe_id: 'r1', recipe_name: 'Pliny the Elder Clone',
          recipe_version_id: 'v1', name: 'Batch 1', status: 'fermenting',
          brew_date: 1716000000000, fermenter_date: 1716086400000,
          conditioning_date: null, packaging_date: null,
          actual_pre_boil_volume_l: 23.5, actual_post_boil_volume_l: 19.2,
          actual_batch_size_l: 18.9, actual_pre_boil_gravity: 1.063,
          actual_og: 1.071, actual_fg: null,
          notes: null, rating: null,
          planned_og: 1.072, planned_fg: 1.009,
          planned_pre_boil_gravity: 1.062, planned_post_boil_volume_l: 19,
          planned_batch_size_l: 19,
          packaging_temp_c: null, carbonation_sugar_type: null,
          priming_sugar_g: null, serving_pressure_kpa: null,
          gravity_readings: [
            { id: 'g1', batch_id: 'b1', recorded_at: 1716086400000, gravity: 1.071, temp_c: 20.0, notes: 'pitch day' },
            { id: 'g2', batch_id: 'b1', recorded_at: 1716259200000, gravity: 1.038, temp_c: 20.5, notes: null },
            { id: 'g3', batch_id: 'b1', recorded_at: 1716432000000, gravity: 1.015, temp_c: 20.0, notes: null },
            { id: 'g4', batch_id: 'b1', recorded_at: 1716604800000, gravity: 1.010, temp_c: 19.5, notes: 'stable' },
          ],
          created_at: 1716000000000, updated_at: 1716604800000,
        };
        const packaged = {
          id: 'b2', recipe_id: 'r1', recipe_name: 'Pliny the Elder Clone',
          recipe_version_id: 'v1', name: 'Batch 2', status: 'packaged',
          brew_date: 1715000000000, fermenter_date: 1715086400000,
          conditioning_date: 1715600000000, packaging_date: 1715700000000,
          actual_pre_boil_volume_l: 23.0, actual_post_boil_volume_l: 19.0,
          actual_batch_size_l: 18.9, actual_pre_boil_gravity: 1.062,
          actual_og: 1.072, actual_fg: 1.010,
          notes: 'Came out great. Dry hop character was spot on.',
          rating: 8,
          planned_og: 1.072, planned_fg: 1.009,
          planned_pre_boil_gravity: 1.062, planned_post_boil_volume_l: 19,
          planned_batch_size_l: 19,
          packaging_temp_c: 18.0, carbonation_sugar_type: 'corn_sugar',
          priming_sugar_g: 142, serving_pressure_kpa: null,
          gravity_readings: [
            { id: 'g5', batch_id: 'b2', recorded_at: 1715086400000, gravity: 1.072, temp_c: 20.0, notes: 'pitch day' },
            { id: 'g6', batch_id: 'b2', recorded_at: 1715259200000, gravity: 1.035, temp_c: 20.5, notes: null },
            { id: 'g7', batch_id: 'b2', recorded_at: 1715432000000, gravity: 1.012, temp_c: 20.0, notes: null },
            { id: 'g8', batch_id: 'b2', recorded_at: 1715604800000, gravity: 1.010, temp_c: 19.5, notes: 'stable, ready to package' },
          ],
          created_at: 1715000000000, updated_at: 1715700000000,
        };
        return args?.id === 'b2' ? packaged : fermenting;
      }
      if (cmd === 'list_batch_attachments') return [];
      if (cmd === 'list_recipe_versions') return [];
```

- [ ] **Step 2: Commit**

```bash
git add scripts/capture-screenshots.mjs
git commit -m "chore: add batch and gravity mock data to screenshot capture script"
```

---

### Task 3: Add docs-gs captures to the script

**Files:**
- Modify: `scripts/capture-screenshots.mjs` (after the existing mobile captures, before `browser.close()`)

The 8 captures are desktop-only (1280×800), output to `DOCS_OUT`. Tab navigation uses `getByRole('button', { name })`. Scrolling uses `scrollIntoViewIfNeeded()` on a visible text node.

- [ ] **Step 1: Add the docs-gs capture section**

Add the following block **immediately before** `await browser.close();`:

```js
// ── Docs: Getting Started (1280×800) ─────────────────────────────────────────
// docs-gs-overview: recipe list — shows the main app shell
await capture(1280, 800, "/", "docs-gs-overview", null, DOCS_OUT);

// docs-gs-recipe-new: recipe list — same view, used in first-recipe Step 1
await capture(1280, 800, "/", "docs-gs-recipe-new", null, DOCS_OUT);

// docs-gs-recipe-ingredients: recipe editor, Ingredients tab
await capture(1280, 800, "/recipe/r1", "docs-gs-recipe-ingredients", async (pg) => {
  await pg.getByRole("button", { name: "Ingredients" }).click();
  await pg.waitForTimeout(400);
}, DOCS_OUT);

// docs-gs-recipe-mash: recipe editor, Mash tab
await capture(1280, 800, "/recipe/r1", "docs-gs-recipe-mash", async (pg) => {
  await pg.getByRole("button", { name: "Mash" }).click();
  await pg.waitForTimeout(400);
}, DOCS_OUT);

// docs-gs-batch-overview: recipe editor, Batches tab (shows "Brew this Recipe")
await capture(1280, 800, "/recipe/r1", "docs-gs-batch-overview", async (pg) => {
  await pg.getByRole("button", { name: "Batches" }).click();
  await pg.waitForTimeout(400);
}, DOCS_OUT);

// docs-gs-batch-gravity: batch detail (fermenting, b1) — shows gravity log
await capture(1280, 800, "/batches/b1", "docs-gs-batch-gravity", async (pg) => {
  await pg.getByText("Gravity Log").scrollIntoViewIfNeeded();
  await pg.waitForTimeout(200);
}, DOCS_OUT);

// docs-gs-batch-carbonation: batch detail (packaged, b2) — shows carbonation section
await capture(1280, 800, "/batches/b2", "docs-gs-batch-carbonation", async (pg) => {
  await pg.getByText("Carbonation").first().scrollIntoViewIfNeeded();
  await pg.waitForTimeout(200);
}, DOCS_OUT);

// docs-gs-import: recipe list — shows Import BeerXML button in sidebar
await capture(1280, 800, "/", "docs-gs-import", null, DOCS_OUT);
```

Also update the final log line from:
```js
console.log(`\nScreenshots written to docs/screenshots/`);
```
to:
```js
console.log(`\nREADME screenshots → docs/screenshots/`);
console.log(`Docs screenshots   → docs/site/public/screenshots/`);
```

- [ ] **Step 2: Commit**

```bash
git add scripts/capture-screenshots.mjs
git commit -m "chore: add docs getting-started screenshot captures"
```

---

### Task 4: Add `just screenshots` to the Justfile

**Files:**
- Modify: `Justfile` (under the `# ── Dev ──` section)

- [ ] **Step 1: Add the command**

After the `dev-web` recipe and before the `# ── Build ──` section, add:

```just
# Capture README and docs screenshots (requires: just dev-web running on :1420)
screenshots:
    bun scripts/capture-screenshots.mjs
```

- [ ] **Step 2: Commit**

```bash
git add Justfile
git commit -m "chore: add just screenshots command"
```

---

### Task 5: Embed screenshots in what-is-brewski.md

**Files:**
- Modify: `docs/site/getting-started/what-is-brewski.md`

- [ ] **Step 1: Add screenshot after the opening paragraph**

The current opening is:
```markdown
Brewski is a free, open-source homebrewing app for Mac, Windows, Linux, iOS, and Android. Download it, open it, start brewing. That's the whole onboarding experience.
```

Add the screenshot immediately after that paragraph, before the `## What you can do with Brewski` heading:

```markdown
![Brewski recipe list](/screenshots/docs-gs-overview.png)
```

- [ ] **Step 2: Commit**

```bash
git add docs/site/getting-started/what-is-brewski.md
git commit -m "docs: add overview screenshot to what-is-brewski page"
```

---

### Task 6: Embed screenshots in first-recipe.md

**Files:**
- Modify: `docs/site/getting-started/first-recipe.md`

- [ ] **Step 1: Add screenshot after Step 1**

After the Step 1 paragraph ending with `...so you can see how close you are.`, add:

```markdown
![New recipe button in the sidebar](/screenshots/docs-gs-recipe-new.png)
```

- [ ] **Step 2: Add screenshot after Step 2**

After the Step 2 paragraph ending with `...Add as many hop additions as your recipe needs.`, add:

```markdown
![Ingredients tab with fermentables, hops, and yeast filled in](/screenshots/docs-gs-recipe-ingredients.png)
```

- [ ] **Step 3: Add screenshot after Step 3**

After the Step 3 paragraph ending with `...Watch the stats sidebar as you work. Your changes save automatically as you go.`, add:

```markdown
![Mash tab with a single-infusion step](/screenshots/docs-gs-recipe-mash.png)
```

- [ ] **Step 4: Commit**

```bash
git add docs/site/getting-started/first-recipe.md
git commit -m "docs: add step screenshots to first-recipe page"
```

---

### Task 7: Embed screenshots in first-batch.md

**Files:**
- Modify: `docs/site/getting-started/first-batch.md`

- [ ] **Step 1: Add screenshot after "Starting a batch"**

After the paragraph ending with `...your batch still shows exactly what you brewed.`, add:

```markdown
![Batches tab showing the Brew this Recipe button](/screenshots/docs-gs-batch-overview.png)
```

- [ ] **Step 2: Add screenshot after "Fermentation and gravity readings"**

After the paragraph ending with `...tells you when it's actually done rather than when you're just hoping it is.`, add:

```markdown
![Gravity log with readings plotted over fermentation](/screenshots/docs-gs-batch-gravity.png)
```

- [ ] **Step 3: Add screenshot after "Packaging and carbonation"**

After the paragraph ending with `...Enter your target volumes of CO2 and your current beer temperature, and Brewski does the rest.`, add:

```markdown
![Carbonation section with priming sugar calculator](/screenshots/docs-gs-batch-carbonation.png)
```

- [ ] **Step 4: Commit**

```bash
git add docs/site/getting-started/first-batch.md
git commit -m "docs: add step screenshots to first-batch page"
```

---

### Task 8: Embed screenshot in importing.md

**Files:**
- Modify: `docs/site/getting-started/importing.md`

- [ ] **Step 1: Add screenshot after "How to import"**

After the paragraph `In the recipe list sidebar, hit **Import BeerXML**. Pick your `.xml` file and Brewski will pull it in.`, add:

```markdown
![Import BeerXML button in the recipe list sidebar](/screenshots/docs-gs-import.png)
```

- [ ] **Step 2: Commit**

```bash
git add docs/site/getting-started/importing.md
git commit -m "docs: add import screenshot to importing page"
```

---

### Task 9: Update AGENTS.md

**Files:**
- Modify: `AGENTS.md`

- [ ] **Step 1: Locate the existing README screenshots section**

Find the section that starts with `## README screenshots` in `AGENTS.md`.

- [ ] **Step 2: Replace the section**

Replace the existing `## README screenshots` section with:

```markdown
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
```

- [ ] **Step 3: Commit**

```bash
git add AGENTS.md
git commit -m "docs: update AGENTS.md screenshot section to cover both README and docs sets"
```

---

### Task 10: Generate and commit the actual screenshots

This task runs the capture script to produce the 8 PNG files and commits them.

- [ ] **Step 1: Start the dev server in the background**

In a separate terminal (or background process):
```bash
just dev-web
```
Wait until you see `Local: http://localhost:1420/` in the output.

- [ ] **Step 2: Run the capture script**

```bash
just screenshots
```

Expected output:
```
✓ recipes
✓ tools
✓ tools-abv
✓ tools-carbonation
✓ library
✓ recipes-mobile
✓ tools-mobile
✓ tools-abv-mobile
✓ docs-gs-overview
✓ docs-gs-recipe-new
✓ docs-gs-recipe-ingredients
✓ docs-gs-recipe-mash
✓ docs-gs-batch-overview
✓ docs-gs-batch-gravity
✓ docs-gs-batch-carbonation
✓ docs-gs-import

README screenshots → docs/screenshots/
Docs screenshots   → docs/site/public/screenshots/
```

If any capture fails (timeout, selector not found), debug the interact callback for that capture. Common issues:
- Tab button not found: inspect the rendered HTML to find the exact button text
- Gravity/Carbonation text not found for scrollIntoViewIfNeeded: find the correct heading text in the component

- [ ] **Step 3: Review the generated PNGs**

Open each file in `docs/site/public/screenshots/` and confirm:
- `docs-gs-overview.png` — recipe list visible with sidebar
- `docs-gs-recipe-new.png` — recipe list, New Recipe button visible
- `docs-gs-recipe-ingredients.png` — Ingredients tab open, stats sidebar showing OG/IBU/SRM
- `docs-gs-recipe-mash.png` — Mash tab open with at least one step
- `docs-gs-batch-overview.png` — Batches tab open, "Brew this Recipe" button visible
- `docs-gs-batch-gravity.png` — Gravity log section visible with 4 readings
- `docs-gs-batch-carbonation.png` — Carbonation section visible
- `docs-gs-import.png` — Recipe list sidebar, Import BeerXML button visible

- [ ] **Step 4: Commit the generated images**

```bash
git add docs/site/public/screenshots/
git commit -m "docs: add generated getting-started screenshots"
```
