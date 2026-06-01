# BeerXML Import Feedback Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a loading state to the Import BeerXML button and a success toast that reports how many recipes were imported.

**Architecture:** Extend `src/lib/stores/error.ts` with a `lastSuccess` store and `setSuccess` helper (auto-clears after 3 s). Both AppShells render a green success banner alongside the existing error banner. The two import handlers (`RecipeList.svelte` and `mobile/RecipesHome.svelte`) track an `importing` boolean and call `setSuccess` on completion.

**Tech Stack:** SvelteKit 5 (Svelte 5 runes), Svelte stores, Vitest + Testing Library

---

### Task 1: Extend error store with `lastSuccess` + `setSuccess`

**Files:**
- Modify: `src/lib/stores/error.ts`
- Create: `tests/error-store.test.ts`

- [ ] **Step 1: Write the failing tests**

Create `tests/error-store.test.ts`:

```ts
import { afterEach, describe, expect, it, vi } from "vitest";
import { get } from "svelte/store";
import { lastSuccess, setSuccess } from "$lib/stores/error";

describe("setSuccess", () => {
  afterEach(() => {
    vi.useRealTimers();
    lastSuccess.set(null);
  });

  it("sets lastSuccess to the provided message", () => {
    setSuccess("2 recipes imported");
    expect(get(lastSuccess)).toBe("2 recipes imported");
  });

  it("auto-clears lastSuccess after 3 seconds", () => {
    vi.useFakeTimers();
    setSuccess("1 recipe imported");
    expect(get(lastSuccess)).toBe("1 recipe imported");
    vi.advanceTimersByTime(3000);
    expect(get(lastSuccess)).toBeNull();
  });

  it("does not clear before 3 seconds", () => {
    vi.useFakeTimers();
    setSuccess("1 recipe imported");
    vi.advanceTimersByTime(2999);
    expect(get(lastSuccess)).toBe("1 recipe imported");
  });
});
```

- [ ] **Step 2: Run tests to confirm they fail**

```bash
npx vitest run tests/error-store.test.ts
```

Expected: FAIL — `lastSuccess` and `setSuccess` are not exported from the module.

- [ ] **Step 3: Implement `lastSuccess` and `setSuccess` in `src/lib/stores/error.ts`**

Add after the existing `lastError` line:

```ts
export const lastSuccess = writable<string | null>(null);

export function setSuccess(message: string) {
  lastSuccess.set(message);
  setTimeout(() => lastSuccess.set(null), 3000);
}
```

The full file becomes:

```ts
import { writable } from "svelte/store";

export const lastError = writable<string | null>(null);

export const lastSuccess = writable<string | null>(null);

export function setSuccess(message: string) {
  lastSuccess.set(message);
  setTimeout(() => lastSuccess.set(null), 3000);
}

/** Await an IPC promise, routing any rejection to the error toast. Returns undefined on failure. */
export async function ipc<T>(promise: Promise<T>): Promise<T | undefined> {
  try {
    return await promise;
  } catch (e) {
    lastError.set(String(e));
    return undefined;
  }
}
```

- [ ] **Step 4: Run tests to confirm they pass**

```bash
npx vitest run tests/error-store.test.ts
```

Expected: 3 passed.

- [ ] **Step 5: Commit**

```bash
git add src/lib/stores/error.ts tests/error-store.test.ts
git commit -m "feat: add lastSuccess store and setSuccess helper to error store"
```

---

### Task 2: Show success banner in both AppShells

**Files:**
- Modify: `src/lib/desktop/AppShell.svelte`
- Modify: `src/lib/mobile/AppShell.svelte`
- Modify: `tests/AppShell.test.ts`
- Modify: `tests/MobileAppShell.test.ts`

- [ ] **Step 1: Add success banner tests to `tests/AppShell.test.ts`**

In the `vi.mock("$lib/stores/error", ...)` call, replace:

```ts
vi.mock("$lib/stores/error", () => ({
  lastError: { subscribe: vi.fn((fn) => { fn(null); return () => {}; }) },
}));
```

