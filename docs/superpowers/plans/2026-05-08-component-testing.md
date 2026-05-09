# Component Testing Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Install Vitest + @testing-library/svelte, migrate the existing bun:test unit tests, and write six MashTab component tests covering the strike-temp display and infuse-amount input flows.

**Architecture:** A `vitest.config.ts` merges the existing `vite.config.ts` so Svelte compilation and `$lib` path aliases are inherited. Tests live in a flat `tests/` directory. Each test file mocks `@tauri-apps/api/core` at the file level and configures per-test return values for the specific invoke calls it exercises. The settings store requires no mock — it defaults to `{}` on init, making `$settings.units` undefined and `units` default to `"metric"`.

**Tech Stack:** Vitest, @testing-library/svelte v5 (Svelte 5 runes support), @testing-library/jest-dom, @testing-library/user-event, happy-dom, SvelteKit 2, Svelte 5, TypeScript.

---

## File Map

| File | Action |
|------|--------|
| `package.json` | Modify — add `test` and `test:watch` scripts, add dev dependencies |
| `vitest.config.ts` | Create — Vitest config merging vite.config.ts |
| `tests/setup.ts` | Create — jest-dom matchers import |
| `tests/units.test.ts` | Create — units tests migrated from src/lib/units.test.ts |
| `src/lib/units.test.ts` | Delete — replaced by tests/units.test.ts |
| `tests/MashTab.test.ts` | Create — 6 component tests |
| `Justfile` | Modify — update `test-frontend` to use `bun run test` |

---

## Task 1: Install packages and configure Vitest

**Files:**
- Modify: `package.json`
- Create: `vitest.config.ts`
- Create: `tests/setup.ts`
- Modify: `Justfile`

- [ ] **Step 1: Install dev dependencies**

```bash
bun add -D vitest @testing-library/svelte @testing-library/jest-dom @testing-library/user-event happy-dom
```

Expected: packages added to `devDependencies` in `package.json`. `@testing-library/svelte` must resolve to v5+ for Svelte 5 runes support — verify with `bun pm ls | grep testing-library/svelte`.

- [ ] **Step 2: Add test scripts to package.json**

Open `package.json`. Add two scripts inside the `"scripts"` block:

```json
"test": "vitest run",
"test:watch": "vitest"
```

The full scripts block becomes:

```json
"scripts": {
  "dev": "vite dev",
  "build": "vite build",
  "preview": "vite preview",
  "check": "svelte-kit sync && svelte-check --tsconfig ./tsconfig.json",
  "check:watch": "svelte-kit sync && svelte-check --tsconfig ./tsconfig.json --watch",
  "test": "vitest run",
  "test:watch": "vitest",
  "tauri": "tauri"
}
```

- [ ] **Step 3: Create vitest.config.ts**

Create `vitest.config.ts` at the project root:

```typescript
import { defineConfig, mergeConfig } from "vitest/config";
import viteConfig from "./vite.config";

export default mergeConfig(
  viteConfig,
  defineConfig({
    test: {
      environment: "happy-dom",
      include: ["tests/**/*.test.ts"],
      setupFiles: ["tests/setup.ts"],
      globals: true,
    },
  })
);
```

- [ ] **Step 4: Create tests/setup.ts**

Create `tests/setup.ts`:

```typescript
import "@testing-library/jest-dom";
```

- [ ] **Step 5: Update Justfile test-frontend command**

Open `Justfile`. Find:

```
# Run frontend unit tests
test-frontend:
    bun test
```

Replace with:

```
# Run frontend unit tests
test-frontend:
    bun run test
```

`bun test` invokes bun's native runner; `bun run test` runs the `"test"` script from `package.json`, which now invokes Vitest.

- [ ] **Step 6: Commit**

```bash
git add package.json vitest.config.ts tests/setup.ts Justfile
git commit -m "chore: install Vitest and @testing-library/svelte, add test config"
```

---

## Task 2: Migrate units.test.ts to Vitest

**Files:**
- Create: `tests/units.test.ts`
- Delete: `src/lib/units.test.ts`

- [ ] **Step 1: Create tests/units.test.ts**

Create `tests/units.test.ts`. The only change from the original is the import source (`$lib/units` instead of `./units`) and replacing `bun:test` imports with `vitest`:

