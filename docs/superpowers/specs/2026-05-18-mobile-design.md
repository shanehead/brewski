# Mobile Support Design

**Date:** 2026-05-18
**Approach:** Shell-first — Tauri mobile targets → navigation shell → screens → touch polish

## Goals

Make Brewski run natively on iOS and Android via Tauri 2's mobile support. Each platform gets a clean, purpose-built implementation with no branching conditionals in shared code.

## Platform Classes

| Class | `TAURI_ENV_PLATFORM` | Navigation |
|-------|----------------------|------------|
| Mobile | `ios`, `android` | Bottom tab bar, full-screen panels, single-scroll views |
| Desktop | `macos`, `windows`, `linux` | Existing icon rail + sidebars + tabbed views |

---

## Core Architecture: Vite Platform Alias

Platform separation is resolved at **build time** via a Vite module alias. No runtime detection, no branching `{#if}` blocks, no dead code shipped.

```ts
// vite.config.ts
const mobile = process.env.TAURI_ENV_PLATFORM === "ios"
            || process.env.TAURI_ENV_PLATFORM === "android";

resolve: {
  alias: {
    "$platform": path.resolve(`./src/lib/${mobile ? "mobile" : "desktop"}`)
  }
}
```

Route files import from `$platform` — they stay thin and platform-agnostic:

```svelte
<!-- src/routes/recipe/[id]/+page.svelte -->
import RecipeView from "$platform/RecipeView.svelte";
```

When building for iOS/Android, Vite resolves `$platform` to `src/lib/mobile/`. For desktop it resolves to `src/lib/desktop/`. The unused platform's components are tree-shaken out entirely.

### File structure

```
src/lib/
  mobile/
    AppShell.svelte       ← bottom tab bar, safe area insets, full-screen panels
    RecipeView.svelte     ← single-scroll recipe editor
    BatchView.svelte      ← single-scroll batch tracker
    BottomTabBar.svelte   ← tab bar component (Recipes, Batches, Tools, More)
  desktop/
    AppShell.svelte       ← existing icon rail + sidebars (moved from AppShell.svelte)
    RecipeView.svelte     ← existing tabbed 3-panel view (extracted from route page)
    BatchView.svelte      ← existing tabbed view (extracted from route page)
```

Route pages become thin shells that import from `$platform` and pass data through. All layout logic lives in the platform-specific files.

---

## Phase 1 — Tauri Mobile Setup + Build Config

### Tauri mobile init

- Run `cargo tauri ios init` and `cargo tauri android init` to scaffold the mobile projects.
- Add file storage permissions to `Info.plist` / `AndroidManifest.xml` as needed for SQLite database access.
- The SQLite DB path uses Tauri's `app_data_dir()` which already resolves correctly on mobile — no Rust changes needed.

### Vite config update

Add the `$platform` alias to `vite.config.ts` as described above. Works transparently in dev: without `TAURI_ENV_PLATFORM` set, defaults to desktop (the existing behaviour).

### Move existing AppShell

Rename `src/lib/components/AppShell.svelte` → `src/lib/desktop/AppShell.svelte`. Update the import in `src/routes/+layout.svelte` to use `$platform/AppShell.svelte`.

### New: `src/lib/mobile/AppShell.svelte`

```
flex-col, h-screen
  <div class="flex-1 overflow-hidden">
    {@render children()}
  </div>
  <BottomTabBar />
```

- Top padding: `env(safe-area-inset-top)` for iOS status bar.
- No icon rail.

### New: `src/lib/mobile/BottomTabBar.svelte`

Four tabs: **Recipes** (`/`), **Batches** (`/batches`), **Tools** (`/tools`), **More** (`/settings`).

- Reuses the SVG icons from the existing desktop AppShell.
- Active tab derived from `$page.url.pathname`.
- Bottom padding: `env(safe-area-inset-bottom, 0px)` for iOS home indicator.

---

## Phase 2 — Recipe Page

### Route change

`src/routes/recipe/[id]/+page.svelte` becomes a thin data-loading shell:

```svelte
import RecipeView from "$platform/RecipeView.svelte";
// ... existing load/save logic unchanged ...
<RecipeView {recipe} {stats} onchange={refreshRecipe} />
```

### New: `src/lib/mobile/RecipeView.svelte`

Full-screen view with a `‹ Recipes` back button in the header. Single scrollable column with sections in order:

1. **Stats card** — OG, FG, ABV, IBU, SRM. Scrolls away with the page.
2. **Overview** — style, batch size, efficiency, boil time, description.
3. **Fermentables** — list + "+ Add fermentable".
4. **Hops** — list + "+ Add hop".
5. **Yeast** — list + "+ Add yeast".
6. **Mash** — profile + steps.
7. **Water** — mineral targets.
8. **Fermentation** — schedule and temperature.
9. **Notes** — textarea.

Each section has a small uppercase label header. Reuses existing leaf components (ingredient rows, pickers, etc.) — only the surrounding tab/panel chrome is replaced.

On mobile, `RecipeList` is the full-screen home for the Recipes tab (`/`). At `/recipe/[id]` the list is not rendered — the detail view fills the screen.

### New: `src/lib/desktop/RecipeView.svelte`

The existing 3-panel layout (RecipeList aside + tabbed content + StatsSidebar) extracted from the current route page verbatim. No behaviour change.

---

## Phase 3 — Batch Page

Same pattern as Phase 2.

`src/routes/batches/[id]/+page.svelte` → thin shell importing `$platform/BatchView.svelte`.

### New: `src/lib/mobile/BatchView.svelte`

Full-screen view with `‹ Batches` back button. Single scrollable column:

1. **Status** — TabBar with planned/brewing/fermenting/packaged/complete.
2. **Dates** — brew date, into fermenter, packaging date.
3. **Measurements** — gravity and volume fields.
4. **Gravity Log** — entries list + add entry.
5. **Notes** — textarea.
6. **Tasting** — tasting notes and rating.

### New: `src/lib/desktop/BatchView.svelte`

Existing tabbed layout extracted from the current route page verbatim.

---

## Phase 4 — Touch Polish

- **Tap targets:** Buttons and list rows get `min-height: 44px` in mobile-specific CSS.
- **Number inputs:** Step spinners hidden on mobile via CSS; numeric keyboard via `inputmode="decimal"`.
- **`IngredientPicker`:** Full-screen on mobile (`position: fixed; inset: 0`) instead of the current fixed-size popover.
- **Date inputs:** Native date pickers work out-of-the-box on iOS/Android.

---

## What Stays the Same

- All Rust backend code — no changes at all.
- All existing desktop components — moved to `src/lib/desktop/`, behaviour unchanged.
- SvelteKit routing structure — same routes, pages just become thinner.
- Theme — midnight CSS variables work on mobile as-is.

---

## Out of Scope

- Push notifications / background sync
- Camera integration (barcode scanning for ingredients)
- Offline mode / conflict resolution
- App store distribution (signing, provisioning profiles)
