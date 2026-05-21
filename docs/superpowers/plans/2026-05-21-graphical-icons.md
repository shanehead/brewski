# Graphical Icon Redesign Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace all monochrome outline SVG icons with custom multi-color filled app-style icons across the nav rail and tab bars.

**Architecture:** All icon artwork lives in `icons.ts` (a record of SVG inner markup keyed by name). `BrewingIcon.svelte` is a thin wrapper that renders them. `AppShell.svelte` switches from inline SVGs to `BrewingIcon`. The active/inactive state changes from `color: currentColor` manipulation to opacity — full opacity for active, 0.45 for inactive — since icons no longer use `currentColor`.

**Tech Stack:** SvelteKit, Svelte 5 runes, inline SVG paths with hard-coded fill colors (no `stroke="currentColor"`)

---

## File Map

| Action | File |
|--------|------|
| **Modify** | `src/lib/icons.ts` |
| **Modify** | `src/lib/components/BrewingIcon.svelte` |
| **Modify** | `src/lib/desktop/AppShell.svelte` |
| **Modify** | `src/lib/components/TabBar.svelte` |
| **Modify** | `src/lib/mobile/BottomTabBar.svelte` |

---

## Task 1: Replace icon artwork in `icons.ts`

**Files:**
- Modify: `src/lib/icons.ts`

Add nav-rail icon names to `BrewingIconName` and replace all SVG path strings with new multi-color filled artwork.

- [ ] **Step 1: Replace the entire file**

