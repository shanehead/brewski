# Ingredient Editor — Design Spec

## Summary

Allow users to create new custom ingredients and fork seeded ingredients into personal copies with different stats. All five ingredient types are covered: hops, fermentables, yeasts, miscs, and waters. Seeded data is never modified — user-owned rows live in the same tables distinguished by a `source` column. The feature surfaces in two places: a new Ingredient Library screen (rail icon) and an inline editable detail panel inside the existing ingredient picker dialog.

---

## Data Layer

### Migration

A new migration (`004_user_ingredients.sql`) adds two columns to each of the five ingredient tables via `ALTER TABLE ... ADD COLUMN`:

```sql
-- repeated for fermentables, hops, yeasts, miscs, waters
ALTER TABLE hops ADD COLUMN source TEXT NOT NULL DEFAULT 'seeded';
ALTER TABLE hops ADD COLUMN forked_from_id TEXT REFERENCES hops(id);
```

All existing rows receive `source = 'seeded'` and `forked_from_id = NULL` automatically via the column DEFAULT. No data backfill is required.

### Row conventions

| Row type | `source` | `forked_from_id` | `id` format |
|---|---|---|---|
| Seeded (existing) | `'seeded'` | `NULL` | `bm-*` prefix (existing) |
| User-created | `'user'` | `NULL` | UUID |
| Forked from seeded | `'user'` | original seeded id | UUID |

### Seed safety

Future migrations that add new seed data use `INSERT OR IGNORE` — they never overwrite existing rows. Future migrations that correct seeded values filter `WHERE source = 'seeded'` to avoid clobbering user forks.

### Name uniqueness

No unique constraint on `name`. Names are a display label; `id` is the true key. If a user creates or renames an ingredient to a name that already exists in the table, the UI shows a soft inline warning ("Another ingredient has this name") but does not block the save.

---

## Rust / API Layer

### Model changes

The existing `Hop`, `Fermentable`, `Yeast`, `Misc`, and `Water` structs in `models.rs` gain two new fields:

```rust
pub source: String,              // "seeded" | "user"
pub forked_from_id: Option<String>,
```

The corresponding SeaORM entity files gain the matching columns.

### New commands

A new `commands/ingredients.rs` exposes CRUD for all five types. The `source` field is never accepted from the caller — it is always forced to `'user'` on create. Updates and deletes guard `WHERE source = 'user'` at the repository layer so seeded rows are immutable regardless of what the frontend sends.

```
create_hop(input) / update_hop(id, input) / delete_hop(id)
create_fermentable / update_fermentable / delete_fermentable
create_yeast / update_yeast / delete_yeast
create_misc / update_misc / delete_misc
create_water / update_water / delete_water
```

A new `repositories/ingredient.rs` handles the DB work: inserts generate a UUID for `id`, set `source = 'user'`, and populate `forked_from_id` when provided.

### Existing library commands

`LibraryRepository` list methods (`list_hops`, `list_fermentables`, etc.) are unchanged — they already return all rows. User and seeded rows now come back together, sorted by name, with the new `source` and `forked_from_id` fields populated.

---

## Ingredient Library Screen

### Navigation

A new rail icon (🌿) is added to the left sidebar, alongside Recipes and Equipment. It opens the Ingredient Library screen.

### Layout

- **Header:** "Ingredient Library" title + "Add New" button
- **Tabs:** Hops | Fermentables | Yeasts | Miscs | Waters
- **Search input:** filters the list client-side by name
- **Table columns:** Name, type-specific stat (e.g. Alpha % for hops), Origin, Source badge, Actions

### Row actions

| Row type | Actions |
|---|---|
| Seeded | Duplicate |
| Custom (new or forked) | Edit, Delete |

Forked rows display lineage below the name: `↳ Citra`.

Clicking **Duplicate** on a seeded row:
1. Creates a fork in the DB with name `"[Original Name] (Custom)"` and `forked_from_id` set
2. Selects the new row
3. Opens it in the edit form

Clicking **Edit** opens the edit form for that custom row inline (or in a modal — implementation detail).

Clicking **Delete** shows a confirmation prompt before removing the row. Only allowed when `source = 'user'`.

---

## Ingredient Picker Enhancement

The existing `IngredientPicker.svelte` dialog gains per-row actions and an editable detail panel state.

### List panel changes

Each row in the left list panel gains a small action button:
- Seeded rows: `⧉ Duplicate` button
- Custom rows: `✎ Edit` button

### Detail panel states

**Read-only (seeded row selected):**
- Existing detail content (stats, badges, notes)
- `⧉ Duplicate & Edit` button in the header
- Recipe inputs (amount, use, time) remain functional — user can add the seeded ingredient as-is

**Edit mode (custom row selected, or after clicking Duplicate & Edit):**
- Name field becomes an editable input
- Key numeric fields (alpha %, yield %, attenuation %, etc.) become editable inputs
- Notes field becomes editable
- Forked rows show `↳ forked from [Original Name]` above the name field
- **Save** and **Cancel** buttons appear
- "Add to Recipe" button is disabled while in edit mode (unsaved changes)

### Duplicate & Edit flow

1. User clicks `⧉ Duplicate & Edit` on a seeded row's detail panel
2. Detail panel switches to edit mode with the seeded row's fields pre-populated (no DB write yet)
3. Name field defaults to `"[Original Name] (Custom)"`; `forked_from_id` is held in component state
4. User edits and clicks **Save** → fork is created in the DB, new row appears in the list selected, "Add to Recipe" re-enabled
5. **Cancel** → no DB write, panel returns to the seeded row's read-only view

### Name collision warning

If the user types a name that matches any existing ingredient in the same table, a soft inline warning appears below the name field: `"Another ingredient already has this name."` Save is not blocked.

---

## Mobile

The app uses platform-specific components resolved at build time via the `$platform` alias (`src/lib/desktop/` vs `src/lib/mobile/`). The data and Rust layers are platform-agnostic; only the UI components differ.

### Library Manager navigation

On **desktop**, a new rail icon is added to `src/lib/desktop/AppShell.svelte` linking to `/library`.

On **mobile**, the Library is accessed via the existing **More** tab (alongside Equipment and Settings). No new tab is added — the bottom tab bar stays at 4 items. The `BottomTabBar.svelte` `activeWhen` for "More" is extended to include `/library`. The More/Settings screen gains a "Ingredient Library" navigation row alongside Equipment.

### Ingredient picker on mobile

The existing `IngredientPicker.svelte` dialog (`80vw × 75vh`, split list/detail) is desktop-only. On mobile it is replaced with a full-screen push-navigation flow implemented as `src/lib/mobile/IngredientPicker.svelte`:

1. **List screen** — full-screen searchable list of ingredients for the selected type. Each row shows name + key stat. Tapping a row pushes to the detail screen.
2. **Detail screen** — full-screen ingredient detail. Seeded rows show stats read-only with a "Duplicate & Edit" button. Custom rows show editable fields directly. Recipe inputs (amount, use, time) sit at the bottom. "Add to Recipe" button confirms and closes.

The edit-in-place behaviour (Duplicate & Edit flow, name collision warning, Save/Cancel) is identical to desktop — only the layout changes.

---

## Error Handling

- Create/update/delete failures surface via the existing `ipc()` error toast — no special handling needed
- The `DELETE` guard (`WHERE source = 'user'`) means attempting to delete a seeded row returns `AppError::Forbidden` (or similar); the frontend never sends such a request but the backend rejects it defensively
