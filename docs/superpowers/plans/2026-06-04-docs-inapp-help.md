# In-App Help System Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a lightweight in-app help layer: a reusable `Tooltip` component for technical fields, a `show_tooltips` setting, and section help-link icons that open the relevant docs page.

**Architecture:** A single `Tooltip.svelte` component renders a `?` icon and tooltip text inline. Visibility is controlled globally by `$settings.show_tooltips`. Each major tab/section gets a small `↗` docs link in its header. The docs base URL is a single constant so it's easy to update.

**Tech Stack:** Svelte 5 runes, Tailwind CSS v4, existing settings store pattern (`saveSetting` / `AppSettings`), `@tauri-apps/plugin-opener` for opening docs URLs in the system browser.

**Run after:** `2026-06-04-docs-site.md` — the docs site must be deployed before the help links point anywhere real. During development, use a `TODO` base URL.

---

## File Map

```
src/lib/
  components/
    Tooltip.svelte          ← NEW: ? icon + tooltip text, respects show_tooltips
    DocLink.svelte          ← NEW: ↗ link that opens a docs URL in system browser
  stores/
    settings.ts             ← MODIFY: add show_tooltips to AppSettings
  docs-urls.ts             ← NEW: central constant for docs base URL + per-section paths
src/routes/
  settings/
    +page.svelte            ← MODIFY: add Show tooltips toggle under a Help section
tests/
  Tooltip.test.ts           ← NEW
  DocLink.test.ts           ← NEW
```

---

## Task 1: Add `show_tooltips` to the settings store and settings page

**Files:**
- Modify: `src/lib/stores/settings.ts`
- Modify: `src/routes/settings/+page.svelte`
- Test: `tests/Tooltip.test.ts` (written in Task 2 — this task has no unit tests, verify by running the app)

- [ ] **Step 1: Add `show_tooltips` to AppSettings**

In `src/lib/stores/settings.ts`, add to the `AppSettings` interface:

```typescript
export interface AppSettings {
  units?: "metric" | "imperial";
  gravity_unit?: GravityUnit;
  theme?: string;
  default_equipment_profile_id?: string;
  last_route?: string;
  starters_collapsed?: boolean;
  show_tooltips?: boolean;   // ← add this line
}
```

No DB migration needed — settings are stored as key-value pairs and new keys are read as `undefined` (treated as the default).

- [ ] **Step 2: Add the Show tooltips toggle to the settings page**

In `src/routes/settings/+page.svelte`, add a `handleTooltipsChange` function in the `<script>` block alongside the other handlers:

```typescript
async function handleTooltipsChange(e: Event) {
  await ipc(saveSetting("show_tooltips", String((e.target as HTMLInputElement).checked)));
}
```

Then add a new section at the bottom of the settings form (before the closing `</div>` of the outer flex container):

```html
<!-- Help -->
<section class="flex flex-col gap-3">
  <h2 class="text-sm font-semibold" style="color: var(--color-text-secondary);">Help</h2>
  <div class="flex items-center justify-between">
    <label for="toggle-tooltips" class="text-sm" style="color: var(--color-text-primary);">
      Show tooltips
    </label>
    <input
      id="toggle-tooltips"
      type="checkbox"
      checked={$settings.show_tooltips ?? true}
      onchange={handleTooltipsChange}
      class="w-4 h-4 accent-[var(--color-accent)]"
    />
  </div>
  <p class="text-xs" style="color: var(--color-text-muted);">
    Turn off once you know your way around.
  </p>
</section>
```

- [ ] **Step 3: Verify the toggle appears and persists**

Run the app with `bun run tauri dev`. Go to Settings. Scroll to the bottom — you should see a "Help" section with a "Show tooltips" checkbox. Toggle it on and off, quit and restart, and verify the setting persists.

- [ ] **Step 4: Commit**

