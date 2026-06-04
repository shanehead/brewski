# Hide Example Recipes Setting Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a settings toggle that completely hides the Example Recipes section from the recipe list on both desktop and mobile.

**Architecture:** A new boolean `hide_example_recipes` field is added to `AppSettings`. The settings page gets a checkbox that writes it via `saveSetting`. Both recipe list components gate their existing `{#if $baselineRecipeList.length > 0}` block with `&& !$settings.hide_example_recipes`. No backend changes are required.

**Tech Stack:** SvelteKit 5 / Svelte 5 runes, Tailwind CSS v4, Vitest + Testing Library

---

## Files

| File | Action |
|---|---|
| `src/lib/stores/settings.ts` | Add `hide_example_recipes?: boolean` to `AppSettings` |
| `src/routes/settings/+page.svelte` | Add Recipes section with checkbox toggle |
| `src/lib/components/RecipeList.svelte` | Gate example recipes block on setting |
| `src/lib/mobile/RecipesHome.svelte` | Gate example recipes block on setting |
| `tests/SettingsPage.test.ts` | New — tests for the toggle UI |
| `tests/RecipeList.test.ts` | New — tests for hiding the section (desktop) |
| `tests/RecipesHome.test.ts` | New — tests for hiding the section (mobile) |

---

### Task 1: Add `hide_example_recipes` to AppSettings

**Files:**
- Modify: `src/lib/stores/settings.ts`

- [ ] **Step 1: Add the field to the interface**

In `src/lib/stores/settings.ts`, add `hide_example_recipes?: boolean` as the last field in `AppSettings`:

```ts
export interface AppSettings {
  units?: "metric" | "imperial";
  gravity_unit?: GravityUnit;
  theme?: string;
  default_equipment_profile_id?: string;
  last_route?: string;
  starters_collapsed?: boolean;
  hide_example_recipes?: boolean;
}
```

- [ ] **Step 2: Verify type check passes**

```bash
npm run check
```

Expected: no type errors

- [ ] **Step 3: Commit**

```bash
git add src/lib/stores/settings.ts
git commit -m "feat: add hide_example_recipes to AppSettings"
```

---

### Task 2: Settings page toggle

**Files:**
- Modify: `src/routes/settings/+page.svelte`
- Create: `tests/SettingsPage.test.ts`

- [ ] **Step 1: Write failing tests**

Create `tests/SettingsPage.test.ts`:

```ts
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, fireEvent } from "@testing-library/svelte";
import SettingsPage from "../src/routes/settings/+page.svelte";

const { saveSettingMock } = vi.hoisted(() => ({
  saveSettingMock: vi.fn(),
}));

let mockSettings: Record<string, unknown> = {};

vi.mock("$lib/stores/settings", () => ({
  loadSettings: vi.fn().mockResolvedValue(undefined),
  saveSetting: saveSettingMock,
  settings: {
    subscribe: vi.fn((fn) => {
      fn(mockSettings);
      return () => {};
    }),
  },
}));

vi.mock("$lib/stores/error", () => ({
  ipc: vi.fn((p) => p),
}));

vi.mock("$lib/api", () => ({
  getDbPath: vi.fn().mockResolvedValue("/data/brewski.db"),
  detectSyncFolders: vi.fn().mockResolvedValue([]),
  moveDatabase: vi.fn().mockResolvedValue(undefined),
}));

beforeEach(() => {
  saveSettingMock.mockClear();
  mockSettings = {};
});

describe("Settings page - hide example recipes", () => {
  it("renders a Recipes section heading", () => {
    const { getByText } = render(SettingsPage);
    expect(getByText("Recipes")).toBeInTheDocument();
  });

  it("renders a Hide Example Recipes label", () => {
    const { getByText } = render(SettingsPage);
    expect(getByText("Hide Example Recipes")).toBeInTheDocument();
  });

  it("checkbox is unchecked when setting is false", () => {
    mockSettings = { hide_example_recipes: false };
    const { getByLabelText } = render(SettingsPage);
    expect((getByLabelText("Hide Example Recipes") as HTMLInputElement).checked).toBe(false);
  });

  it("checkbox is checked when setting is true", () => {
    mockSettings = { hide_example_recipes: true };
    const { getByLabelText } = render(SettingsPage);
    expect((getByLabelText("Hide Example Recipes") as HTMLInputElement).checked).toBe(true);
  });

  it("calls saveSetting with 'true' when checkbox is clicked while unchecked", async () => {
    mockSettings = { hide_example_recipes: false };
    const { getByLabelText } = render(SettingsPage);
    await fireEvent.click(getByLabelText("Hide Example Recipes"));
    expect(saveSettingMock).toHaveBeenCalledWith("hide_example_recipes", "true");
  });
});
```

- [ ] **Step 2: Run tests to verify they fail**

```bash
npm test -- --reporter=verbose tests/SettingsPage.test.ts
```

Expected: FAIL — "Recipes" section not found

- [ ] **Step 3: Add the handler to the script block**

