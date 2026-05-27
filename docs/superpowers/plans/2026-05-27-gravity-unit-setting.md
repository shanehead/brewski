# Gravity Unit Setting Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a gravity unit preference (SG / Plato / Brix) to app settings that defaults all gravity displays and inputs across recipes and batches to the chosen unit.

**Architecture:** All gravity values are stored and returned from the Rust backend as SG. The existing `convertGravity` IPC command converts between units; components call it for display (SG → preferred) and for input (preferred → SG before saving). Display strings are held in local `$state` populated by `$effect` that re-fires when the SG value or the `gravity_unit` setting changes. A small `gravity-display.ts` file centralises the pure UI formatting rules (toFixed precision + suffix string).

**Tech Stack:** Svelte 5 (runes), TypeScript, Vitest + @testing-library/svelte, Tauri IPC.

---

## File Map

| Action | Path | Responsibility |
|--------|------|----------------|
| Create | `src/lib/gravity-display.ts` | Format a `GravityConversionResult` to a display string; input step/placeholder helpers |
| Create | `tests/gravity-display.test.ts` | Unit tests for all formatting helpers |
| Create | `tests/StatsSidebar.test.ts` | Component test: OG/FG/pre-boil display in preferred unit |
| Create | `tests/BatchGravityTab.test.ts` | Component test: table display + input converts before save |
| Modify | `src/lib/stores/settings.ts` | Add `gravity_unit?: GravityUnit` to `AppSettings` |
| Modify | `src/routes/settings/+page.svelte` | Add gravity unit `<select>` in Units section |
| Modify | `src/lib/components/StatsSidebar.svelte` | Convert OG, FG, pre-boil gravity for display |
| Modify | `src/lib/components/batch/BatchGravityTab.svelte` | Convert table display; accept preferred-unit input |
| Modify | `src/lib/components/batch/BatchOverviewTab.svelte` | Convert stage-targets banner + measurements grid gravity fields |
| Modify | `src/lib/mobile/RecipeView.svelte` | Convert OG, FG display in stats card |
| Modify | `src/lib/mobile/BaselineRecipeView.svelte` | Same as RecipeView |

---

## Task 1: Create `gravity-display.ts` utility

**Files:**
- Create: `src/lib/gravity-display.ts`
- Test: `tests/gravity-display.test.ts`

- [ ] **Step 1: Write the failing tests**

```ts
// tests/gravity-display.test.ts
import { describe, it, expect } from "vitest";
import { formatGravity, gravityStep, gravityPlaceholder } from "$lib/gravity-display";

const result = { sg: 1.054, plato: 13.3, brix: 13.5 };

describe("formatGravity", () => {
  it("formats SG to 3 decimal places with no suffix", () => {
    expect(formatGravity(result, "sg")).toBe("1.054");
  });
  it("formats Plato to 1 decimal place with °P suffix", () => {
    expect(formatGravity(result, "plato")).toBe("13.3°P");
  });
  it("formats Brix to 1 decimal place with °Bx suffix", () => {
    expect(formatGravity(result, "brix")).toBe("13.5°Bx");
  });
});

describe("gravityStep", () => {
  it("returns 0.001 for sg", () => { expect(gravityStep("sg")).toBe("0.001"); });
  it("returns 0.1 for plato", () => { expect(gravityStep("plato")).toBe("0.1"); });
  it("returns 0.1 for brix", () => { expect(gravityStep("brix")).toBe("0.1"); });
});

describe("gravityPlaceholder", () => {
  it("returns SG example for sg", () => {
    expect(gravityPlaceholder("sg")).toBe("Gravity (e.g. 1.058)");
  });
  it("returns °P for plato", () => {
    expect(gravityPlaceholder("plato")).toBe("Gravity (°P)");
  });
  it("returns °Bx for brix", () => {
    expect(gravityPlaceholder("brix")).toBe("Gravity (°Bx)");
  });
});
```

- [ ] **Step 2: Run tests to confirm they fail**

```bash
npm test -- --reporter=verbose 2>&1 | grep -A3 "gravity-display"
```

Expected: FAIL — `Cannot find module '$lib/gravity-display'`

- [ ] **Step 3: Create the implementation**