with:

```ts
let mockLastError: string | null = null;
let mockLastSuccess: string | null = null;

vi.mock("$lib/stores/error", () => ({
  lastError: {
    subscribe: vi.fn((fn) => { fn(mockLastError); return () => {}; }),
    set: vi.fn(),
  },
  lastSuccess: {
    subscribe: vi.fn((fn) => { fn(mockLastSuccess); return () => {}; }),
    set: vi.fn(),
  },
}));
```

Add `mockLastError = null; mockLastSuccess = null;` to the `beforeEach` block.

Then add a new describe block at the bottom of the file:

```ts
describe("AppShell success banner", () => {
  it("shows success banner when lastSuccess has a message", () => {
    mockLastSuccess = "2 recipes imported";
    const { getByText } = render(AppShell, { children: () => null });
    expect(getByText("2 recipes imported")).toBeTruthy();
  });

  it("does not show success banner when lastSuccess is null", () => {
    mockLastSuccess = null;
    const { queryByText } = render(AppShell, { children: () => null });
    expect(queryByText(/recipes imported/)).toBeNull();
  });
});
```

- [ ] **Step 2: Add success banner tests to `tests/MobileAppShell.test.ts`**

In the `vi.mock("$lib/stores/error", ...)` call, replace:

```ts
vi.mock("$lib/stores/error", () => ({
  lastError: { subscribe: vi.fn((fn) => { fn(null); return () => {}; }) },
}));
```

with:

```ts
let mockLastError: string | null = null;
let mockLastSuccess: string | null = null;

vi.mock("$lib/stores/error", () => ({
  lastError: {
    subscribe: vi.fn((fn) => { fn(mockLastError); return () => {}; }),
    set: vi.fn(),
  },
  lastSuccess: {
    subscribe: vi.fn((fn) => { fn(mockLastSuccess); return () => {}; }),
    set: vi.fn(),
  },
}));
```

Add `mockLastError = null; mockLastSuccess = null;` to the `beforeEach` block.

Add at the bottom:

```ts
describe("MobileAppShell success banner", () => {
  it("shows success banner when lastSuccess has a message", () => {
    mockLastSuccess = "1 recipe imported";
    const { getByText } = render(MobileAppShell, { children: () => null });
    expect(getByText("1 recipe imported")).toBeTruthy();
  });

  it("does not show success banner when lastSuccess is null", () => {
    mockLastSuccess = null;
    const { queryByText } = render(MobileAppShell, { children: () => null });
    expect(queryByText(/recipe imported/)).toBeNull();
  });
});
```

- [ ] **Step 3: Run tests to confirm they fail**

```bash
npx vitest run tests/AppShell.test.ts tests/MobileAppShell.test.ts
```

Expected: new success banner tests FAIL — the AppShells don't yet render the success store.

- [ ] **Step 4: Update `src/lib/desktop/AppShell.svelte`**

In the `<script>` block, change the import:

```ts
import { lastError } from "$lib/stores/error";
```

to:

```ts
import { lastError, lastSuccess } from "$lib/stores/error";
```

After the existing error banner block (after the closing `{/if}`), add:

```svelte
{#if $lastSuccess}
  <div class="fixed bottom-4 left-1/2 -translate-x-1/2 z-50 flex items-center gap-3 px-4 py-2.5 rounded shadow-lg text-sm"
       style="background: #14532d; color: #bbf7d0; max-width: 480px;">
    <span class="flex-1 truncate">{$lastSuccess}</span>
    <button onclick={() => lastSuccess.set(null)} class="opacity-70 hover:opacity-100 flex-shrink-0">✕</button>
  </div>
{/if}
```

- [ ] **Step 5: Update `src/lib/mobile/AppShell.svelte`**

In the `<script>` block, change the import:

```ts
import { lastError } from "$lib/stores/error";
```

to:

```ts
import { lastError, lastSuccess } from "$lib/stores/error";
```

After the existing error banner block, add:

