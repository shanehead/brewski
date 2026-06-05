# Batch Version Picker Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** When creating a new batch from the "New Batch" modal, show a version picker if the selected recipe has two or more saved versions.

**Architecture:** Add an optional `version_id` field to `CreateBatchInput` in the OpenAPI schema, regenerate Rust and TS types, update `BatchRepository::create` to use an explicit version when provided, and extend both `desktop/BatchesHome.svelte` and `mobile/BatchesHome.svelte` with a two-step drill-down modal (recipe → version).

**Tech Stack:** Rust/SeaORM (backend), Svelte 5 (frontend), Vitest + Testing Library (frontend tests), `just` for codegen

---

## File Map

| File | Change |
|------|--------|
| `docs/openapi/components/schemas/CreateBatchInput.yaml` | Add `version_id` optional field |
| `src-tauri/src/models.gen.rs` | Regenerated — do not edit by hand |
| `src-tauri/src/repositories/batches.rs` | Update `create` to use explicit version when `version_id` is set |
| `src/lib/api.gen.ts` | Regenerated — do not edit by hand |
| `src/lib/desktop/BatchesHome.svelte` | Add step state + version picker rendering |
| `src/lib/mobile/BatchesHome.svelte` | Add step state + version picker rendering (same logic, mobile styles) |
| `tests/BatchesHome.test.ts` | New — tests for version picker behavior |

---

## Task 1: Add `version_id` to OpenAPI schema and regenerate types

**Files:**
- Modify: `docs/openapi/components/schemas/CreateBatchInput.yaml`
- Modify (regenerated): `src-tauri/src/models.gen.rs`
- Modify (regenerated): `src/lib/api.gen.ts`

- [ ] **Step 1.1: Add `version_id` to the schema**

Replace the contents of `docs/openapi/components/schemas/CreateBatchInput.yaml` with:

```yaml
type: object
required:
  - recipe_id
properties:
  recipe_id:
    type: string
  name:
    type: [string, "null"]
  version_id:
    type: [string, "null"]
```

- [ ] **Step 1.2: Regenerate Rust and TypeScript types**

```bash
just gen
```

Expected: no errors; `src-tauri/src/models.gen.rs` and `src/lib/api.gen.ts` are updated.

Verify `models.gen.rs` now has `version_id` in `CreateBatchInput`:

```bash
grep -A5 "pub struct CreateBatchInput" src-tauri/src/models.gen.rs
```

Expected output includes:
```
pub version_id: ::std::option::Option<::std::string::String>,
```

Verify `api.gen.ts` now has `version_id` in `CreateBatchInput`:

```bash
grep -A6 "CreateBatchInput:" src/lib/api.gen.ts
```

Expected output includes `version_id?: string | null`.

- [ ] **Step 1.3: Commit**

```bash
git add docs/openapi/components/schemas/CreateBatchInput.yaml src-tauri/src/models.gen.rs src/lib/api.gen.ts
git commit -m "feat: add version_id to CreateBatchInput schema and regenerate types"
```

---

## Task 2: Write failing Rust test for explicit `version_id` in batch creation

**Files:**
- Modify: `src-tauri/src/repositories/batches.rs` (test block only)

- [ ] **Step 2.1: Add the failing test**

In `src-tauri/src/repositories/batches.rs`, inside the existing `#[cfg(test)]` block, add this test after the existing `test_create_and_get` test:

```rust
#[tokio::test]
async fn test_create_with_explicit_version_id() {
    let db = setup_test_db().await;
    // setup() creates v1 via create_or_reuse
    let (recipe_id, v1_id) = setup(&db).await;

    // Save v2 — without the fix, create_or_reuse will pick v2 (the latest)
    let _v2 = RecipeVersionRepository::new(&db)
        .save_named(&recipe_id, "v2")
        .await
        .unwrap();

    let repo = BatchRepository::new(&db);
    // Explicitly request v1 (the older version) — this must bypass create_or_reuse
    let batch = repo
        .create(CreateBatchInput {
            recipe_id: recipe_id.clone(),
            name: None,
            version_id: Some(v1_id.clone()),
        })
        .await
        .unwrap();

    assert_eq!(batch.recipe_version_id, v1_id);
}
```