```typescript
import { describe, expect, it } from "vitest";
import {
  kgToLb, lbToKg,
  kgToHopDisplay, hopDisplayToKg,
  lToGal, galToL,
  cToF, fToC,
  weightLabel, hopWeightLabel, volumeLabel, tempLabel,
  lPerKgToQtPerLb, qtPerLbToLPerKg, ratioLabel,
} from "$lib/units";

describe("fermentable weight: kg ↔ lb", () => {
  it("converts 1 kg to lb", () => { expect(kgToLb(1)).toBeCloseTo(2.20462, 4); });
  it("converts 1 lb to kg", () => { expect(lbToKg(1)).toBeCloseTo(0.453592, 4); });
  it("round-trips", () => { expect(lbToKg(kgToLb(5))).toBeCloseTo(5, 4); });
  it("zero stays zero", () => { expect(kgToLb(0)).toBe(0); expect(lbToKg(0)).toBe(0); });
});

describe("hop weight: kg ↔ g (metric) / oz (imperial)", () => {
  it("converts kg to grams in metric", () => {
    expect(kgToHopDisplay(0.028, "metric")).toBeCloseTo(28, 1);
    expect(kgToHopDisplay(0.1, "metric")).toBeCloseTo(100, 1);
  });
  it("converts kg to oz in imperial", () => {
    expect(kgToHopDisplay(1 / 35.274, "imperial")).toBeCloseTo(1, 4);
    expect(kgToHopDisplay(0.028, "imperial")).toBeCloseTo(0.988, 2);
  });
  it("round-trips in metric", () => {
    expect(hopDisplayToKg(kgToHopDisplay(0.057, "metric"), "metric")).toBeCloseTo(0.057, 4);
  });
  it("round-trips in imperial", () => {
    expect(hopDisplayToKg(kgToHopDisplay(0.057, "imperial"), "imperial")).toBeCloseTo(0.057, 4);
  });
});

describe("volume: L ↔ gal", () => {
  it("converts 1 gal to L", () => { expect(galToL(1)).toBeCloseTo(3.78541, 4); });
  it("converts 1 L to gal", () => { expect(lToGal(1)).toBeCloseTo(0.264172, 4); });
  it("converts a typical batch (23 L ≈ 6.06 gal)", () => { expect(lToGal(23)).toBeCloseTo(6.076, 2); });
  it("round-trips", () => { expect(galToL(lToGal(23))).toBeCloseTo(23, 4); });
});

describe("temperature: °C ↔ °F", () => {
  it("converts 0 °C → 32 °F", () => { expect(cToF(0)).toBeCloseTo(32, 4); });
  it("converts 100 °C → 212 °F", () => { expect(cToF(100)).toBeCloseTo(212, 4); });
  it("converts typical mash temp (67 °C → 152.6 °F)", () => { expect(cToF(67)).toBeCloseTo(152.6, 1); });
  it("converts typical sparge temp (75 °C → 167 °F)", () => { expect(cToF(75)).toBeCloseTo(167, 1); });
  it("converges at -40 (same in both scales)", () => { expect(cToF(-40)).toBeCloseTo(-40, 4); });
  it("round-trips", () => { expect(fToC(cToF(67))).toBeCloseTo(67, 4); });
});

describe("water:grain ratio: L/kg ↔ qt/lb", () => {
  it("converts 3.0 L/kg to qt/lb", () => { expect(lPerKgToQtPerLb(3.0)).toBeCloseTo(1.438, 2); });
  it("converts 1.5 qt/lb to L/kg", () => { expect(qtPerLbToLPerKg(1.5)).toBeCloseTo(3.130, 2); });
  it("round-trips", () => { expect(qtPerLbToLPerKg(lPerKgToQtPerLb(3.0))).toBeCloseTo(3.0, 4); });
});

describe("label helpers", () => {
  it("weightLabel", () => {
    expect(weightLabel("metric")).toBe("kg");
    expect(weightLabel("imperial")).toBe("lb");
  });
  it("hopWeightLabel", () => {
    expect(hopWeightLabel("metric")).toBe("g");
    expect(hopWeightLabel("imperial")).toBe("oz");
  });
  it("volumeLabel", () => {
    expect(volumeLabel("metric")).toBe("L");
    expect(volumeLabel("imperial")).toBe("gal");
  });
  it("tempLabel", () => {
    expect(tempLabel("metric")).toBe("°C");
    expect(tempLabel("imperial")).toBe("°F");
  });
  it("ratioLabel", () => {
    expect(ratioLabel("metric")).toBe("L/kg");
    expect(ratioLabel("imperial")).toBe("qt/lb");
  });
});
```

