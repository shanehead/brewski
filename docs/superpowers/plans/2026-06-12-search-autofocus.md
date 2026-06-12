# Search Fields and Autofocus Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add search filtering to the Equipment page and mobile Recipes list, and add autofocus-on-mount to all full-page views with a search field.

**Architecture:** Each full-page list view gets a `query` state variable and a `filtered` derived value (case-insensitive substring on `name`). Autofocus uses `onMount` + `setTimeout(() => el?.focus(), 0)` via a `bind:this` ref — consistent with the existing IngredientPicker pattern. The Ingredient Library already has search; it only needs the autofocus wired up.

**Tech Stack:** SvelteKit, Svelte 5 runes (`$state`, `$derived`), Vitest, @testing-library/svelte

---

## Files

| Action | Path |
|---|---|
| Modify | `src/routes/equipment/+page.svelte` |
| Modify | `src/lib/mobile/RecipesHome.svelte` |
| Modify | `src/routes/library/+page.svelte` |
| Modify | `tests/EquipmentPage.test.ts` |
| Modify | `tests/RecipesHome.test.ts` |
| Create | `tests/LibraryPage.test.ts` |

---

## Task 1: Equipment page search and autofocus

**Files:**
- Modify: `src/routes/equipment/+page.svelte`
- Modify: `tests/EquipmentPage.test.ts`

- [ ] **Step 1: Write failing tests**

Add these tests to `tests/EquipmentPage.test.ts` after the existing `describe` block. The existing mock already returns a profile named `"My Kettle"`.

```ts
describe("EquipmentPage - search", () => {
  it("renders the search input", async () => {
    const { getByPlaceholderText } = render(EquipmentPage);
    expect(getByPlaceholderText("Search profiles…")).toBeInTheDocument();
  });

  it("shows a matching profile", async () => {
    const { getByPlaceholderText, getByText } = render(EquipmentPage);
    await new Promise((r) => setTimeout(r, 10));
    await tick();
    const input = getByPlaceholderText("Search profiles…");
    await userEvent.type(input, "Kettle");
    expect(getByText("My Kettle")).toBeInTheDocument();
  });

  it("hides a non-matching profile", async () => {
    const { getByPlaceholderText, queryByText } = render(EquipmentPage);
    await new Promise((r) => setTimeout(r, 10));
    await tick();
    const input = getByPlaceholderText("Search profiles…");
    await userEvent.type(input, "Zzz");
    expect(queryByText("My Kettle")).toBeNull();
  });

  it("shows empty state when no profiles match", async () => {
    const { getByPlaceholderText, getByText } = render(EquipmentPage);
    await new Promise((r) => setTimeout(r, 10));
    await tick();
    const input = getByPlaceholderText("Search profiles…");
    await userEvent.type(input, "Zzz");
    expect(getByText(/No profiles match/)).toBeInTheDocument();
  });
});
```

Add the `userEvent` import at the top of the file (after the existing imports):

```ts
import userEvent from "@testing-library/user-event";
```

- [ ] **Step 2: Run tests to verify they fail**

```bash
npm test -- --reporter=verbose tests/EquipmentPage.test.ts
```

Expected: the four new tests FAIL with "Unable to find an element with the placeholder text: Search profiles…"

- [ ] **Step 3: Implement search in the equipment page**

In `src/routes/equipment/+page.svelte`, make these changes:

In the `<script>` block, add after the existing `let` declarations:

```ts
  let searchEl = $state<HTMLInputElement | null>(null);
  let query = $state("");

  const filtered = $derived(
    query.trim() === ""
      ? profiles
      : profiles.filter((p) => p.name.toLowerCase().includes(query.trim().toLowerCase()))
  );
```

Add `onMount` to the existing import:

```ts
  import { onMount } from "svelte";
```

(It's already imported — just confirm it's there. If it is, no change needed.)

Update the existing `onMount` body to add autofocus:

```ts
  onMount(async () => {
    await ipc(loadSettings());
    profiles = await ipc(listEquipmentProfiles()) ?? [];
    setTimeout(() => searchEl?.focus(), 0);
  });
```

In the template, add the search input between the `<h2>Equipment Profiles</h2>` heading and the `{#each profiles ...}` loop. Replace the `{#each profiles as p (p.id)}` block with `{#each filtered as p (p.id)}`. Full section after the edit:

```svelte
      <h2 class="text-sm font-semibold text-text-secondary">Equipment Profiles</h2>

      <div class="relative max-w-xs">
        <svg class="text-text-muted" style="position: absolute; left: 8px; top: 50%; transform: translateY(-50%); pointer-events: none;"
             width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
        </svg>
        <input
          bind:this={searchEl}
          bind:value={query}
          placeholder="Search profiles…"
          class="pl-8 pr-3 py-1.5 rounded text-sm w-full bg-bg-elevated border border-border text-text-primary"
          style="outline: none;"
        />
      </div>

      <div class="flex items-center justify-between">
        <label for="select-default-profile" class="text-sm text-text-primary">Default Profile</label>
        ...existing select...
      </div>

      {#if filtered.length === 0 && query.trim() !== ""}
        <p class="text-sm text-text-muted py-2">No profiles match "{query}"</p>
      {/if}

      {#each filtered as p (p.id)}
        ...existing row markup, unchanged...
      {/each}
```

