# Hide Recipe/Batch Sidebar on Detail View — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Remove the recipe/batch list sidebar from detail views so content gets full width; update rail navigation so the Recipes and Batches icons return to the list home when you're already in that section.

**Architecture:** Four surgical deletions/changes across four files. AppShell rail handlers get new conditional logic. RecipeView and BaselineRecipeView lose their `<RecipeList>` elements. BatchView loses its `<aside>` block plus all imports and calls that only existed to feed it.

**Tech Stack:** Svelte 5 (runes), SvelteKit, Vitest + @testing-library/svelte

---

### Task 1: AppShell — update rail button navigation logic

When the user is already in the Recipes section and clicks the Recipes rail icon, they should go to `/` (the list), not back to the same recipe detail. Same for Batches.

**Files:**
- Modify: `src/lib/desktop/AppShell.svelte:58-70`
- Modify: `tests/AppShell.test.ts`

---

- [ ] **Step 1: Write the failing tests**

Add these tests to `tests/AppShell.test.ts`, inside the existing `describe("AppShell rail dynamic navigation")` block.

The existing test "Recipes button navigates to last_route_recipes when set" sets `mockSettings = { last_route_recipes: "/recipe/abc?tab=mash" }` but uses the default `mockPathname = "/"` — which is inside the Recipes section. With the new logic, clicking Recipes from `/` goes to `/`, not to the last recipe. Fix it so the test exercises cross-section navigation by setting `mockPathname = "/tools"` before rendering.

Add at the end of the existing `describe("AppShell rail dynamic navigation")` block, before the closing `});`:

```ts
  it("Recipes button navigates to / when already on a recipe page", async () => {
    mockPathname = "/recipe/abc";
    mockSettings = { last_route_recipes: "/recipe/abc?tab=mash" };
    const { container } = render(AppShell, { children: () => null });
    const btn = container.querySelector('button[aria-label="Recipes"]') as HTMLButtonElement;
    btn.click();
    await tick();
    expect(gotoMock).toHaveBeenCalledWith("/");
  });

  it("Batches button navigates to /batches when already on a batch page", async () => {
    mockPathname = "/batches/xyz";
    mockSettings = { last_route_batches: "/batches/xyz" };
    const { container } = render(AppShell, { children: () => null });
    const btn = container.querySelector('button[aria-label="Batches"]') as HTMLButtonElement;
    btn.click();
    await tick();
    expect(gotoMock).toHaveBeenCalledWith("/batches");
  });
```

Also update the existing "Recipes button navigates to last_route_recipes when set" test — add `mockPathname = "/tools";` before `mockSettings = ...`:

```ts
  it("Recipes button navigates to last_route_recipes when set", async () => {
    mockPathname = "/tools";
    mockSettings = { last_route_recipes: "/recipe/abc?tab=mash" };
    const { container } = render(AppShell, { children: () => null });
    const btn = container.querySelector('button[aria-label="Recipes"]') as HTMLButtonElement;
    btn.click();
    await tick();
    expect(gotoMock).toHaveBeenCalledWith("/recipe/abc?tab=mash");
  });
```

- [ ] **Step 2: Run tests to confirm failures**

```bash
cd /Users/shead/Documents/code/brewski && npx vitest run tests/AppShell.test.ts 2>&1 | tail -30
```

Expected: 2 new tests FAIL ("Recipes button navigates to / when already on a recipe page", "Batches button navigates to /batches when already on a batch page"). The updated "Recipes button navigates to last_route_recipes when set" should also fail until the implementation is in place.

- [ ] **Step 3: Implement the new rail logic**

In `src/lib/desktop/AppShell.svelte`, change lines 58–70:

From:
```svelte
    <button onclick={() => goto($settings.last_route_recipes ?? "/")}
            class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors"
            aria-label="Recipes"
            style={isRecipes ? "background: color-mix(in srgb, var(--color-accent) 18%, transparent);" : ""}>
      <BrewingIcon name="recipes" size={22} />
    </button>

    <button onclick={() => goto($settings.last_route_batches ?? "/batches")}
            class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors"
            aria-label="Batches"
            style={isBatches ? "background: color-mix(in srgb, var(--color-accent) 18%, transparent);" : ""}>
      <BrewingIcon name="batches" size={22} />
    </button>
```

To:
```svelte
    <button onclick={() => goto(isRecipes ? "/" : ($settings.last_route_recipes ?? "/"))}
            class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors"
            aria-label="Recipes"
            style={isRecipes ? "background: color-mix(in srgb, var(--color-accent) 18%, transparent);" : ""}>
      <BrewingIcon name="recipes" size={22} />
    </button>

    <button onclick={() => goto(isBatches ? "/batches" : ($settings.last_route_batches ?? "/batches"))}
            class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors"
            aria-label="Batches"
            style={isBatches ? "background: color-mix(in srgb, var(--color-accent) 18%, transparent);" : ""}>
      <BrewingIcon name="batches" size={22} />
    </button>
```

