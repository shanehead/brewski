# Per-Section Navigation State

**Date:** 2026-06-08
**Status:** Approved

## Goal

When the user switches between rail sections (Recipes, Batches, Tools, etc.), clicking back into a section restores the exact URL they were last on — including which record was open and which tab was selected within that record. State persists across app restarts.

## Scope

- Desktop icon rail (AppShell)
- Mobile bottom tab bar (BottomTabBar + mobile AppShell)
- Recipe tab selection (Overview, Ingredients, Mash, Water, Fermentation, Notes, Batches)
- Batch status tabs are data, not UI state — already persisted in DB, no change needed

## Settings Keys

Add 6 new optional string keys to `AppSettings` in `settings.ts`:

| Key | Section | Default fallback |
|-----|---------|-----------------|
| `last_route_recipes` | `/`, `/recipe/*`, `/baseline-recipe/*` | `/` |
| `last_route_batches` | `/batches/*` | `/batches` |
| `last_route_tools` | `/tools/*` | `/tools` |
| `last_route_equipment` | `/equipment*` | `/equipment` |
| `last_route_library` | `/library*` | `/library` |
| `last_route_settings` | `/settings*` | `/settings` |

These are stored as strings via the existing `saveSetting` / `updateSetting` mechanism. No backend schema changes needed — the settings table is already a free-form key-value store.

## URL as Source of Truth for Tab State

Currently `activeTab` in `RecipeView.svelte` is local `$state`. It resets to `"overview"` on every navigation. Move it into the URL as a query param:

- Active URL shape: `/recipe/abc123?tab=ingredients`
- `activeTab` becomes `$derived($page.url.searchParams.get('tab') ?? 'overview')`
- Tab changes call `goto(\`/recipe/${id}?tab=${key}\`, { replaceState: true, noScroll: true })`
  - `replaceState: true` — tab switches don't create browser history entries
  - `noScroll: true` — prevents scroll position jumping

## afterNavigate Changes

Both `AppShell` components currently save `to.url.pathname` to `last_route`. Change to:

1. Save `to.url.pathname + to.url.search` (captures query params like `?tab=ingredients`)
2. Additionally detect which section the new URL belongs to and save to the matching section key

Helper (inline in each AppShell):

```ts
function sectionKeyFor(pathname: string): keyof AppSettings | null {
  if (pathname === '/' || pathname.startsWith('/recipe') || pathname.startsWith('/baseline-recipe'))
    return 'last_route_recipes';
  if (pathname.startsWith('/batches')) return 'last_route_batches';
  if (pathname.startsWith('/tools'))   return 'last_route_tools';
  if (pathname.startsWith('/equipment')) return 'last_route_equipment';
  if (pathname.startsWith('/library')) return 'last_route_library';
  if (pathname.startsWith('/settings')) return 'last_route_settings';
  return null;
}
```

## Desktop Rail Icon Navigation

Change the 6 rail `<a href="...">` elements to buttons with `onclick` handlers. Each uses its section key with a fallback:

```svelte
<button onclick={() => goto($settings.last_route_recipes ?? '/')}>
  <BrewingIcon name="recipes" size={22} />
</button>
```

Preserve the existing active-state highlighting logic (derived from `$page.url.pathname`).

## Mobile Bottom Tab Bar

The `BottomTabBar` "More" tab covers `/settings`, `/equipment`, and `/library`. It navigates to `$settings.last_route_settings ?? '/settings'`. If the user was last on Equipment or Library, they have separate section keys saved — clicking into "More" lands on settings as the entry point, which is fine for the mobile grouped tab.

The Recipes, Batches, and Tools tabs follow the same pattern as desktop.

`BottomTabBar` needs access to the settings store to read section keys. Currently it only uses `page`. Import `settings` store and use it for `onclick` navigation, same as the desktop rail.

## Startup Restore

The existing `onMount` logic in both AppShells restores `last_route` on startup. Since `last_route` now includes query params (e.g., `/recipe/abc123?tab=mash`), startup restore automatically brings back both the record and the tab. No other change needed here.

## Files Changed

1. `src/lib/stores/settings.ts` — add 6 keys to `AppSettings`
2. `src/lib/desktop/AppShell.svelte` — section tracking in `afterNavigate`, dynamic rail nav
3. `src/lib/mobile/AppShell.svelte` — section tracking in `afterNavigate`
4. `src/lib/mobile/BottomTabBar.svelte` — import settings, dynamic tab navigation
5. `src/lib/desktop/RecipeView.svelte` — `activeTab` from URL query param, tab changes via `goto`