```bash
git add src/lib/stores/settings.ts src/routes/settings/+page.svelte
git commit -m "feat: add show_tooltips setting with toggle in Settings page"
```

---

## Task 2: Create the `Tooltip` component

**Files:**
- Create: `src/lib/components/Tooltip.svelte`
- Create: `tests/Tooltip.test.ts`

- [ ] **Step 1: Write the failing test**

Create `tests/Tooltip.test.ts`:

```typescript
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import { tick } from "svelte";
import Tooltip from "$lib/components/Tooltip.svelte";

vi.mock("$lib/stores/settings", () => ({
  settings: {
    subscribe: vi.fn((fn) => {
      fn({ show_tooltips: true });
      return () => {};
    }),
  },
}));

describe("Tooltip", () => {
  it("renders the ? icon", () => {
    render(Tooltip, { text: "Alpha % is the bitterness potential of the hop." });
    expect(screen.getByRole("button", { name: /\?/ })).toBeInTheDocument();
  });

  it("does not show tooltip text by default", () => {
    render(Tooltip, { text: "Alpha % is the bitterness potential of the hop." });
    expect(screen.queryByText("Alpha % is the bitterness potential of the hop.")).not.toBeInTheDocument();
  });

  it("shows tooltip text after clicking the ? icon", async () => {
    const user = userEvent.setup();
    render(Tooltip, { text: "Alpha % is the bitterness potential of the hop." });
    await user.click(screen.getByRole("button", { name: /\?/ }));
    await tick();
    expect(screen.getByText("Alpha % is the bitterness potential of the hop.")).toBeInTheDocument();
  });

  it("hides after clicking again", async () => {
    const user = userEvent.setup();
    render(Tooltip, { text: "Some tooltip text." });
    await user.click(screen.getByRole("button", { name: /\?/ }));
    await tick();
    await user.click(screen.getByRole("button", { name: /\?/ }));
    await tick();
    expect(screen.queryByText("Some tooltip text.")).not.toBeInTheDocument();
  });

  it("does not render the ? icon when show_tooltips is false", () => {
    vi.mocked((await import("$lib/stores/settings")).settings.subscribe).mockImplementation((fn) => {
      fn({ show_tooltips: false });
      return () => {};
    });
    render(Tooltip, { text: "Should be hidden." });
    expect(screen.queryByRole("button", { name: /\?/ })).not.toBeInTheDocument();
  });
});
```

- [ ] **Step 2: Run the test to confirm it fails**

```bash
bun run test tests/Tooltip.test.ts
```

Expected: FAIL — `Cannot find module '$lib/components/Tooltip.svelte'`

- [ ] **Step 3: Create `Tooltip.svelte`**

Create `src/lib/components/Tooltip.svelte`:

```svelte
<script lang="ts">
  import { settings } from "$lib/stores/settings";

  let { text }: { text: string } = $props();

  let open = $state(false);
  const showTooltips = $derived($settings.show_tooltips ?? true);

  function toggle() {
    open = !open;
  }
</script>

{#if showTooltips}
  <span class="relative inline-flex items-center">
    <button
      type="button"
      onclick={toggle}
      aria-label="?"
      class="inline-flex items-center justify-center w-4 h-4 rounded-full text-[10px] opacity-40 hover:opacity-80 transition-opacity cursor-pointer flex-shrink-0"
      style="border: 1px solid var(--color-border); color: var(--color-text-muted);"
    >?</button>

    {#if open}
      <div
        role="tooltip"
        class="absolute left-6 top-0 z-50 w-56 rounded-md px-3 py-2 text-xs leading-relaxed shadow-lg"
        style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);"
      >
        {text}
        <button
          type="button"
          onclick={toggle}
          class="block mt-1 text-[10px] opacity-40 hover:opacity-70"
          style="color: var(--color-text-muted);"
        >tap to dismiss</button>
      </div>
    {/if}
  </span>
{/if}
```

- [ ] **Step 4: Run the tests to confirm they pass**