- [ ] **Step 2: Run the migrated tests**

```bash
bun run test
```

Expected: 26 tests pass, 0 fail. Output ends with something like `26 tests passed`.

If tests fail due to the `$lib/units` path alias not resolving: verify `vitest.config.ts` uses `mergeConfig` with `viteConfig` (which activates the SvelteKit plugin that registers `$lib`). Run `bunx svelte-kit sync` first if needed.

- [ ] **Step 3: Delete the old test file**

```bash
rm src/lib/units.test.ts
```

- [ ] **Step 4: Commit**

```bash
git add tests/units.test.ts
git rm src/lib/units.test.ts
git commit -m "chore: migrate units.test.ts to Vitest under tests/"
```

---

## Task 3: Write MashTab component tests

**Files:**
- Create: `tests/MashTab.test.ts`

These tests verify existing behaviour — all six should pass immediately. They cover the two bugs that motivated this work (missing infuse input, strike temp not displaying) plus the surrounding conditional-display logic.

- [ ] **Step 1: Create tests/MashTab.test.ts**

Create `tests/MashTab.test.ts`:

```typescript
import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import type { Recipe, Mash, MashStep, RecipeStats, RecipeAdditionFermentable } from "$lib/api";
import MashTab from "$lib/components/tabs/MashTab.svelte";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));

// --- Fixtures ---
// Use `as unknown as T` casts: these are test doubles that only populate
// the fields MashTab actually reads, not the full interface.

function makeStep(overrides: Partial<MashStep> = {}): MashStep {
  return {
    id: "s1",
    mash_id: "m1",
    name: "Mash In",
    type_: "infusion",
    infuse_amount_l: null,
    step_temp_c: 67,
    step_time_min: 60,
    ramp_time_min: null,
    end_temp_c: null,
    step_order: 0,
    ...overrides,
  };
}

function makeMash(overrides: Partial<Mash> = {}): Mash {
  return {
    id: "m1",
    recipe_id: "r1",
    name: "Single Infusion",
    grain_temp_c: 21,
    tun_temp_c: null,
    sparge_temp_c: null,
    ph: null,
    tun_weight_kg: null,
    tun_specific_heat: null,
    equip_adjust: false,
    ratio_l_per_kg: null,
    notes: null,
    steps: [],
    ...overrides,
  };
}

function makeRecipe(overrides: Partial<{ mash: Mash | null; fermentables: RecipeAdditionFermentable[] }> = {}): Recipe {
  return {
    id: "r1",
    fermentables: [],
    hops: [],
    yeasts: [],
    miscs: [],
    waters: [],
    mash: null,
    ...overrides,
  } as unknown as Recipe;
}

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
    ...overrides,
  };
}

// --- Tests ---

describe("MashTab: strike temp display", () => {
  it("renders strike temp when stats.strike_temp_c is set", () => {
    const recipe = makeRecipe({
      mash: makeMash({ steps: [makeStep({ infuse_amount_l: 15 })] }),
      fermentables: [{ amount_kg: 4.5 } as RecipeAdditionFermentable],
    });
    const stats = makeStats({ strike_temp_c: 69.82 });

    render(MashTab, { recipe, stats, onchange: vi.fn() });

    // units defaults to "metric" (settings store initialises to {} in tests)
    // 69.82.toFixed(1) + tempLabel("metric") → "69.8°C"
    expect(screen.getByText(/69\.8°C/)).toBeInTheDocument();
  });

  it("does not render strike temp when stats.strike_temp_c is null", () => {
    render(MashTab, {
      recipe: makeRecipe(),
      stats: makeStats({ strike_temp_c: null }),
      onchange: vi.fn(),
    });

    expect(screen.queryByText(/Strike Temp/)).not.toBeInTheDocument();
  });
});

describe("MashTab: infuse amount input in Add Step form", () => {
  it("shows infuse amount input when step type is infusion", async () => {
    const user = userEvent.setup();
    render(MashTab, { recipe: makeRecipe(), stats: makeStats(), onchange: vi.fn() });

    await user.click(screen.getByRole("button", { name: /\+ Add Step/i }));

    // stepType defaults to "infusion" — infuse input should appear
    // placeholder is "Infuse L" with metric units
    expect(screen.getByPlaceholderText("Infuse L")).toBeInTheDocument();
  });

  it("hides infuse amount input when step type is not infusion", async () => {
    const user = userEvent.setup();
    render(MashTab, { recipe: makeRecipe(), stats: makeStats(), onchange: vi.fn() });

    await user.click(screen.getByRole("button", { name: /\+ Add Step/i }));
    await user.selectOptions(screen.getByRole("combobox"), "temperature");

    expect(screen.queryByPlaceholderText("Infuse L")).not.toBeInTheDocument();
  });
});

describe("MashTab: water:grain ratio fallback input", () => {
  it("shows ratio input when mash has no step with an infuse amount", () => {
    const recipe = makeRecipe({
      // step exists but infuse_amount_l is null → canAutoDerive = false
      mash: makeMash({ steps: [makeStep({ infuse_amount_l: null })] }),
      fermentables: [{ amount_kg: 4.5 } as RecipeAdditionFermentable],
    });

    render(MashTab, { recipe, stats: makeStats(), onchange: vi.fn() });

    // label is "Water:Grain Ratio (L/kg)" with metric units
    expect(screen.getByLabelText(/Water:Grain Ratio/)).toBeInTheDocument();
  });

  it("hides ratio input when a step has an infuse amount and fermentables exist", () => {
    const recipe = makeRecipe({
      // infuse_amount_l set + fermentables → canAutoDerive = true
      mash: makeMash({ steps: [makeStep({ infuse_amount_l: 15 })] }),
      fermentables: [{ amount_kg: 4.5 } as RecipeAdditionFermentable],
    });

    render(MashTab, { recipe, stats: makeStats(), onchange: vi.fn() });

    expect(screen.queryByLabelText(/Water:Grain Ratio/)).not.toBeInTheDocument();
  });
});
```

