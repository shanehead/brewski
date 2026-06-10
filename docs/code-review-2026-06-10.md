# Brewski Code Review — 2026-06-10

Reviewed: full codebase sweep for AI slop, bad practices, maintainability, and software design issues. Focus areas: the Svelte component layer, the recipe-version repository, and the brewing/IPC boundaries.

---

## High impact

### 1. 957 inline `style=` attributes instead of Tailwind theme tokens — ✅ DONE (2026-06-10)

> **Resolved.** Added a `@theme inline` block in `src/app.css` registering the `--color-*` tokens (which live in `src/themes/*.css` and swap at runtime), generating `bg-*`/`text-*`/`border-*` utilities that resolve against the active theme. Converted 1,640 color declarations across 59 components to utilities; inline color-var styles dropped from 957 → 38 (the remainder are interpolated `{…}` / `color-mix()` styles that can't be utilities). Also defined the two previously-undefined tokens — `--color-text-tertiary` (muted blended toward bg) and `--color-text-danger` (feedback red) — directly in the `@theme` block. Verified: `svelte-check` 0 errors, `vitest` 212 passing, production build emits utilities referencing the theme vars.

The app uses Tailwind v4 (`@import "tailwindcss"` in `src/app.css`) but defines its design tokens in `:root` **without a `@theme` block**, so theme colors can't be used as utility classes. Every component instead hardcodes the colors as inline styles, mixed onto the same elements that already carry Tailwind classes (`class="px-2 py-1.5 rounded" style="color: var(--color-text-primary);"`).

The same verbatim strings repeat across the codebase:

- `style="color: var(--color-text-secondary);"` — **207 times**
- `style="color: var(--color-text-primary);"` — 129 times
- `style="color: var(--color-text-muted);"` — 93 times
- `style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);"` — 60 times (the select-input styling)

1093 inline `style=` attributes total, 957 of them referencing CSS vars. This is the classic verbose, error-prone "slop" pattern: impossible to restyle globally and noisy to read.

**Suggestion:** add a Tailwind v4 `@theme` block mapping the existing `--color-*` tokens, then replace inline styles with utilities (`text-text-secondary`, `bg-bg-elevated`, `border-border`). Deletes ~950 inline strings and makes theming a one-place change. Highest-leverage cleanup in the repo.

### 2. `matches_current` is a fragile manual field allowlist with a correctness gap

`src-tauri/src/repositories/recipe_version.rs:98` decides whether a re-brew creates a new recipe version by comparing fields one at a time. It checks **17 scalar fields**, but the recipe has ~30 user-editable ones. Notably missing:

- `equipment_profile_id` — **switching equipment profiles will not create a new version** (silent gap in version history)
- `fermentation_stages`, `age_days` / `age_temp_c`, `tertiary_*`, `forced_carbonation`, `priming_sugar_*`, `carbonation_temp_c`, `hopstand_temp_c`

The same field list must stay in sync across three large functions in this file — `matches_current`, `snapshot` (`recipe_version.rs:312`), and `get_full` (`recipe_version.rs:788`). Adding a recipe field means editing all three with no compile-time check that you did.

**Suggestion:** compare a serialized snapshot + hash, or drive all three functions from one shared field-mapping struct. Removes both the latent bug class and the triple maintenance.

---

## Medium

### 3. Dead code: a stale 23KB component

`src/lib/components/ingredients/IngredientPicker.svelte` (655 lines) is imported nowhere. It was superseded by the `$platform`-aliased `src/lib/desktop/IngredientPicker.svelte` / `src/lib/mobile/IngredientPicker.svelte` versions but never deleted. Safe to remove.

### 4. Duplicated handler logic across desktop/mobile views

The desktop/mobile split is a deliberate pattern, but `handleExport`, `handleImageUpload`, and `handleImageRemove` are copy-pasted between `src/lib/desktop/RecipeView.svelte` and `src/lib/mobile/RecipeView.svelte`. These are platform-agnostic and belong in a shared `$lib` helper.

---

## Low / notes

- **`#![allow(dead_code)]`** was added to `src-tauri/src/models.gen.rs` to silence a warning on the unused `default_i64` helper. Fine for generated code, but it suppresses dead-code warnings for the whole file. If the generator can emit a targeted `#[allow]` on just that function, that's cleaner. Not worth hand-editing generated output.
- **`dest.parent().unwrap()`** at `src-tauri/src/commands/recipe_image.rs:30` can panic on a pathological path. Minor; worth a graceful error.

---

## What's genuinely good

- The brewing math (`src-tauri/src/brewing/`) uses named constants for defaults/tolerances and keeps standard Tinseth/ABV formula coefficients inline where appropriate — clean and well-tested.
- `src/lib/api.ts` is a tidy, fully-typed IPC layer.
- Almost no `as any`, no TODO/FIXME debt, and production code is nearly panic-free (unwraps are confined to tests).
