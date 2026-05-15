# Equipment Rail Icon Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Move equipment profiles out of Settings into a dedicated `/equipment` route with its own rail icon, positioned between Tools and the spacer.

**Architecture:** Create `src/routes/equipment/+page.svelte` by extracting all equipment-related state, handlers, and markup from the settings page. Update `AppShell.svelte` to add an Equipment rail icon and `isEquipment` active-state derived. Strip the equipment section from `settings/+page.svelte`.

**Tech Stack:** SvelteKit 2, Svelte 5 (runes), TypeScript, Vitest + Testing Library, Tauri IPC

---

## File Map

| File | Action |
|------|--------|
| `src/routes/equipment/+page.svelte` | **Create** — new equipment management page |
| `src/lib/components/AppShell.svelte` | **Modify** — add Equipment icon + `isEquipment` derived |
| `src/routes/settings/+page.svelte` | **Modify** — remove equipment section, imports, state, handlers |
| `tests/AppShell.test.ts` | **Create** — verify Equipment icon renders and active state |
| `tests/EquipmentPage.test.ts` | **Create** — verify equipment page renders profiles and form |

---

## Task 1: Create the Equipment page

**Files:**
- Create: `src/routes/equipment/+page.svelte`
- Create (test first): `tests/EquipmentPage.test.ts`

The new page is a direct extraction of the equipment section from Settings plus the default profile selector, which currently lives in the same section. The `ipc` wrapper and all API calls are identical to the existing Settings implementation.

- [ ] **Step 1: Write the failing test**

Create `tests/EquipmentPage.test.ts`:

```typescript
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render } from "@testing-library/svelte";
import EquipmentPage from "../src/routes/equipment/+page.svelte";

vi.mock("$lib/api", () => ({
  listEquipmentProfiles: vi.fn().mockResolvedValue([
    { id: "1", name: "My Kettle", batch_size_l: 23, boil_size_l: 27, efficiency_pct: 72 },
  ]),
  createEquipmentProfile: vi.fn().mockResolvedValue({}),
  deleteEquipmentProfile: vi.fn().mockResolvedValue({}),
}));

vi.mock("$lib/stores/settings", () => ({
  settings: { subscribe: vi.fn((fn) => { fn({ theme: "midnight", units: "metric", default_equipment_profile_id: "" }); return () => {}; }) },
  loadSettings: vi.fn().mockResolvedValue(undefined),
  saveSetting: vi.fn().mockResolvedValue(undefined),
}));

vi.mock("$lib/stores/error", () => ({
  ipc: vi.fn((p) => p),
}));

describe("EquipmentPage", () => {
  it("renders the page heading", async () => {
    const { getByText } = render(EquipmentPage);
    expect(getByText("Equipment")).toBeTruthy();
  });

  it("renders the Default Profile label", async () => {
    const { getByText } = render(EquipmentPage);
    expect(getByText("Default Profile")).toBeTruthy();
  });

  it("renders the new profile name input", async () => {
    const { getByPlaceholderText } = render(EquipmentPage);
    expect(getByPlaceholderText("New profile name")).toBeTruthy();
  });

  it("renders the Add button", async () => {
    const { getByText } = render(EquipmentPage);
    expect(getByText("Add")).toBeTruthy();
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

```bash
npm test -- --reporter=verbose tests/EquipmentPage.test.ts
```

Expected: FAIL — `../src/routes/equipment/+page.svelte` not found.

- [ ] **Step 3: Create the Equipment page**

Create `src/routes/equipment/+page.svelte`:

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { settings, loadSettings, saveSetting } from "$lib/stores/settings";
  import { listEquipmentProfiles, createEquipmentProfile, deleteEquipmentProfile } from "$lib/api";
  import type { EquipmentProfile } from "$lib/api";
  import { ipc } from "$lib/stores/error";

  let profiles = $state<EquipmentProfile[]>([]);
  let newProfileName = $state("");

  onMount(async () => {
    await ipc(loadSettings());
    profiles = await ipc(listEquipmentProfiles()) ?? [];
  });

  async function handleDefaultEquipChange(e: Event) {
    await ipc(saveSetting("default_equipment_profile_id", (e.target as HTMLSelectElement).value));
  }

  async function handleAddProfile() {
    if (!newProfileName.trim()) return;
    await ipc(createEquipmentProfile({
      name: newProfileName,
      boil_size_l: 27.0,
      batch_size_l: 23.0,
      efficiency_pct: 72.0,
    }));
    profiles = await ipc(listEquipmentProfiles()) ?? profiles;
    newProfileName = "";
  }

  async function handleDeleteProfile(id: string) {
    if (!confirm("Delete this equipment profile?")) return;
    await ipc(deleteEquipmentProfile(id));
    profiles = await ipc(listEquipmentProfiles()) ?? profiles;
  }
</script>

<div class="flex-1 overflow-y-auto p-6" style="background: var(--color-bg-base);">
  <h1 class="text-lg font-semibold mb-6" style="color: var(--color-text-primary);">Equipment</h1>

  <div class="flex flex-col gap-6 max-w-md">
    <section class="flex flex-col gap-3">
      <h2 class="text-sm font-semibold" style="color: var(--color-text-secondary);">Equipment Profiles</h2>
      <div class="flex items-center justify-between">
        <label for="select-default-profile" class="text-sm" style="color: var(--color-text-primary);">Default Profile</label>
        <select id="select-default-profile" value={$settings.default_equipment_profile_id ?? ""}
                onchange={handleDefaultEquipChange}
                class="px-2 py-1.5 rounded text-sm"
                style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);">
          <option value="">None</option>
          {#each profiles as p}
            <option value={p.id}>{p.name}</option>
          {/each}
        </select>
      </div>

      {#each profiles as p (p.id)}
        <div class="flex items-center justify-between py-1 border-t" style="border-color: var(--color-border);">
          <div>
            <p class="text-sm" style="color: var(--color-text-primary);">{p.name}</p>
            <p class="text-xs" style="color: var(--color-text-secondary);">
              {p.batch_size_l}L batch · {p.efficiency_pct}% efficiency
            </p>
          </div>
          <button onclick={() => handleDeleteProfile(p.id)} class="text-xs px-2 py-1 rounded"
                  style="color: var(--color-text-secondary); background: var(--color-bg-elevated);">Delete</button>
        </div>
      {/each}

      <div class="flex gap-2 pt-1">
        <input type="text" bind:value={newProfileName} placeholder="New profile name"
               class="flex-1 px-2 py-1.5 rounded text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
        <button onclick={handleAddProfile} class="text-xs px-3 py-1.5 rounded"
                style="background: var(--color-accent); color: #fff;">Add</button>
      </div>
    </section>
  </div>
</div>
```