```typescript
export type BrewingIconName =
  | "fermentable"
  | "hop"
  | "yeast"
  | "overview"
  | "ingredients"
  | "mash"
  | "water"
  | "fermentation"
  | "notes"
  | "batches"
  | "recipes"
  | "tools"
  | "equipment"
  | "library"
  | "settings";

export const ICONS: Record<BrewingIconName, string> = {
  recipes: `
    <path d="M4 4C4 3 5 2 6 2H12V12L9 10.5L6 12V2" fill="#3b82f6"/>
    <path d="M12 2H18C19 2 20 3 20 4V20C20 21 19 22 18 22H6C5 22 4 21 4 20V12" fill="#2563eb"/>
    <path d="M12 2V22" fill="#1d4ed8" opacity="0.4"/>
    <rect x="8" y="14" width="8" height="1.5" rx="0.75" fill="white" opacity="0.5"/>
    <rect x="8" y="17" width="5" height="1.5" rx="0.75" fill="white" opacity="0.5"/>
  `,
  batches: `
    <ellipse cx="12" cy="6" rx="7" ry="2.5" fill="#10b981"/>
    <rect x="5" y="6" width="14" height="4" fill="#059669"/>
    <ellipse cx="12" cy="10" rx="7" ry="2.5" fill="#10b981"/>
    <rect x="5" y="10" width="14" height="5" fill="#047857"/>
    <ellipse cx="12" cy="15" rx="7" ry="2.5" fill="#10b981"/>
    <rect x="5" y="15" width="14" height="3" fill="#065f46"/>
    <ellipse cx="12" cy="18" rx="7" ry="2.5" fill="#10b981"/>
    <line x1="12" y1="6" x2="12" y2="18" stroke="#6ee7b7" stroke-width="1" opacity="0.5"/>
  `,
  tools: `
    <path d="M14.5 2.5C11.5 2.5 9 5 9 8C9 8.8 9.2 9.5 9.5 10.2L3 17C2.4 17.6 2.4 18.6 3 19.2L4.8 21C5.4 21.6 6.4 21.6 7 21L13.8 14.5C14.5 14.8 15.2 15 16 15C19 15 21.5 12.5 21.5 9.5C21.5 8.9 21.4 8.3 21.2 7.8L18 11L16 9L19.2 5.8C18.7 5.6 18.1 5.5 17.5 5.5" fill="#f59e0b"/>
    <path d="M3 17L7 21L9.5 18.5L5.5 14.5Z" fill="#d97706"/>
    <circle cx="5.5" cy="18.5" r="1.2" fill="#fcd34d"/>
  `,
  equipment: `
    <path d="M3 9C1 9 1 12 3 12" stroke="#8b5cf6" stroke-width="2" fill="none" stroke-linecap="round"/>
    <path d="M21 9C23 9 23 12 21 12" stroke="#8b5cf6" stroke-width="2" fill="none" stroke-linecap="round"/>
    <rect x="4" y="7" width="16" height="11" rx="3" fill="#7c3aed"/>
    <rect x="4" y="7" width="16" height="5" rx="3" fill="#8b5cf6"/>
    <rect x="6" y="4" width="12" height="4" rx="2" fill="#a78bfa"/>
    <rect x="10" y="2" width="4" height="3" rx="1.5" fill="#c4b5fd"/>
    <rect x="6" y="9" width="4" height="2" rx="1" fill="white" opacity="0.2"/>
  `,
  library: `
    <rect x="3" y="4" width="5" height="16" rx="1.5" fill="#ec4899"/>
    <rect x="3" y="4" width="2" height="16" rx="1" fill="#be185d"/>
    <rect x="9" y="6" width="4" height="14" rx="1.5" fill="#f97316"/>
    <rect x="9" y="6" width="1.5" height="14" rx="0.75" fill="#c2410c"/>
    <rect x="14.5" y="3" width="6" height="17" rx="1.5" fill="#06b6d4"/>
    <rect x="14.5" y="3" width="2.5" height="17" rx="1" fill="#0e7490"/>
  `,
  settings: `
    <circle cx="12" cy="12" r="10" fill="#64748b"/>
    <circle cx="12" cy="12" r="7" fill="#475569"/>
    <rect x="10.5" y="2" width="3" height="3" rx="0.5" fill="#64748b"/>
    <rect x="10.5" y="19" width="3" height="3" rx="0.5" fill="#64748b"/>
    <rect x="2" y="10.5" width="3" height="3" rx="0.5" fill="#64748b"/>
    <rect x="19" y="10.5" width="3" height="3" rx="0.5" fill="#64748b"/>
    <rect x="4.5" y="4.5" width="2.5" height="2.5" rx="0.5" fill="#64748b" transform="rotate(45 5.75 5.75)"/>
    <rect x="17" y="4.5" width="2.5" height="2.5" rx="0.5" fill="#64748b" transform="rotate(45 18.25 5.75)"/>
    <rect x="4.5" y="17" width="2.5" height="2.5" rx="0.5" fill="#64748b" transform="rotate(45 5.75 18.25)"/>
    <rect x="17" y="17" width="2.5" height="2.5" rx="0.5" fill="#64748b" transform="rotate(45 18.25 18.25)"/>
    <circle cx="12" cy="12" r="4" fill="#94a3b8"/>
    <circle cx="12" cy="12" r="2.5" fill="#334155"/>
  `,
  overview: `
    <rect x="5" y="3" width="14" height="18" rx="2" fill="#3b82f6"/>
    <rect x="5" y="3" width="14" height="9" rx="2" fill="#2563eb"/>
    <rect x="9" y="1" width="6" height="4" rx="2" fill="#93c5fd"/>
    <rect x="7" y="10" width="10" height="1.5" rx="0.75" fill="white" opacity="0.6"/>
    <rect x="7" y="13" width="7" height="1.5" rx="0.75" fill="white" opacity="0.6"/>
    <rect x="7" y="16" width="8" height="1.5" rx="0.75" fill="white" opacity="0.6"/>
  `,
  ingredients: `
    <circle cx="8" cy="12" r="6" fill="#84cc16"/>
    <path d="M8 6C5 6 2 9 2 12C2 15 5 18 8 18" fill="#65a30d"/>
    <path d="M6 12L8 9L10 12L8 15Z" fill="white" opacity="0.8"/>
    <line x1="16" y1="20" x2="16" y2="8" stroke="#d97706" stroke-width="2"/>
    <ellipse cx="16" cy="8" rx="2.5" ry="4" fill="#f59e0b"/>
    <ellipse cx="13.5" cy="11" rx="2" ry="3" fill="#fbbf24" transform="rotate(-20 13.5 11)"/>
    <ellipse cx="18.5" cy="11" rx="2" ry="3" fill="#f59e0b" transform="rotate(20 18.5 11)"/>
  `,
  mash: `
    <rect x="10" y="3" width="4" height="13" rx="2" fill="#fed7aa"/>
    <rect x="10.5" y="3" width="3" height="10" rx="1.5" fill="#f97316" opacity="0.3"/>
    <rect x="11" y="9" width="2" height="7" fill="#f97316"/>
    <circle cx="12" cy="17" r="4" fill="#f97316"/>
    <circle cx="12" cy="17" r="2.5" fill="#ea580c"/>
    <rect x="14" y="7" width="2" height="1" rx="0.5" fill="#fb923c"/>
    <rect x="14" y="10" width="2" height="1" rx="0.5" fill="#fb923c"/>
  `,
  water: `
    <path d="M12 3C12 3 4 12 4 16C4 20 7.6 23 12 23C16.4 23 20 20 20 16C20 12 12 3 12 3Z" fill="#38bdf8"/>
    <path d="M12 3C12 3 20 12 20 16C20 20 16.4 23 12 23L12 3Z" fill="#0284c7"/>
    <path d="M8.5 16C8.5 13.5 10 12 12 11.5" stroke="white" stroke-width="1.5" stroke-linecap="round" fill="none" opacity="0.7"/>
  `,
  fermentation: `
    <path d="M7 8L5 19C5 20.1 5.9 21 7 21H17C18.1 21 19 20.1 19 19L17 8Z" fill="#8b5cf6"/>
    <path d="M17 8L19 19C19 20.1 18.1 21 17 21H12L12 8Z" fill="#7c3aed"/>
    <rect x="6" y="6" width="12" height="3" rx="1.5" fill="#a78bfa"/>
    <rect x="11" y="2" width="2" height="5" rx="1" fill="#c4b5fd"/>
    <rect x="9" y="1" width="6" height="2.5" rx="1.25" fill="#a78bfa"/>
    <circle cx="9" cy="14" r="1.5" fill="#c4b5fd" opacity="0.6"/>
    <circle cx="14" cy="12" r="1" fill="#c4b5fd" opacity="0.5"/>
    <circle cx="11" cy="17" r="1" fill="#c4b5fd" opacity="0.4"/>
  `,
  notes: `
    <rect x="4" y="3" width="16" height="18" rx="2" fill="#fbbf24"/>
    <rect x="4" y="3" width="16" height="9" rx="2" fill="#f59e0b"/>
    <rect x="7" y="9" width="10" height="1.5" rx="0.75" fill="white" opacity="0.7"/>
    <rect x="7" y="12" width="10" height="1.5" rx="0.75" fill="white" opacity="0.7"/>
    <rect x="7" y="15" width="7" height="1.5" rx="0.75" fill="white" opacity="0.7"/>
    <path d="M14 4L18 8L16 10L12 6Z" fill="#d97706"/>
    <path d="M12 6L11 9L14 8Z" fill="#92400e"/>
  `,
  fermentable: `
    <line x1="12" y1="22" x2="12" y2="8" stroke="#d97706" stroke-width="2"/>
    <ellipse cx="12" cy="7" rx="3" ry="5" fill="#f59e0b"/>
    <ellipse cx="8.5" cy="11" rx="2.5" ry="4" fill="#fbbf24" transform="rotate(-25 8.5 11)"/>
    <ellipse cx="15.5" cy="11" rx="2.5" ry="4" fill="#f59e0b" transform="rotate(25 15.5 11)"/>
    <ellipse cx="6" cy="15" rx="2" ry="3" fill="#fcd34d" transform="rotate(-30 6 15)"/>
    <ellipse cx="18" cy="15" rx="2" ry="3" fill="#fbbf24" transform="rotate(30 18 15)"/>
  `,
  hop: `
    <path d="M12 4C9 4 6 7 6 12C6 17 9 20 12 20C15 20 18 17 18 12C18 7 15 4 12 4Z" fill="#22c55e"/>
    <path d="M12 4C15 4 18 7 18 12C18 17 15 20 12 20L12 4Z" fill="#16a34a"/>
    <path d="M9 9L12 6L15 9L12 18Z" fill="#bbf7d0" opacity="0.7"/>
    <path d="M7 12C5 10 5 8 7 7" stroke="#4ade80" stroke-width="1.5" fill="none" stroke-linecap="round"/>
    <path d="M17 12C19 10 19 8 17 7" stroke="#4ade80" stroke-width="1.5" fill="none" stroke-linecap="round"/>
  `,
  yeast: `
    <path d="M9 3L15 3L15 10L20 19C20 20.1 19.1 21 18 21L6 21C4.9 21 4 20.1 4 19L9 10Z" fill="#14b8a6"/>
    <path d="M9 3L15 3L15 10L20 19C20 20.1 19.1 21 18 21L12 21L12 10L15 3Z" fill="#0d9488"/>
    <rect x="8" y="2" width="8" height="2" rx="1" fill="#5eead4"/>
    <circle cx="9" cy="16" r="2" fill="white" opacity="0.6"/>
    <circle cx="14" cy="14" r="1.5" fill="white" opacity="0.45"/>
    <circle cx="11" cy="18" r="1" fill="white" opacity="0.35"/>
  `,
};
```