```bash
bun run test tests/Tooltip.test.ts
```

Expected: all 5 tests pass.

- [ ] **Step 5: Commit**

```bash
git add src/lib/components/Tooltip.svelte tests/Tooltip.test.ts
git commit -m "feat: add Tooltip component with show_tooltips setting support"
```

---

## Task 3: Create the `DocLink` component and docs URL registry

**Files:**
- Create: `src/lib/docs-urls.ts`
- Create: `src/lib/components/DocLink.svelte`
- Create: `tests/DocLink.test.ts`

- [ ] **Step 1: Write the failing test**

Create `tests/DocLink.test.ts`:

```typescript
import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import DocLink from "$lib/components/DocLink.svelte";

vi.mock("@tauri-apps/plugin-opener", () => ({ openUrl: vi.fn() }));

describe("DocLink", () => {
  it("renders the link with the correct label", () => {
    render(DocLink, { label: "Hops guide", url: "https://example.com/hops" });
    expect(screen.getByText("Hops guide ↗")).toBeInTheDocument();
  });

  it("calls openUrl with the correct URL on click", async () => {
    const { openUrl } = await import("@tauri-apps/plugin-opener");
    const user = userEvent.setup();
    render(DocLink, { label: "Hops guide", url: "https://example.com/hops" });
    await user.click(screen.getByText("Hops guide ↗"));
    expect(openUrl).toHaveBeenCalledWith("https://example.com/hops");
  });
});
```

- [ ] **Step 2: Run the test to confirm it fails**

```bash
bun run test tests/DocLink.test.ts
```

Expected: FAIL — `Cannot find module '$lib/components/DocLink.svelte'`

- [ ] **Step 3: Create `docs-urls.ts`**

Create `src/lib/docs-urls.ts`:

```typescript
const BASE = "https://shanehead.github.io/brewski";

export const DOCS = {
  gettingStarted: `${BASE}/getting-started/what-is-brewski`,
  firstRecipe: `${BASE}/getting-started/first-recipe`,
  firstBatch: `${BASE}/getting-started/first-batch`,
  fermentables: `${BASE}/guides/fermentables`,
  hops: `${BASE}/guides/hops`,
  yeast: `${BASE}/guides/yeast`,
  mash: `${BASE}/guides/mash`,
  waterChemistry: `${BASE}/guides/water-chemistry`,
  scaling: `${BASE}/guides/scaling`,
  recipeVersions: `${BASE}/guides/recipe-versions`,
  brewDay: `${BASE}/guides/brew-day`,
  gravityTracking: `${BASE}/guides/gravity-tracking`,
  carbonation: `${BASE}/guides/carbonation`,
  ingredientLibrary: `${BASE}/guides/ingredient-library`,
  cloudSync: `${BASE}/guides/cloud-sync`,
  equipmentProfiles: `${BASE}/reference/equipment-profiles`,
  settings: `${BASE}/reference/settings`,
  beerxml: `${BASE}/reference/beerxml`,
  calcAbv: `${BASE}/reference/calculators/abv-calories`,
  calcHydrometer: `${BASE}/reference/calculators/hydrometer-correction`,
  calcRefractometer: `${BASE}/reference/calculators/refractometer`,
  calcGravity: `${BASE}/reference/calculators/gravity-conversions`,
  calcColor: `${BASE}/reference/calculators/color-conversions`,
  calcPitchRate: `${BASE}/reference/calculators/pitch-rate`,
  calcCarbonation: `${BASE}/reference/calculators/carbonation`,
  calcUnits: `${BASE}/reference/calculators/unit-conversions`,
  conceptGravity: `${BASE}/concepts/gravity`,
  conceptIbu: `${BASE}/concepts/ibu`,
  conceptColor: `${BASE}/concepts/color`,
  conceptHopForms: `${BASE}/concepts/hop-forms`,
  conceptPitchRate: `${BASE}/concepts/pitch-rate-starters`,
  conceptWaterIons: `${BASE}/concepts/water-ions`,
  faq: `${BASE}/faq`,
} as const;
```