- [ ] **Step 4: Run test to verify it passes**

```bash
npm test -- --reporter=verbose tests/EquipmentPage.test.ts
```

Expected: PASS — 4 tests pass.

- [ ] **Step 5: Commit**

```bash
git add src/routes/equipment/+page.svelte tests/EquipmentPage.test.ts
git commit -m "feat(ui): add equipment page at /equipment"
```

---

## Task 2: Add Equipment icon to the rail

**Files:**
- Modify: `src/lib/components/AppShell.svelte`
- Create (test first): `tests/AppShell.test.ts`

The Equipment icon uses a brewing kettle SVG — a rounded vessel with a spout, inline at 20×20px, matching the existing rail icon style.

- [ ] **Step 1: Write the failing test**

Create `tests/AppShell.test.ts`:

```typescript
import { describe, it, expect, vi } from "vitest";
import { render } from "@testing-library/svelte";
import AppShell from "../src/lib/components/AppShell.svelte";

vi.mock("$app/stores", () => ({
  page: {
    subscribe: vi.fn((fn) => {
      fn({ url: { pathname: "/" } });
      return () => {};
    }),
  },
}));

vi.mock("$lib/stores/settings", () => ({
  loadSettings: vi.fn().mockResolvedValue(undefined),
}));

vi.mock("$lib/stores/error", () => ({
  lastError: { subscribe: vi.fn((fn) => { fn(null); return () => {}; }) },
}));

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
```

- [ ] **Step 2: Run test to verify it fails**

```bash
npm test -- --reporter=verbose tests/AppShell.test.ts
```

Expected: FAIL — no `/equipment` link found.

- [ ] **Step 3: Add Equipment icon and derived to AppShell**

In `src/lib/components/AppShell.svelte`, make these changes:

Add `isEquipment` after the existing derived declarations (around line 13):

```svelte
  const isEquipment = $derived($page.url.pathname.startsWith("/equipment"));
```

Add the Equipment icon link after the Tools icon block and before `<div class="flex-1"></div>` (around line 51):

```svelte
    <!-- Equipment icon -->
    <a href="/equipment" class="w-9 h-9 flex items-center justify-center rounded transition-colors"
       aria-label="Equipment"
       style={isEquipment ? "background: var(--color-accent); color: #fff;" : "color: var(--color-text-secondary);"}>
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M3 6h18"/>
        <path d="M4 6v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V6"/>
        <path d="M8 6V4a1 1 0 0 1 1-1h6a1 1 0 0 1 1 1v2"/>
        <path d="M10 11h4"/>
      </svg>
    </a>
```