```ts
// src/lib/gravity-display.ts
import type { GravityConversionResult, GravityUnit } from "$lib/api";

export function formatGravity(result: GravityConversionResult, unit: GravityUnit): string {
  switch (unit) {
    case "plato": return result.plato.toFixed(1) + "°P";
    case "brix":  return result.brix.toFixed(1) + "°Bx";
    default:      return result.sg.toFixed(3);
  }
}

export function gravityStep(unit: GravityUnit): string {
  return unit === "sg" ? "0.001" : "0.1";
}

export function gravityPlaceholder(unit: GravityUnit): string {
  switch (unit) {
    case "plato": return "Gravity (°P)";
    case "brix":  return "Gravity (°Bx)";
    default:      return "Gravity (e.g. 1.058)";
  }
}
```

- [ ] **Step 4: Run tests to confirm they pass**

```bash
npm test -- --reporter=verbose 2>&1 | grep -A5 "gravity-display"
```

Expected: all 7 tests PASS

- [ ] **Step 5: Commit**

```bash
git add src/lib/gravity-display.ts tests/gravity-display.test.ts
git commit -m "feat: add gravity-display formatting utility"
```

---

## Task 2: Add `gravity_unit` to settings store

**Files:**
- Modify: `src/lib/stores/settings.ts`

No separate test needed — this is a TypeScript type addition that TypeScript's compiler enforces.

- [ ] **Step 1: Add the field**

Open `src/lib/stores/settings.ts`. The file already imports from `$lib/api` — add `GravityUnit` to that existing import:

```ts
// Change this line:
import { getSettings, updateSetting } from "$lib/api";
// To:
import { getSettings, updateSetting } from "$lib/api";
import type { GravityUnit } from "$lib/api";
```

Then add `gravity_unit` to `AppSettings`:

```ts
export interface AppSettings {
  units?: "metric" | "imperial";
  gravity_unit?: GravityUnit;
  theme?: string;
  default_equipment_profile_id?: string;
  last_route?: string;
  starters_collapsed?: boolean;
}
```

- [ ] **Step 2: Type-check**

```bash
npm run check 2>&1 | tail -5
```

Expected: `0 errors` (or only pre-existing errors unrelated to this change)

- [ ] **Step 3: Commit**

```bash
git add src/lib/stores/settings.ts
git commit -m "feat: add gravity_unit field to AppSettings"
```

---

## Task 3: Add gravity unit dropdown to Settings page

**Files:**
- Modify: `src/routes/settings/+page.svelte`

- [ ] **Step 1: Add the handler and dropdown**

Open `src/routes/settings/+page.svelte`. Add `handleGravityUnitChange` in the `<script>` block after `handleUnitsChange`:

```ts
async function handleGravityUnitChange(e: Event) {
  await ipc(saveSetting("gravity_unit", (e.target as HTMLSelectElement).value));
}
```

In the Units `<section>`, add a new row below the Measurement System row:

```html
<div class="flex items-center justify-between">
  <label for="select-gravity-unit" class="text-sm" style="color: var(--color-text-primary);">Gravity Unit</label>
  <select id="select-gravity-unit" value={$settings.gravity_unit ?? "sg"} onchange={handleGravityUnitChange}
          class="px-2 py-1.5 rounded text-sm"
          style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);">
    <option value="sg">SG (1.050)</option>
    <option value="plato">Plato (°P)</option>
    <option value="brix">Brix (°Bx)</option>
  </select>
</div>
```

- [ ] **Step 2: Type-check**

```bash
npm run check 2>&1 | tail -5
```

Expected: `0 errors`

- [ ] **Step 3: Commit**

```bash
git add src/routes/settings/+page.svelte
git commit -m "feat: add gravity unit dropdown to settings"
```

---

## Task 4: Update StatsSidebar to display gravity in preferred unit

**Files:**
- Modify: `src/lib/components/StatsSidebar.svelte`
- Test: `tests/StatsSidebar.test.ts`

- [ ] **Step 1: Write the failing tests**