- [ ] **Step 2: Run type-check**

```bash
npm run check
```

Expected: 0 errors, 0 warnings.

- [ ] **Step 3: Commit**

```bash
git add src/lib/icons.ts
git commit -m "feat: replace icon artwork with multi-color filled SVGs"
```

---

## Task 2: Update `BrewingIcon.svelte` — remove stroke attributes, add size prop

**Files:**
- Modify: `src/lib/components/BrewingIcon.svelte`

The new icons use hard-coded `fill` colors, not `stroke="currentColor"`. Remove the stroke attributes from the SVG wrapper so they don't bleed into icon paths. Add a `size` prop (default 18, matching current behavior) so callers can request larger icons.

- [ ] **Step 1: Replace the file**

```svelte
<script lang="ts">
  import { ICONS, type BrewingIconName } from "$lib/icons";

  let { name, size = 18 }: { name: BrewingIconName; size?: number } = $props();
</script>

<svg
  aria-hidden="true"
  data-icon={name}
  width={size}
  height={size}
  viewBox="0 0 24 24"
>
  {@html ICONS[name]}
</svg>
```

- [ ] **Step 2: Run type-check**

```bash
npm run check
```

Expected: 0 errors, 0 warnings.

- [ ] **Step 3: Commit**

```bash
git add src/lib/components/BrewingIcon.svelte
git commit -m "feat: remove stroke attrs from BrewingIcon, add size prop"
```