- [ ] **Step 2.2: Run test to verify it fails**

```bash
cd src-tauri && cargo test test_create_with_explicit_version_id 2>&1 | tail -20
```

Expected: test compiles (Task 1 added `version_id` to the generated struct) but the assertion fails — without the fix `create_or_reuse` picks `v2`, not `v1`.

---

## Task 3: Update `BatchRepository::create` to handle `version_id`

**Files:**
- Modify: `src-tauri/src/repositories/batches.rs`

- [ ] **Step 3.1: Add `recipe_versions` entity import**

In `src-tauri/src/repositories/batches.rs`, update the entities import line from:

```rust
use crate::entities::{batch_gravity_readings, batches, recipes};
```

to:

```rust
use crate::entities::{batch_gravity_readings, batches, recipe_versions, recipes};
```

- [ ] **Step 3.2: Update the `create` method body**

In `src-tauri/src/repositories/batches.rs`, replace the `create` method body. Find this block (the beginning of the `create` method):

```rust
    pub async fn create(&self, input: CreateBatchInput) -> Result<Batch, AppError> {
        let version = RecipeVersionRepository::new(self.db)
            .create_or_reuse(&input.recipe_id)
            .await?;
```

Replace with:

```rust
    pub async fn create(&self, input: CreateBatchInput) -> Result<Batch, AppError> {
        let version_id = if let Some(vid) = input.version_id {
            recipe_versions::Entity::find_by_id(&vid)
                .one(self.db)
                .await?
                .ok_or(AppError::NotFound)?
                .id
        } else {
            RecipeVersionRepository::new(self.db)
                .create_or_reuse(&input.recipe_id)
                .await?
                .id
        };
```

Then find the line inside the same method that reads:

```rust
            recipe_version_id: Set(version.id),
```

Replace with:

```rust
            recipe_version_id: Set(version_id),
```

- [ ] **Step 3.3: Run the new test to verify it passes**

```bash
cd src-tauri && cargo test test_create_with_explicit_version_id -- --nocapture
```

Expected: `test test_create_with_explicit_version_id ... ok`

- [ ] **Step 3.4: Run the full Rust test suite**

```bash
cd src-tauri && cargo test
```

Expected: all tests pass.

- [ ] **Step 3.5: Commit**

```bash
git add src-tauri/src/repositories/batches.rs
git commit -m "feat: support explicit version_id in batch creation"
```

---

## Task 4: Write failing frontend tests for version picker

**Files:**
- Create: `tests/BatchesHome.test.ts`

- [ ] **Step 4.1: Create the test file**

Create `tests/BatchesHome.test.ts` with these contents:

```typescript
// tests/BatchesHome.test.ts
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, waitFor } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import DesktopBatchesHome from "../src/lib/desktop/BatchesHome.svelte";
import MobileBatchesHome from "../src/lib/mobile/BatchesHome.svelte";
import type { RecipeSummary, RecipeVersionSummary } from "$lib/api";

vi.mock("$app/navigation", () => ({ goto: vi.fn() }));
vi.mock("$lib/stores/error", () => ({ ipc: vi.fn((p) => p) }));

const mockRefreshBatchList = vi.fn().mockResolvedValue(undefined);
vi.mock("$lib/stores/batches", () => ({
  batchList: { subscribe: vi.fn((fn) => { fn([]); return () => {}; }) },
  refreshBatchList: mockRefreshBatchList,
}));

const mockListRecipes = vi.fn();
const mockListRecipeVersions = vi.fn();
const mockCreateBatch = vi.fn();

vi.mock("$lib/api", () => ({
  listRecipes: mockListRecipes,
  listRecipeVersions: mockListRecipeVersions,
  createBatch: mockCreateBatch,
}));

function makeRecipe(overrides: Partial<RecipeSummary> = {}): RecipeSummary {
  return {
    id: "r1",
    name: "Pliny the Elder",
    type_: "All Grain",
    batch_size_l: 19,
    style_name: null,
    image_path: null,
    created_at: 0,
    updated_at: 0,
    source: "user",
    ...overrides,
  } as RecipeSummary;
}

function makeVersion(overrides: Partial<RecipeVersionSummary> = {}): RecipeVersionSummary {
  return {
    id: "ver1",
    recipe_id: "r1",
    version_number: 1,
    name: null,
    parent_version_id: null,
    created_at: 1700000000,
    ...overrides,
  };
}

beforeEach(() => {
  mockListRecipes.mockReset();
  mockListRecipeVersions.mockReset();
  mockCreateBatch.mockReset();
  mockRefreshBatchList.mockClear();
});

describe.each([
  { label: "desktop", Component: DesktopBatchesHome },
  { label: "mobile", Component: MobileBatchesHome },
])("BatchesHome ($label) — version picker", ({ Component }) => {
  it("creates batch immediately when recipe has only one version", async () => {
    const user = userEvent.setup();
    const recipe = makeRecipe();
    mockListRecipes.mockResolvedValue([recipe]);
    mockListRecipeVersions.mockResolvedValue([makeVersion()]);
    mockCreateBatch.mockResolvedValue({ id: "b1" });

    render(Component);

    await user.click(screen.getByRole("button", { name: /\+ New Batch/i }));
    await waitFor(() => screen.getByText("Pliny the Elder"));
    await user.click(screen.getByText("Pliny the Elder"));

    await waitFor(() => expect(mockCreateBatch).toHaveBeenCalledWith(
      expect.objectContaining({ recipe_id: "r1" })
    ));
    expect(mockCreateBatch).toHaveBeenCalledWith(
      expect.not.objectContaining({ version_id: expect.anything() })
    );
    expect(screen.queryByText(/Choose a version/i)).not.toBeInTheDocument();
  });

  it("shows version picker when recipe has two or more versions", async () => {
    const user = userEvent.setup();
    const recipe = makeRecipe();
    mockListRecipes.mockResolvedValue([recipe]);
    mockListRecipeVersions.mockResolvedValue([
      makeVersion({ id: "ver2", version_number: 2, name: "Added Citra dry hop", created_at: 1710000000 }),
      makeVersion({ id: "ver1", version_number: 1, name: null, created_at: 1700000000 }),
    ]);

    render(Component);

    await user.click(screen.getByRole("button", { name: /\+ New Batch/i }));
    await waitFor(() => screen.getByText("Pliny the Elder"));
    await user.click(screen.getByText("Pliny the Elder"));

    await waitFor(() => expect(screen.getByText(/v2/i)).toBeInTheDocument());
    expect(screen.getByText(/v1/i)).toBeInTheDocument();
    expect(mockCreateBatch).not.toHaveBeenCalled();
  });

  it("creates batch with explicit version_id when version is selected", async () => {
    const user = userEvent.setup();
    const recipe = makeRecipe();
    mockListRecipes.mockResolvedValue([recipe]);
    mockListRecipeVersions.mockResolvedValue([
      makeVersion({ id: "ver2", version_number: 2, name: "Added Citra dry hop", created_at: 1710000000 }),
      makeVersion({ id: "ver1", version_number: 1, name: null, created_at: 1700000000 }),
    ]);
    mockCreateBatch.mockResolvedValue({ id: "b1" });

    render(Component);

    await user.click(screen.getByRole("button", { name: /\+ New Batch/i }));
    await waitFor(() => screen.getByText("Pliny the Elder"));
    await user.click(screen.getByText("Pliny the Elder"));

    await waitFor(() => screen.getByText(/v2/i));
    await user.click(screen.getByText(/v1/i));

    await waitFor(() => expect(mockCreateBatch).toHaveBeenCalledWith(
      expect.objectContaining({ recipe_id: "r1", version_id: "ver1", name: null })
    ));
  });

  it("back link in version picker returns to recipe list", async () => {
    const user = userEvent.setup();
    const recipe = makeRecipe();
    mockListRecipes.mockResolvedValue([recipe]);
    mockListRecipeVersions.mockResolvedValue([
      makeVersion({ id: "ver2", version_number: 2, created_at: 1710000000 }),
      makeVersion({ id: "ver1", version_number: 1, created_at: 1700000000 }),
    ]);

    render(Component);

    await user.click(screen.getByRole("button", { name: /\+ New Batch/i }));
    await waitFor(() => screen.getByText("Pliny the Elder"));
    await user.click(screen.getByText("Pliny the Elder"));

    await waitFor(() => screen.getByText(/v2/i));
    await user.click(screen.getByRole("button", { name: /Pliny the Elder/i }));

    await waitFor(() => screen.getByText("Pliny the Elder"));
    expect(screen.queryByText(/v2/i)).not.toBeInTheDocument();
  });
});
```