```ts
// tests/StatsSidebar.test.ts
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/svelte";
import { tick } from "svelte";
import StatsSidebar from "$lib/components/StatsSidebar.svelte";
import type { RecipeStats } from "$lib/api";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));

const mockInvoke = vi.mocked((await import("@tauri-apps/api/core")).invoke);

function makeSettings(gravityUnit: string = "sg") {
  return {
    subscribe: vi.fn((fn) => {
      fn({ theme: "midnight", units: "metric", gravity_unit: gravityUnit });
      return () => {};
    }),
  };
}

vi.mock("$lib/stores/settings", () => ({
  settings: makeSettings("sg"),
}));

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
  } as RecipeStats;
}

describe("StatsSidebar gravity display", () => {
  beforeEach(() => {
    mockInvoke.mockReset();
  });

  it("shows SG values when gravity_unit is sg", async () => {
    mockInvoke.mockResolvedValue({ sg: 1.054, plato: 13.3, brix: 13.5 });
    render(StatsSidebar, { stats: makeStats() });
    await tick();
    await new Promise(r => setTimeout(r, 0));
    await tick();
    expect(screen.getByText("1.054")).toBeInTheDocument();
  });

  it("shows Plato values when gravity_unit is plato", async () => {
    vi.doMock("$lib/stores/settings", () => ({ settings: makeSettings("plato") }));
    mockInvoke.mockResolvedValue({ sg: 1.054, plato: 13.3, brix: 13.5 });
    render(StatsSidebar, { stats: makeStats() });
    await tick();
    await new Promise(r => setTimeout(r, 0));
    await tick();
    expect(screen.getAllByText(/13\.3°P/).length).toBeGreaterThan(0);
  });

  it("shows — when stats is null", () => {
    render(StatsSidebar, { stats: null });
    expect(screen.getByText(/Add ingredients/)).toBeInTheDocument();
  });
});
```

- [ ] **Step 2: Run tests to confirm they fail**

```bash
npm test -- --reporter=verbose 2>&1 | grep -A3 "StatsSidebar"
```

Expected: FAIL — `StatsSidebar` renders but ignores gravity_unit (still shows SG regardless)

- [ ] **Step 3: Update StatsSidebar**

Replace the entire `<script>` block content in `src/lib/components/StatsSidebar.svelte`:

```ts
import type { RecipeStats } from "$lib/api";
import { convertGravity } from "$lib/api";
import { settings } from "$lib/stores/settings";
import { type Units, lToGal, volumeLabel } from "$lib/units";
import { srmToHex } from "$lib/utils/srm";
import { formatGravity } from "$lib/gravity-display";
import { ipc } from "$lib/stores/error";

let { stats }: { stats: RecipeStats | null } = $props();

const units = $derived<Units>($settings.units === "imperial" ? "imperial" : "metric");
const gravityUnit = $derived($settings.gravity_unit ?? "sg");

let displayOg = $state("—");
let displayFg = $state("—");
let displayPreBoil = $state("—");

$effect(() => {
  const unit = gravityUnit;
  if (stats?.og != null) {
    ipc(convertGravity(stats.og, "sg")).then(r => { if (r) displayOg = formatGravity(r, unit); });
  } else {
    displayOg = "—";
  }
});

$effect(() => {
  const unit = gravityUnit;
  if (stats?.fg != null) {
    ipc(convertGravity(stats.fg, "sg")).then(r => { if (r) displayFg = formatGravity(r, unit); });
  } else {
    displayFg = "—";
  }
});

$effect(() => {
  const unit = gravityUnit;
  if (stats?.pre_boil_gravity != null) {
    ipc(convertGravity(stats.pre_boil_gravity, "sg")).then(r => {
      if (r) displayPreBoil = formatGravity(r, unit);
    });
  } else {
    displayPreBoil = "—";
  }
});

function fmt(val: number | undefined | null, decimals = 3): string {
  if (val === undefined || val === null) return "—";
  return val.toFixed(decimals);
}

function pct(value: number, min: number, max: number): number {
  return Math.min(100, Math.max(0, ((value - min) / (max - min)) * 100));
}
```

Then in the template, replace the OG, FG, and Pre-boil G display values:

OG card — change `{fmt(stats.og, 3)}` to `{displayOg}`. Keep `pct(stats.og, 1.000, 1.120)` unchanged for the progress bar.

FG card — change `{fmt(stats.fg, 3)}` to `{displayFg}`. Keep `pct(stats.fg, 1.000, 1.030)` unchanged.

Pre-boil G card — change `{fmt(stats.pre_boil_gravity, 3)}` to `{displayPreBoil}`.

- [ ] **Step 4: Run tests**

```bash
npm test -- --reporter=verbose 2>&1 | grep -A5 "StatsSidebar"
```

Expected: all 3 tests PASS

- [ ] **Step 5: Type-check**

```bash
npm run check 2>&1 | tail -5
```

Expected: `0 errors`

- [ ] **Step 6: Commit**

```bash
git add src/lib/components/StatsSidebar.svelte tests/StatsSidebar.test.ts
git commit -m "feat: display gravity in preferred unit in StatsSidebar"
```

---

## Task 5: Update BatchGravityTab — display and input

**Files:**
- Modify: `src/lib/components/batch/BatchGravityTab.svelte`
- Test: `tests/BatchGravityTab.test.ts`

