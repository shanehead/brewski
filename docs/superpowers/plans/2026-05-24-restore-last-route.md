# Restore Last Route Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** On launch, navigate the app to the last route the user visited, persisted across sessions via the existing settings store.

**Architecture:** Save `last_route` (a plain pathname string) to the existing SQLite settings table via `saveSetting` on every navigation. On mount, after settings load, call `goto(last_route)` if a saved route exists and differs from the current path. Both desktop and mobile AppShells get identical logic.

**Tech Stack:** SvelteKit (`afterNavigate`, `goto`, `page` store), Svelte 5 (`$props`, `onMount`), existing `$lib/stores/settings` (`settings`, `loadSettings`, `saveSetting`), Vitest + Testing Library.

---

### Task 1: Add `last_route` to AppSettings

**Files:**
- Modify: `src/lib/stores/settings.ts`

- [ ] **Step 1: Add the field to the interface**

In `src/lib/stores/settings.ts`, add `last_route?: string` to `AppSettings`:

```typescript
export interface AppSettings {
  units?: "metric" | "imperial";
  theme?: string;
  default_equipment_profile_id?: string;
  last_route?: string;
}
```

- [ ] **Step 2: Verify TypeScript is happy**

```bash
just check-ts
```

Expected: no errors.

- [ ] **Step 3: Commit**

```bash
git add src/lib/stores/settings.ts
git commit -m "feat(settings): add last_route field to AppSettings"
```

---

### Task 2: Restore and save last route — desktop AppShell

**Files:**
- Modify: `src/lib/desktop/AppShell.svelte`
- Modify: `tests/AppShell.test.ts`

- [ ] **Step 1: Write failing tests**

Replace the contents of `tests/AppShell.test.ts` with:

```typescript
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render } from "@testing-library/svelte";
import AppShell from "../src/lib/desktop/AppShell.svelte";

// Capture the afterNavigate callback so we can trigger it manually
let afterNavigateCb: ((nav: { to: { url: { pathname: string } } | null }) => void) | null = null;
const gotoMock = vi.fn();

vi.mock("$app/navigation", () => ({
  afterNavigate: vi.fn((cb) => { afterNavigateCb = cb; }),
  goto: gotoMock,
}));

let mockPathname = "/";
vi.mock("$app/stores", () => ({
  page: {
    subscribe: vi.fn((fn) => {
      fn({ url: { pathname: mockPathname } });
      return () => {};
    }),
  },
}));

let mockSettings: Record<string, string> = {};
const saveSettingMock = vi.fn();

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
  lastError: { subscribe: vi.fn((fn) => { fn(null); return () => {}; }) },
}));

vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: vi.fn(() => ({ show: vi.fn() })),
}));

beforeEach(() => {
  gotoMock.mockClear();
  saveSettingMock.mockClear();
  afterNavigateCb = null;
  mockPathname = "/";
  mockSettings = {};
});

describe("AppShell rail", () => {
  it("renders an Equipment nav link", () => {
    const { container } = render(AppShell, { children: () => null });
    const links = container.querySelectorAll("nav a");
    const hrefs = Array.from(links).map((a) => a.getAttribute("href"));
    expect(hrefs).toContain("/equipment");
  });

  it("Equipment link has aria-label Equipment", () => {
    const { container } = render(AppShell, { children: () => null });
    const equipLink = container.querySelector('a[href="/equipment"]');
    expect(equipLink?.getAttribute("aria-label")).toBe("Equipment");
  });

  it("Equipment link appears before the spacer and after the Tools link", () => {
    const { container } = render(AppShell, { children: () => null });
    const links = Array.from(container.querySelectorAll("nav a"));
    const toolsIdx = links.findIndex((a) => a.getAttribute("href") === "/tools");
    const equipIdx = links.findIndex((a) => a.getAttribute("href") === "/equipment");
    const settingsIdx = links.findIndex((a) => a.getAttribute("href") === "/settings");
    expect(equipIdx).toBeGreaterThan(toolsIdx);
    expect(equipIdx).toBeLessThan(settingsIdx);
  });
});

describe("AppShell last route", () => {
  it("calls goto with last_route when it differs from current path", async () => {
    mockSettings = { last_route: "/tools" };
    mockPathname = "/";
    render(AppShell, { children: () => null });
    // Wait for onMount async logic to settle
    await new Promise((r) => setTimeout(r, 0));
    expect(gotoMock).toHaveBeenCalledWith("/tools");
  });

  it("does not call goto when last_route matches current path", async () => {
    mockSettings = { last_route: "/tools" };
    mockPathname = "/tools";
    render(AppShell, { children: () => null });
    await new Promise((r) => setTimeout(r, 0));
    expect(gotoMock).not.toHaveBeenCalled();
  });

  it("does not call goto when last_route is absent", async () => {
    mockSettings = {};
    mockPathname = "/";
    render(AppShell, { children: () => null });
    await new Promise((r) => setTimeout(r, 0));
    expect(gotoMock).not.toHaveBeenCalled();
  });

  it("calls saveSetting with the navigated-to pathname", () => {
    render(AppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/library" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route", "/library");
  });

  it("does not call saveSetting when navigation.to is null", () => {
    render(AppShell, { children: () => null });
    afterNavigateCb!({ to: null });
    expect(saveSettingMock).not.toHaveBeenCalled();
  });
});
```

