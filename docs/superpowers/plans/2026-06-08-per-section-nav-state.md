# Per-Section Navigation State Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Clicking a rail icon restores the exact URL (including recipe/batch tab) the user was last on in that section, persisted across app restarts.

**Architecture:** Add 6 section-specific keys to `AppSettings` (free-form KV store, no backend changes). `afterNavigate` saves the full URL (pathname + search) to both `last_route` and the matching section key. Rail icons navigate to the stored section URL. Recipe tab selection moves into the URL as `?tab=<key>` so it's captured automatically.

**Tech Stack:** SvelteKit, Svelte 5, Tauri, Vitest + Testing Library

---

### Task 1: Add section route keys to AppSettings

**Files:**
- Modify: `src/lib/stores/settings.ts`

- [ ] **Add 6 new optional keys to the `AppSettings` interface**

```typescript
export interface AppSettings {
  units?: "metric" | "imperial";
  gravity_unit?: GravityUnit;
  theme?: string;
  default_equipment_profile_id?: string;
  last_route?: string;
  last_route_recipes?: string;
  last_route_batches?: string;
  last_route_tools?: string;
  last_route_equipment?: string;
  last_route_library?: string;
  last_route_settings?: string;
  starters_collapsed?: boolean;
  hide_example_recipes?: boolean;
  show_tooltips?: boolean;
}
```

- [ ] **Run type-check to confirm no errors**

```bash
npx tsc --noEmit
```

Expected: no output (zero errors).

- [ ] **Commit**

```bash
git add src/lib/stores/settings.ts
git commit -m "feat: add per-section last-route keys to AppSettings"
```

---

### Task 2: Update desktop AppShell — afterNavigate and onMount

**Files:**
- Modify: `src/lib/desktop/AppShell.svelte`
- Modify: `tests/AppShell.test.ts`

`afterNavigate` currently saves only `to.url.pathname` to `last_route`. It needs to save `pathname + search` and also save to the matching section key. `onMount` compares `last_route` against `$page.url.pathname`; it must compare against `pathname + search` so it doesn't re-navigate when a query param is already in the current URL.

- [ ] **Update the `afterNavigate` mock in `tests/AppShell.test.ts` to include `search`**

The mock callback type on line 6 currently only has `pathname`. Update it and the test on line 122 so the simulated navigation includes `search`:

```typescript
// line 6 — update the type
let afterNavigateCb: ((nav: { to: { url: { pathname: string; search: string } } | null }) => void) | null = null;
```

```typescript
// line 122 — add search to the call
afterNavigateCb!({ to: { url: { pathname: "/library", search: "" } } });
```

- [ ] **Update the existing `saveSetting` test to expect `pathname + search`**

The test at line 120 currently checks:
```typescript
expect(saveSettingMock).toHaveBeenCalledWith("last_route", "/library");
```

Change to:
```typescript
expect(saveSettingMock).toHaveBeenCalledWith("last_route", "/library");
// pathname + search where search="" still produces "/library"
```

This test stays the same since `"/library" + ""` is still `"/library"`. No change needed here.

- [ ] **Add tests for section key saving and query param capture**

Add this describe block at the end of `tests/AppShell.test.ts`:

```typescript
describe("AppShell section key saving", () => {
  it("saves last_route_recipes when navigating to /recipe/abc", () => {
    render(AppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/recipe/abc", search: "" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_recipes", "/recipe/abc");
  });

  it("saves last_route_recipes when navigating to /", () => {
    render(AppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/", search: "" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_recipes", "/");
  });

  it("saves last_route_batches when navigating to /batches/xyz", () => {
    render(AppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/batches/xyz", search: "" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_batches", "/batches/xyz");
  });

  it("saves last_route_tools when navigating to /tools/carbonation", () => {
    render(AppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/tools/carbonation", search: "" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_tools", "/tools/carbonation");
  });

  it("saves last_route_equipment when navigating to /equipment", () => {
    render(AppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/equipment", search: "" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_equipment", "/equipment");
  });

  it("saves last_route_library when navigating to /library", () => {
    render(AppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/library", search: "" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_library", "/library");
  });

  it("saves last_route_settings when navigating to /settings", () => {
    render(AppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/settings", search: "" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_settings", "/settings");
  });

  it("includes query string in saved URL", () => {
    render(AppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/recipe/abc", search: "?tab=mash" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route", "/recipe/abc?tab=mash");
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_recipes", "/recipe/abc?tab=mash");
  });

  it("does not save a section key for unrecognised paths", () => {
    render(AppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/baseline-recipe/abc", search: "" } } });
    // baseline-recipe belongs to the recipes section
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_recipes", "/baseline-recipe/abc");
  });
});
```

