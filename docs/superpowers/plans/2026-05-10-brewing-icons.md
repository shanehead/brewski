# Brewing Icons Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add emoji icons throughout the app (ingredient headings, tab bar, IngredientPicker dialog) via a single abstraction layer that makes swapping to custom SVGs a one-file change.

**Architecture:** A central `src/lib/icons.ts` registry maps `BrewingIconName` values to emoji strings. A single `BrewingIcon.svelte` component reads from that registry and renders a `<span>`. All call sites use `<BrewingIcon name="hop" />` — when switching to SVGs, only `BrewingIcon.svelte` changes.

**Tech Stack:** Svelte 5, TypeScript, Vitest + @testing-library/svelte + happy-dom. Test command: `bunx vitest run`. Type check: `bunx svelte-check`.

---

## File Map

| Action | Path | Responsibility |
|--------|------|---------------|
| Create | `src/lib/icons.ts` | `BrewingIconName` type + `ICONS` record |
| Create | `src/lib/components/BrewingIcon.svelte` | Renders emoji from registry; future SVG swap point |
| Create | `tests/icons.test.ts` | Unit tests for the registry |
| Create | `tests/BrewingIcon.test.ts` | Component rendering tests |
| Modify | `src/routes/recipe/[id]/+page.svelte` | Add `icon` field to TABS, render in tab bar |
| Modify | `src/lib/components/ingredients/FermentablesTable.svelte` | Icon next to "Fermentables" heading |
| Modify | `src/lib/components/ingredients/HopsTable.svelte` | Icon next to "Hops" heading |
| Modify | `src/lib/components/ingredients/YeastsTable.svelte` | Icon next to "Yeast" heading |
| Modify | `src/lib/components/ingredients/IngredientPicker.svelte` | Icon + title header in dialog |

---

### Task 1: Icon registry (`src/lib/icons.ts`)

**Files:**
- Create: `src/lib/icons.ts`
- Create: `tests/icons.test.ts`

- [ ] **Step 1: Write the failing test**

Create `tests/icons.test.ts`:

```ts
import { describe, it, expect } from "vitest";
import { ICONS, type BrewingIconName } from "$lib/icons";

const ALL_NAMES: BrewingIconName[] = [
  "fermentable", "hop", "yeast",
  "overview", "ingredients", "mash", "fermentation", "notes",
];

describe("ICONS registry", () => {
  it("has an entry for every BrewingIconName", () => {
    for (const name of ALL_NAMES) {
      expect(ICONS[name], `missing icon for "${name}"`).toBeDefined();
    }
  });

  it("all entries are non-empty strings", () => {
    for (const name of ALL_NAMES) {
      expect(typeof ICONS[name]).toBe("string");
      expect(ICONS[name].length).toBeGreaterThan(0);
    }
  });
});
```

- [ ] **Step 2: Run test — verify it fails**

```bash
bunx vitest run tests/icons.test.ts
```

Expected: `Cannot find module '$lib/icons'`

- [ ] **Step 3: Create `src/lib/icons.ts`**

```ts
export type BrewingIconName =
  | "fermentable"
  | "hop"
  | "yeast"
  | "overview"
  | "ingredients"
  | "mash"
  | "fermentation"
  | "notes";

export const ICONS: Record<BrewingIconName, string> = {
  fermentable: "🌾",
  hop: "🍃",
  yeast: "🧫",
  overview: "📋",
  ingredients: "🛒",
  mash: "🌡️",
  fermentation: "🍺",
  notes: "✏️",
};
```

- [ ] **Step 4: Run test — verify it passes**

```bash
bunx vitest run tests/icons.test.ts
```

Expected: 2 tests pass

- [ ] **Step 5: Commit**

```bash
git add src/lib/icons.ts tests/icons.test.ts
git commit -m "feat: add BrewingIconName registry"
```

---

### Task 2: BrewingIcon component

**Files:**
- Create: `src/lib/components/BrewingIcon.svelte`
- Create: `tests/BrewingIcon.test.ts`

- [ ] **Step 1: Write the failing test**

Create `tests/BrewingIcon.test.ts`:

```ts
import { describe, it, expect } from "vitest";
import { render } from "@testing-library/svelte";
import BrewingIcon from "$lib/components/BrewingIcon.svelte";

describe("BrewingIcon", () => {
  it("renders the hop emoji for name='hop'", () => {
    const { container } = render(BrewingIcon, { name: "hop" });
    expect(container.textContent).toBe("🍃");
  });

  it("renders the fermentable emoji for name='fermentable'", () => {
    const { container } = render(BrewingIcon, { name: "fermentable" });
    expect(container.textContent).toBe("🌾");
  });

  it("renders a span with aria-hidden", () => {
    const { container } = render(BrewingIcon, { name: "yeast" });
    const span = container.querySelector("span");
    expect(span).not.toBeNull();
    expect(span!.getAttribute("aria-hidden")).toBe("true");
  });
});
```

- [ ] **Step 2: Run test — verify it fails**

```bash
bunx vitest run tests/BrewingIcon.test.ts
```

Expected: `Cannot find module '$lib/components/BrewingIcon.svelte'`

- [ ] **Step 3: Create `src/lib/components/BrewingIcon.svelte`**

```svelte
<script lang="ts">
  import { ICONS, type BrewingIconName } from "$lib/icons";
  let { name }: { name: BrewingIconName } = $props();
</script>

<span aria-hidden="true">{ICONS[name]}</span>
```

- [ ] **Step 4: Run test — verify it passes**

```bash
bunx vitest run tests/BrewingIcon.test.ts
```

Expected: 3 tests pass

- [ ] **Step 5: Run full test suite — verify no regressions**

```bash
bunx vitest run
```

Expected: all tests pass

- [ ] **Step 6: Commit**