---

## Task 3: Update `AppShell.svelte` — use BrewingIcon, opacity-based active state

**Files:**
- Modify: `src/lib/desktop/AppShell.svelte`

Replace 6 inline `<svg>` elements with `<BrewingIcon>`. Change active/inactive styling from `color`-based to `opacity`-based: active icons are full opacity (1.0), inactive are 0.45. Remove the accent background fill from active icons — the colorful icon itself provides the visual weight.

- [ ] **Step 1: Replace the file**

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { loadSettings } from "$lib/stores/settings";
  import { lastError } from "$lib/stores/error";
  import BrewingIcon from "$lib/components/BrewingIcon.svelte";

  let { children } = $props();

  onMount(async () => {
    try {
      await loadSettings();
    } finally {
      const { getCurrentWindow } = await import('@tauri-apps/api/window');
      getCurrentWindow().show();
    }
  });

  const isRecipes = $derived($page.url.pathname === "/" || $page.url.pathname.startsWith("/recipe"));
  const isBatches = $derived($page.url.pathname.startsWith("/batches"));
  const isTools = $derived($page.url.pathname.startsWith("/tools"));
  const isEquipment = $derived($page.url.pathname.startsWith("/equipment"));
  const isLibrary = $derived($page.url.pathname.startsWith("/library"));
</script>

