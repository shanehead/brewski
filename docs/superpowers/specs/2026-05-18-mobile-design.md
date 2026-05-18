# Mobile Support Design

**Date:** 2026-05-18
**Approach:** Shell-first — Tauri mobile targets → navigation shell → screens → touch polish

## Goals

Make Brewski run natively on iOS and Android via Tauri 2's mobile support. The layout adapts per device class: phones get a simplified single-scroll experience with bottom tab navigation; tablets retain the existing sidebar layout.

## Device Breakpoints

| Class | Width | Navigation |
|-------|-------|------------|
| Phone | < 640px | Bottom tab bar, full-screen panels |
| Tablet | ≥ 640px | Existing icon rail + sidebars |

Breakpoint is detected at runtime via a Svelte store watching `window.innerWidth`.

---

## Phase 1 — Tauri Mobile Setup + Navigation Shell

### Tauri mobile init

- Run `cargo tauri ios init` and `cargo tauri android init` to scaffold the mobile projects.
- Add file storage permissions to `Info.plist` / `AndroidManifest.xml` as needed (database file access).
- The SQLite DB path uses Tauri's `app_data_dir()` which already resolves correctly on mobile.

### New: `src/lib/stores/breakpoint.ts`

Exports two derived booleans:

```ts
export const isMobile: Readable<boolean>  // window.innerWidth < 640
export const isTablet: Readable<boolean>  // window.innerWidth >= 640
```

Updated on `window` resize via an event listener mounted once in AppShell.

### New: `src/lib/components/BottomTabBar.svelte`

Four tabs: **Recipes** (`/`), **Batches** (`/batches`), **Tools** (`/tools`), **More** (`/settings`).

- Uses the same SVG icons already in `AppShell.svelte`.
- Active tab derived from `$page.url.pathname`.
- Bottom padding accounts for iOS safe area: `padding-bottom: env(safe-area-inset-bottom, 0px)`.

### Updated: `src/lib/components/AppShell.svelte`

```
if isMobile:
  flex-col layout
    {@render children()}  ← full height, scrollable
    <BottomTabBar />      ← fixed at bottom
else:
  flex-row layout (unchanged)
    <nav icon rail />
    {@render children()}
```

The icon rail is hidden on mobile. The `h-screen overflow-hidden` root div gains a safe-area top padding for the iOS status bar.

---

## Phase 2 — Recipe Page (Phone)

### Navigation flow

- `/` and `/recipe/[id]` both include `RecipeList` as a sidebar today.
- On phone: `RecipeList` renders full-screen when there is no selected recipe. Tapping a recipe navigates to `/recipe/[id]`, which renders `RecipeMobileView` full-screen with a `‹ Recipes` back button.
- On tablet/desktop: unchanged.

### New: `src/lib/components/RecipeMobileView.svelte`

Single scrollable column with all recipe sections in order. Each section has a small uppercase label header. Sections:

1. **Stats card** — OG, FG, ABV, IBU, SRM as a compact card. Scrolls away with the page.
2. **Overview** — style, batch size, efficiency, boil time, description.
3. **Fermentables** — list rows + "+ Add fermentable" button.
4. **Hops** — list rows + "+ Add hop" button.
5. **Yeast** — list rows + "+ Add yeast" button.
6. **Mash** — mash profile + steps.
7. **Water** — mineral targets.
8. **Fermentation** — schedule and temperature steps.
9. **Notes** — textarea.

Each section reuses the existing tab components (`IngredientsTab`, `MashTab`, etc.) where possible, extracting only the content portions without tab chrome.

### Updated: `src/routes/recipe/[id]/+page.svelte`

```svelte
{#if $isMobile}
  <!-- full-screen detail, back button in header -->
  <RecipeMobileView {recipe} {stats} onchange={refreshRecipe} />
{:else}
  <!-- existing 3-panel layout unchanged -->
{/if}
```

The `RecipeList` aside is conditionally hidden on mobile when a recipe is selected.

### Tablet: stats sidebar

On tablet the `StatsSidebar` is hidden and stats appear as an inline card at the top of the Overview tab content. This reduces the 3-panel layout to 2 panels on tablet, giving more room for the content area.

---

## Phase 3 — Batch Page (Phone)

Same pattern as Phase 2.

### Navigation flow

- `/batches` → full-screen `BatchList` on phone.
- `/batches/[id]` → full-screen `BatchMobileView` with `‹ Batches` back button.

### New: `src/lib/components/BatchMobileView.svelte`

Single scrollable column with all batch sections in order:

1. **Status** — existing `TabBar` with planned/brewing/fermenting/packaged/complete.
2. **Dates** — brew date, into fermenter, packaging date.
3. **Measurements** — gravity and volume fields.
4. **Gravity Log** — entries list + add entry.
5. **Notes** — textarea.
6. **Tasting** — tasting notes and rating.

Reuses existing `BatchOverviewTab`, `BatchGravityTab`, `BatchNotesTab`, `BatchTastingTab` content, stripped of tab chrome.

### Updated: `src/routes/batches/[id]/+page.svelte`

```svelte
{#if $isMobile}
  <BatchMobileView {batch} onUpdate={handleUpdate} />
{:else}
  <!-- existing tabbed layout unchanged -->
{/if}
```

---

## Phase 4 — Touch Polish

- **Tap targets:** All interactive elements (buttons, list rows, inputs) get a minimum height of 44px on mobile.
- **Number inputs:** On mobile, `type="number"` inputs show a numeric keyboard. Step spinners (↑↓) are hidden via CSS on mobile; values are edited by tapping the field.
- **`IngredientPicker`:** On mobile, the picker renders as a full-screen modal (100vw × 100vh) instead of the current fixed-size popover. Close button in the header.
- **`ConfirmModal`:** Already uses a fixed overlay; no changes needed.
- **Date inputs:** Native date pickers (`type="date"`) work out-of-the-box on iOS/Android. No changes needed.

---

## What Stays the Same

- All Rust backend code — no changes. SQLite, commands, migrations are all platform-agnostic.
- All desktop layouts — mobile changes are additive, gated by `$isMobile`.
- Routing structure — same SvelteKit routes, mobile/desktop branching happens inside route components.
- Theme — dark midnight theme works on mobile as-is.

---

## Out of Scope

- Push notifications / background sync
- Camera integration (e.g. barcode scanning for ingredients)
- Offline mode / conflict resolution
- App store distribution setup (signing, provisioning profiles)