```svelte
{#if $lastSuccess}
  <div class="fixed bottom-20 left-4 right-4 z-50 flex items-center gap-3 px-4 py-2.5 rounded shadow-lg text-sm"
       style="background: #14532d; color: #bbf7d0;">
    <span class="flex-1">{$lastSuccess}</span>
    <button onclick={() => lastSuccess.set(null)} class="opacity-70 flex-shrink-0">✕</button>
  </div>
{/if}
```

- [ ] **Step 6: Run tests to confirm they pass**

```bash
npx vitest run tests/AppShell.test.ts tests/MobileAppShell.test.ts
```

Expected: all tests pass.

- [ ] **Step 7: Commit**

```bash
git add src/lib/desktop/AppShell.svelte src/lib/mobile/AppShell.svelte tests/AppShell.test.ts tests/MobileAppShell.test.ts
git commit -m "feat: show success toast banner in desktop and mobile AppShell"
```

---

### Task 3: Add loading state + success call to import handlers

**Files:**
- Modify: `src/lib/components/RecipeList.svelte`
- Modify: `src/lib/mobile/RecipesHome.svelte`
- Modify: `tests/BeerXMLImport.test.ts`

- [ ] **Step 1: Update the mock in `tests/BeerXMLImport.test.ts` to include `setSuccess`**

Replace:

```ts
vi.mock("$lib/stores/error", () => ({
  ipc: vi.fn((p: Promise<unknown>) => p),
}));
```

with:

```ts
const setSuccessMock = vi.fn();

vi.mock("$lib/stores/error", () => ({
  ipc: vi.fn((p: Promise<unknown>) => p),
  setSuccess: setSuccessMock,
}));
```