- [ ] **Step 4: Run tests to verify they pass**

```bash
npm test -- --reporter=verbose tests/EquipmentPage.test.ts
```

Expected: all tests PASS

- [ ] **Step 5: Commit**

```bash
git add src/routes/equipment/+page.svelte tests/EquipmentPage.test.ts
git commit -m "feat: add search and autofocus to equipment page"
```

---

## Task 2: Mobile RecipesHome search and autofocus

**Files:**
- Modify: `src/lib/mobile/RecipesHome.svelte`
- Modify: `tests/RecipesHome.test.ts`

- [ ] **Step 1: Write failing tests**

The existing mock for `recipeList` returns `[]`. Update the mock to use a mutable variable so tests can inject recipes, then add search tests.

Replace the existing `recipeList` mock in `tests/RecipesHome.test.ts`:

```ts
let mockRecipes: {
  id: string; name: string; type_: string; batch_size_l: number;
  style_name: string | null; image_path: string | null;
  created_at: number; updated_at: number; source: "user" | "seeded";
}[] = [];
```

Update the `vi.mock("$lib/stores/recipes", ...)` factory to use `mockRecipes`:

```ts
vi.mock("$lib/stores/recipes", () => ({
  recipeList: { subscribe: vi.fn((fn) => { fn(mockRecipes); return () => {}; }) },
  baselineRecipeList: { subscribe: vi.fn((fn) => { fn(mockBaselineRecipes); return () => {}; }) },
  refreshRecipeList: vi.fn().mockResolvedValue(undefined),
  refreshBaselineRecipeList: vi.fn().mockResolvedValue(undefined),
}));
```

Reset it in `beforeEach`:

```ts
beforeEach(() => {
  mockSettings = {};
  mockBaselineRecipes = [];
  mockRecipes = [];
});
```

Add the `userEvent` import and a new describe block:

```ts
import userEvent from "@testing-library/user-event";
```

```ts
const userRecipe = {
  id: "u1", name: "Citra Pale Ale", type_: "All Grain",
  batch_size_l: 23, style_name: null, image_path: null,
  created_at: 0, updated_at: 0, source: "user" as const,
};

describe("RecipesHome (mobile) - search", () => {
  it("renders the search input", () => {
    const { getByPlaceholderText } = render(RecipesHome);
    expect(getByPlaceholderText("Search recipes…")).toBeInTheDocument();
  });

  it("shows a matching recipe", async () => {
    mockRecipes = [userRecipe];
    const { getByPlaceholderText, getByText } = render(RecipesHome);
    const input = getByPlaceholderText("Search recipes…");
    await userEvent.type(input, "Citra");
    expect(getByText("Citra Pale Ale")).toBeInTheDocument();
  });

  it("hides a non-matching recipe", async () => {
    mockRecipes = [userRecipe];
    const { getByPlaceholderText, queryByText } = render(RecipesHome);
    const input = getByPlaceholderText("Search recipes…");
    await userEvent.type(input, "Zzz");
    expect(queryByText("Citra Pale Ale")).toBeNull();
  });

  it("example recipes are not filtered by search", async () => {
    mockBaselineRecipes = [exampleRecipe];
    mockSettings = {};
    const { getByPlaceholderText, getByText } = render(RecipesHome);
    const input = getByPlaceholderText("Search recipes…");
    await userEvent.type(input, "Zzz");
    expect(getByText("Example Recipes")).toBeInTheDocument();
  });
});
```

- [ ] **Step 2: Run tests to verify they fail**

```bash
npm test -- --reporter=verbose tests/RecipesHome.test.ts
```

Expected: the four new tests FAIL with "Unable to find an element with the placeholder text: Search recipes…"

- [ ] **Step 3: Implement search in mobile RecipesHome**

In `src/lib/mobile/RecipesHome.svelte`, add to the `<script>` block after the existing `let` declarations:

```ts
  let searchEl = $state<HTMLInputElement | null>(null);
  let search = $state("");

  const filtered = $derived(
    search.trim() === ""
      ? $recipeList
      : $recipeList.filter((r) => r.name.toLowerCase().includes(search.trim().toLowerCase()))
  );
```

Update the existing `onMount` to add autofocus:

```ts
  onMount(async () => {
    appDataDir = await getAppDataDir();
    ipc(refreshRecipeList());
    ipc(refreshBaselineRecipeList());
    setTimeout(() => searchEl?.focus(), 0);
  });
```

