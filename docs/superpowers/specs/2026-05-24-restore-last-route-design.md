# Restore Last Route on Launch

**Date:** 2026-05-24  
**Status:** Approved

## Goal

When the app is reopened, navigate directly to the last screen the user was on — including exact paths like `/recipe/abc-123` or `/tools/abv-calories`.

## Approach

Use the existing settings store (SQLite, via `update_setting` IPC). No new Rust code is required. `last_route` becomes an ordinary free-form setting key.

## Changes

### 1. `src/lib/stores/settings.ts`

Add `last_route?: string` to the `AppSettings` interface.

### 2. `src/lib/desktop/AppShell.svelte`

- Import `afterNavigate` from `$app/navigation`.
- In the `afterNavigate` callback, call `saveSetting('last_route', navigation.to.url.pathname)`.
- In `onMount`, after `await loadSettings()` resolves, if `$settings.last_route` is set and differs from the current path (`$page.url.pathname`), call `goto($settings.last_route)`.

### 3. `src/lib/mobile/AppShell.svelte`

Same two changes as the desktop shell.

## Behaviour Details

- **Scope:** The full `pathname` is saved — `/recipe/abc-123`, `/tools/carbonation`, `/batches/xyz`, etc.
- **404 handling:** No special handling. Existing pages already deal gracefully with missing records (deleted recipe → shows an empty/error state). This is acceptable.
- **First launch:** `last_route` is absent from settings; `onMount` skips the `goto` and the app opens on `/` as today.
- **Settings page:** `/settings` is a valid route and will be restored like any other.
- **Write frequency:** One upsert per navigation — negligible overhead on the settings table.

## Out of scope

- Scroll position within a page.
- Restoring open modals or tab state within a page.
- Any UI preference or settings page to disable this behaviour.