- [ ] **Step 4: Create `DocLink.svelte`**

Create `src/lib/components/DocLink.svelte`:

```svelte
<script lang="ts">
  import { openUrl } from "@tauri-apps/plugin-opener";

  let { label, url }: { label: string; url: string } = $props();

  function handleClick(e: MouseEvent) {
    e.preventDefault();
    openUrl(url);
  }
</script>

<button
  type="button"
  onclick={handleClick}
  class="text-xs opacity-40 hover:opacity-70 transition-opacity"
  style="color: var(--color-text-muted);"
>
  {label} ↗
</button>
```

- [ ] **Step 5: Run the tests to confirm they pass**

```bash
bun run test tests/DocLink.test.ts
```

Expected: both tests pass.

- [ ] **Step 6: Commit**

```bash
git add src/lib/docs-urls.ts src/lib/components/DocLink.svelte tests/DocLink.test.ts
git commit -m "feat: add DocLink component and docs URL registry"
```

---

## Task 4: Wire tooltips into the recipe editor tabs

**Files:**
- Modify: `src/lib/components/ingredients/FermentablesTable.svelte`
- Modify: `src/lib/components/ingredients/HopsTable.svelte`
- Modify: `src/lib/components/ingredients/YeastsTable.svelte`
- Modify: `src/lib/components/tabs/MashTab.svelte`
- Modify: `src/lib/components/tabs/WaterTab.svelte`

For each tab, add `Tooltip` next to technical fields and a `DocLink` in the tab's section header. The pattern is the same in every tab — shown once fully here, then abbreviated.

- [ ] **Step 1: Add imports to each tab**

At the top of each tab's `<script lang="ts">` block, add:

```typescript
import Tooltip from "$lib/components/Tooltip.svelte";
import DocLink from "$lib/components/DocLink.svelte";
import { DOCS } from "$lib/docs-urls";
```

- [ ] **Step 2: Wire `FermentablesTable.svelte`**

Find the section header (the row showing "FERMENTABLES" label or equivalent, inside `FermentablesTable.svelte` at `src/lib/components/ingredients/`). Add a `DocLink` at the far right:

```svelte
<div class="flex items-center justify-between mb-2">
  <span class="text-xs opacity-40" style="color: var(--color-text-muted);">FERMENTABLES</span>
  <DocLink label="Fermentables guide" url={DOCS.fermentables} />
</div>
```

Find the **Yield %** label in the ingredient form. Add a `Tooltip` next to it:

```svelte
<span class="text-xs" style="color: var(--color-text-muted);">Yield %</span>
<Tooltip text="How much fermentable sugar this ingredient provides per unit weight. Base malts are typically 78–82%. Sugars are 100%." />
```

Find the **Color (Lovibond)** label. Add a `Tooltip`:

```svelte
<span class="text-xs" style="color: var(--color-text-muted);">Color (Lov)</span>
<Tooltip text="The color of this fermentable in degrees Lovibond. Pale malts are 1–3°L. Crystal malts range from 10 to 120°L+." />
```

- [ ] **Step 3: Wire `HopsTable.svelte`** (`src/lib/components/ingredients/HopsTable.svelte`)

Add `DocLink` to the section header:

```svelte
<div class="flex items-center justify-between mb-2">
  <span class="text-xs opacity-40" style="color: var(--color-text-muted);">HOPS</span>
  <DocLink label="Hops guide" url={DOCS.hops} />
</div>
```

Add `Tooltip` next to **Alpha %**:

```svelte
<span class="text-xs" style="color: var(--color-text-muted);">Alpha %</span>
<Tooltip text="The percentage of alpha acids — this drives bitterness. Higher alpha means fewer grams to hit your IBU target." />
```