- [ ] **Step 2: Run the component tests**

```bash
bun run test
```

Expected: 32 tests pass (26 unit + 6 component), 0 fail.

If any component test fails:
- `getByText(/69\.8°C/)` not found → check that `stats` prop is wired in `+page.svelte` and `tests/setup.ts` is listed in `setupFiles`
- `getByPlaceholderText("Infuse L")` not found → verify the infuse input was added in the previous fix (placeholder is `"Infuse {volumeLabel(units)}"`)
- `getByLabelText(/Water:Grain Ratio/)` not found → check the `<label for="mash-ratio">` in MashTab has matching `id` on the input

- [ ] **Step 3: Commit**

```bash
git add tests/MashTab.test.ts
git commit -m "test: add MashTab component tests for strike temp display and infuse input"
```

---

## Self-Review

**Spec coverage:**

| Spec requirement | Task |
|---|---|
| Install vitest, @testing-library/svelte, jest-dom, user-event, happy-dom | Task 1 Step 1 |
| `vitest.config.ts` merging vite config, happy-dom, tests/, setupFiles | Task 1 Step 3 |
| `tests/setup.ts` with jest-dom import | Task 1 Step 4 |
| `test` and `test:watch` scripts in package.json | Task 1 Step 2 |
| Justfile `test-frontend` updated to `bun run test` | Task 1 Step 5 |
| units.test.ts migrated to `tests/`, import updated to `$lib/units` | Task 2 |
| Old `src/lib/units.test.ts` deleted | Task 2 Step 3 |
| `@tauri-apps/api/core` mocked at file level | Task 3 Step 1 |
| Per-test invoke configuration pattern established | Task 3 Step 1 |
| Test: strike temp renders when `stats.strike_temp_c` set | Task 3 Step 1 |
| Test: strike temp hidden when `stats.strike_temp_c` null | Task 3 Step 1 |
| Test: infuse input visible for infusion step type | Task 3 Step 1 |
| Test: infuse input hidden for non-infusion type | Task 3 Step 1 |
| Test: ratio input shown when no infuse amounts | Task 3 Step 1 |
| Test: ratio input hidden when auto-derive possible | Task 3 Step 1 |

All requirements covered. No placeholders.