- [ ] **Step 4.2: Run the tests to verify they fail**

```bash
bun run test tests/BatchesHome.test.ts
```

Expected: tests fail (components don't have version picker behavior yet).

---

## Task 5: Update `desktop/BatchesHome.svelte` with version picker

**Files:**
- Modify: `src/lib/desktop/BatchesHome.svelte`

- [ ] **Step 5.1: Replace the component**

Replace the full contents of `src/lib/desktop/BatchesHome.svelte` with:

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import BatchList from "$lib/components/BatchList.svelte";
  import { batchList, refreshBatchList } from "$lib/stores/batches";
  import { createBatch, listRecipes, listRecipeVersions } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import type { RecipeSummary, RecipeVersionSummary } from "$lib/api";

  let showPicker = $state(false);
  let step = $state<"recipe" | "version">("recipe");
  let recipes = $state<RecipeSummary[]>([]);
  let pickedRecipe = $state<RecipeSummary | null>(null);
  let versions = $state<RecipeVersionSummary[]>([]);

  onMount(() => ipc(refreshBatchList()));

  async function handleNew() {
    recipes = (await ipc(listRecipes())) ?? [];
    step = "recipe";
    pickedRecipe = null;
    versions = [];
    showPicker = true;
  }

  async function handlePickRecipe(recipe: RecipeSummary) {
    const vers = (await ipc(listRecipeVersions(recipe.id))) ?? [];
    if (vers.length >= 2) {
      pickedRecipe = recipe;
      versions = vers;
      step = "version";
    } else {
      showPicker = false;
      const batch = await ipc(createBatch({ recipe_id: recipe.id, name: null }));
      if (!batch) return;
      await ipc(refreshBatchList());
      goto(`/batches/${batch.id}`);
    }
  }

  async function handlePickVersion(version: RecipeVersionSummary) {
    showPicker = false;
    const batch = await ipc(
      createBatch({ recipe_id: pickedRecipe!.id, version_id: version.id, name: null })
    );
    if (!batch) return;
    await ipc(refreshBatchList());
    goto(`/batches/${batch.id}`);
  }

  function handleBack() {
    step = "recipe";
    pickedRecipe = null;
    versions = [];
  }

  function formatDate(ts: number): string {
    return new Date(ts * 1000).toLocaleDateString(undefined, {
      month: "short",
      day: "numeric",
      year: "numeric",
    });
  }
</script>

<aside class="w-56 flex flex-col flex-shrink-0 border-r overflow-hidden"
       style="background: var(--color-bg-surface); border-color: var(--color-border);">
  <div class="p-2 border-b" style="border-color: var(--color-border);">
    <button
      onclick={handleNew}
      class="w-full px-2 py-1.5 rounded text-sm text-left"
      style="background: var(--color-accent); color: #fff;"
    >+ New Batch</button>
  </div>
  <div class="flex-1 overflow-y-auto">
    <BatchList batches={$batchList} onRefresh={() => ipc(refreshBatchList())} />
  </div>
</aside>

<div class="flex-1 flex items-center justify-center" style="color: var(--color-text-muted);">
  <p class="text-sm">Select a batch to view</p>
</div>

{#if showPicker}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center"
    style="background: rgba(0,0,0,0.5);"
    role="dialog"
    aria-modal="true"
  >
    <div class="rounded-lg p-4 w-80 max-h-96 flex flex-col gap-2 overflow-hidden"
         style="background: var(--color-bg-surface); border: 1px solid var(--color-border);">
      {#if step === "recipe"}
        <div class="font-medium text-sm">Choose a recipe to brew</div>
        <div class="flex-1 overflow-y-auto flex flex-col gap-1">
          {#each recipes as r (r.id)}
            <button
              onclick={() => handlePickRecipe(r)}
              class="text-left px-3 py-2 rounded text-sm hover:opacity-80"
              style="background: var(--color-bg-elevated); color: var(--color-text-primary);"
            >{r.name}</button>
          {/each}
        </div>
        <button onclick={() => showPicker = false}
          class="text-xs" style="color: var(--color-text-muted);">Cancel</button>
      {:else}
        <button
          onclick={handleBack}
          class="text-xs text-left font-medium"
          style="color: var(--color-accent);"
          aria-label={pickedRecipe?.name}
        >← {pickedRecipe?.name}</button>
        <div class="font-medium text-sm">Choose a version</div>
        <div class="flex-1 overflow-y-auto flex flex-col gap-1">
          {#each versions as v, i (v.id)}
            <button
              onclick={() => handlePickVersion(v)}
              class="text-left px-3 py-2 rounded text-sm hover:opacity-80"
              style="background: {i === 0 ? 'var(--color-accent)' : 'var(--color-bg-elevated)'}; color: {i === 0 ? '#fff' : 'var(--color-text-primary)'};"
            >
              <span class="font-mono">v{v.version_number}</span>
              {#if v.name}<span class="ml-1">· {v.name}</span>{/if}
              <span class="ml-1 text-xs opacity-60">{formatDate(v.created_at)}</span>
            </button>
          {/each}
        </div>
        <button onclick={() => showPicker = false}
          class="text-xs" style="color: var(--color-text-muted);">Cancel</button>
      {/if}
    </div>
  </div>
{/if}
```

- [ ] **Step 5.2: Run the frontend tests**

```bash
bun run test tests/BatchesHome.test.ts
```

Expected: desktop tests pass, mobile tests still fail.

---

## Task 6: Update `mobile/BatchesHome.svelte` with version picker

**Files:**
- Modify: `src/lib/mobile/BatchesHome.svelte`

- [ ] **Step 6.1: Replace the component**

Replace the full contents of `src/lib/mobile/BatchesHome.svelte` with:

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { batchList, refreshBatchList } from "$lib/stores/batches";
  import { createBatch, listRecipes, listRecipeVersions } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import type { RecipeSummary, RecipeVersionSummary } from "$lib/api";

  let showPicker = $state(false);
  let step = $state<"recipe" | "version">("recipe");
  let recipes = $state<RecipeSummary[]>([]);
  let pickedRecipe = $state<RecipeSummary | null>(null);
  let versions = $state<RecipeVersionSummary[]>([]);

  onMount(() => ipc(refreshBatchList()));

  async function handleNew() {
    recipes = (await ipc(listRecipes())) ?? [];
    step = "recipe";
    pickedRecipe = null;
    versions = [];
    showPicker = true;
  }

  async function handlePickRecipe(recipe: RecipeSummary) {
    const vers = (await ipc(listRecipeVersions(recipe.id))) ?? [];
    if (vers.length >= 2) {
      pickedRecipe = recipe;
      versions = vers;
      step = "version";
    } else {
      showPicker = false;
      const batch = await ipc(createBatch({ recipe_id: recipe.id, name: null }));
      if (!batch) return;
      await ipc(refreshBatchList());
      goto(`/batches/${batch.id}`);
    }
  }

  async function handlePickVersion(version: RecipeVersionSummary) {
    showPicker = false;
    const batch = await ipc(
      createBatch({ recipe_id: pickedRecipe!.id, version_id: version.id, name: null })
    );
    if (!batch) return;
    await ipc(refreshBatchList());
    goto(`/batches/${batch.id}`);
  }

  function handleBack() {
    step = "recipe";
    pickedRecipe = null;
    versions = [];
  }

  function formatDate(ts: number): string {
    return new Date(ts * 1000).toLocaleDateString(undefined, {
      month: "short",
      day: "numeric",
      year: "numeric",
    });
  }
</script>

<div class="flex flex-col h-full overflow-hidden" style="background: var(--color-bg-surface);">
  <div class="p-3 border-b" style="border-color: var(--color-border);">
    <button
      onclick={handleNew}
      class="w-full py-3 rounded text-sm font-medium"
      style="background: var(--color-accent); color: #fff;"
    >+ New Batch</button>
  </div>
  <div class="flex-1 overflow-y-auto">
    {#each $batchList as batch (batch.id)}
      <a
        href="/batches/{batch.id}"
        class="flex items-center justify-between px-4 py-3 border-b text-sm"
        style="border-color: var(--color-border); color: var(--color-text-primary);"
      >
        <div class="flex flex-col gap-0.5 min-w-0">
          <span class="truncate font-medium">{batch.recipe_name}</span>
          <span class="text-xs truncate" style="color: var(--color-text-muted);">
            {batch.name ?? "Batch"} · {batch.status}
          </span>
        </div>
        <span style="color: var(--color-text-muted);">›</span>
      </a>
    {:else}
      <p class="p-4 text-sm" style="color: var(--color-text-muted);">No batches yet. Tap + to start one.</p>
    {/each}
  </div>
</div>

{#if showPicker}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center"
    style="background: rgba(0,0,0,0.5);"
    role="dialog"
    aria-modal="true"
  >
    <div class="rounded-lg p-4 mx-4 max-h-96 flex flex-col gap-2 overflow-hidden"
         style="background: var(--color-bg-surface); border: 1px solid var(--color-border); width: calc(100% - 2rem);">
      {#if step === "recipe"}
        <div class="font-medium text-sm">Choose a recipe to brew</div>
        <div class="flex-1 overflow-y-auto flex flex-col gap-1">
          {#each recipes as r (r.id)}
            <button
              onclick={() => handlePickRecipe(r)}
              class="text-left px-3 py-3 rounded text-sm"
              style="background: var(--color-bg-elevated); color: var(--color-text-primary);"
            >{r.name}</button>
          {/each}
        </div>
        <button onclick={() => showPicker = false}
          class="text-xs py-2" style="color: var(--color-text-muted);">Cancel</button>
      {:else}
        <button
          onclick={handleBack}
          class="text-xs text-left font-medium py-1"
          style="color: var(--color-accent);"
          aria-label={pickedRecipe?.name}
        >← {pickedRecipe?.name}</button>
        <div class="font-medium text-sm">Choose a version</div>
        <div class="flex-1 overflow-y-auto flex flex-col gap-1">
          {#each versions as v, i (v.id)}
            <button
              onclick={() => handlePickVersion(v)}
              class="text-left px-3 py-3 rounded text-sm"
              style="background: {i === 0 ? 'var(--color-accent)' : 'var(--color-bg-elevated)'}; color: {i === 0 ? '#fff' : 'var(--color-text-primary)'};"
            >
              <span class="font-mono">v{v.version_number}</span>
              {#if v.name}<span class="ml-1">· {v.name}</span>{/if}
              <span class="ml-1 text-xs opacity-60">{formatDate(v.created_at)}</span>
            </button>
          {/each}
        </div>
        <button onclick={() => showPicker = false}
          class="text-xs py-2" style="color: var(--color-text-muted);">Cancel</button>
      {/if}
    </div>
  </div>
{/if}
```

- [ ] **Step 6.2: Run all frontend tests**

```bash
bun run test
```

Expected: all tests pass.

- [ ] **Step 6.3: Run Rust tests to confirm nothing regressed**

```bash
cd src-tauri && cargo test
```

Expected: all tests pass.

- [ ] **Step 6.4: Commit**

```bash
git add src/lib/desktop/BatchesHome.svelte src/lib/mobile/BatchesHome.svelte tests/BatchesHome.test.ts
git commit -m "feat: version picker in New Batch modal — drill-down when recipe has 2+ versions"
```