- [ ] **Run tests — expect ALL to fail (new tests not yet implemented)**

```bash
npx vitest run tests/AppShell.test.ts
```

Expected: the new "section key saving" tests fail; existing tests still pass.

- [ ] **Implement the changes in `src/lib/desktop/AppShell.svelte`**

In the `<script>` block, add a `sectionKeyFor` helper and update `afterNavigate` and `onMount`. The full updated script block:

```typescript
import { onMount } from "svelte";
import { afterNavigate, goto } from "$app/navigation";
import { page } from "$app/stores";
import { loadSettings, settings, saveSetting } from "$lib/stores/settings";
import type { AppSettings } from "$lib/stores/settings";
import { lastError, lastSuccess } from "$lib/stores/error";
import BrewingIcon from "$lib/components/BrewingIcon.svelte";

let { children } = $props();

function sectionKeyFor(pathname: string): keyof AppSettings | null {
  if (pathname === "/" || pathname.startsWith("/recipe") || pathname.startsWith("/baseline-recipe"))
    return "last_route_recipes";
  if (pathname.startsWith("/batches"))   return "last_route_batches";
  if (pathname.startsWith("/tools"))     return "last_route_tools";
  if (pathname.startsWith("/equipment")) return "last_route_equipment";
  if (pathname.startsWith("/library"))   return "last_route_library";
  if (pathname.startsWith("/settings"))  return "last_route_settings";
  return null;
}

onMount(async () => {
  try {
    await loadSettings();
    const lastRoute = $settings.last_route;
    const currentUrl = $page.url.pathname + $page.url.search;
    if (lastRoute && lastRoute !== currentUrl) {
      goto(lastRoute);
    }
  } finally {
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    getCurrentWindow().show();
  }
});

afterNavigate(({ to }) => {
  if (to) {
    const url = to.url.pathname + to.url.search;
    saveSetting("last_route", url);
    const key = sectionKeyFor(to.url.pathname);
    if (key) saveSetting(key, url);
  }
});

const isRecipes   = $derived($page.url.pathname === "/" || $page.url.pathname.startsWith("/recipe"));
const isBatches   = $derived($page.url.pathname.startsWith("/batches"));
const isTools     = $derived($page.url.pathname.startsWith("/tools"));
const isEquipment = $derived($page.url.pathname.startsWith("/equipment"));
const isLibrary   = $derived($page.url.pathname.startsWith("/library"));
```

- [ ] **Run tests — all should pass now**

```bash
npx vitest run tests/AppShell.test.ts
```

Expected: all tests pass.

- [ ] **Commit**

```bash
git add src/lib/desktop/AppShell.svelte tests/AppShell.test.ts
git commit -m "feat: save per-section last route in desktop AppShell"
```

---

### Task 3: Convert desktop AppShell rail icons to dynamic buttons

**Files:**
- Modify: `src/lib/desktop/AppShell.svelte`
- Modify: `tests/AppShell.test.ts`

The rail icons are currently `<a href="...">` elements. They must become buttons that navigate to the stored section URL (falling back to the section root). The active-state styling stays the same.

- [ ] **Update the existing rail tests to query by aria-label instead of href**

Replace the three tests in the `"AppShell rail"` describe block:

```typescript
describe("AppShell rail", () => {
  it("renders an Equipment nav button", () => {
    const { container } = render(AppShell, { children: () => null });
    const btn = container.querySelector('button[aria-label="Equipment"]');
    expect(btn).not.toBeNull();
  });

  it("Equipment button has aria-label Equipment", () => {
    const { container } = render(AppShell, { children: () => null });
    const btn = container.querySelector('button[aria-label="Equipment"]');
    expect(btn?.getAttribute("aria-label")).toBe("Equipment");
  });

  it("Equipment button appears after Tools and before Settings in the rail", () => {
    const { container } = render(AppShell, { children: () => null });
    const buttons = Array.from(container.querySelectorAll("nav button"));
    const toolsIdx = buttons.findIndex((b) => b.getAttribute("aria-label") === "Tools");
    const equipIdx = buttons.findIndex((b) => b.getAttribute("aria-label") === "Equipment");
    const settingsIdx = buttons.findIndex((b) => b.getAttribute("aria-label") === "Settings");
    expect(equipIdx).toBeGreaterThan(toolsIdx);
    expect(equipIdx).toBeLessThan(settingsIdx);
  });
});
```

- [ ] **Add tests for dynamic navigation**

Add to `tests/AppShell.test.ts`:

```typescript
describe("AppShell rail dynamic navigation", () => {
  it("Recipes button navigates to last_route_recipes when set", async () => {
    mockSettings = { last_route_recipes: "/recipe/abc?tab=mash" };
    const { container } = render(AppShell, { children: () => null });
    const btn = container.querySelector('button[aria-label="Recipes"]') as HTMLButtonElement;
    btn.click();
    await tick();
    expect(gotoMock).toHaveBeenCalledWith("/recipe/abc?tab=mash");
  });

  it("Recipes button falls back to / when no last_route_recipes", async () => {
    mockSettings = {};
    const { container } = render(AppShell, { children: () => null });
    const btn = container.querySelector('button[aria-label="Recipes"]') as HTMLButtonElement;
    btn.click();
    await tick();
    expect(gotoMock).toHaveBeenCalledWith("/");
  });

  it("Batches button navigates to last_route_batches when set", async () => {
    mockSettings = { last_route_batches: "/batches/xyz" };
    const { container } = render(AppShell, { children: () => null });
    const btn = container.querySelector('button[aria-label="Batches"]') as HTMLButtonElement;
    btn.click();
    await tick();
    expect(gotoMock).toHaveBeenCalledWith("/batches/xyz");
  });
});
```

- [ ] **Run tests — expect the three new navigation tests and the updated rail tests to fail**

```bash
npx vitest run tests/AppShell.test.ts
```

Expected: updated rail tests and new navigation tests fail; section key tests still pass.

- [ ] **Replace the `<nav>` icon rail markup in `src/lib/desktop/AppShell.svelte`**

Replace the entire `<!-- Icon rail -->` nav block with:

```svelte
<!-- Icon rail -->
<nav class="flex flex-col items-center w-14 py-3 gap-2 border-r flex-shrink-0"
     style="background: var(--color-bg-surface); border-color: var(--color-border);">

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

  <button onclick={() => goto($settings.last_route_tools ?? "/tools")}
          class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors"
          aria-label="Tools"
          style={isTools ? "background: color-mix(in srgb, var(--color-accent) 18%, transparent);" : ""}>
    <BrewingIcon name="tools" size={22} />
  </button>

  <button onclick={() => goto($settings.last_route_equipment ?? "/equipment")}
          class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors"
          aria-label="Equipment"
          style={isEquipment ? "background: color-mix(in srgb, var(--color-accent) 18%, transparent);" : ""}>
    <BrewingIcon name="equipment" size={22} />
  </button>

  <button onclick={() => goto($settings.last_route_library ?? "/library")}
          class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors"
          aria-label="Library"
          style={isLibrary ? "background: color-mix(in srgb, var(--color-accent) 18%, transparent);" : ""}>
    <BrewingIcon name="library" size={22} />
  </button>

  <div class="flex-1"></div>

  <button onclick={() => goto($settings.last_route_settings ?? "/settings")}
          class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors"
          aria-label="Settings"
          style={$page.url.pathname.startsWith("/settings") ? "background: color-mix(in srgb, var(--color-accent) 18%, transparent);" : ""}>
    <BrewingIcon name="settings" size={22} />
  </button>
</nav>
```

- [ ] **Run tests — all should pass**

```bash
npx vitest run tests/AppShell.test.ts
```

Expected: all tests pass.

- [ ] **Commit**