- [ ] **Step 2: Run tests to confirm they fail**

```bash
bun run test -- --reporter=verbose 2>&1 | grep -E "FAIL|PASS|✓|✗|×|last route"
```

Expected: the five new `last route` tests fail; the three rail tests still pass.

- [ ] **Step 3: Implement the feature in desktop AppShell**

Replace the `<script>` block in `src/lib/desktop/AppShell.svelte` with:

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { afterNavigate, goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { loadSettings, settings, saveSetting } from "$lib/stores/settings";
  import { lastError } from "$lib/stores/error";
  import BrewingIcon from "$lib/components/BrewingIcon.svelte";

  let { children } = $props();

  onMount(async () => {
    try {
      await loadSettings();
      if ($settings.last_route && $settings.last_route !== $page.url.pathname) {
        goto($settings.last_route);
      }
    } finally {
      const { getCurrentWindow } = await import('@tauri-apps/api/window');
      getCurrentWindow().show();
    }
  });

  afterNavigate(({ to }) => {
    if (to) saveSetting('last_route', to.url.pathname);
  });

  const isRecipes = $derived($page.url.pathname === "/" || $page.url.pathname.startsWith("/recipe"));
  const isBatches = $derived($page.url.pathname.startsWith("/batches"));
  const isTools = $derived($page.url.pathname.startsWith("/tools"));
  const isEquipment = $derived($page.url.pathname.startsWith("/equipment"));
  const isLibrary = $derived($page.url.pathname.startsWith("/library"));
</script>
```

(Leave the template section — everything after `</script>` — unchanged.)

- [ ] **Step 4: Run tests to confirm they pass**

```bash
bun run test -- --reporter=verbose 2>&1 | grep -E "FAIL|PASS|✓|✗|×|last route"
```

Expected: all eight tests in `AppShell.test.ts` pass.

- [ ] **Step 5: TypeScript check**

```bash
just check-ts
```

Expected: no errors.

- [ ] **Step 6: Commit**

```bash
git add src/lib/desktop/AppShell.svelte tests/AppShell.test.ts
git commit -m "feat(nav): restore and save last route in desktop AppShell"
```

---

### Task 3: Restore and save last route — mobile AppShell

**Files:**
- Modify: `src/lib/mobile/AppShell.svelte`
- Create: `tests/MobileAppShell.test.ts`

- [ ] **Step 1: Write failing tests**

Create `tests/MobileAppShell.test.ts`:

```typescript
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render } from "@testing-library/svelte";
import MobileAppShell from "../src/lib/mobile/AppShell.svelte";