- [ ] **Step 4: Run tests to confirm all pass**

```bash
cd /Users/shead/Documents/code/brewski && npx vitest run tests/AppShell.test.ts 2>&1 | tail -20
```

Expected: all tests PASS.

- [ ] **Step 5: Commit**

```bash
git add tests/AppShell.test.ts src/lib/desktop/AppShell.svelte
git commit -m "feat: Recipes/Batches rail goes to list home when already in section"
```

---

### Task 2: RecipeView and BaselineRecipeView — remove RecipeList sidebar

Both files render `<RecipeList selectedId={id} />` just before the main content. Remove those elements and the associated imports.

**Files:**
- Modify: `src/lib/desktop/RecipeView.svelte:26,281`
- Modify: `src/lib/desktop/BaselineRecipeView.svelte:7,65`

No new tests needed — the existing `RecipeViewVersionNav.test.ts` suite will catch regressions.

---

- [ ] **Step 1: Remove RecipeList from RecipeView**

In `src/lib/desktop/RecipeView.svelte`:

Remove line 26 (the import):
```ts
  import RecipeList from "$lib/components/RecipeList.svelte";
```

Remove line 281 (the component usage — it's between the `</script>` block and the `{#if recipe}` block):
```svelte
<RecipeList selectedId={id} />
```

- [ ] **Step 2: Remove RecipeList from BaselineRecipeView**

In `src/lib/desktop/BaselineRecipeView.svelte`:

Remove line 7 (the import):
```ts
  import RecipeList from "$lib/components/RecipeList.svelte";
```

Remove line 65 (the component usage — it's between `</script>` and `{#if recipe}`):
```svelte
<RecipeList selectedId={id} />
```

- [ ] **Step 3: Run tests**

```bash
cd /Users/shead/Documents/code/brewski && npx vitest run tests/RecipeViewVersionNav.test.ts 2>&1 | tail -20
```

Expected: all tests PASS.

- [ ] **Step 4: Commit**

```bash
git add src/lib/desktop/RecipeView.svelte src/lib/desktop/BaselineRecipeView.svelte
git commit -m "feat: remove recipe list sidebar from recipe detail views"
```

---

### Task 3: BatchView — remove aside, BatchList, and related store usage

The `<aside>` block in BatchView contains the batch list sidebar with a "+ New Batch" button. Remove it entirely. The `BatchList` import, `batchList` store, and `refreshBatchList` function are only used by this sidebar — remove them too.

`refreshBatchList()` is called in two places: `onMount` (line 41) and `handleUpdate` (line 52). Both must be removed.

**Files:**
- Modify: `src/lib/desktop/BatchView.svelte`

---

- [ ] **Step 1: Remove BatchList and batches store imports**

In `src/lib/desktop/BatchView.svelte`, remove lines 9–10:
```ts
  import { batchList, refreshBatchList } from "$lib/stores/batches";
  import BatchList from "$lib/components/BatchList.svelte";
```

- [ ] **Step 2: Remove refreshBatchList call from onMount**

Change `onMount` (lines 40–43) from:
```ts
  onMount(async () => {
    await refreshBatchList();
    await loadBatch();
  });
```
To:
```ts
  onMount(async () => {
    await loadBatch();
  });
```

- [ ] **Step 3: Remove refreshBatchList call from handleUpdate**

Change `handleUpdate` (lines 49–53) from:
```ts
  async function handleUpdate(input: UpdateBatchInput) {
    if (!batch) return;
    batch = await ipc(updateBatch(batch.id, input)) ?? batch;
    await refreshBatchList();
  }
```
To:
```ts
  async function handleUpdate(input: UpdateBatchInput) {
    if (!batch) return;
    batch = await ipc(updateBatch(batch.id, input)) ?? batch;
  }
```

- [ ] **Step 4: Remove the aside block**

Remove lines 58–70 (the entire `<aside>` element):
```svelte
<aside class="w-56 flex flex-col flex-shrink-0 border-r overflow-hidden bg-bg-surface border-border"
      >
  <div class="p-2 border-b border-border">
    <button
      onclick={() => goto("/batches")}
      class="w-full px-2 py-1.5 rounded text-sm text-left bg-accent"
      style="color: #fff;"
    >+ New Batch</button>
  </div>
  <div class="flex-1 overflow-y-auto">
    <BatchList batches={$batchList} onRefresh={async () => { await ipc(refreshBatchList()); }} />
  </div>
</aside>
```

- [ ] **Step 5: Run full test suite**

```bash
cd /Users/shead/Documents/code/brewski && npx vitest run 2>&1 | tail -30
```

Expected: all tests PASS.

- [ ] **Step 6: Commit**

```bash
git add src/lib/desktop/BatchView.svelte
git commit -m "feat: remove batch list sidebar from batch detail view"
```

---

### Task 4: Push

- [ ] **Step 1: Push to remote**

```bash
git push
```