- [ ] **Step 1: Write the failing tests**

```ts
// tests/BatchGravityTab.test.ts
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/svelte";
import { tick } from "svelte";
import userEvent from "@testing-library/user-event";
import BatchGravityTab from "$lib/components/batch/BatchGravityTab.svelte";
import type { Batch } from "$lib/api";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("$lib/stores/error", () => ({ ipc: vi.fn((p) => p) }));
vi.mock("$lib/stores/settings", () => ({
  settings: {
    subscribe: vi.fn((fn) => {
      fn({ gravity_unit: "sg" });
      return () => {};
    }),
  },
}));

const mockInvoke = vi.mocked((await import("@tauri-apps/api/core")).invoke);

function makeBatch(overrides: Partial<Batch> = {}): Batch {
  return {
    id: "b1",
    recipe_id: "r1",
    recipe_name: "Test IPA",
    recipe_version_id: "v1",
    name: null,
    status: "fermenting",
    brew_date: null,
    fermenter_date: null,
    conditioning_date: null,
    packaging_date: null,
    actual_pre_boil_volume_l: null,
    actual_post_boil_volume_l: null,
    actual_batch_size_l: null,
    actual_pre_boil_gravity: null,
    actual_og: null,
    actual_fg: null,
    notes: null,
    rating: null,
    planned_og: null,
    planned_fg: null,
    planned_pre_boil_gravity: null,
    planned_post_boil_volume_l: null,
    planned_batch_size_l: null,
    packaging_temp_c: null,
    carbonation_sugar_type: null,
    priming_sugar_g: null,
    serving_pressure_kpa: null,
    gravity_readings: [],
    created_at: 0,
    updated_at: 0,
    ...overrides,
  } as unknown as Batch;
}

describe("BatchGravityTab", () => {
  beforeEach(() => { mockInvoke.mockReset(); });

  it("shows 'No readings yet' when batch has no readings", () => {
    render(BatchGravityTab, { batch: makeBatch(), onRefresh: vi.fn() });
    expect(screen.getByText(/No readings yet/)).toBeInTheDocument();
  });

  it("displays existing reading in preferred unit (plato)", async () => {
    // First call: convert_gravity for the displayed reading
    // Second call would be add_gravity_reading
    mockInvoke.mockResolvedValue({ sg: 1.050, plato: 12.4, brix: 12.6 });

    vi.doMock("$lib/stores/settings", () => ({
      settings: {
        subscribe: vi.fn((fn) => {
          fn({ gravity_unit: "plato" });
          return () => {};
        }),
      },
    }));

    const batch = makeBatch({
      gravity_readings: [{ id: "gr1", gravity: 1.050, temp_c: null, recorded_at: 1700000000, notes: null }] as any,
    });
    render(BatchGravityTab, { batch, onRefresh: vi.fn() });
    await tick();
    await new Promise(r => setTimeout(r, 0));
    await tick();
    expect(screen.getByText("12.4°P")).toBeInTheDocument();
  });

  it("converts input to SG before saving when unit is brix", async () => {
    // convert_gravity call for input conversion: { sg: 1.054, plato: 13.3, brix: 13.5 }
    // add_gravity_reading call
    mockInvoke
      .mockResolvedValueOnce({ sg: 1.054, plato: 13.3, brix: 13.5 })
      .mockResolvedValueOnce(undefined);

    vi.doMock("$lib/stores/settings", () => ({
      settings: {
        subscribe: vi.fn((fn) => {
          fn({ gravity_unit: "brix" });
          return () => {};
        }),
      },
    }));

    const onRefresh = vi.fn();
    const user = userEvent.setup();
    render(BatchGravityTab, { batch: makeBatch(), onRefresh });

    const gravityInput = screen.getByPlaceholderText(/Gravity.*°Bx/i);
    const dateInput = screen.getByDisplayValue(/.*/); // date field
    await user.type(gravityInput, "13.5");

    const addBtn = screen.getByRole("button", { name: /Add/i });
    await user.click(addBtn);
    await tick();
    await new Promise(r => setTimeout(r, 0));
    await tick();

    // The IPC call for convert_gravity should have been called with (13.5, "brix")
    expect(mockInvoke).toHaveBeenCalledWith("convert_gravity", { value: 13.5, fromUnit: "brix" });
    // The IPC call for add_gravity_reading should use the converted SG value
    expect(mockInvoke).toHaveBeenCalledWith("add_gravity_reading",
      expect.objectContaining({ input: expect.objectContaining({ gravity: 1.054 }) })
    );
  });
});
```