Add `Tooltip` next to **Use** (use type selector):

```svelte
<span class="text-xs" style="color: var(--color-text-muted);">Use</span>
<Tooltip text="When and how the hop is added. Boil adds bitterness. Whirlpool and Hopstand add flavor and aroma. Dry Hop adds aroma only (zero IBU)." />
```

Add `Tooltip` next to **Hopstand Temp** (only visible when use type is Hopstand):

```svelte
<span class="text-xs" style="color: var(--color-text-muted);">Hopstand Temp (°C)</span>
<Tooltip text="The temperature of the wort during the hopstand. Lower temps extract more aroma and less bitterness. 80°C is a common starting point." />
```

- [ ] **Step 4: Wire `YeastsTable.svelte`** (`src/lib/components/ingredients/YeastsTable.svelte`)

Add `DocLink` to the section header:

```svelte
<div class="flex items-center justify-between mb-2">
  <span class="text-xs opacity-40" style="color: var(--color-text-muted);">YEAST</span>
  <DocLink label="Yeast guide" url={DOCS.yeast} />
</div>
```

Add `Tooltip` next to **Attenuation %**:

```svelte
<span class="text-xs" style="color: var(--color-text-muted);">Attenuation %</span>
<Tooltip text="How much of the fermentable sugar the yeast will eat. Higher attenuation means a drier beer and more alcohol. US-05 is typically around 81%." />
```

- [ ] **Step 5: Wire `MashTab.svelte`**

Add `DocLink` to the section header:

```svelte
<div class="flex items-center justify-between mb-2">
  <span class="text-xs opacity-40" style="color: var(--color-text-muted);">MASH</span>
  <DocLink label="Mash guide" url={DOCS.mash} />
</div>
```

Add `Tooltip` next to **Water:Grain Ratio**:

```svelte
<span class="text-xs" style="color: var(--color-text-muted);">Ratio (L/kg)</span>
<Tooltip text="How much strike water per kg of grain. Lower ratios (2.5 L/kg) give thicker mashes with better efficiency. Higher ratios (4+ L/kg) are easier to stir. BIAB often uses 5+ L/kg." />
```

Add `Tooltip` next to **Grain Temp**:

```svelte
<span class="text-xs" style="color: var(--color-text-muted);">Grain Temp (°C)</span>
<Tooltip text="The temperature of your grain before mashing. Affects the strike water temperature calculation. Room temp (20°C) is fine for most situations." />
```

- [ ] **Step 6: Wire `WaterTab.svelte`**

Add `DocLink` to the section header:

```svelte
<div class="flex items-center justify-between mb-2">
  <span class="text-xs opacity-40" style="color: var(--color-text-muted);">WATER</span>
  <DocLink label="Water chemistry guide" url={DOCS.waterChemistry} />
</div>
```

Add `Tooltip` next to each ion target field. Use these texts:

- **Ca (Calcium)**: `"Promotes enzyme activity and yeast health. Target 50–150 ppm for most styles."`
- **Mg (Magnesium)**: `"A yeast nutrient at low levels. Tastes harsh above 30 ppm — keep it under that."`
- **Na (Sodium)**: `"Adds roundness and fullness at under 150 ppm. Salty and harsh above it."`
- **Cl (Chloride)**: `"Accentuates malt character and body. A higher Cl:SO₄ ratio makes beer taste maltier."`
- **SO₄ (Sulfate)**: `"Accentuates hop dryness and bitterness. A higher SO₄:Cl ratio makes beer taste drier and more bitter."`
- **HCO₃ (Bicarbonate)**: `"Raises mash pH. Useful for dark beers that lower pH naturally. Keep it low for pale styles."`

- [ ] **Step 7: Run the full test suite to check nothing broke**

```bash
bun run test
```

Expected: all tests pass.

- [ ] **Step 8: Commit**

```bash
git add src/lib/components/tabs/
git commit -m "feat: wire tooltips and doc links into recipe editor tabs"
```