- [ ] **Step 4: Run test to verify it passes**

```bash
npm test -- --reporter=verbose tests/AppShell.test.ts
```

Expected: PASS — 3 tests pass.

- [ ] **Step 5: Commit**

```bash
git add src/lib/components/AppShell.svelte tests/AppShell.test.ts
git commit -m "feat(ui): add Equipment icon to nav rail"
```

---

## Task 3: Strip equipment section from Settings

**Files:**
- Modify: `src/routes/settings/+page.svelte`

Settings retains only Appearance and Units. All equipment-related code is removed.

- [ ] **Step 1: Run existing tests to establish baseline**

```bash
npm test -- --reporter=verbose
```

Expected: all tests pass.

- [ ] **Step 2: Remove equipment imports, state, and handlers**

Replace the entire `<script>` block in `src/routes/settings/+page.svelte` with:

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { settings, loadSettings, saveSetting } from "$lib/stores/settings";
  import { ipc } from "$lib/stores/error";

  onMount(async () => {
    await ipc(loadSettings());
  });

  async function handleThemeChange(e: Event) {
    await ipc(saveSetting("theme", (e.target as HTMLSelectElement).value));
  }

  async function handleUnitsChange(e: Event) {
    await ipc(saveSetting("units", (e.target as HTMLSelectElement).value));
  }
</script>
```

- [ ] **Step 3: Remove the Equipment Profiles section from the template**

Replace the entire `<div class="flex-1 overflow-y-auto p-6" ...>` block in `src/routes/settings/+page.svelte` with:

```svelte
<div class="flex-1 overflow-y-auto p-6" style="background: var(--color-bg-base);">
  <h1 class="text-lg font-semibold mb-6" style="color: var(--color-text-primary);">Settings</h1>

  <div class="flex flex-col gap-6 max-w-md">
    <!-- Appearance -->
    <section class="flex flex-col gap-3">
      <h2 class="text-sm font-semibold" style="color: var(--color-text-secondary);">Appearance</h2>
      <div class="flex items-center justify-between">
        <label for="select-theme" class="text-sm" style="color: var(--color-text-primary);">Theme</label>
        <select id="select-theme" value={$settings.theme ?? "midnight"} onchange={handleThemeChange}
                class="px-2 py-1.5 rounded text-sm"
                style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);">
          <option value="midnight">Midnight</option>
          <option value="dracula">Dracula</option>
          <option value="tokyo-night">Tokyo Night</option>
          <option value="catppuccin">Catppuccin</option>
          <option value="nord">Nord</option>
          <option value="monokai">Monokai</option>
          <option value="catppuccin-latte">Catppuccin Latte</option>
          <option value="solarized-light">Solarized Light</option>
          <option value="ayu-light">Ayu Light</option>
          <option value="github-light">GitHub Light</option>
        </select>
      </div>
    </section>

    <!-- Units -->
    <section class="flex flex-col gap-3">
      <h2 class="text-sm font-semibold" style="color: var(--color-text-secondary);">Units</h2>
      <div class="flex items-center justify-between">
        <label for="select-units" class="text-sm" style="color: var(--color-text-primary);">Measurement System</label>
        <select id="select-units" value={$settings.units ?? "metric"} onchange={handleUnitsChange}
                class="px-2 py-1.5 rounded text-sm"
                style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);">
          <option value="metric">Metric (L, kg, °C)</option>
          <option value="imperial">Imperial (gal, lb, °F)</option>
        </select>
      </div>
    </section>
  </div>
</div>
```

- [ ] **Step 4: Run all tests**

```bash
npm test -- --reporter=verbose
```

Expected: all tests pass.

- [ ] **Step 5: Run type check**

```bash
npm run check
```

Expected: no errors.

- [ ] **Step 6: Commit**

```bash
git add src/routes/settings/+page.svelte
git commit -m "feat(ui): remove equipment section from Settings"
```

---

## Task 4: Mark notes.md item complete

**Files:**
- Modify: `notes.md`

- [ ] **Step 1: Mark the todo item done**

In `notes.md`, change:

```
- [ ] Move equipment into it's own rail icon
```

to:

```
- [x] Move equipment into it's own rail icon
```

- [ ] **Step 2: Commit**

```bash
git add notes.md
git commit -m "chore: mark equipment rail icon todo complete"
```