- [ ] **Step 2: Run tests to confirm they fail**

```bash
npm test -- --reporter=verbose 2>&1 | grep -A3 "BatchGravityTab"
```

Expected: first test PASS, others FAIL (conversion not yet implemented)

- [ ] **Step 3: Update BatchGravityTab**

Replace the entire contents of `src/lib/components/batch/BatchGravityTab.svelte`:

```svelte
<!-- src/lib/components/batch/BatchGravityTab.svelte -->
<script lang="ts">
  import type { Batch, CreateGravityReadingInput } from "$lib/api";
  import { addGravityReading, deleteGravityReading, convertGravity } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { settings } from "$lib/stores/settings";
  import { formatGravity, gravityStep, gravityPlaceholder } from "$lib/gravity-display";

  let { batch, onRefresh }: { batch: Batch; onRefresh: () => void } = $props();

  const gravityUnit = $derived($settings.gravity_unit ?? "sg");

  let newGravity = $state("");
  let newTemp = $state("");
  let newDate = $state(new Date().toISOString().slice(0, 10));
  let newNotes = $state("");

  let displayReadings = $state<string[]>([]);

  $effect(() => {
    const unit = gravityUnit;
    const readings = batch.gravity_readings;
    if (readings.length === 0) { displayReadings = []; return; }
    Promise.all(readings.map(r => convertGravity(r.gravity, "sg")))
      .then(results => { displayReadings = results.map(r => formatGravity(r, unit)); });
  });

  function formatDate(ts: number): string {
    return new Date(ts * 1000).toLocaleDateString();
  }

  async function handleAdd() {
    if (!newGravity || !newDate) return;
    const unit = gravityUnit;
    const converted = await ipc(convertGravity(parseFloat(newGravity), unit));
    if (!converted) return;
    const input: CreateGravityReadingInput = {
      recorded_at: Math.floor(new Date(newDate).getTime() / 1000),
      gravity: converted.sg,
      temp_c: newTemp ? parseFloat(newTemp) : null,
      notes: newNotes || null,
    };
    await ipc(addGravityReading(batch.id, input));
    newGravity = "";
    newTemp = "";
    newNotes = "";
    onRefresh();
  }

  async function handleDelete(id: string) {
    await ipc(deleteGravityReading(id));
    onRefresh();
  }
</script>

<div class="p-4 flex flex-col gap-4 overflow-y-auto">
  {#if batch.gravity_readings.length > 0}
    <table class="w-full text-sm">
      <thead>
        <tr style="color: var(--color-text-muted);">
          <th class="text-left py-1 font-normal">Date</th>
          <th class="text-left py-1 font-normal">Gravity</th>
          <th class="text-left py-1 font-normal">Temp (°C)</th>
          <th class="text-left py-1 font-normal">Notes</th>
          <th></th>
        </tr>
      </thead>
      <tbody>
        {#each batch.gravity_readings as r, i (r.id)}
          <tr class="border-t" style="border-color: var(--color-border);">
            <td class="py-1.5">{formatDate(r.recorded_at)}</td>
            <td class="py-1.5">{displayReadings[i] ?? r.gravity.toFixed(3)}</td>
            <td class="py-1.5">{r.temp_c != null ? r.temp_c + "°" : "—"}</td>
            <td class="py-1.5 text-xs" style="color: var(--color-text-muted);">{r.notes ?? ""}</td>
            <td class="py-1.5">
              <button
                onclick={() => handleDelete(r.id)}
                class="opacity-40 hover:opacity-100 text-xs"
                style="color: var(--color-text-muted);"
              >✕</button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {:else}
    <p class="text-sm" style="color: var(--color-text-muted);">No readings yet.</p>
  {/if}

  <div class="flex flex-col gap-2 pt-2 border-t" style="border-color: var(--color-border);">
    <div class="text-xs" style="color: var(--color-text-muted);">ADD READING</div>
    <div class="flex gap-2 flex-wrap">
      <input type="date" bind:value={newDate}
        class="px-2 py-1.5 rounded text-sm outline-none"
        style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      <input type="number" inputmode="decimal" step={gravityStep(gravityUnit)}
        placeholder={gravityPlaceholder(gravityUnit)} bind:value={newGravity}
        class="px-2 py-1.5 rounded text-sm outline-none w-40"
        style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      <input type="number" inputmode="decimal" step="0.1" placeholder="Temp °C" bind:value={newTemp}
        class="px-2 py-1.5 rounded text-sm outline-none w-24"
        style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      <input type="text" placeholder="Notes" bind:value={newNotes}
        class="px-2 py-1.5 rounded text-sm outline-none flex-1"
        style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      <button
        onclick={handleAdd}
        class="px-3 py-1.5 rounded text-sm"
        style="background: var(--color-accent); color: #fff;"
      >Add</button>
    </div>
  </div>
</div>
```