Add `setSuccessMock.mockClear();` to the `beforeEach` (add one if it doesn't exist):

```ts
import { beforeEach, describe, it, expect, vi } from "vitest";
// ...existing imports...

beforeEach(() => {
  setSuccessMock.mockClear();
});
```

- [ ] **Step 2: Write the failing tests in `tests/BeerXMLImport.test.ts`**

Add these two tests inside `describe("RecipeList", ...)`:

```ts
import { tick } from "svelte";
import userEvent from "@testing-library/user-event";

// Add to the existing RecipeList describe block:

it("disables the Import button and shows 'Importing…' while in-flight", async () => {
  const user = userEvent.setup();
  const { createRecipesFromBeerxml } = await import("$lib/api");
  let resolve!: (val: unknown[]) => void;
  vi.mocked(createRecipesFromBeerxml).mockReturnValue(
    new Promise((r) => { resolve = r; }) as Promise<never>
  );

  const { container, getByText } = render(RecipeList);

  const xml = "<RECIPES></RECIPES>";
  const file = new File([xml], "recipe.xml", { type: "text/xml" });
  vi.spyOn(file, "text").mockResolvedValue(xml);

  const input = container.querySelector('input[type="file"]') as HTMLInputElement;
  Object.defineProperty(input, "files", { value: [file], configurable: true });
  await fireEvent.change(input);
  await tick();

  const btn = getByText("Importing…");
  expect(btn).toBeTruthy();
  expect((btn as HTMLButtonElement).disabled).toBe(true);

  resolve([]);
  await tick();
  await tick();
  expect(getByText("Import BeerXML")).toBeTruthy();
});

it("calls setSuccess with 'N recipes imported' after a successful import", async () => {
  const { createRecipesFromBeerxml } = await import("$lib/api");
  vi.mocked(createRecipesFromBeerxml).mockResolvedValue([
    { id: "r1" } as never,
    { id: "r2" } as never,
  ]);

  const { container } = render(RecipeList);

  const xml = "<RECIPES></RECIPES>";
  const file = new File([xml], "recipe.xml", { type: "text/xml" });
  vi.spyOn(file, "text").mockResolvedValue(xml);

  const input = container.querySelector('input[type="file"]') as HTMLInputElement;
  Object.defineProperty(input, "files", { value: [file], configurable: true });
  await fireEvent.change(input);
  await tick();
  await tick();

  expect(setSuccessMock).toHaveBeenCalledWith("2 recipes imported");
});

it("uses singular 'recipe' when exactly 1 is imported", async () => {
  const { createRecipesFromBeerxml } = await import("$lib/api");
  vi.mocked(createRecipesFromBeerxml).mockResolvedValue([{ id: "r1" } as never]);

  const { container } = render(RecipeList);

  const xml = "<RECIPES></RECIPES>";
  const file = new File([xml], "recipe.xml", { type: "text/xml" });
  vi.spyOn(file, "text").mockResolvedValue(xml);

  const input = container.querySelector('input[type="file"]') as HTMLInputElement;
  Object.defineProperty(input, "files", { value: [file], configurable: true });
  await fireEvent.change(input);
  await tick();
  await tick();

  expect(setSuccessMock).toHaveBeenCalledWith("1 recipe imported");
});
```

- [ ] **Step 3: Run tests to confirm they fail**

```bash
npx vitest run tests/BeerXMLImport.test.ts
```

Expected: the three new tests FAIL — no `importing` state or `setSuccess` call exists yet.

- [ ] **Step 4: Update `handleImport` in `src/lib/components/RecipeList.svelte`**

In the `<script>` block, change the `error` store import line:

```ts
import { ipc } from "$lib/stores/error";
```

to:

```ts
import { ipc, setSuccess } from "$lib/stores/error";
```

Add `let importing = $state(false);` alongside the other `$state` declarations (near `let search` and `let fileInput`).

Replace the existing `handleImport` function:

```ts
async function handleImport(event: Event) {
  const file = (event.target as HTMLInputElement).files?.[0];
  if (!file) return;
  importing = true;
  const xml = await file.text();
  const imported = await ipc(createRecipesFromBeerxml(xml));
  importing = false;
  if (!imported) return;
  setSuccess(`${imported.length} recipe${imported.length === 1 ? "" : "s"} imported`);
  await ipc(refreshRecipeList());
  fileInput.value = "";
}
```

Update the Import button element (around line 98) to reflect the loading state:

```svelte
<button
  onclick={() => fileInput.click()}
  disabled={importing}
  class="w-full py-1.5 rounded text-sm font-medium transition-colors"
  style="border: 1px solid var(--color-accent); color: var(--color-accent); background: transparent;"
>
  {importing ? "Importing…" : "Import BeerXML"}
</button>
```

- [ ] **Step 5: Update `handleImport` in `src/lib/mobile/RecipesHome.svelte`**

In the `<script>` block, change:

```ts
import { ipc } from "$lib/stores/error";
```

to:

```ts
import { ipc, setSuccess } from "$lib/stores/error";
```

Add `let importing = $state(false);` alongside the other state declarations.

Replace the existing `handleImport` function:

```ts
async function handleImport(event: Event) {
  const file = (event.target as HTMLInputElement).files?.[0];
  if (!file) return;
  importing = true;
  const xml = await file.text();
  const imported = await ipc(createRecipesFromBeerxml(xml));
  importing = false;
  if (!imported) return;
  setSuccess(`${imported.length} recipe${imported.length === 1 ? "" : "s"} imported`);
  await ipc(refreshRecipeList());
  fileInput.value = "";
}
```

Update the Import button element to reflect the loading state:

```svelte
<button
  onclick={() => fileInput.click()}
  disabled={importing}
  class="w-full py-3 rounded text-sm font-medium"
  style="border: 1px solid var(--color-accent); color: var(--color-accent); background: transparent;"
>
  {importing ? "Importing…" : "Import BeerXML"}
</button>
```

- [ ] **Step 6: Run all tests to confirm they pass**

```bash
npx vitest run tests/BeerXMLImport.test.ts
```

Expected: all tests pass (including the 3 existing + 3 new).

Then run the full suite:

```bash
npx vitest run
```

Expected: all tests pass.

- [ ] **Step 7: Commit**

```bash
git add src/lib/components/RecipeList.svelte src/lib/mobile/RecipesHome.svelte tests/BeerXMLImport.test.ts
git commit -m "feat: add loading state and success toast to BeerXML import"
```