let afterNavigateCb: ((nav: { to: { url: { pathname: string } } | null }) => void) | null = null;
const gotoMock = vi.fn();

vi.mock("$app/navigation", () => ({
  afterNavigate: vi.fn((cb) => { afterNavigateCb = cb; }),
  goto: gotoMock,
}));

let mockPathname = "/";
vi.mock("$app/stores", () => ({
  page: {
    subscribe: vi.fn((fn) => {
      fn({ url: { pathname: mockPathname } });
      return () => {};
    }),
  },
}));

let mockSettings: Record<string, string> = {};
const saveSettingMock = vi.fn();

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
  lastError: { subscribe: vi.fn((fn) => { fn(null); return () => {}; }) },
}));

// BottomTabBar uses page store and BrewingIcon — stub it out
vi.mock("../src/lib/mobile/BottomTabBar.svelte", () => ({
  default: { render: () => ({ html: "" }) },
}));

beforeEach(() => {
  gotoMock.mockClear();
  saveSettingMock.mockClear();
  afterNavigateCb = null;
  mockPathname = "/";
  mockSettings = {};
});

describe("MobileAppShell last route", () => {
  it("calls goto with last_route when it differs from current path", async () => {
    mockSettings = { last_route: "/library" };
    mockPathname = "/";
    render(MobileAppShell, { children: () => null });
    await new Promise((r) => setTimeout(r, 0));
    expect(gotoMock).toHaveBeenCalledWith("/library");
  });

  it("does not call goto when last_route matches current path", async () => {
    mockSettings = { last_route: "/library" };
    mockPathname = "/library";
    render(MobileAppShell, { children: () => null });
    await new Promise((r) => setTimeout(r, 0));
    expect(gotoMock).not.toHaveBeenCalled();
  });

  it("does not call goto when last_route is absent", async () => {
    mockSettings = {};
    render(MobileAppShell, { children: () => null });
    await new Promise((r) => setTimeout(r, 0));
    expect(gotoMock).not.toHaveBeenCalled();
  });

  it("calls saveSetting with the navigated-to pathname", () => {
    render(MobileAppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/tools/carbonation" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route", "/tools/carbonation");
  });

  it("does not call saveSetting when navigation.to is null", () => {
    render(MobileAppShell, { children: () => null });
    afterNavigateCb!({ to: null });
    expect(saveSettingMock).not.toHaveBeenCalled();
  });
});
```

- [ ] **Step 2: Run tests to confirm they fail**

```bash
bun run test -- --reporter=verbose 2>&1 | grep -E "FAIL|PASS|✓|✗|×|MobileAppShell"
```

Expected: all five `MobileAppShell` tests fail.

- [ ] **Step 3: Implement the feature in mobile AppShell**

Replace the `<script>` block in `src/lib/mobile/AppShell.svelte` with:

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { afterNavigate, goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { loadSettings, settings, saveSetting } from "$lib/stores/settings";
  import { lastError } from "$lib/stores/error";
  import BottomTabBar from "./BottomTabBar.svelte";

  let { children } = $props();

  onMount(async () => {
    await loadSettings();
    if ($settings.last_route && $settings.last_route !== $page.url.pathname) {
      goto($settings.last_route);
    }
  });

  afterNavigate(({ to }) => {
    if (to) saveSetting('last_route', to.url.pathname);
  });
</script>
```

(Leave the template section unchanged.)

- [ ] **Step 4: Run tests to confirm they pass**

```bash
bun run test -- --reporter=verbose 2>&1 | grep -E "FAIL|PASS|✓|✗|×|MobileAppShell"
```

Expected: all five tests pass.

- [ ] **Step 5: Run the full test suite**

```bash
just test
```

Expected: all tests pass, no Rust or frontend failures.

- [ ] **Step 6: Commit**

```bash
git add src/lib/mobile/AppShell.svelte tests/MobileAppShell.test.ts
git commit -m "feat(nav): restore and save last route in mobile AppShell"
```