- [ ] **Step 4: Run tests**

```bash
npm test -- --reporter=verbose 2>&1 | grep -A5 "BatchGravityTab"
```

Expected: all 3 tests PASS

- [ ] **Step 5: Type-check**

```bash
npm run check 2>&1 | tail -5
```

Expected: `0 errors`

- [ ] **Step 6: Commit**

```bash
git add src/lib/components/batch/BatchGravityTab.svelte tests/BatchGravityTab.test.ts
git commit -m "feat: convert gravity readings to preferred unit in BatchGravityTab"
```

---

## Task 6: Update BatchOverviewTab — stage targets + measurements grid

**Files:**
- Modify: `src/lib/components/batch/BatchOverviewTab.svelte`

The stage-targets banner and measurements grid both display and accept gravity values. No new test file — the existing `BatchCarbonationSection.test.ts` pattern shows this component is integration-tested; manual verification is the final check for this task.

- [ ] **Step 1: Add imports to the `<script>` block**

At the top of the `<script lang="ts">` block in `src/lib/components/batch/BatchOverviewTab.svelte`, add after the existing imports:

```ts
import { convertGravity } from "$lib/api";
import { settings } from "$lib/stores/settings";
import { formatGravity, gravityStep } from "$lib/gravity-display";
import { ipc } from "$lib/stores/error";
```

- [ ] **Step 2: Add gravity display state and effect**

After the existing state declarations (after `let recipe = $state<Recipe | null>(null);`), add:

```ts
const gravityUnit = $derived($settings.gravity_unit ?? "sg");

// Pre-converted display strings for all gravity values shown in the banner and grid
let gravityDisplays = $state<Record<string, string>>({});

$effect(() => {
  const unit = gravityUnit;
  const b = batch;
  const gravityFields = [
    "planned_og", "planned_fg", "planned_pre_boil_gravity",
    "actual_og", "actual_fg", "actual_pre_boil_gravity",
  ] as const;

  const toConvert = gravityFields.filter(f => (b as Record<string, unknown>)[f] != null);

  Promise.all(
    toConvert.map(f =>
      convertGravity((b as Record<string, number>)[f], "sg")
        .then(r => [f, formatGravity(r, unit)] as const)
    )
  ).then(entries => {
    const next: Record<string, string> = {};
    for (const f of gravityFields) next[f] = "";
    for (const [f, v] of entries) next[f] = v;
    gravityDisplays = next;
  });
});
```

- [ ] **Step 3: Update `stageTargets` derived to use `gravityDisplays`**

Replace the `stageTargets` `$derived.by` block. Change every `.toFixed(3)` on a gravity value to reference `gravityDisplays`:

```ts
const stageTargets = $derived.by(() => {
  const { planned_og: og, planned_fg: fg, planned_pre_boil_gravity: pbg,
          planned_post_boil_volume_l: pbv, planned_batch_size_l: bs,
          actual_og, actual_fg } = batch;
  const targetAbv = og && fg ? ((og - fg) * 131.25).toFixed(1) : null;
  const actualAbv = actual_og && actual_fg ? ((actual_og - actual_fg) * 131.25).toFixed(1) : null;
  const items: { label: string; value: string }[] = [];
  switch (batch.status) {
    case "planned":
      if (og && gravityDisplays.planned_og) items.push({ label: "OG", value: gravityDisplays.planned_og });
      if (fg && gravityDisplays.planned_fg) items.push({ label: "FG", value: gravityDisplays.planned_fg });
      if (bs) items.push({ label: "Batch", value: `${bs.toFixed(1)} L` });
      break;
    case "brewing":
      if (pbg && gravityDisplays.planned_pre_boil_gravity) items.push({ label: "Pre-boil", value: gravityDisplays.planned_pre_boil_gravity });
      if (og && gravityDisplays.planned_og) items.push({ label: "OG", value: gravityDisplays.planned_og });
      if (pbv) items.push({ label: "Post-boil", value: `${pbv.toFixed(1)} L` });
      break;
    case "fermenting":
      if (actual_og && gravityDisplays.actual_og) items.push({ label: "Actual OG", value: gravityDisplays.actual_og });
      if (fg && gravityDisplays.planned_fg) items.push({ label: "Target FG", value: gravityDisplays.planned_fg });
      if (targetAbv) items.push({ label: "Target ABV", value: `${targetAbv}%` });
      break;
    case "conditioning":
    case "packaged":
      if (actual_og && gravityDisplays.actual_og) items.push({ label: "OG", value: gravityDisplays.actual_og });
      if (actual_fg && gravityDisplays.actual_fg) items.push({ label: "FG", value: gravityDisplays.actual_fg });
      if (actualAbv) items.push({ label: "ABV", value: `${actualAbv}%` });
      break;
  }
  return items;
});
```

