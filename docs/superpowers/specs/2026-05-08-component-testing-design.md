# Component Testing Foundation Design

**Date:** 2026-05-08
**Goal:** Set up Vitest + @testing-library/svelte as the component testing foundation for Brewski, migrate the existing bun:test unit tests to Vitest, and write the first component tests covering the MashTab strike-temp display flow.

---

## Motivation

The infuse-amount input was shipped without a UI input binding — discovered only by manual inspection. Type checking (`just check-ts`) cannot catch a missing `<input>` element. A component test that renders MashTab with appropriate props and asserts on the DOM would have caught this immediately.

---

## Architecture

### Test runner

**Vitest** replaces `bun:test`. All existing tests (`units.test.ts`) migrate to Vitest with no logic changes — only import paths update (`./units` → `$lib/units`).

Vitest is chosen over bun:test for component testing because it integrates with the Vite/SvelteKit plugin pipeline, which is required to compile `.svelte` files and resolve `$lib` aliases inside tests.

### DOM environment

**happy-dom** — faster than jsdom, sufficient for the DOM APIs used in Brewski's components.

### Config file

A separate `vitest.config.ts` merges the existing `vite.config.ts` so the SvelteKit plugin (which sets up `$lib` path aliases and compiles Svelte files) is inherited automatically:

```ts
import { defineConfig, mergeConfig } from 'vitest/config';
import viteConfig from './vite.config';

export default mergeConfig(viteConfig, defineConfig({
  test: {
    environment: 'happy-dom',
    include: ['tests/**/*.test.ts'],
    setupFiles: ['tests/setup.ts'],
    globals: true,
  },
}));
```

### Package scripts

```json
"test": "vitest run",
"test:watch": "vitest"
```

---

## Directory layout

```
tests/
  setup.ts          ← jest-dom matchers + settings store mock
  units.test.ts     ← moved from src/lib/units.test.ts
  MashTab.test.ts   ← new
```

Flat — no subdirectories until there's a reason to split.

---

## Mock strategy

### Tauri invoke — per-test, Option A

Each test file mocks `@tauri-apps/api/core` at the module level and configures return values per test:

```ts
import { invoke } from '@tauri-apps/api/core';
vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));

// per test:
vi.mocked(invoke).mockResolvedValueOnce(returnValue);
```

**Rationale:** Mock config lives close to the test that uses it. Tests that don't trigger Tauri calls need no configuration — `vi.fn()` returns `undefined` by default.

### Settings store — global mock in setup.ts

MashTab reads `$settings.units` to determine display units. The store calls `invoke` internally on init, which would require non-trivial configuration in every test. Instead, `tests/setup.ts` mocks the store module globally with a metric default:

```ts
import { readable } from 'svelte/store';
vi.mock('$lib/stores/settings', () => ({
  settings: readable({ units: 'metric' }),
}));
```

Tests that need to assert imperial display can override this per-test with `vi.mocked`.

---

## MashTab tests

**Fixtures:** Minimal inline `Recipe` and `RecipeStats` objects — only the fields MashTab reads. No shared fixture files; each test constructs what it needs.

### Strike temp display (2 tests)

| # | Scenario | Props | Assertion |
|---|---|---|---|
| 1 | Strike temp rendered | `stats.strike_temp_c: 69.82`, recipe with mash step | Text "69.8°C" is in the document |
| 2 | Strike temp hidden | `stats.strike_temp_c: null` | Strike temp row is not in the document |

These are pure render tests — no user interaction.

### Infuse amount input in Add Step form (2 tests)

| # | Scenario | Interaction | Assertion |
|---|---|---|---|
| 3 | Infuse input visible for infusion type | Click "+ Add Step", step type is "infusion" (default) | Infuse amount input is in the document |
| 4 | Infuse input hidden for non-infusion type | Click "+ Add Step", change type to "temperature" | Infuse amount input is not in the document |

These use `@testing-library/user-event` to click and change the select.

### Water:grain ratio fallback input (2 tests)

| # | Scenario | Props | Assertion |
|---|---|---|---|
| 5 | Ratio input shown — no infuse amount | Recipe with mash, steps without `infuse_amount_l` | Ratio input is in the document |
| 6 | Ratio input hidden — auto-derive possible | Recipe with mash, step with `infuse_amount_l` set, and fermentables | Ratio input is not in the document |

Pure render tests.

---

## Packages to install

All dev dependencies:

| Package | Purpose |
|---|---|
| `vitest` | Test runner |
| `@testing-library/svelte` | Component rendering and DOM queries |
| `@testing-library/jest-dom` | DOM matchers (`toBeInTheDocument`, etc.) |
| `@testing-library/user-event` | Realistic user interaction simulation |
| `happy-dom` | DOM environment |

---

## What this does not cover

- End-to-end tests (Playwright) — not in scope
- Backend Rust tests — already covered by `just test-rust`
- `$app/navigation` mocking — not needed for MashTab (it doesn't call `goto`)