<div class="flex h-screen overflow-hidden" style="background: var(--color-bg-base); color: var(--color-text-primary);">
  <!-- Icon rail -->
  <nav class="flex flex-col items-center w-14 py-3 gap-2 border-r flex-shrink-0"
       style="background: var(--color-bg-surface); border-color: var(--color-border);">

    <a href="/" class="w-9 h-9 flex items-center justify-center rounded-lg transition-opacity"
       aria-label="Recipes"
       style={isRecipes ? "opacity: 1;" : "opacity: 0.45;"}>
      <BrewingIcon name="recipes" size={22} />
    </a>

    <a href="/batches" class="w-9 h-9 flex items-center justify-center rounded-lg transition-opacity"
       aria-label="Batches"
       style={isBatches ? "opacity: 1;" : "opacity: 0.45;"}>
      <BrewingIcon name="batches" size={22} />
    </a>

    <a href="/tools" class="w-9 h-9 flex items-center justify-center rounded-lg transition-opacity"
       aria-label="Tools"
       style={isTools ? "opacity: 1;" : "opacity: 0.45;"}>
      <BrewingIcon name="tools" size={22} />
    </a>

    <a href="/equipment" class="w-9 h-9 flex items-center justify-center rounded-lg transition-opacity"
       aria-label="Equipment"
       style={isEquipment ? "opacity: 1;" : "opacity: 0.45;"}>
      <BrewingIcon name="equipment" size={22} />
    </a>

    <a href="/library" class="w-9 h-9 flex items-center justify-center rounded-lg transition-opacity"
       aria-label="Library"
       style={isLibrary ? "opacity: 1;" : "opacity: 0.45;"}>
      <BrewingIcon name="library" size={22} />
    </a>

    <div class="flex-1"></div>

    <a href="/settings" class="w-9 h-9 flex items-center justify-center rounded-lg transition-opacity"
       aria-label="Settings"
       style={$page.url.pathname.startsWith('/settings') ? "opacity: 1;" : "opacity: 0.45;"}>
      <BrewingIcon name="settings" size={22} />
    </a>
  </nav>

  <!-- Main content area -->
  <div class="flex flex-1 overflow-hidden">
    {@render children()}
  </div>
</div>