In `src/routes/settings/+page.svelte`, add this handler after `handleGravityUnitChange`:

```ts
async function handleHideExamplesChange(e: Event) {
  await ipc(saveSetting("hide_example_recipes", (e.target as HTMLInputElement).checked ? "true" : "false"));
}
```

- [ ] **Step 4: Add the Recipes section to the template**

In `src/routes/settings/+page.svelte`, add this section between the closing `</section>` of Units and the `<DatabaseLocation />` line:

```svelte
<!-- Recipes -->
<section class="flex flex-col gap-3">
  <h2 class="text-sm font-semibold" style="color: var(--color-text-secondary);">Recipes</h2>
  <div class="flex items-center justify-between">
    <label for="toggle-hide-examples" class="text-sm" style="color: var(--color-text-primary);">Hide Example Recipes</label>
    <input
      id="toggle-hide-examples"
      type="checkbox"
      checked={$settings.hide_example_recipes ?? false}
      onchange={handleHideExamplesChange}
      class="w-4 h-4 rounded cursor-pointer"
      style="accent-color: var(--color-accent);"
    />
  </div>
</section>
```

- [ ] **Step 5: Run tests to verify they pass**

```bash
npm test -- --reporter=verbose tests/SettingsPage.test.ts
```

Expected: 5 tests PASS

- [ ] **Step 6: Commit**

```bash
git add src/routes/settings/+page.svelte tests/SettingsPage.test.ts
git commit -m "feat: add hide example recipes toggle to settings page"
```

---

### Task 3: Gate desktop RecipeList

**Files:**
- Modify: `src/lib/components/RecipeList.svelte`
- Create: `tests/RecipeList.test.ts`

- [ ] **Step 1: Write failing tests**

Create `tests/RecipeList.test.ts`:

```ts
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render } from "@testing-library/svelte";
import RecipeList from "../src/lib/components/RecipeList.svelte";

vi.mock("$app/navigation", () => ({ goto: vi.fn() }));

let mockSettings: Record<string, unknown> = {};
let mockBaselineRecipes: {
  id: string; name: string; type_: string; batch_size_l: number;
  style_name: string | null; image_path: string | null;
  created_at: number; updated_at: number; source: "user" | "seeded";
}[] = [];

vi.mock("$lib/stores/recipes", () => ({
  recipeList: { subscribe: vi.fn((fn) => { fn([]); return () => {}; }) },
  baselineRecipeList: { subscribe: vi.fn((fn) => { fn(mockBaselineRecipes); return () => {}; }) },
  refreshRecipeList: vi.fn().mockResolvedValue(undefined),
  refreshBaselineRecipeList: vi.fn().mockResolvedValue(undefined),
}));

vi.mock("$lib/stores/settings", () => ({
  settings: {
    subscribe: vi.fn((fn) => { fn(mockSettings); return () => {}; }),
  },
  saveSetting: vi.fn(),
}));

vi.mock("$lib/stores/error", () => ({
  ipc: vi.fn((p) => p),
  setSuccess: vi.fn(),
}));

vi.mock("$lib/api", () => ({
  createRecipe: vi.fn(),
  deleteRecipe: vi.fn(),
  createRecipesFromBeerxml: vi.fn(),
}));

vi.mock("@tauri-apps/api/core", () => ({
  convertFileSrc: vi.fn((p: string) => p),
}));

vi.mock("@tauri-apps/api/path", () => ({
  appDataDir: vi.fn().mockResolvedValue("/data"),
}));

beforeEach(() => {
  mockSettings = {};
  mockBaselineRecipes = [];
});

const exampleRecipe = {
  id: "ex1", name: "Pliny the Elder", type_: "All Grain",
  batch_size_l: 20.8, style_name: null, image_path: null,
  created_at: 0, updated_at: 0, source: "seeded" as const,
};

describe("RecipeList - example recipes visibility", () => {
  it("shows Example Recipes section when setting is absent", () => {
    mockBaselineRecipes = [exampleRecipe];
    mockSettings = {};
    const { getByText } = render(RecipeList, { selectedId: null });
    expect(getByText("Example Recipes")).toBeInTheDocument();
  });

  it("shows Example Recipes section when setting is false", () => {
    mockBaselineRecipes = [exampleRecipe];
    mockSettings = { hide_example_recipes: false };
    const { getByText } = render(RecipeList, { selectedId: null });
    expect(getByText("Example Recipes")).toBeInTheDocument();
  });

  it("hides Example Recipes section when setting is true", () => {
    mockBaselineRecipes = [exampleRecipe];
    mockSettings = { hide_example_recipes: true };
    const { queryByText } = render(RecipeList, { selectedId: null });
    expect(queryByText("Example Recipes")).toBeNull();
  });

  it("hides individual example recipe names when setting is true", () => {
    mockBaselineRecipes = [exampleRecipe];
    mockSettings = { hide_example_recipes: true };
    const { queryByText } = render(RecipeList, { selectedId: null });
    expect(queryByText("Pliny the Elder")).toBeNull();
  });
});
```

- [ ] **Step 2: Run tests to verify they fail**