- [ ] **Step 4: Update the measurements grid**

Replace the `{#each [...] as row}` block in the MEASUREMENTS section. The new version distinguishes gravity fields from volume fields:

```svelte
<div class="grid grid-cols-2 md:grid-cols-3 gap-3">
  {#each [
    { label: "Pre-Boil Gravity", field: "actual_pre_boil_gravity", isGravity: true },
    { label: "Original Gravity (OG)", field: "actual_og", isGravity: true },
    { label: "Final Gravity (FG)", field: "actual_fg", isGravity: true },
    { label: "Pre-Boil Volume (L)", field: "actual_pre_boil_volume_l" },
    { label: "Post-Boil Volume (L)", field: "actual_post_boil_volume_l" },
    { label: "Batch Size (L)", field: "actual_batch_size_l" },
  ] as row}
    {@const rawValue = (batch as Record<string, number | null>)[row.field]}
    {@const highlighted = highlightedFields.has(row.field)}
    <div
      class="p-3 rounded"
      style="background: var(--color-bg-elevated);
             border: 1px solid {highlighted ? 'rgba(99,102,241,0.4)' : 'var(--color-border)'};
             opacity: {highlighted || rawValue != null ? '1' : '0.55'};"
    >
      <label for="batch-{row.field}" class="text-xs block mb-1" style="color: var(--color-text-secondary);">{row.label}</label>
      {#if row.isGravity}
        <input
          id="batch-{row.field}"
          type="number" inputmode="decimal"
          step={gravityStep(gravityUnit)}
          value={gravityDisplays[row.field] ?? ""}
          onblur={async (e) => {
            const v = e.currentTarget.value;
            if (!v) { onUpdate({ [row.field]: null } as UpdateBatchInput); return; }
            const converted = await ipc(convertGravity(parseFloat(v), gravityUnit));
            if (converted) onUpdate({ [row.field]: converted.sg } as UpdateBatchInput);
          }}
          placeholder="—"
          class="w-full bg-transparent text-sm outline-none"
          style="color: {highlighted ? 'var(--color-accent)' : 'var(--color-text-primary)'}; font-weight: {highlighted ? '600' : '400'};"
        />
      {:else}
        <input
          id="batch-{row.field}"
          type="number" inputmode="decimal"
          step="0.1"
          value={rawValue != null ? rawValue.toFixed(1) : ""}
          onblur={(e) => {
            const v = e.currentTarget.value;
            onUpdate({ [row.field]: v ? parseFloat(v) : null } as UpdateBatchInput);
          }}
          placeholder="—"
          class="w-full bg-transparent text-sm outline-none"
          style="color: {highlighted ? 'var(--color-accent)' : 'var(--color-text-primary)'}; font-weight: {highlighted ? '600' : '400'};"
        />
      {/if}
    </div>
  {/each}
</div>
```

- [ ] **Step 5: Type-check**

```bash
npm run check 2>&1 | tail -5
```

Expected: `0 errors`

- [ ] **Step 6: Run full test suite**

```bash
npm test 2>&1 | tail -10
```

Expected: all tests PASS (no regressions)

- [ ] **Step 7: Commit**

```bash
git add src/lib/components/batch/BatchOverviewTab.svelte
git commit -m "feat: convert gravity to preferred unit in BatchOverviewTab"
```

---

## Task 7: Update mobile RecipeView

**Files:**
- Modify: `src/lib/mobile/RecipeView.svelte`

- [ ] **Step 1: Add imports**

In the `<script lang="ts">` block of `src/lib/mobile/RecipeView.svelte`, add after existing imports:

```ts
import { convertGravity } from "$lib/api";
import { settings } from "$lib/stores/settings";
import { formatGravity } from "$lib/gravity-display";
import { ipc } from "$lib/stores/error";
```

(`settings` is already imported — skip that line if so.)

