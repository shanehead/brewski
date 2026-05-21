# BeerXML Import Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add an "Import BeerXML" button to the Recipes page on both desktop and mobile so users can import recipes from `.xml` files.

**Architecture:** The backend (`create_recipes_from_beerxml`) and its `api.ts` wrapper (`createRecipesFromBeerxml`) are already complete. This is purely a frontend change: add a hidden `<input type="file" accept=".xml">` and a button to trigger it in two components. On file selection, read the file as text, call the API, and refresh the recipe list.

**Tech Stack:** Svelte 5 (`$props`, `$state`, `bind:this`), TypeScript, Vitest, @testing-library/svelte

---

## Files

- Modify: `src/lib/components/RecipeList.svelte` — desktop sidebar (add button + handler)
- Modify: `src/lib/mobile/RecipesHome.svelte` — mobile recipe list (add button + handler)
- Create: `tests/BeerXMLImport.test.ts` — component tests for both

---

### Task 1: Desktop — RecipeList.svelte

**Files:**
- Modify: `src/lib/components/RecipeList.svelte`
- Create: `tests/BeerXMLImport.test.ts`

- [ ] **Step 1: Write the failing test**

Create `tests/BeerXMLImport.test.ts`:

```typescript
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render } from "@testing-library/svelte";
import RecipeList from "$lib/components/RecipeList.svelte";

vi.mock("$lib/stores/recipes", () => ({
  recipeList: {
    subscribe: (fn: (val: unknown[]) => void) => {
      fn([]);
      return () => {};
    },
  },
  refreshRecipeList: vi.fn().mockResolvedValue(undefined),
}));

vi.mock("$lib/stores/settings", () => ({
  settings: {
    subscribe: (fn: (val: { units: string }) => void) => {
      fn({ units: "metric" });
      return () => {};
    },
  },
}));

vi.mock("$lib/stores/error", () => ({
  ipc: vi.fn((p: Promise<unknown>) => p),
}));

vi.mock("$lib/api", () => ({
  createRecipe: vi.fn(),
  deleteRecipe: vi.fn(),
  createRecipesFromBeerxml: vi.fn().mockResolvedValue([]),
}));

vi.mock("$app/navigation", () => ({
  goto: vi.fn(),
}));

describe("RecipeList", () => {
  it("renders the Import BeerXML button", () => {
    const { getByText } = render(RecipeList);
    expect(getByText("Import BeerXML")).toBeTruthy();
  });
});
```

- [ ] **Step 2: Run the test to verify it fails**

```bash
bun run test -- --reporter=verbose BeerXMLImport
```

Expected: FAIL — `Unable to find an element with the text: Import BeerXML`

- [ ] **Step 3: Add the import button and handler to RecipeList.svelte**

Replace the entire file with:

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { recipeList, refreshRecipeList } from "$lib/stores/recipes";
  import { createRecipe, deleteRecipe, createRecipesFromBeerxml } from "$lib/api";
  import type { RecipeSummary } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { settings } from "$lib/stores/settings";
  import { type Units, lToGal, volumeLabel } from "$lib/units";

  let { selectedId = $bindable<string | null>(null) } = $props();
  let search = $state("");
  let fileInput: HTMLInputElement;

  const units = $derived<Units>($settings.units === "imperial" ? "imperial" : "metric");

  const filtered = $derived(
    search.trim()
      ? $recipeList.filter((r) => r.name.toLowerCase().includes(search.toLowerCase()))
      : $recipeList
  );

  onMount(() => ipc(refreshRecipeList()));

  async function handleNew() {
    const recipe = await ipc(createRecipe({ name: "New Recipe" }));
    if (!recipe) return;
    await ipc(refreshRecipeList());
    goto(`/recipe/${recipe.id}`);
  }

  async function handleDelete(id: string) {
    await ipc(deleteRecipe(id));
    await ipc(refreshRecipeList());
    if (selectedId === id) goto("/");
  }

  async function handleImport(event: Event) {
    const file = (event.target as HTMLInputElement).files?.[0];
    if (!file) return;
    const xml = await file.text();
    await ipc(createRecipesFromBeerxml(xml));
    await ipc(refreshRecipeList());
    fileInput.value = "";
  }
</script>