```bash
git add src/lib/components/BrewingIcon.svelte tests/BrewingIcon.test.ts
git commit -m "feat: add BrewingIcon component"
```

---

### Task 3: Tab bar icons

**Files:**
- Modify: `src/routes/recipe/[id]/+page.svelte`

- [ ] **Step 1: Import BrewingIcon and add `icon` to TABS**

In `src/routes/recipe/[id]/+page.svelte`, add the import after the existing imports at the top of the `<script>` block:

```ts
import BrewingIcon from "$lib/components/BrewingIcon.svelte";
import type { BrewingIconName } from "$lib/icons";
```

Replace the `TABS` constant (currently around line 23):

```ts
const TABS: { key: "overview" | "ingredients" | "mash" | "fermentation" | "notes"; label: string; icon: BrewingIconName }[] = [
  { key: "overview",     label: "Overview",     icon: "overview"     },
  { key: "ingredients",  label: "Ingredients",  icon: "ingredients"  },
  { key: "mash",         label: "Mash",         icon: "mash"         },
  { key: "fermentation", label: "Fermentation", icon: "fermentation" },
  { key: "notes",        label: "Notes",        icon: "notes"        },
];
```

- [ ] **Step 2: Render icon in the tab button**

In the tab bar template (the `{#each TABS as tab}` block around line 80), replace:

```svelte
        >
          {tab.label}
        </button>
```

with:

```svelte
        >
          <BrewingIcon name={tab.icon} /> {tab.label}
        </button>
```

- [ ] **Step 3: Type-check**

```bash
bunx svelte-check
```

Expected: 0 errors

- [ ] **Step 4: Commit**

```bash
git add src/routes/recipe/[id]/+page.svelte
git commit -m "feat: add icons to recipe tab bar"
```

---

### Task 4: Ingredient section heading icons

**Files:**
- Modify: `src/lib/components/ingredients/FermentablesTable.svelte`
- Modify: `src/lib/components/ingredients/HopsTable.svelte`
- Modify: `src/lib/components/ingredients/YeastsTable.svelte`

- [ ] **Step 1: Update FermentablesTable**

In `src/lib/components/ingredients/FermentablesTable.svelte`, add the import after the existing imports:

```ts
import BrewingIcon from "../BrewingIcon.svelte";
```

Replace the heading line (currently `<h3 class="text-sm font-semibold" ...>Fermentables</h3>`):

```svelte
    <h3 class="text-sm font-semibold flex items-center gap-1" style="color: var(--color-text-primary);"><BrewingIcon name="fermentable" /> Fermentables</h3>
```

- [ ] **Step 2: Update HopsTable**

In `src/lib/components/ingredients/HopsTable.svelte`, add the import after the existing imports:

```ts
import BrewingIcon from "../BrewingIcon.svelte";
```

Replace the heading line (currently `<h3 class="text-sm font-semibold" ...>Hops</h3>`):

```svelte
    <h3 class="text-sm font-semibold flex items-center gap-1" style="color: var(--color-text-primary);"><BrewingIcon name="hop" /> Hops</h3>
```

- [ ] **Step 3: Update YeastsTable**

In `src/lib/components/ingredients/YeastsTable.svelte`, add the import after the existing imports:

```ts
import BrewingIcon from "../BrewingIcon.svelte";
```

Replace the heading line (currently `<h3 class="text-sm font-semibold" ...>Yeast</h3>`):

```svelte
    <h3 class="text-sm font-semibold flex items-center gap-1" style="color: var(--color-text-primary);"><BrewingIcon name="yeast" /> Yeast</h3>
```

- [ ] **Step 4: Type-check**

```bash
bunx svelte-check
```

Expected: 0 errors

- [ ] **Step 5: Commit**

```bash
git add src/lib/components/ingredients/FermentablesTable.svelte \
        src/lib/components/ingredients/HopsTable.svelte \
        src/lib/components/ingredients/YeastsTable.svelte
git commit -m "feat: add icons to ingredient section headings"
```

---

### Task 5: IngredientPicker dialog header

**Files:**
- Modify: `src/lib/components/ingredients/IngredientPicker.svelte`

- [ ] **Step 1: Import BrewingIcon and derive icon name + title**

In `src/lib/components/ingredients/IngredientPicker.svelte`, add after the existing imports:

```ts
import BrewingIcon from "../BrewingIcon.svelte";
import type { BrewingIconName } from "$lib/icons";
```

Add these two derived values after the existing `$derived` declarations (e.g. after the `units` line):

```ts
const iconName = $derived<BrewingIconName>(
  type === "hop" ? "hop" : type === "fermentable" ? "fermentable" : "yeast"
);
const dialogTitle = $derived(
  type === "hop" ? "Add Hop" : type === "fermentable" ? "Add Fermentable" : "Add Yeast"
);
```

- [ ] **Step 2: Add the header to the dialog template**

Inside the `<dialog>` element, immediately before the `<div style="display: flex; height: 100%;">` wrapper (around line 149), insert:

```svelte
  <div style="display: flex; align-items: center; gap: 8px; padding: 14px 16px 12px; border-bottom: 1px solid var(--color-border);">
    <span style="font-size: 20px;"><BrewingIcon name={iconName} /></span>
    <span style="font-size: 15px; font-weight: 700; color: var(--color-text-primary);">{dialogTitle}</span>
  </div>
```

- [ ] **Step 3: Type-check**

```bash
bunx svelte-check
```

Expected: 0 errors

- [ ] **Step 4: Run full test suite**

```bash
bunx vitest run
```

Expected: all tests pass

- [ ] **Step 5: Commit**

```bash
git add src/lib/components/ingredients/IngredientPicker.svelte
git commit -m "feat: add icon and title header to IngredientPicker dialog"
```
