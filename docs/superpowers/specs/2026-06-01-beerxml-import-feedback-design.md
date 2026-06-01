# BeerXML Import Feedback — Design Spec

**Date:** 2026-06-01

## Context

BeerXML import is already wired up on both desktop (`RecipeList.svelte`) and mobile (`mobile/RecipesHome.svelte`): a hidden file input + "Import BeerXML" button calls `createRecipesFromBeerxml`. What's missing is any feedback — no loading state, no success message, and silent failure (handled only by the error store).

## Goal

Add a loading state to the Import button and a success toast that reports how many recipes were imported.

## Design

### 1. `src/lib/stores/error.ts` — add `lastSuccess`

Add a `lastSuccess` writable and a `setSuccess(message)` helper that sets the store and auto-clears after 3 seconds:

```ts
export const lastSuccess = writable<string | null>(null);

export function setSuccess(message: string) {
  lastSuccess.set(message);
  setTimeout(() => lastSuccess.set(null), 3000);
}
```

No changes to `lastError` or `ipc()`.

### 2. AppShells — display success banner

Both `src/lib/desktop/AppShell.svelte` and `src/lib/mobile/AppShell.svelte` already show a `lastError` banner. Add a sibling `lastSuccess` banner with green styling (using `--color-accent` or a dedicated success colour) and a dismiss button. The banner auto-dismisses via the store timeout; the `✕` button lets users dismiss it early.

```
┌─────────────────────────────────┐
│ ✓ 2 recipes imported        [✕] │  ← green, auto-dismisses in 3 s
└─────────────────────────────────┘
```

### 3. Import handler — loading state + success call

In both `RecipeList.svelte` and `mobile/RecipesHome.svelte`, update `handleImport`:

- Add `let importing = $state(false)`.
- Set `importing = true` before the `ipc()` call; set `importing = false` after (in a `finally` or after the call).
- Disable the Import button and change its label to `"Importing…"` while `importing` is true.
- On success (non-null result): call `setSuccess(\`${n} recipe${n === 1 ? "" : "s"} imported\`)`.
- On failure: the existing `ipc()` error path sets `lastError`; no additional handling needed.

## Files changed

| File | Change |
|---|---|
| `src/lib/stores/error.ts` | Add `lastSuccess` + `setSuccess` |
| `src/lib/desktop/AppShell.svelte` | Render `lastSuccess` banner |
| `src/lib/mobile/AppShell.svelte` | Render `lastSuccess` banner |
| `src/lib/components/RecipeList.svelte` | Loading state + success call |
| `src/lib/mobile/RecipesHome.svelte` | Loading state + success call |

## Out of scope

- Navigation to the imported recipe (toast is sufficient).
- Multi-file import (single file only, as today).
- Success toasts for any other action.