```bash
git add src/lib/desktop/AppShell.svelte tests/AppShell.test.ts
git commit -m "feat: desktop rail navigates to per-section last route"
```

---

### Task 4: Encode recipe tab in URL

**Files:**
- Modify: `src/lib/desktop/RecipeView.svelte`

Currently `activeTab` is `$state("overview")`. Moving it to a URL query param (`?tab=ingredients`) means the tab is captured automatically when `last_route_recipes` is saved.

- [ ] **Add `page` import and change `activeTab` to a derived value**

At the top of the `<script>` block, add `page` to the `$app/stores` import (RecipeView doesn't import it yet):

```typescript
import { page } from "$app/stores";
```

Replace line 43:
```typescript
let activeTab = $state<"overview" | "ingredients" | "mash" | "water" | "fermentation" | "notes" | "batches">("overview");
```

With:
```typescript
const VALID_TABS = ["overview", "ingredients", "mash", "water", "fermentation", "notes", "batches"] as const;
type TabKey = typeof VALID_TABS[number];

const activeTab = $derived.by<TabKey>(() => {
  const raw = $page.url.searchParams.get("tab") ?? "";
  return (VALID_TABS as readonly string[]).includes(raw) ? (raw as TabKey) : "overview";
});
```

- [ ] **Update the TabBar `onchange` handler to navigate instead of setting state**

Replace line 400:
```svelte
<TabBar tabs={TABS} active={activeTab} onchange={(key) => activeTab = key as typeof activeTab} />
```

With:
```svelte
<TabBar tabs={TABS} active={activeTab} onchange={(key) => goto(`/recipe/${id}?tab=${key}`, { replaceState: true, noScroll: true })} />
```

- [ ] **Run type-check**

```bash
npx tsc --noEmit
```

Expected: no errors.

- [ ] **Run full test suite to check for regressions**

```bash
npx vitest run
```

Expected: all tests pass.

- [ ] **Commit**

```bash
git add src/lib/desktop/RecipeView.svelte
git commit -m "feat: encode recipe tab selection in URL query param"
```

---

### Task 5: Update mobile AppShell afterNavigate and onMount

**Files:**
- Modify: `src/lib/mobile/AppShell.svelte`

Same two changes as Task 2 but for the mobile shell. Mobile has no existing navigation tests, so no test file to update.

- [ ] **Update `src/lib/mobile/AppShell.svelte` script block**

Replace the entire `<script>` block with:

```typescript
<script lang="ts">
  import { onMount } from "svelte";
  import { afterNavigate, goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { loadSettings, settings, saveSetting } from "$lib/stores/settings";
  import type { AppSettings } from "$lib/stores/settings";
  import { lastError, lastSuccess } from "$lib/stores/error";
  import BottomTabBar from "./BottomTabBar.svelte";

  let { children } = $props();

  function sectionKeyFor(pathname: string): keyof AppSettings | null {
    if (pathname === "/" || pathname.startsWith("/recipe") || pathname.startsWith("/baseline-recipe"))
      return "last_route_recipes";
    if (pathname.startsWith("/batches"))   return "last_route_batches";
    if (pathname.startsWith("/tools"))     return "last_route_tools";
    if (pathname.startsWith("/equipment")) return "last_route_equipment";
    if (pathname.startsWith("/library"))   return "last_route_library";
    if (pathname.startsWith("/settings"))  return "last_route_settings";
    return null;
  }

  onMount(async () => {
    await loadSettings();
    const lastRoute = $settings.last_route;
    const currentUrl = $page.url.pathname + $page.url.search;
    if (lastRoute && lastRoute !== currentUrl) {
      goto(lastRoute);
    }
  });

  afterNavigate(({ to }) => {
    if (to) {
      const url = to.url.pathname + to.url.search;
      saveSetting("last_route", url);
      const key = sectionKeyFor(to.url.pathname);
      if (key) saveSetting(key, url);
    }
  });
</script>
```

- [ ] **Run type-check**

```bash
npx tsc --noEmit
```

Expected: no errors.

- [ ] **Commit**

```bash
git add src/lib/mobile/AppShell.svelte
git commit -m "feat: save per-section last route in mobile AppShell"
```

---

### Task 6: Update mobile BottomTabBar to use dynamic navigation

**Files:**
- Modify: `src/lib/mobile/BottomTabBar.svelte`

The bottom tab bar currently uses `<a href="...">` elements. Replace with buttons that navigate to the stored section URL. The "More" tab (which covers `/settings`, `/equipment`, and `/library`) navigates to `last_route_settings` first, falling back to `last_route_equipment`, `last_route_library`, then `/settings`.

- [ ] **Replace `src/lib/mobile/BottomTabBar.svelte` with the updated version**

Active state is inlined directly in the template (not via a helper function) so it reacts to `$page` changes correctly.

```svelte
<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { settings } from "$lib/stores/settings";
  import BrewingIcon from "$lib/components/BrewingIcon.svelte";
</script>

<nav
  class="flex border-t flex-shrink-0"
  style="background: var(--color-bg-surface); border-color: var(--color-border); padding-bottom: env(safe-area-inset-bottom, 0px);"
>
  {@const recipesActive = $page.url.pathname === "/" || $page.url.pathname.startsWith("/recipe")}
  {@const batchesActive = $page.url.pathname.startsWith("/batches")}
  {@const toolsActive   = $page.url.pathname.startsWith("/tools")}
  {@const moreActive    = $page.url.pathname.startsWith("/settings") || $page.url.pathname.startsWith("/equipment") || $page.url.pathname.startsWith("/library")}

  <button
    onclick={() => goto($settings.last_route_recipes ?? "/")}
    aria-current={recipesActive ? "page" : undefined}
    class="flex flex-col items-center justify-center flex-1 py-2 gap-1 text-xs transition-colors"
    style={recipesActive ? "color: var(--color-accent);" : "color: var(--color-text-secondary);"}
  >
    <span style={recipesActive ? "opacity: 1; transition: opacity 0.15s;" : "opacity: 0.45; transition: opacity 0.15s;"}>
      <BrewingIcon name="recipes" size={22} />
    </span>
    Recipes
  </button>

  <button
    onclick={() => goto($settings.last_route_batches ?? "/batches")}
    aria-current={batchesActive ? "page" : undefined}
    class="flex flex-col items-center justify-center flex-1 py-2 gap-1 text-xs transition-colors"
    style={batchesActive ? "color: var(--color-accent);" : "color: var(--color-text-secondary);"}
  >
    <span style={batchesActive ? "opacity: 1; transition: opacity 0.15s;" : "opacity: 0.45; transition: opacity 0.15s;"}>
      <BrewingIcon name="batches" size={22} />
    </span>
    Batches
  </button>

  <button
    onclick={() => goto($settings.last_route_tools ?? "/tools")}
    aria-current={toolsActive ? "page" : undefined}
    class="flex flex-col items-center justify-center flex-1 py-2 gap-1 text-xs transition-colors"
    style={toolsActive ? "color: var(--color-accent);" : "color: var(--color-text-secondary);"}
  >
    <span style={toolsActive ? "opacity: 1; transition: opacity 0.15s;" : "opacity: 0.45; transition: opacity 0.15s;"}>
      <BrewingIcon name="tools" size={22} />
    </span>
    Tools
  </button>

  <button
    onclick={() => goto($settings.last_route_settings ?? $settings.last_route_equipment ?? $settings.last_route_library ?? "/settings")}
    aria-current={moreActive ? "page" : undefined}
    class="flex flex-col items-center justify-center flex-1 py-2 gap-1 text-xs transition-colors"
    style={moreActive ? "color: var(--color-accent);" : "color: var(--color-text-secondary);"}
  >
    <span style={moreActive ? "opacity: 1; transition: opacity 0.15s;" : "opacity: 0.45; transition: opacity 0.15s;"}>
      <BrewingIcon name="settings" size={22} />
    </span>
    More
  </button>
</nav>
```

- [ ] **Run type-check and full test suite**

```bash
npx tsc --noEmit && npx vitest run
```

Expected: no type errors, all tests pass.

- [ ] **Commit**

```bash
git add src/lib/mobile/BottomTabBar.svelte
git commit -m "feat: mobile bottom tab bar navigates to per-section last route"
```