{#if $lastError}
  <div class="fixed bottom-4 left-1/2 -translate-x-1/2 z-50 flex items-center gap-3 px-4 py-2.5 rounded shadow-lg text-sm"
       style="background: #7f1d1d; color: #fecaca; max-width: 480px;">
    <span class="flex-1 truncate">{$lastError}</span>
    <button onclick={() => lastError.set(null)} class="opacity-70 hover:opacity-100 flex-shrink-0">✕</button>
  </div>
{/if}
```

- [ ] **Step 2: Run type-check**

```bash
npm run check
```

Expected: 0 errors, 0 warnings.

- [ ] **Step 3: Commit**

```bash
git add src/lib/desktop/AppShell.svelte
git commit -m "feat: use BrewingIcon in AppShell nav rail with opacity active state"
```

---

## Task 4: Update `TabBar.svelte` — opacity-based active/inactive icons

**Files:**
- Modify: `src/lib/components/TabBar.svelte`

Icons no longer inherit color from the button, so `color: var(--color-accent)` no longer affects them. Apply `opacity: 0.45` to inactive icons via a wrapper span. The label text still gets the accent color via the button's `color` style.

- [ ] **Step 1: Replace the file**

```svelte
<!-- src/lib/components/TabBar.svelte -->
<script lang="ts">
  import BrewingIcon from "$lib/components/BrewingIcon.svelte";
  import type { BrewingIconName } from "$lib/icons";

  interface Tab {
    key: string;
    label: string;
    icon?: BrewingIconName;
  }

  let {
    tabs,
    active,
    onchange,
  }: {
    tabs: readonly Tab[];
    active: string;
    onchange: (key: string) => void;
  } = $props();
</script>

<div class="flex gap-0 border-b" style="border-color: var(--color-border);">
  {#each tabs as tab}
    <button
      onclick={() => onchange(tab.key)}
      class="px-4 py-2.5 text-sm transition-colors inline-flex items-center gap-1.5 border-b-2 -mb-px"
      style={active === tab.key
        ? "color: var(--color-accent); border-color: var(--color-accent); background: transparent;"
        : "color: var(--color-text-secondary); border-color: transparent; background: transparent;"}
    >
      {#if tab.icon}
        <span style={active === tab.key ? "opacity: 1;" : "opacity: 0.45;"}>
          <BrewingIcon name={tab.icon} size={20} />
        </span>
      {/if}
      {tab.label}
    </button>
  {/each}
</div>
```

- [ ] **Step 2: Run type-check**

```bash
npm run check
```

Expected: 0 errors, 0 warnings.

- [ ] **Step 3: Commit**

```bash
git add src/lib/components/TabBar.svelte
git commit -m "feat: opacity-based active/inactive state for tab bar icons"
```

---

## Task 5: Update `BottomTabBar.svelte` — switch to BrewingIcon

**Files:**
- Modify: `src/lib/mobile/BottomTabBar.svelte`

The mobile bottom tab bar has its own inline SVG paths. Replace them with `BrewingIcon` and apply the same opacity treatment: `opacity: 1` for active, `opacity: 0.45` for inactive. The mobile nav has 4 tabs (Recipes, Batches, Tools, More/Settings); "More" maps to the `settings` icon.

- [ ] **Step 1: Replace the file**

```svelte
<script lang="ts">
  import { page } from "$app/stores";
  import BrewingIcon from "$lib/components/BrewingIcon.svelte";

  const TABS = [
    {
      label: "Recipes",
      href: "/",
      activeWhen: (p: string) => p === "/" || p.startsWith("/recipe"),
      icon: "recipes",
    },
    {
      label: "Batches",
      href: "/batches",
      activeWhen: (p: string) => p.startsWith("/batches"),
      icon: "batches",
    },
    {
      label: "Tools",
      href: "/tools",
      activeWhen: (p: string) => p.startsWith("/tools"),
      icon: "tools",
    },
    {
      label: "More",
      href: "/settings",
      activeWhen: (p: string) => p.startsWith("/settings") || p.startsWith("/equipment") || p.startsWith("/library"),
      icon: "settings",
    },
  ] as const;
</script>

<nav
  class="flex border-t flex-shrink-0"
  style="background: var(--color-bg-surface); border-color: var(--color-border); padding-bottom: env(safe-area-inset-bottom, 0px);"
>
  {#each TABS as tab}
    {@const active = tab.activeWhen($page.url.pathname)}
    <a
      href={tab.href}
      aria-current={active ? "page" : undefined}
      class="flex flex-col items-center justify-center flex-1 py-2 gap-1 text-xs transition-colors"
      style={active ? "color: var(--color-accent);" : "color: var(--color-text-secondary);"}
    >
      <span style={active ? "opacity: 1;" : "opacity: 0.45;"}>
        <BrewingIcon name={tab.icon} size={22} />
      </span>
      {tab.label}
    </a>
  {/each}
</nav>
```

- [ ] **Step 2: Run type-check**

```bash
npm run check
```

Expected: 0 errors, 0 warnings.

- [ ] **Step 3: Run full test suite**

```bash
npm test
```

Expected: all tests pass (55+).

- [ ] **Step 4: Commit**

```bash
git add src/lib/mobile/BottomTabBar.svelte
git commit -m "feat: use BrewingIcon in mobile bottom tab bar"
```

---

## Visual Verification

After all tasks are committed, run the dev server and verify manually:

```bash
npm run dev
```

Open the app and check:
1. **Nav rail**: All 6 icons show in color. Active icon is full brightness. Inactive icons are visibly dimmed (opacity 0.45). No background tint or glow.
2. **Recipe tab bar**: Overview, Ingredients, Mash, Water, Fermentation, Notes, Batches — active tab icon is full brightness with accent underline, inactive are dimmed.
3. **Ingredient tables**: Fermentable (grain), Hop, Yeast icons appear in `FermentablesTable`, `HopsTable`, `YeastsTable` headings — amber grain, green hop cone, teal flask.
4. **Mobile** (if testable): Bottom tab bar shows 4 colorful icons with same opacity behavior.