---

## Task 5: Wire tooltips into batch tabs

**Files:**
- Modify: `src/lib/components/batch/BatchGravityTab.svelte`
- Modify: `src/lib/components/batch/BatchCarbonationSection.svelte`
- Modify: `src/lib/components/batch/BatchOverviewTab.svelte`

- [ ] **Step 1: Add imports to each batch component**

In each file's `<script lang="ts">`:

```typescript
import Tooltip from "$lib/components/Tooltip.svelte";
import DocLink from "$lib/components/DocLink.svelte";
import { DOCS } from "$lib/docs-urls";
```

- [ ] **Step 2: Wire `BatchGravityTab.svelte`**

Add `DocLink` to the section header:

```svelte
<div class="flex items-center justify-between mb-2">
  <span class="text-xs opacity-40" style="color: var(--color-text-muted);">GRAVITY READINGS</span>
  <DocLink label="Gravity tracking guide" url={DOCS.gravityTracking} />
</div>
```

Add `Tooltip` next to the gravity input field label:

```svelte
<Tooltip text="Enter in your current gravity unit (set in Settings). If you're using a refractometer after fermentation has started, correct the reading first using Tools → Refractometer." />
```

- [ ] **Step 3: Wire `BatchCarbonationSection.svelte`**

Add `DocLink` to the section header:

```svelte
<div class="flex items-center justify-between mb-2">
  <span class="text-xs opacity-40" style="color: var(--color-text-muted);">CARBONATION</span>
  <DocLink label="Carbonation guide" url={DOCS.carbonation} />
</div>
```

Add `Tooltip` next to **Target Volumes**:

```svelte
<Tooltip text="Volumes of CO₂ dissolved in your beer. Lagers and British ales: 2.0–2.3. American ales: 2.3–2.6. Hefeweizens and Belgian styles: 3.0+." />
```

Add `Tooltip` next to **Fermentation Temp**:

```svelte
<Tooltip text="The temperature at the end of fermentation (not after cold crashing). This is used to estimate how much CO₂ is already dissolved in the beer." />
```

- [ ] **Step 4: Wire `BatchOverviewTab.svelte`**

Add `DocLink` to the section header:

```svelte
<DocLink label="Brew day guide" url={DOCS.brewDay} />
```

Add `Tooltip` next to **Pre-boil gravity**:

```svelte
<Tooltip text="The gravity of your wort before the boil. Compare to the planned pre-boil gravity in the stats — a lower reading means lower mash efficiency than expected." />
```

- [ ] **Step 5: Run the full test suite**

```bash
bun run test
```

Expected: all tests pass.

- [ ] **Step 6: Commit**

```bash
git add src/lib/components/batch/
git commit -m "feat: wire tooltips and doc links into batch tabs"
```

---

## Task 6: Wire tooltips into the Tools pages

**Files:**
- Modify: `src/routes/tools/abv-calories/+page.svelte`
- Modify: `src/routes/tools/hydrometer-temp/+page.svelte`
- Modify: `src/routes/tools/refractometer/+page.svelte`
- Modify: `src/routes/tools/pitch-rate/+page.svelte`
- Modify: `src/routes/tools/carbonation/+page.svelte`

- [ ] **Step 1: Add imports to each tools page**

In each file's `<script lang="ts">`:

```typescript
import Tooltip from "$lib/components/Tooltip.svelte";
import DocLink from "$lib/components/DocLink.svelte";
import { DOCS } from "$lib/docs-urls";
```

- [ ] **Step 2: Wire `abv-calories/+page.svelte`**

Add `DocLink` near the page title: `label="ABV & Calories reference"`, `url={DOCS.calcAbv}`.

Add `Tooltip` next to OG input: `text="Original gravity — the sugar content of your wort before fermentation. Enter as SG (e.g. 1.052), Plato, or Brix depending on your settings."`

