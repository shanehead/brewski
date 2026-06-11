# Recipe Version Selection & Change Detection — Design

**Date:** 2026-06-10
**Status:** Approved (pending spec review)
**Related:** [Code Review 2026-06-10](../../code-review-2026-06-10.md) item #2

## Problem

When a batch is created it is pinned to an immutable recipe-version snapshot
(`batches.recipe_version_id`) so it records exactly what was brewed. Today the
version to pin is resolved by `RecipeVersionRepository::create_or_reuse`, which
calls `matches_current` — a hand-maintained, field-by-field comparison of the
live recipe against the most recent snapshot. If they "match," the existing
snapshot is silently reused; otherwise a new one is created.

Two problems:

1. **Silent, fragile change detection.** `matches_current` compares only 17 of
   the recipe's ~30 scalar fields and omits `equipment_profile_id` entirely. A
   missed field means a genuine change is read as "no change," so the new batch
   is silently pinned to a stale snapshot that does not reflect what is being
   brewed. The same field list is duplicated across `matches_current`,
   `snapshot`, and `get_full`, so it drifts.
2. **No user visibility.** The reuse/snapshot decision is invisible. The user is
   never told their recipe has un-versioned changes, and (when only one version
   exists) is never offered a choice of which version to brew.

## Goals

- Replace silent auto-reuse with **explicit version selection** at brew time.
- **Surface un-versioned changes** to the user (at brew time and on the recipe
  view) instead of resolving them silently.
- Make change detection **total and order-independent** via a content hash, so
  no field can be silently forgotten.
- Keep zero friction when the recipe is unchanged.

## Non-goals

- Freezing equipment-profile or source-water-profile **values** into snapshots.
  These are treated as shared, live environment (see Decisions). Their values are
  intentionally not versioned.
- Reworking the `branch_from` flow (it already always snapshots) beyond storing
  the new `content_hash`.

## Decisions (from brainstorming)

1. **Version picker always shown:** "New Batch" always opens the version picker
   modal. It defaults to the latest saved version. A warning appears when there
   are un-versioned changes or no versions yet. This gives users full visibility
   and control in every case.
2. **New-version naming:** "Brew with current changes" offers an *optional* name,
   defaulting to the next auto-number.
3. **Dirty indicator placement:** both the batch-creation prompt **and** a
   persistent badge on the recipe view.
4. **Detection mechanism:** a content hash stored on each version, computed from
   a single canonical projection (no field-by-field compare).
5. **New-field default:** include-by-default — new Recipe fields count toward
   identity unless explicitly excluded.
6. **Referenced profiles:** equipment profiles and source-water profiles are
   shared/live environment. Their *values* are **not** snapshotted and **not**
   hashed. Editing a profile is reflected immediately in all recipes and batches,
   and this contract is documented.
7. **Profile selection vs values:** the recipe's *selection* of a profile
   (`equipment_profile_id`, `mash_water_id`, `sparge_water_id`) **is** part of
   recipe identity (switching profiles creates a version); the profiles' resolved
   values are not.

## Section 1 — Recipe identity & change detection

A single function `canonical(recipe: &Recipe) -> Vec<u8>` (or a stable string)
produces a deterministic serialization that is hashed with SHA-256. The stored
hash is prefixed with a projection version, e.g. `"1:9f3a…"`.

**Include-by-default over the `Recipe` aggregate, with these exclusions:**

- **Metadata (excluded):** `id`, `created_at`, `updated_at`, `source`,
  `taste_notes`, `taste_rating`, `image_path`, `name`, `brewer`, `asst_brewer`,
  `date`, and `style` / `style_id` (descriptive classification, not brew
  content).
- **Profile values (excluded):** the nested resolved `equipment_profile` object
  and the resolved source-water objects are dropped. The **id** fields
  `equipment_profile_id`, `mash_water_id`, `sparge_water_id` are **kept**.
- **Children (included, normalized):** fermentables, hops, yeasts, miscs, water
  additions, water adjustments, mash + mash steps. Serialized by **content
  only** — per-row surrogate keys (`id`, `recipe_id`, `recipe_version_id`) are
  excluded from the hashed content. Rows are emitted **in `addition_order`
  sequence**, so reordering additions changes the serialization and therefore the
  hash (reordering counts as a recipe change); the raw `addition_order` integer
  itself is not hashed, only the resulting position. A stable tiebreak orders any
  rows sharing an `addition_order`.

**Stability requirements:**

- The same logical recipe content must hash identically whether loaded as a live
  recipe (`RecipeRepository::get`) or as a snapshot (`get_full`), since both
  return the `Recipe` shape. Surrogate ids differ between the two and must not
  affect the hash.
- Field ordering in the serialization is fixed (not dependent on map iteration
  order).

**Projection versioning:** the `"<n>:"` prefix identifies which canonical
projection produced a stored hash. When comparing, if a stored hash's prefix is
older than the current projection version, recompute it from that version's
snapshot via `get_full` rather than trusting the stale value. Changing the
identity set later is therefore a backfill, never a silent mismatch.

## Section 2 — Backend behavior

- **Schema:** add `content_hash TEXT` to `recipe_versions`. Written by
  `snapshot()` whenever a version is created (the auto/`save_named`/`branch_from`
  paths all flow through `snapshot`). A migration adds the column and backfills
  existing rows via `hash(canonical(get_full(v)))`.