<aside class="w-56 flex flex-col flex-shrink-0 border-r overflow-hidden"
       style="background: var(--color-bg-surface); border-color: var(--color-border);">
  <!-- Search + New -->
  <div class="p-2 flex flex-col gap-1.5 border-b" style="border-color: var(--color-border);">
    <div class="relative">
      <svg class="absolute left-2 top-1/2 -translate-y-1/2 pointer-events-none" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" style="color: var(--color-text-muted);">
        <circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
      </svg>
      <input
        type="search"
        placeholder="Search recipes…"
        bind:value={search}
        class="w-full pl-7 pr-2.5 py-1.5 rounded text-sm outline-none"
        style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);"
      />
    </div>
    <button
      onclick={handleNew}
      class="w-full py-1.5 rounded text-sm font-medium transition-colors"
      style="background: var(--color-accent); color: #fff;"
    >
      + New Recipe
    </button>
    <input
      type="file"
      accept=".xml"
      bind:this={fileInput}
      onchange={handleImport}
      class="hidden"
    />
    <button
      onclick={() => fileInput.click()}
      class="w-full py-1.5 rounded text-sm font-medium transition-colors"
      style="border: 1px solid var(--color-border); color: var(--color-text-secondary); background: transparent;"
    >
      Import BeerXML
    </button>
  </div>

  <!-- Recipe list -->
  <ul class="flex-1 overflow-y-auto py-1">
    {#each filtered as recipe (recipe.id)}
      <li class="group relative">
        <a
          href="/recipe/{recipe.id}"
          class="flex flex-col px-3 py-2 pr-7 cursor-pointer transition-colors hover:bg-[var(--color-bg-elevated)]"
          style={selectedId === recipe.id
            ? "background: var(--color-bg-elevated); border-left: 2px solid var(--color-accent); padding-left: calc(0.75rem - 2px);"
            : "color: var(--color-text-primary); border-left: 2px solid transparent; padding-left: calc(0.75rem - 2px);"}
        >
          <span class="text-sm font-medium truncate" style="color: var(--color-text-primary);">{recipe.name}</span>
          <span class="text-xs truncate mt-0.5" style="color: var(--color-text-secondary);">
            {recipe.style_name ?? recipe.type_} · {(units === "imperial" ? lToGal(recipe.batch_size_l) : recipe.batch_size_l).toFixed(1)}{volumeLabel(units)}
          </span>
        </a>
        <button
          onclick={() => handleDelete(recipe.id)}
          aria-label="Delete recipe"
          class="absolute right-2 top-1/2 -translate-y-1/2 opacity-0 group-hover:opacity-100 transition-opacity text-sm leading-none"
          style="color: var(--color-text-muted);"
        >×</button>
      </li>
    {:else}
      <li class="px-3 py-6 text-center text-sm" style="color: var(--color-text-muted);">
        {search ? "No matches" : "No recipes yet"}
      </li>
    {/each}
  </ul>
</aside>
```

- [ ] **Step 4: Run the test to verify it passes**

```bash
bun run test -- --reporter=verbose BeerXMLImport
```

Expected: PASS — `renders the Import BeerXML button`

- [ ] **Step 5: Commit**

```bash
git add src/lib/components/RecipeList.svelte tests/BeerXMLImport.test.ts
git commit -m "feat: add BeerXML import button to desktop recipe list"
```

---

### Task 2: Mobile — RecipesHome.svelte

**Files:**
- Modify: `src/lib/mobile/RecipesHome.svelte`
- Modify: `tests/BeerXMLImport.test.ts`

- [ ] **Step 1: Add a failing test for the mobile component**

Add the import at the top of `tests/BeerXMLImport.test.ts` (after the existing imports), then append the `describe` block at the bottom of the file:

```typescript
// Add at top with other imports:
import MobileRecipesHome from "$lib/mobile/RecipesHome.svelte";
```

```typescript
// Append at bottom of file:
describe("Mobile RecipesHome", () => {
  it("renders the Import BeerXML button", () => {
    const { getByText } = render(MobileRecipesHome);
    expect(getByText("Import BeerXML")).toBeTruthy();
  });
});
```

- [ ] **Step 2: Run the test to verify it fails**

```bash
bun run test -- --reporter=verbose BeerXMLImport
```

Expected: 1 pass, 1 FAIL — `Unable to find an element with the text: Import BeerXML` in the mobile test.

- [ ] **Step 3: Update RecipesHome.svelte**

Replace the entire file with:

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { recipeList, refreshRecipeList } from "$lib/stores/recipes";
  import { createRecipe, createRecipesFromBeerxml } from "$lib/api";
  import type { RecipeSummary } from "$lib/api";
  import { ipc } from "$lib/stores/error";

  let fileInput: HTMLInputElement;

  onMount(() => ipc(refreshRecipeList()));

  async function handleNew() {
    const recipe = await ipc(createRecipe({ name: "New Recipe" }));
    if (recipe) goto(`/recipe/${recipe.id}`);
  }

  async function handleImport(event: Event) {
    const file = (event.target as HTMLInputElement).files?.[0];
    if (!file) return;
    const xml = await file.text();
    await ipc(createRecipesFromBeerxml(xml));
    await ipc(refreshRecipeList());
    fileInput.value = "";
  }
</script>

<div class="flex flex-col h-full overflow-hidden" style="background: var(--color-bg-surface);">
  <div class="p-3 border-b flex flex-col gap-2" style="border-color: var(--color-border);">
    <button
      onclick={handleNew}
      class="w-full py-3 rounded text-sm font-medium"
      style="background: var(--color-accent); color: #fff;"
    >+ New Recipe</button>
    <input
      type="file"
      accept=".xml"
      bind:this={fileInput}
      onchange={handleImport}
      class="hidden"
    />
    <button
      onclick={() => fileInput.click()}
      class="w-full py-3 rounded text-sm font-medium"
      style="border: 1px solid var(--color-border); color: var(--color-text-secondary); background: transparent;"
    >
      Import BeerXML
    </button>
  </div>
  <div class="flex-1 overflow-y-auto">
    {#each $recipeList as recipe (recipe.id)}
      <a
        href="/recipe/{recipe.id}"
        class="flex items-center justify-between px-4 py-3 border-b text-sm"
        style="border-color: var(--color-border); color: var(--color-text-primary);"
      >
        <span class="truncate">{recipe.name}</span>
        <span style="color: var(--color-text-muted);">›</span>
      </a>
    {:else}
      <p class="p-4 text-sm" style="color: var(--color-text-muted);">No recipes yet. Tap + to create one.</p>
    {/each}
  </div>
</div>
```

- [ ] **Step 4: Run all tests to verify everything passes**

```bash
bun run test
```

Expected: All tests pass (the 2 new BeerXML tests + all existing tests).

- [ ] **Step 5: Commit**

```bash
git add src/lib/mobile/RecipesHome.svelte tests/BeerXMLImport.test.ts
git commit -m "feat: add BeerXML import button to mobile recipe list"
```