- [ ] **Step 2: Add gravity state and effect**

After the existing state declarations, add:

```ts
const gravityUnit = $derived($settings.gravity_unit ?? "sg");
let displayOg = $state("—");
let displayFg = $state("—");

$effect(() => {
  const unit = gravityUnit;
  if (stats?.og != null) {
    ipc(convertGravity(stats.og, "sg")).then(r => { if (r) displayOg = formatGravity(r, unit); });
  } else {
    displayOg = "—";
  }
});

$effect(() => {
  const unit = gravityUnit;
  if (stats?.fg != null) {
    ipc(convertGravity(stats.fg, "sg")).then(r => { if (r) displayFg = formatGravity(r, unit); });
  } else {
    displayFg = "—";
  }
});
```

- [ ] **Step 3: Update the stats card in the template**

Find the stats card array in the template:

```svelte
{#each [
  { label: "OG", value: fmt(stats.og, 3) },
  { label: "FG", value: fmt(stats.fg, 3) },
  ...
] as stat}
```

Change to:

```svelte
{#each [
  { label: "OG", value: displayOg },
  { label: "FG", value: displayFg },
  { label: "ABV", value: fmt(stats.abv_pct, 1) + "%" },
  { label: "IBU", value: fmt(stats.ibu, 0) },
  { label: "SRM", value: fmt(stats.srm, 1) },
] as stat}
```

- [ ] **Step 4: Type-check**

```bash
npm run check 2>&1 | tail -5
```

Expected: `0 errors`

- [ ] **Step 5: Commit**

```bash
git add src/lib/mobile/RecipeView.svelte
git commit -m "feat: convert OG/FG to preferred unit in mobile RecipeView"
```

---

## Task 8: Update mobile BaselineRecipeView

**Files:**
- Modify: `src/lib/mobile/BaselineRecipeView.svelte`

This file has the same stats card pattern as `RecipeView`. Apply the identical changes.

- [ ] **Step 1: Add imports**

In `src/lib/mobile/BaselineRecipeView.svelte`, add after existing imports:

```ts
import { convertGravity } from "$lib/api";
import { settings } from "$lib/stores/settings";
import { formatGravity } from "$lib/gravity-display";
import { ipc } from "$lib/stores/error";
```

- [ ] **Step 2: Add gravity state and effects**

After the existing state declarations, add:

```ts
const gravityUnit = $derived($settings.gravity_unit ?? "sg");
let displayOg = $state("—");
let displayFg = $state("—");

$effect(() => {
  const unit = gravityUnit;
  if (stats?.og != null) {
    ipc(convertGravity(stats.og, "sg")).then(r => { if (r) displayOg = formatGravity(r, unit); });
  } else {
    displayOg = "—";
  }
});

$effect(() => {
  const unit = gravityUnit;
  if (stats?.fg != null) {
    ipc(convertGravity(stats.fg, "sg")).then(r => { if (r) displayFg = formatGravity(r, unit); });
  } else {
    displayFg = "—";
  }
});
```

- [ ] **Step 3: Update the stats card in the template**

Find:

```svelte
{#each [
  { label: "OG", value: fmt(stats.og, 3) },
  { label: "FG", value: fmt(stats.fg, 3) },
  ...
] as stat}
```

Change to:

```svelte
{#each [
  { label: "OG", value: displayOg },
  { label: "FG", value: displayFg },
  { label: "ABV", value: fmt(stats.abv_pct, 1) + "%" },
  { label: "IBU", value: fmt(stats.ibu, 0) },
  { label: "SRM", value: fmt(stats.srm, 1) },
] as stat}
```

- [ ] **Step 4: Type-check and run all tests**

```bash
npm run check 2>&1 | tail -5
npm test 2>&1 | tail -10
```

Expected: `0 errors`, all tests PASS

- [ ] **Step 5: Commit**

```bash
git add src/lib/mobile/BaselineRecipeView.svelte
git commit -m "feat: convert OG/FG to preferred unit in mobile BaselineRecipeView"
```

---

## Task 9: Save memory and verify

- [ ] **Step 1: Run the full test suite one final time**

```bash
npm test 2>&1 | tail -15
```

Expected: all tests PASS, no regressions

- [ ] **Step 2: Type-check**

```bash
npm run check 2>&1 | tail -5
```

Expected: `0 errors`

- [ ] **Step 3: Final commit if anything unstaged**

```bash
git status
```

If clean, done. If any files modified but uncommitted:

```bash
git add -p
git commit -m "chore: tidy gravity unit setting implementation"
```