- **Status query:** new repository method + Tauri command
  `recipe_version_status(recipe_id) -> RecipeVersionStatus { version_count: i64,
  latest_version_id: Option<String>, has_unversioned_changes: bool }`.
  `has_unversioned_changes = version_count > 0 && hash(canonical(live)) !=
  latest.content_hash` (with recompute-on-stale-prefix as above). When
  `version_count == 0`, `has_unversioned_changes` is `false`.
- **`create_batch` requires `version_id`:** drop the `Option<String>`. Remove
  `create_or_reuse` and `matches_current`. Update internal/test/seed callers to
  snapshot explicitly (call the version repo to create/choose a version, then
  pass its id).
- **Optional version name:** `save_recipe_version` / `save_named` accept an
  optional name; blank ⇒ next auto-number, unnamed. Used by both "Save as
  version" and "Brew with current changes."

## Section 3 — Frontend brew flow (unified, desktop + mobile)

One path for both entry points: the global "+ New Batch" button
(`BatchesHome`, desktop + mobile) and a recipe's Batches tab
(`BatchesTab`). After the recipe is known, call `recipe_version_status` and
`list_recipe_versions`, then always show `BrewVersionModal`:

- **0 versions** → modal shows with warning "This recipe isn't saved as a
  version yet." Only the "Brew with current changes" path is available (which
  snapshots v1 then creates the batch). The saved-version picker is hidden.
- **Clean (>=1 version, `has_unversioned_changes == false`)** → modal shows
  the saved-version picker, defaulting to `latest_version_id`. No warning.
  User confirms or picks a different version.
- **Un-versioned changes (`has_unversioned_changes == true`)** → modal shows
  both the warning ("This recipe has un-versioned changes.") with "Brew with
  current changes" and the saved-version picker. Two choices:
  - **Brew with current changes** — optional name field →
    `save_recipe_version(name?)` → `create_batch({ version_id: new })`.
  - **Brew a saved version** → version picker → `create_batch({ version_id:
    chosen })`.

The existing `>= 2 versions` picker in `BatchesHome` is subsumed by this flow,
and the no-choice quick-brew in `BatchesTab` now routes through the same logic.

## Section 4 — Recipe-view dirty badge + save-as-version (desktop + mobile)

- A **"⚠ un-versioned changes" badge** is shown when `has_unversioned_changes`,
  with a **"Save as version"** action (optional name) →
  `save_recipe_version` → badge clears. Status is fetched on recipe load and
  refreshed after edits (the tabs already call an `onchange`/`refreshRecipe`
  hook).
- **Desktop:** rendered in/near `VersionHistoryPanel`
  (`src/lib/desktop/RecipeView.svelte`).
- **Mobile:** `src/lib/mobile/RecipeView.svelte` has no version UI today; add a
  compact version/status row to host the badge + "Save as version" action so the
  feature reaches parity.

## Section 5 — Testing & docs

**Backend tests:**

- Hash determinism: identical content hashes identically regardless of row ids or
  child ordering; live recipe and its snapshot hash identically.
- Each identity field flips the hash when changed; each excluded field does
  **not** (explicitly including: switching `equipment_profile_id` flips it;
  editing an equipment-profile *value* does not change the recipe hash).
- Projection-version mismatch triggers recompute from the snapshot.
- Backfill migration populates `content_hash` for pre-existing versions.
- `recipe_version_status` returns correct `version_count` /
  `has_unversioned_changes` for 0-version, clean, and dirty recipes.
- `create_batch` rejects a missing `version_id`.

**Frontend tests:**

- Brew flow: modal always shown. 0 versions shows only "Brew with current
  changes" with the not-saved-yet warning. Clean shows saved-version picker,
  default latest, no warning. Dirty shows both warning and picker.
- Dirty badge appears/clears with status; "Save as version" creates a version.

**Docs:**

- Add a note (user-facing docs) that equipment and source-water profiles are
  shared and live: editing their values is immediately reflected in all recipes
  and batches and is not captured by recipe versions.

## Affected code (orientation, not exhaustive)

- `src-tauri/src/repositories/recipe_version.rs` — add canonical/hash, store
  `content_hash` in `snapshot`, add status method, remove `matches_current` /
  `create_or_reuse`.
- `src-tauri/src/repositories/batches.rs`, `src-tauri/src/commands/batches.rs` —
  `create_batch` requires `version_id`; new `recipe_version_status` command;
  optional version name.
- `src-tauri/src/entities/recipe_versions.rs` + migration — `content_hash`
  column.
- `src/lib/api.ts` — `recipeVersionStatus`, optional-name `saveRecipeVersion`,
  `create_batch` typing.
- `src/lib/desktop/BatchesHome.svelte`, `src/lib/mobile/BatchesHome.svelte`,
  `src/lib/components/tabs/BatchesTab.svelte` — unified brew flow + prompt.
- `src/lib/desktop/RecipeView.svelte`, `src/lib/mobile/RecipeView.svelte`,
  `src/lib/components/VersionHistoryPanel.svelte` — dirty badge + save action.

## Open risks

- **Canonical stability is load-bearing.** If the live and snapshot
  serializations diverge (e.g. a child field present in one path but not the
  other, like the already-noted `hopstand_temp_c` "not captured in version
  snapshot"), recipes will read as permanently dirty. Tests must assert
  live-vs-snapshot hash equality for an unmodified recipe. Any
  snapshot/restore fidelity gaps surfaced here should be fixed as part of the
  work.
- **Backfill** freezes existing versions' hashes from their current snapshot
  content; this is correct because snapshots already store their own child rows.