`onMount` is already imported. Add `onMount` to the import if it's missing — check the existing imports first.

In the template, add the search input inside the existing header block, below the Import BeerXML button:

```svelte
    <div class="relative">
      <svg class="text-text-muted" style="position: absolute; left: 8px; top: 50%; transform: translateY(-50%); pointer-events: none;"
           width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
      </svg>
      <input
        bind:this={searchEl}
        type="search"
        placeholder="Search recipes…"
        bind:value={search}
        class="w-full pl-8 pr-2.5 py-2 rounded text-sm outline-none bg-bg-elevated text-text-primary border border-border"
      />
    </div>
```

Change the `{#each $recipeList as recipe (recipe.id)}` loop to use `filtered`:

```svelte
    {#each filtered as recipe (recipe.id)}
```

Update the empty state in the `{:else}` of that loop:

```svelte
    {:else}
      <p class="p-4 text-sm text-text-muted">
        {search ? "No matches" : "No recipes yet. Tap + to create one."}
      </p>
```

- [ ] **Step 4: Run tests to verify they pass**

```bash
npm test -- --reporter=verbose tests/RecipesHome.test.ts
```

Expected: all tests PASS

- [ ] **Step 5: Commit**

```bash
git add src/lib/mobile/RecipesHome.svelte tests/RecipesHome.test.ts
git commit -m "feat: add search and autofocus to mobile recipes list"
```

---

## Task 3: Ingredient Library autofocus

**Files:**
- Modify: `src/routes/library/+page.svelte`
- Create: `tests/LibraryPage.test.ts`

- [ ] **Step 1: Write a failing test**

Create `tests/LibraryPage.test.ts`:

```ts
import { describe, it, expect, vi } from "vitest";
import { render } from "@testing-library/svelte";
import LibraryPage from "../src/routes/library/+page.svelte";

vi.mock("$lib/api", () => ({
  listHopLibrary: vi.fn().mockResolvedValue([]),
  listFermentableLibrary: vi.fn().mockResolvedValue([]),
  listYeastLibrary: vi.fn().mockResolvedValue([]),
  listMiscLibrary: vi.fn().mockResolvedValue([]),
  listWaterLibrary: vi.fn().mockResolvedValue([]),
  deleteHop: vi.fn().mockResolvedValue(undefined),
  deleteFermentable: vi.fn().mockResolvedValue(undefined),
  deleteYeast: vi.fn().mockResolvedValue(undefined),
  deleteMisc: vi.fn().mockResolvedValue(undefined),
  deleteWater: vi.fn().mockResolvedValue(undefined),
}));

vi.mock("$lib/stores/error", () => ({
  ipc: vi.fn((p) => p),
}));

describe("LibraryPage", () => {
  it("renders the search input", () => {
    const { getByPlaceholderText } = render(LibraryPage);
    expect(getByPlaceholderText("Search hops…")).toBeInTheDocument();
  });
});
```

- [ ] **Step 2: Run test to verify it passes already**

```bash
npm test -- --reporter=verbose tests/LibraryPage.test.ts
```

Expected: PASS (the search input already exists — this confirms the baseline before we add autofocus).

- [ ] **Step 3: Add autofocus to the library search input**

In `src/routes/library/+page.svelte`, add a ref for the search element. In the `<script>` block, after the existing `let query = $state('');` line, add:

```ts
  let searchEl = $state<HTMLInputElement | null>(null);
```

Update the existing `onMount` call. Currently it is:

```ts
  onMount(() => loadTab('hop'));
```

Replace with:

```ts
  onMount(() => {
    loadTab('hop');
    setTimeout(() => searchEl?.focus(), 0);
  });
```

Add `bind:this={searchEl}` to the existing search `<input>` in the template. The existing input looks like:

```svelte
      <input bind:value={query}
             placeholder="Search {TAB_LABELS[activeTab].toLowerCase()}…"
             class="pl-8 pr-3 py-1.5 rounded text-sm w-full bg-bg-elevated border border-border text-text-primary"
             style="outline: none;" />
```

Add `bind:this={searchEl}` as an attribute:

```svelte
      <input bind:this={searchEl}
             bind:value={query}
             placeholder="Search {TAB_LABELS[activeTab].toLowerCase()}…"
             class="pl-8 pr-3 py-1.5 rounded text-sm w-full bg-bg-elevated border border-border text-text-primary"
             style="outline: none;" />
```

- [ ] **Step 4: Run all tests to verify nothing broke**

```bash
npm test -- --reporter=verbose
```

Expected: all tests PASS

- [ ] **Step 5: Commit**

```bash
git add src/routes/library/+page.svelte tests/LibraryPage.test.ts
git commit -m "feat: autofocus search input on library page"
```

---

## Task 4: Push

- [ ] **Step 1: Push all commits**

```bash
git push
```