```bash
npm test -- --reporter=verbose tests/RecipeList.test.ts
```

Expected: "hides Example Recipes section when setting is true" and "hides individual example recipe names when setting is true" FAIL

- [ ] **Step 3: Gate the block**

In `src/lib/components/RecipeList.svelte`, find this line (inside the `<ul>`, near the bottom of the template):

```svelte
{#if $baselineRecipeList.length > 0}
```

Change it to:

```svelte
{#if $baselineRecipeList.length > 0 && !$settings.hide_example_recipes}
```

- [ ] **Step 4: Run tests to verify they pass**

```bash
npm test -- --reporter=verbose tests/RecipeList.test.ts
```

Expected: 4 tests PASS

- [ ] **Step 5: Commit**

```bash
git add src/lib/components/RecipeList.svelte tests/RecipeList.test.ts
git commit -m "feat: hide example recipes in desktop recipe list"
```

---

### Task 4: Gate mobile RecipesHome

**Files:**
- Modify: `src/lib/mobile/RecipesHome.svelte`
- Create: `tests/RecipesHome.test.ts`

- [ ] **Step 1: Write failing tests**

Create `tests/RecipesHome.test.ts`:

```ts
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render } from "@testing-library/svelte";
import RecipesHome from "../src/lib/mobile/RecipesHome.svelte";

vi.mock("$app/navigation", () => ({ goto: vi.fn() }));

let mockSettings: Record<string, unknown> = {};
let mockBaselineRecipes: {
  id: string; name: string; type_: string; batch_size_l: number;
  style_name: string | null; image_path: string | null;
  created_at: number; updated_at: number; source: "user" | "seeded";
}[] = [];

vi.mock("$lib/stores/recipes", () => ({
  recipeList: { subscribe: vi.fn((fn) => { fn([]); return () => {}; }) },
  baselineRecipeList: { subscribe: vi.fn((fn) => { fn(mockBaselineRecipes); return () => {}; }) },
  refreshRecipeList: vi.fn().mockResolvedValue(undefined),
  refreshBaselineRecipeList: vi.fn().mockResolvedValue(undefined),
}));

vi.mock("$lib/stores/settings", () => ({
  settings: {
    subscribe: vi.fn((fn) => { fn(mockSettings); return () => {}; }),
  },
  saveSetting: vi.fn(),
}));

vi.mock("$lib/stores/error", () => ({
  ipc: vi.fn((p) => p),
  setSuccess: vi.fn(),
}));

vi.mock("$lib/api", () => ({
  createRecipe: vi.fn(),
  createRecipesFromBeerxml: vi.fn(),
}));

vi.mock("@tauri-apps/api/core", () => ({
  convertFileSrc: vi.fn((p: string) => p),
}));

vi.mock("@tauri-apps/api/path", () => ({
  appDataDir: vi.fn().mockResolvedValue("/data"),
}));

beforeEach(() => {
  mockSettings = {};
  mockBaselineRecipes = [];
});

const exampleRecipe = {
  id: "ex1", name: "Heady Topper", type_: "All Grain",
  batch_size_l: 18.9, style_name: null, image_path: null,
  created_at: 0, updated_at: 0, source: "seeded" as const,
};

describe("RecipesHome (mobile) - example recipes visibility", () => {
  it("shows Example Recipes section when setting is absent", () => {
    mockBaselineRecipes = [exampleRecipe];
    mockSettings = {};
    const { getByText } = render(RecipesHome);
    expect(getByText("Example Recipes")).toBeInTheDocument();
  });

  it("hides Example Recipes section when setting is true", () => {
    mockBaselineRecipes = [exampleRecipe];
    mockSettings = { hide_example_recipes: true };
    const { queryByText } = render(RecipesHome);
    expect(queryByText("Example Recipes")).toBeNull();
  });

  it("hides individual example recipe names when setting is true", () => {
    mockBaselineRecipes = [exampleRecipe];
    mockSettings = { hide_example_recipes: true };
    const { queryByText } = render(RecipesHome);
    expect(queryByText("Heady Topper")).toBeNull();
  });
});
```

- [ ] **Step 2: Run tests to verify they fail**

```bash
npm test -- --reporter=verbose tests/RecipesHome.test.ts
```

Expected: "hides Example Recipes section when setting is true" and "hides individual example recipe names when setting is true" FAIL

- [ ] **Step 3: Gate the block**

In `src/lib/mobile/RecipesHome.svelte`, find this line (inside the scrollable div, near the bottom of the template):

```svelte
{#if $baselineRecipeList.length > 0}
```

Change it to:

```svelte
{#if $baselineRecipeList.length > 0 && !$settings.hide_example_recipes}
```

- [ ] **Step 4: Run the full test suite**

```bash
npm test
```

Expected: all tests PASS, including pre-existing ones

- [ ] **Step 5: Commit**

```bash
git add src/lib/mobile/RecipesHome.svelte tests/RecipesHome.test.ts
git commit -m "feat: hide example recipes in mobile recipe list"
```