Add `Tooltip` next to FG input: `text="Final gravity — the sugar remaining after fermentation is complete. Take this reading when gravity has been stable for 2–3 days."`

- [ ] **Step 3: Wire `hydrometer-temp/+page.svelte`**

Add `DocLink`: `label="Hydrometer correction reference"`, `url={DOCS.calcHydrometer}`.

Add `Tooltip` next to calibration temp: `text="The temperature your hydrometer is calibrated for. Most hydrometers are calibrated at 20°C (68°F). Check yours — it's usually printed on the label or included in the packaging."`

- [ ] **Step 4: Wire `refractometer/+page.svelte`**

Add `DocLink`: `label="Refractometer reference"`, `url={DOCS.calcRefractometer}`.

Add `Tooltip` next to WCF input: `text="Wort correction factor. Most wort reads slightly higher than pure sugar. The default (1.04) works for most beers. Check your refractometer's manual — some are pre-corrected and need 1.00."`

Add `Tooltip` next to the FG Brix input: `text="Your raw refractometer reading post-fermentation. Alcohol skews refractometer readings, so the corrected FG shown here is an estimate using the Novotný formula."`

- [ ] **Step 5: Wire `pitch-rate/+page.svelte`**

Add `DocLink`: `label="Pitch rate reference"`, `url={DOCS.calcPitchRate}`.

Add `Tooltip` next to pitch rate input: `text="Cells per mL per degree Plato. Use 0.75 for ales, 1.5 for lagers. Higher rates are more reliable for big beers."`

Add `Tooltip` next to cell count input: `text="How many billion cells are in your yeast pack. Most liquid yeast packs contain 100 billion cells when fresh. Check the manufacture date — viability drops over time."`

Add `Tooltip` next to viability input: `text="The percentage of cells in your pack that are still alive. A fresh pack is close to 100%. A 3-month-old pack might be 65%. Some labs publish viability curves on their websites."`

- [ ] **Step 6: Wire `carbonation/+page.svelte`**

Add `DocLink`: `label="Carbonation reference"`, `url={DOCS.calcCarbonation}`.

Add `Tooltip` next to target volumes: `text="How much CO₂ you want dissolved in the finished beer. British ales: 2.0–2.2. American ales: 2.4–2.6. Hefeweizens: 3.0–3.6."`

- [ ] **Step 7: Run full test suite**

```bash
bun run test
```

Expected: all tests pass.

- [ ] **Step 8: Commit**

```bash
git add src/routes/tools/
git commit -m "feat: wire tooltips and doc links into Tools pages"
```

---

## Task 7: Verify tooltip behaviour on mobile layout

This task is manual — no unit tests cover the mobile layout directly.

- [ ] **Step 1: Run the app in mobile layout**

```bash
bun run tauri dev
```

Open the recipe editor. In the browser DevTools (if using webview), set viewport to 375px width, or resize the app window to simulate a narrow layout.

- [ ] **Step 2: Verify tooltip renders inline, not off-screen**

On the Hops tab, tap the `?` next to "Alpha %". Verify the tooltip appears below or beside the icon without being clipped by the container edge. The tooltip `div` should be readable without scrolling.

If any tooltip clips off-screen: in `Tooltip.svelte`, change `left-6` to `right-0` on the tooltip div's class, or adjust positioning to keep it in view.

- [ ] **Step 3: Verify the Settings toggle hides all tooltips**

In the running app, go to Settings → Help → Show tooltips → uncheck. Navigate back to the recipe editor. Verify all `?` icons are gone. Re-check the toggle. Verify they return.

- [ ] **Step 4: Verify DocLinks open the browser**

On the Hops tab, tap the "Hops guide ↗" link. Verify the system browser opens to the correct docs URL.

- [ ] **Step 5: Commit any layout fixes**

```bash
git add -A
git commit -m "fix: adjust tooltip positioning for narrow/mobile layout"
```
