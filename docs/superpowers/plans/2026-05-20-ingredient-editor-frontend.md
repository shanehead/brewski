# Ingredient Editor Frontend Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Library screen and ingredient editor UI that lets users create custom ingredients and fork seeded ones, both from a standalone library page and from within the ingredient picker modal when adding ingredients to a recipe.

**Architecture:** A new `/library` route shows all 5 ingredient types in tabs; an `IngredientEditModal` dialog handles create/edit for all types. The existing `IngredientPicker` is split into desktop/mobile variants via the `$platform` alias — the desktop version gains a "Duplicate & Edit" flow; the mobile version is a new full-screen overlay.

**Tech Stack:** Svelte 5 runes (`$state`, `$derived`, `$effect`, `$props`), SvelteKit file routing, `invoke()` via `ipc()` helper, TypeScript strict, Tailwind utility classes + inline CSS vars, `$platform` Vite alias (`src/lib/desktop/` or `src/lib/mobile/`).

---

## File Map

| File | Action | Purpose |
|------|--------|---------|
| `src/lib/api.ts` | Modify | Add 10 ingredient input type exports + 15 CRUD functions |
| `src/lib/desktop/AppShell.svelte` | Modify | Add Library icon to nav rail |
| `src/lib/mobile/BottomTabBar.svelte` | Modify | Include `/library` in More tab `activeWhen` |
| `src/routes/settings/+page.svelte` | Modify | Add "Ingredient Library" nav row (mobile More screen) |
| `src/routes/library/+page.svelte` | Create | Library screen with 5 tabs, search, source badges, row actions |
| `src/lib/components/ingredients/IngredientEditModal.svelte` | Create | Create/edit dialog for all 5 ingredient types |
| `src/lib/desktop/IngredientPicker.svelte` | Create (moved) | Desktop picker + Duplicate & Edit flow |
| `src/lib/mobile/IngredientPicker.svelte` | Create | Full-screen mobile picker |
| `src/lib/components/ingredients/HopsTable.svelte` | Modify | Update import to `$platform/IngredientPicker.svelte` |
| `src/lib/components/ingredients/FermentablesTable.svelte` | Modify | Update import to `$platform/IngredientPicker.svelte` |
| `src/lib/components/ingredients/YeastsTable.svelte` | Modify | Update import to `$platform/IngredientPicker.svelte` |

---

### Task 1: API Functions

**Files:**
- Modify: `src/lib/api.ts`

- [ ] **Step 1: Add ingredient input type exports**

Add after the existing `export type Water = ...` line (line 36) in `src/lib/api.ts`:

```typescript
export type CreateHopInput = components["schemas"]["CreateHopInput"];
export type UpdateHopInput = components["schemas"]["UpdateHopInput"];
export type CreateFermentableInput = components["schemas"]["CreateFermentableInput"];
export type UpdateFermentableInput = components["schemas"]["UpdateFermentableInput"];
export type CreateYeastInput = components["schemas"]["CreateYeastInput"];
export type UpdateYeastInput = components["schemas"]["UpdateYeastInput"];
export type CreateMiscInput = components["schemas"]["CreateMiscInput"];
export type UpdateMiscInput = components["schemas"]["UpdateMiscInput"];
export type CreateWaterInput = components["schemas"]["CreateWaterInput"];
export type UpdateWaterInput = components["schemas"]["UpdateWaterInput"];
```

- [ ] **Step 2: Add ingredient CRUD functions**

Add a new section at the end of `src/lib/api.ts`:

```typescript
// --- Ingredients (user library CRUD) ---
export const createHop = (input: CreateHopInput) =>
  invoke<Hop>("create_hop", { input });
export const updateHop = (id: string, input: UpdateHopInput) =>
  invoke<Hop>("update_hop", { id, input });
export const deleteHop = (id: string) =>
  invoke<void>("delete_hop", { id });

export const createFermentable = (input: CreateFermentableInput) =>
  invoke<Fermentable>("create_fermentable", { input });
export const updateFermentable = (id: string, input: UpdateFermentableInput) =>
  invoke<Fermentable>("update_fermentable", { id, input });
export const deleteFermentable = (id: string) =>
  invoke<void>("delete_fermentable", { id });

export const createYeast = (input: CreateYeastInput) =>
  invoke<Yeast>("create_yeast", { input });
export const updateYeast = (id: string, input: UpdateYeastInput) =>
  invoke<Yeast>("update_yeast", { id, input });
export const deleteYeast = (id: string) =>
  invoke<void>("delete_yeast", { id });

export const createMisc = (input: CreateMiscInput) =>
  invoke<Misc>("create_misc", { input });
export const updateMisc = (id: string, input: UpdateMiscInput) =>
  invoke<Misc>("update_misc", { id, input });
export const deleteMisc = (id: string) =>
  invoke<void>("delete_misc", { id });

export const createWater = (input: CreateWaterInput) =>
  invoke<Water>("create_water", { input });
export const updateWater = (id: string, input: UpdateWaterInput) =>
  invoke<Water>("update_water", { id, input });
export const deleteWater = (id: string) =>
  invoke<void>("delete_water", { id });
```

- [ ] **Step 3: Verify TypeScript compilation**

Run: `just check-ts`

Expected: No errors.

- [ ] **Step 4: Commit**

```bash
git add src/lib/api.ts
git commit -m "feat: add ingredient CRUD API functions and input types"
```

---

### Task 2: Desktop Navigation

**Files:**
- Modify: `src/lib/desktop/AppShell.svelte`

- [ ] **Step 1: Add `isLibrary` derived state**

In `src/lib/desktop/AppShell.svelte`, add after the `isEquipment` derived (line 14):

```typescript
const isLibrary = $derived($page.url.pathname.startsWith('/library'));
```

- [ ] **Step 2: Add Library icon to nav rail**

In `src/lib/desktop/AppShell.svelte`, add a Library link after the Equipment `<a>` block and before the `<div class="flex-1"></div>` spacer:

```svelte
<!-- Library icon -->
<a href="/library" class="w-9 h-9 flex items-center justify-center rounded transition-colors"
   aria-label="Library"
   style={isLibrary ? "background: var(--color-accent); color: #fff;" : "color: var(--color-text-secondary);"}>
  <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
    <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/>
    <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/>
  </svg>
</a>
```

- [ ] **Step 3: Verify TypeScript compilation**

Run: `just check-ts`

Expected: No errors.

- [ ] **Step 4: Commit**

```bash
git add src/lib/desktop/AppShell.svelte
git commit -m "feat: add Library nav icon to desktop rail"
```

---

### Task 3: Mobile Navigation

**Files:**
- Modify: `src/lib/mobile/BottomTabBar.svelte`
- Modify: `src/routes/settings/+page.svelte`

- [ ] **Step 1: Extend More tab `activeWhen` in BottomTabBar**

In `src/lib/mobile/BottomTabBar.svelte`, find the More tab's `activeWhen` (line 26) and add `/library`:

```typescript
activeWhen: (p: string) => p.startsWith("/settings") || p.startsWith("/equipment") || p.startsWith("/library"),
```

- [ ] **Step 2: Add Ingredient Library row to settings page**

In `src/routes/settings/+page.svelte`, add an import for `page` and a navigation row. Add `import { page } from "$app/stores";` to the script block, then add a new section after `<DatabaseLocation />`:

```svelte
<!-- Navigation -->
<section class="flex flex-col gap-3">
  <h2 class="text-sm font-semibold" style="color: var(--color-text-secondary);">Ingredients</h2>
  <a href="/library"
     class="flex items-center justify-between py-2 px-3 rounded"
     style="background: var(--color-bg-elevated); color: var(--color-text-primary); text-decoration: none;">
    <span class="text-sm">Ingredient Library</span>
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <polyline points="9 18 15 12 9 6"/>
    </svg>
  </a>
</section>
```

- [ ] **Step 3: Verify TypeScript compilation**

Run: `just check-ts`

Expected: No errors.

- [ ] **Step 4: Commit**

```bash
git add src/lib/mobile/BottomTabBar.svelte src/routes/settings/+page.svelte
git commit -m "feat: add Library to mobile navigation"
```

---

### Task 4: Library Screen

**Files:**
- Create: `src/routes/library/+page.svelte`
- Create: `src/lib/components/ingredients/IngredientEditModal.svelte`

The library page shows all 5 ingredient types in tabs. Each tab has a search box, a list of rows with source badges (seeded rows are read-only; user rows have Edit/Delete actions), and a "New" button. The `IngredientEditModal` handles create and edit for all types.

#### 4a: IngredientEditModal

- [ ] **Step 1: Create `IngredientEditModal.svelte`**

Create `src/lib/components/ingredients/IngredientEditModal.svelte`:

```svelte
<script lang="ts">
  import type {
    Hop, Fermentable, Yeast, Misc, Water,
    CreateHopInput, UpdateHopInput,
    CreateFermentableInput, UpdateFermentableInput,
    CreateYeastInput, UpdateYeastInput,
    CreateMiscInput, UpdateMiscInput,
    CreateWaterInput, UpdateWaterInput,
  } from '$lib/api';
  import {
    createHop, updateHop,
    createFermentable, updateFermentable,
    createYeast, updateYeast,
    createMisc, updateMisc,
    createWater, updateWater,
  } from '$lib/api';
  import { ipc } from '$lib/stores/error';

  type IngredientType = 'hop' | 'fermentable' | 'yeast' | 'misc' | 'water';
  type AnyIngredient = Hop | Fermentable | Yeast | Misc | Water;

  let {
    type,
    ingredient = null,
    existingNames = [],
    onsave,
    oncancel,
  }: {
    type: IngredientType;
    ingredient?: AnyIngredient | null;
    existingNames?: string[];
    onsave: (saved: AnyIngredient) => void;
    oncancel: () => void;
  } = $props();

  const isEdit = $derived(ingredient !== null);
  const title = $derived(
    isEdit
      ? `Edit ${type.charAt(0).toUpperCase() + type.slice(1)}`
      : `New ${type.charAt(0).toUpperCase() + type.slice(1)}`
  );

  // --- Hop fields ---
  let hopName = $state(isEdit && type === 'hop' ? (ingredient as Hop).name : '');
  let hopAlpha = $state(isEdit && type === 'hop' ? (ingredient as Hop).alpha_pct : 0);
  let hopBeta = $state(isEdit && type === 'hop' ? ((ingredient as Hop).beta_pct ?? '') : '');
  let hopForm = $state(isEdit && type === 'hop' ? (ingredient as Hop).form : 'Pellet');
  let hopType = $state(isEdit && type === 'hop' ? ((ingredient as Hop).type_ ?? '') : '');
  let hopOrigin = $state(isEdit && type === 'hop' ? ((ingredient as Hop).origin ?? '') : '');
  let hopNotes = $state(isEdit && type === 'hop' ? ((ingredient as Hop).notes ?? '') : '');
  let hopSubstitutes = $state(isEdit && type === 'hop' ? ((ingredient as Hop).substitutes ?? '') : '');

  // --- Fermentable fields ---
  let fermName = $state(isEdit && type === 'fermentable' ? (ingredient as Fermentable).name : '');
  let fermType = $state(isEdit && type === 'fermentable' ? (ingredient as Fermentable).type_ : 'Grain');
  let fermYield = $state(isEdit && type === 'fermentable' ? (ingredient as Fermentable).yield_pct : 75);
  let fermColor = $state(isEdit && type === 'fermentable' ? (ingredient as Fermentable).color_lovibond : 2);
  let fermOrigin = $state(isEdit && type === 'fermentable' ? ((ingredient as Fermentable).origin ?? '') : '');
  let fermNotes = $state(isEdit && type === 'fermentable' ? ((ingredient as Fermentable).notes ?? '') : '');
  let fermAddAfterBoil = $state(isEdit && type === 'fermentable' ? (ingredient as Fermentable).add_after_boil : false);

  // --- Yeast fields ---
  let yeastName = $state(isEdit && type === 'yeast' ? (ingredient as Yeast).name : '');
  let yeastType = $state(isEdit && type === 'yeast' ? (ingredient as Yeast).type_ : 'Ale');
  let yeastForm = $state(isEdit && type === 'yeast' ? (ingredient as Yeast).form : 'Dry');
  let yeastLab = $state(isEdit && type === 'yeast' ? ((ingredient as Yeast).laboratory ?? '') : '');
  let yeastProductId = $state(isEdit && type === 'yeast' ? ((ingredient as Yeast).product_id ?? '') : '');
  let yeastAttenuation = $state(isEdit && type === 'yeast' ? ((ingredient as Yeast).attenuation_pct ?? '') : '');
  let yeastFlocculation = $state(isEdit && type === 'yeast' ? ((ingredient as Yeast).flocculation ?? '') : '');
  let yeastNotes = $state(isEdit && type === 'yeast' ? ((ingredient as Yeast).notes ?? '') : '');
  let yeastAddToSecondary = $state(isEdit && type === 'yeast' ? (ingredient as Yeast).add_to_secondary : false);

  // --- Misc fields ---
  let miscName = $state(isEdit && type === 'misc' ? (ingredient as Misc).name : '');
  let miscType = $state(isEdit && type === 'misc' ? (ingredient as Misc).type_ : 'Spice');
  let miscUse = $state(isEdit && type === 'misc' ? (ingredient as Misc).use_ : 'Boil');
  let miscTime = $state(isEdit && type === 'misc' ? (ingredient as Misc).time_min : 15);
  let miscAmountIsWeight = $state(isEdit && type === 'misc' ? (ingredient as Misc).amount_is_weight : true);
  let miscNotes = $state(isEdit && type === 'misc' ? ((ingredient as Misc).notes ?? '') : '');
  let miscUseFor = $state(isEdit && type === 'misc' ? ((ingredient as Misc).use_for ?? '') : '');

  // --- Water fields ---
  let waterName = $state(isEdit && type === 'water' ? (ingredient as Water).name : '');
  let waterCa = $state(isEdit && type === 'water' ? (ingredient as Water).calcium_ppm : 0);
  let waterBicarb = $state(isEdit && type === 'water' ? (ingredient as Water).bicarbonate_ppm : 0);
  let waterSulfate = $state(isEdit && type === 'water' ? (ingredient as Water).sulfate_ppm : 0);
  let waterChloride = $state(isEdit && type === 'water' ? (ingredient as Water).chloride_ppm : 0);
  let waterSodium = $state(isEdit && type === 'water' ? (ingredient as Water).sodium_ppm : 0);
  let waterMg = $state(isEdit && type === 'water' ? (ingredient as Water).magnesium_ppm : 0);
  let waterNotes = $state(isEdit && type === 'water' ? ((ingredient as Water).notes ?? '') : '');

  let saving = $state(false);

  // Name collision check: warn if another ingredient in existingNames has the same name (case-insensitive),
  // excluding the current ingredient's own name when editing.
  const currentName = $derived(
    type === 'hop' ? hopName :
    type === 'fermentable' ? fermName :
    type === 'yeast' ? yeastName :
    type === 'misc' ? miscName :
    waterName
  );
  const nameCollision = $derived(
    existingNames
      .filter(n => !isEdit || n.toLowerCase() !== (ingredient as AnyIngredient).name.toLowerCase())
      .some(n => n.toLowerCase() === currentName.trim().toLowerCase())
  );

  async function handleSave() {
    if (saving || nameCollision || !currentName.trim()) return;
    saving = true;
    let saved: AnyIngredient | null | undefined = null;
    if (type === 'hop') {
      const input: CreateHopInput | UpdateHopInput = {
        name: hopName.trim(),
        alpha_pct: hopAlpha,
        form: hopForm,
        beta_pct: hopBeta === '' ? null : Number(hopBeta),
        type_: hopType.trim() || null,
        origin: hopOrigin.trim() || null,
        notes: hopNotes.trim() || null,
        substitutes: hopSubstitutes.trim() || null,
      };
      if (isEdit) {
        saved = await ipc(updateHop((ingredient as Hop).id, input as UpdateHopInput));
      } else {
        saved = await ipc(createHop(input as CreateHopInput));
      }
    } else if (type === 'fermentable') {
      const input: CreateFermentableInput | UpdateFermentableInput = {
        name: fermName.trim(),
        type_: fermType,
        yield_pct: fermYield,
        color_lovibond: fermColor,
        origin: fermOrigin.trim() || null,
        notes: fermNotes.trim() || null,
        add_after_boil: fermAddAfterBoil,
      };
      if (isEdit) {
        saved = await ipc(updateFermentable((ingredient as Fermentable).id, input as UpdateFermentableInput));
      } else {
        saved = await ipc(createFermentable(input as CreateFermentableInput));
      }
    } else if (type === 'yeast') {
      const input: CreateYeastInput | UpdateYeastInput = {
        name: yeastName.trim(),
        type_: yeastType,
        form: yeastForm,
        laboratory: yeastLab.trim() || null,
        product_id: yeastProductId.trim() || null,
        attenuation_pct: yeastAttenuation === '' ? null : Number(yeastAttenuation),
        flocculation: yeastFlocculation.trim() || null,
        notes: yeastNotes.trim() || null,
        add_to_secondary: yeastAddToSecondary,
      };
      if (isEdit) {
        saved = await ipc(updateYeast((ingredient as Yeast).id, input as UpdateYeastInput));
      } else {
        saved = await ipc(createYeast(input as CreateYeastInput));
      }
    } else if (type === 'misc') {
      const input: CreateMiscInput | UpdateMiscInput = {
        name: miscName.trim(),
        type_: miscType,
        use_: miscUse,
        time_min: miscTime,
        amount_is_weight: miscAmountIsWeight,
        notes: miscNotes.trim() || null,
        use_for: miscUseFor.trim() || null,
      };
      if (isEdit) {
        saved = await ipc(updateMisc((ingredient as Misc).id, input as UpdateMiscInput));
      } else {
        saved = await ipc(createMisc(input as CreateMiscInput));
      }
    } else {
      const input: CreateWaterInput | UpdateWaterInput = {
        name: waterName.trim(),
        calcium_ppm: waterCa,
        bicarbonate_ppm: waterBicarb,
        sulfate_ppm: waterSulfate,
        chloride_ppm: waterChloride,
        sodium_ppm: waterSodium,
        magnesium_ppm: waterMg,
        notes: waterNotes.trim() || null,
      };
      if (isEdit) {
        saved = await ipc(updateWater((ingredient as Water).id, input as UpdateWaterInput));
      } else {
        saved = await ipc(createWater(input as CreateWaterInput));
      }
    }
    saving = false;
    if (saved) onsave(saved);
  }
</script>

<div class="fixed inset-0 flex items-center justify-center" style="z-index: 1000;">
  <!-- Backdrop -->
  <div class="absolute inset-0" style="background: rgba(0,0,0,0.4);" role="none"
       onclick={oncancel} onkeydown={() => {}}></div>

  <!-- Modal -->
  <div class="relative flex flex-col rounded-lg overflow-hidden"
       style="background: var(--color-bg-elevated); border: 1px solid var(--color-border);
              z-index: 1001; width: 480px; max-width: 95vw; max-height: 80vh;">

    <!-- Header -->
    <div class="flex items-center justify-between px-5 py-3 flex-shrink-0"
         style="border-bottom: 1px solid var(--color-border);">
      <h2 class="text-base font-semibold" style="color: var(--color-text-primary);">{title}</h2>
      <button onclick={oncancel}
              style="background: none; border: none; cursor: pointer; color: var(--color-text-muted); font-size: 18px; padding: 2px 6px; border-radius: 4px;">×</button>
    </div>

    <!-- Body -->
    <div class="flex-1 overflow-y-auto p-5 flex flex-col gap-4">

      {#if nameCollision}
        <div class="text-sm px-3 py-2 rounded" style="background: #7f1d1d20; border: 1px solid #dc262650; color: #fca5a5;">
          An ingredient with this name already exists. Please choose a different name.
        </div>
      {/if}

      {#if type === 'hop'}
        <div class="grid grid-cols-2 gap-3">
          <div class="col-span-2 flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Name *</label>
            <input bind:value={hopName} class="px-2 py-1.5 rounded text-sm"
                   style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Alpha % *</label>
            <input type="number" step="0.1" min="0" bind:value={hopAlpha} class="px-2 py-1.5 rounded text-sm"
                   style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Beta %</label>
            <input type="number" step="0.1" min="0" bind:value={hopBeta} placeholder="—" class="px-2 py-1.5 rounded text-sm"
                   style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Form *</label>
            <select bind:value={hopForm} class="px-2 py-1.5 rounded text-sm"
                    style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);">
              <option>Pellet</option><option>Plug</option><option>Leaf</option>
            </select>
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Type</label>
            <input bind:value={hopType} placeholder="e.g. Bittering" class="px-2 py-1.5 rounded text-sm"
                   style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Origin</label>
            <input bind:value={hopOrigin} class="px-2 py-1.5 rounded text-sm"
                   style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
          </div>
          <div class="col-span-2 flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Substitutes</label>
            <input bind:value={hopSubstitutes} class="px-2 py-1.5 rounded text-sm"
                   style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
          </div>
          <div class="col-span-2 flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Notes</label>
            <textarea bind:value={hopNotes} rows="3" class="px-2 py-1.5 rounded text-sm resize-none"
                      style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);"></textarea>
          </div>
        </div>

      {:else if type === 'fermentable'}
        <div class="grid grid-cols-2 gap-3">
          <div class="col-span-2 flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Name *</label>
            <input bind:value={fermName} class="px-2 py-1.5 rounded text-sm"
                   style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Type *</label>
            <select bind:value={fermType} class="px-2 py-1.5 rounded text-sm"
                    style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);">
              <option>Grain</option><option>Sugar</option><option>Extract</option><option>Dry Extract</option><option>Adjunct</option>
            </select>
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Origin</label>
            <input bind:value={fermOrigin} class="px-2 py-1.5 rounded text-sm"
                   style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Yield % *</label>
            <input type="number" step="0.1" min="0" max="100" bind:value={fermYield} class="px-2 py-1.5 rounded text-sm"
                   style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Color (°L) *</label>
            <input type="number" step="0.1" min="0" bind:value={fermColor} class="px-2 py-1.5 rounded text-sm"
                   style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
          </div>
          <div class="col-span-2 flex items-center gap-2">
            <input type="checkbox" id="add-after-boil" bind:checked={fermAddAfterBoil} class="rounded" />
            <label for="add-after-boil" class="text-sm" style="color: var(--color-text-primary);">Add after boil</label>
          </div>
          <div class="col-span-2 flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Notes</label>
            <textarea bind:value={fermNotes} rows="3" class="px-2 py-1.5 rounded text-sm resize-none"
                      style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);"></textarea>
          </div>
        </div>

      {:else if type === 'yeast'}
        <div class="grid grid-cols-2 gap-3">
          <div class="col-span-2 flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Name *</label>
            <input bind:value={yeastName} class="px-2 py-1.5 rounded text-sm"
                   style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Type *</label>
            <select bind:value={yeastType} class="px-2 py-1.5 rounded text-sm"
                    style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);">
              <option>Ale</option><option>Lager</option><option>Wheat</option><option>Wine</option><option>Champagne</option>
            </select>
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Form *</label>
            <select bind:value={yeastForm} class="px-2 py-1.5 rounded text-sm"
                    style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);">
              <option>Liquid</option><option>Dry</option><option>Slant</option><option>Culture</option>
            </select>
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Laboratory</label>
            <input bind:value={yeastLab} class="px-2 py-1.5 rounded text-sm"
                   style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Product ID</label>
            <input bind:value={yeastProductId} class="px-2 py-1.5 rounded text-sm"
                   style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Attenuation %</label>
            <input type="number" step="1" min="0" max="100" bind:value={yeastAttenuation} placeholder="—" class="px-2 py-1.5 rounded text-sm"
                   style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Flocculation</label>
            <select bind:value={yeastFlocculation} class="px-2 py-1.5 rounded text-sm"
                    style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);">
              <option value="">—</option><option>Low</option><option>Medium</option><option>High</option><option>Very High</option>
            </select>
          </div>
          <div class="col-span-2 flex items-center gap-2">
            <input type="checkbox" id="add-to-secondary" bind:checked={yeastAddToSecondary} class="rounded" />
            <label for="add-to-secondary" class="text-sm" style="color: var(--color-text-primary);">Add to secondary</label>
          </div>
          <div class="col-span-2 flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Notes</label>
            <textarea bind:value={yeastNotes} rows="3" class="px-2 py-1.5 rounded text-sm resize-none"
                      style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);"></textarea>
          </div>
        </div>

      {:else if type === 'misc'}
        <div class="grid grid-cols-2 gap-3">
          <div class="col-span-2 flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Name *</label>
            <input bind:value={miscName} class="px-2 py-1.5 rounded text-sm"
                   style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Type *</label>
            <select bind:value={miscType} class="px-2 py-1.5 rounded text-sm"
                    style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);">
              <option>Spice</option><option>Fining</option><option>Water Agent</option><option>Herb</option><option>Flavor</option><option>Other</option>
            </select>
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Default Use *</label>
            <select bind:value={miscUse} class="px-2 py-1.5 rounded text-sm"
                    style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);">
              <option>Boil</option><option>Mash</option><option>Primary</option><option>Secondary</option><option>Bottling</option>
            </select>
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Default Time (min) *</label>
            <input type="number" step="1" min="0" bind:value={miscTime} class="px-2 py-1.5 rounded text-sm"
                   style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
          </div>
          <div class="col-span-2 flex items-center gap-2">
            <input type="checkbox" id="amount-is-weight" bind:checked={miscAmountIsWeight} class="rounded" />
            <label for="amount-is-weight" class="text-sm" style="color: var(--color-text-primary);">Amount is weight (vs. volume)</label>
          </div>
          <div class="col-span-2 flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Use For</label>
            <input bind:value={miscUseFor} class="px-2 py-1.5 rounded text-sm"
                   style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
          </div>
          <div class="col-span-2 flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Notes</label>
            <textarea bind:value={miscNotes} rows="3" class="px-2 py-1.5 rounded text-sm resize-none"
                      style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);"></textarea>
          </div>
        </div>

      {:else}
        <!-- Water -->
        <div class="grid grid-cols-2 gap-3">
          <div class="col-span-2 flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Name *</label>
            <input bind:value={waterName} class="px-2 py-1.5 rounded text-sm"
                   style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
          </div>
          {#each [
            ['Calcium (ppm)', waterCa, (v: number) => { waterCa = v; }],
            ['Bicarbonate (ppm)', waterBicarb, (v: number) => { waterBicarb = v; }],
            ['Sulfate (ppm)', waterSulfate, (v: number) => { waterSulfate = v; }],
            ['Chloride (ppm)', waterChloride, (v: number) => { waterChloride = v; }],
            ['Sodium (ppm)', waterSodium, (v: number) => { waterSodium = v; }],
            ['Magnesium (ppm)', waterMg, (v: number) => { waterMg = v; }],
          ] as [string, number, (v: number) => void]}
            <div class="flex flex-col gap-1">
              <label class="text-xs" style="color: var(--color-text-secondary);">{label} *</label>
              <input type="number" step="1" min="0" value={val}
                     oninput={(e) => setter(Number((e.target as HTMLInputElement).value))}
                     class="px-2 py-1.5 rounded text-sm"
                     style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </div>
          {/each}
          <div class="col-span-2 flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Notes</label>
            <textarea bind:value={waterNotes} rows="2" class="px-2 py-1.5 rounded text-sm resize-none"
                      style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);"></textarea>
          </div>
        </div>
      {/if}
    </div>

    <!-- Footer -->
    <div class="flex justify-end gap-2 px-5 py-3 flex-shrink-0"
         style="border-top: 1px solid var(--color-border);">
      <button onclick={oncancel} class="px-4 py-1.5 rounded text-sm"
              style="background: var(--color-bg-surface); color: var(--color-text-primary); border: 1px solid var(--color-border);">
        Cancel
      </button>
      <button onclick={handleSave} disabled={saving || nameCollision || !currentName.trim()} class="px-4 py-1.5 rounded text-sm font-medium"
              style="background: {saving || nameCollision || !currentName.trim() ? 'var(--color-bg-elevated)' : 'var(--color-accent)'}; color: {saving || nameCollision || !currentName.trim() ? 'var(--color-text-muted)' : '#fff'}; border: none; cursor: {saving || nameCollision || !currentName.trim() ? 'default' : 'pointer'};">
        {saving ? 'Saving…' : isEdit ? 'Save' : 'Create'}
      </button>
    </div>
  </div>
</div>
```

**Note:** The `{#each}` block for water fields uses destructuring `[label, val, setter]` from the array tuples. Svelte 5 handles this fine. The three items in each tuple are `string`, `number`, and `(v: number) => void`.

- [ ] **Step 2: Verify TypeScript compilation**

Run: `just check-ts`

Expected: No errors.

#### 4b: Library Page

- [ ] **Step 3: Create `src/routes/library/+page.svelte`**

```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import type { Hop, Fermentable, Yeast, Misc, Water } from '$lib/api';
  import {
    listHopLibrary, deleteHop,
    listFermentableLibrary, deleteFermentable,
    listYeastLibrary, deleteYeast,
    listMiscLibrary, deleteMisc,
    listWaterLibrary, deleteWater,
  } from '$lib/api';
  import { ipc } from '$lib/stores/error';
  import IngredientEditModal from '$lib/components/ingredients/IngredientEditModal.svelte';
  import ConfirmModal from '$lib/components/ConfirmModal.svelte';

  type Tab = 'hop' | 'fermentable' | 'yeast' | 'misc' | 'water';
  type AnyIngredient = Hop | Fermentable | Yeast | Misc | Water;

  let activeTab = $state<Tab>('hop');
  let query = $state('');

  let hops = $state<Hop[]>([]);
  let fermentables = $state<Fermentable[]>([]);
  let yeasts = $state<Yeast[]>([]);
  let miscs = $state<Misc[]>([]);
  let waters = $state<Water[]>([]);
  let loaded = $state<Record<Tab, boolean>>({ hop: false, fermentable: false, yeast: false, misc: false, water: false });

  let editModalOpen = $state(false);
  let editType = $state<Tab>('hop');
  let editIngredient = $state<AnyIngredient | null>(null);

  let deleteModalOpen = $state(false);
  let deleteCandidate = $state<AnyIngredient | null>(null);
  let deleteType = $state<Tab>('hop');

  const TAB_LABELS: Record<Tab, string> = { hop: 'Hops', fermentable: 'Fermentables', yeast: 'Yeasts', misc: 'Misc', water: 'Water' };

  async function loadTab(tab: Tab) {
    if (loaded[tab]) return;
    if (tab === 'hop') hops = (await ipc(listHopLibrary())) ?? [];
    else if (tab === 'fermentable') fermentables = (await ipc(listFermentableLibrary())) ?? [];
    else if (tab === 'yeast') yeasts = (await ipc(listYeastLibrary())) ?? [];
    else if (tab === 'misc') miscs = (await ipc(listMiscLibrary())) ?? [];
    else waters = (await ipc(listWaterLibrary())) ?? [];
    loaded[tab] = true;
  }

  async function refreshTab(tab: Tab) {
    loaded[tab] = false;
    await loadTab(tab);
  }

  onMount(() => loadTab('hop'));

  $effect(() => { loadTab(activeTab); });

  function switchTab(tab: Tab) {
    activeTab = tab;
    query = '';
  }

  const currentList = $derived<AnyIngredient[]>(
    activeTab === 'hop' ? hops :
    activeTab === 'fermentable' ? fermentables :
    activeTab === 'yeast' ? yeasts :
    activeTab === 'misc' ? miscs :
    waters
  );

  const filtered = $derived(
    query.trim() === ''
      ? currentList
      : currentList.filter(i => i.name.toLowerCase().includes(query.trim().toLowerCase()))
  );

  const existingNames = $derived(currentList.map(i => i.name));

  function rowSubtext(item: AnyIngredient): string {
    if (activeTab === 'hop') return `${(item as Hop).alpha_pct}% AA · ${(item as Hop).form}`;
    if (activeTab === 'fermentable') return `${(item as Fermentable).type_} · ${(item as Fermentable).yield_pct.toFixed(0)}% yield · ${(item as Fermentable).color_lovibond}°L`;
    if (activeTab === 'yeast') return `${(item as Yeast).type_} · ${(item as Yeast).form}`;
    if (activeTab === 'misc') return `${(item as Misc).type_} · ${(item as Misc).use_}`;
    const w = item as Water;
    return `Ca:${w.calcium_ppm} Mg:${w.magnesium_ppm} Na:${w.sodium_ppm} SO₄:${w.sulfate_ppm} Cl:${w.chloride_ppm} HCO₃:${w.bicarbonate_ppm}`;
  }

  function openCreate() {
    editIngredient = null;
    editType = activeTab;
    editModalOpen = true;
  }

  function openEdit(item: AnyIngredient) {
    editIngredient = item;
    editType = activeTab;
    editModalOpen = true;
  }

  function openDelete(item: AnyIngredient) {
    deleteCandidate = item;
    deleteType = activeTab;
    deleteModalOpen = true;
  }

  async function confirmDelete() {
    if (!deleteCandidate) return;
    deleteModalOpen = false;
    const id = deleteCandidate.id;
    if (deleteType === 'hop') await ipc(deleteHop(id));
    else if (deleteType === 'fermentable') await ipc(deleteFermentable(id));
    else if (deleteType === 'yeast') await ipc(deleteYeast(id));
    else if (deleteType === 'misc') await ipc(deleteMisc(id));
    else await ipc(deleteWater(id));
    deleteCandidate = null;
    await refreshTab(deleteType);
  }

  async function handleSave() {
    editModalOpen = false;
    await refreshTab(editType);
    editIngredient = null;
  }
</script>

<div class="flex flex-col flex-1 overflow-hidden" style="background: var(--color-bg-base);">
  <!-- Header -->
  <div class="flex items-center justify-between px-6 py-4 flex-shrink-0"
       style="border-bottom: 1px solid var(--color-border);">
    <h1 class="text-lg font-semibold" style="color: var(--color-text-primary);">Ingredient Library</h1>
    <button onclick={openCreate} class="px-3 py-1.5 rounded text-sm font-medium"
            style="background: var(--color-accent); color: #fff; border: none; cursor: pointer;">
      + New {TAB_LABELS[activeTab].replace(/s$/, '')}
    </button>
  </div>

  <!-- Tabs -->
  <div class="flex gap-0 flex-shrink-0 px-6 pt-3"
       style="border-bottom: 1px solid var(--color-border);">
    {#each (['hop', 'fermentable', 'yeast', 'misc', 'water'] as Tab[]) as tab}
      <button onclick={() => switchTab(tab)}
              class="px-4 py-2 text-sm font-medium transition-colors"
              style="
                border: none; background: none; cursor: pointer;
                color: {activeTab === tab ? 'var(--color-accent)' : 'var(--color-text-secondary)'};
                border-bottom: 2px solid {activeTab === tab ? 'var(--color-accent)' : 'transparent'};
                margin-bottom: -1px;
              ">
        {TAB_LABELS[tab]}
      </button>
    {/each}
  </div>

  <!-- Search -->
  <div class="px-6 py-3 flex-shrink-0">
    <div class="relative max-w-xs">
      <svg style="position: absolute; left: 8px; top: 50%; transform: translateY(-50%); pointer-events: none; color: var(--color-text-muted);"
           width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
      </svg>
      <input bind:value={query}
             placeholder="Search {TAB_LABELS[activeTab].toLowerCase()}…"
             class="pl-8 pr-3 py-1.5 rounded text-sm w-full"
             style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); color: var(--color-text-primary); outline: none;" />
    </div>
  </div>

  <!-- List -->
  <div class="flex-1 overflow-y-auto px-6 pb-6">
    {#if filtered.length === 0}
      <p class="text-sm mt-8 text-center" style="color: var(--color-text-muted);">
        {query ? `No results for "${query}"` : `No ${TAB_LABELS[activeTab].toLowerCase()} yet.`}
      </p>
    {:else}
      <div class="flex flex-col gap-1">
        {#each filtered as item (item.id)}
          {@const isSeeded = item.source === 'seeded'}
          <div class="flex items-center gap-3 px-3 py-2.5 rounded"
               style="background: var(--color-bg-elevated); border: 1px solid var(--color-border);">
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2">
                <span class="text-sm font-medium truncate" style="color: var(--color-text-primary);">{item.name}</span>
                <span class="text-xs px-1.5 py-0.5 rounded-full flex-shrink-0"
                      style="background: {isSeeded ? 'var(--color-bg-surface)' : 'color-mix(in srgb, var(--color-accent) 15%, transparent)'}; color: {isSeeded ? 'var(--color-text-muted)' : 'var(--color-accent)'}; border: 1px solid {isSeeded ? 'var(--color-border)' : 'color-mix(in srgb, var(--color-accent) 40%, transparent)'};">
                  {isSeeded ? 'built-in' : 'custom'}
                </span>
              </div>
              <div class="text-xs mt-0.5 truncate" style="color: var(--color-text-secondary);">{rowSubtext(item)}</div>
            </div>
            {#if !isSeeded}
              <button onclick={() => openEdit(item)}
                      class="text-xs px-2 py-1 rounded flex-shrink-0"
                      style="background: var(--color-bg-surface); color: var(--color-text-secondary); border: 1px solid var(--color-border); cursor: pointer;">
                Edit
              </button>
              <button onclick={() => openDelete(item)}
                      class="text-xs px-2 py-1 rounded flex-shrink-0"
                      style="background: var(--color-bg-surface); color: #f87171; border: 1px solid var(--color-border); cursor: pointer;">
                Delete
              </button>
            {/if}
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

{#if editModalOpen}
  <IngredientEditModal
    type={editType}
    ingredient={editIngredient}
    existingNames={existingNames}
    onsave={handleSave}
    oncancel={() => { editModalOpen = false; editIngredient = null; }}
  />
{/if}

{#if deleteModalOpen && deleteCandidate}
  <ConfirmModal
    message="Delete {deleteCandidate.name}? This cannot be undone."
    confirmLabel="Delete"
    dangerous={true}
    onconfirm={confirmDelete}
    oncancel={() => { deleteModalOpen = false; deleteCandidate = null; }}
  />
{/if}
```

- [ ] **Step 4: Verify TypeScript compilation**

Run: `just check-ts`

Expected: No errors.

- [ ] **Step 5: Commit**

```bash
git add src/lib/components/ingredients/IngredientEditModal.svelte src/routes/library/+page.svelte
git commit -m "feat: add library screen and ingredient edit modal"
```

---

### Task 5: Desktop IngredientPicker Enhancement

**Files:**
- Create: `src/lib/desktop/IngredientPicker.svelte` (contents based on existing `src/lib/components/ingredients/IngredientPicker.svelte`, with Duplicate & Edit additions)
- Modify: `src/lib/components/ingredients/HopsTable.svelte` — update import
- Modify: `src/lib/components/ingredients/FermentablesTable.svelte` — update import
- Modify: `src/lib/components/ingredients/YeastsTable.svelte` — update import

The Duplicate & Edit flow: when a seeded ingredient is selected in the detail panel, a "Duplicate & Edit" button appears. Clicking it activates `forkMode`, which swaps the detail panel for a compact form pre-filled with the ingredient's values (same fields as IngredientEditModal but inline). On Save, `createHop({...input, forked_from_id: original.id})` is called; on success the library reloads, the new item is selected, and `forkMode` exits. A name collision warning shows inline if the draft name matches an existing library item.

- [ ] **Step 1: Copy IngredientPicker to desktop variant and add fork-mode state**

Create `src/lib/desktop/IngredientPicker.svelte`. Start with the full content of `src/lib/components/ingredients/IngredientPicker.svelte` and apply the following additions to the `<script>` section:

Add imports at the top:
```typescript
import type {
  CreateHopInput, CreateFermentableInput, CreateYeastInput,
} from '$lib/api';
import {
  createHop, createFermentable, createYeast,
  listHopLibrary, listFermentableLibrary, listYeastLibrary,
} from '$lib/api';
import IngredientEditModal from '$lib/components/ingredients/IngredientEditModal.svelte';
```

Add state variables after the existing `let` declarations:
```typescript
let forkMode = $state(false);
let forkSaving = $state(false);
// Draft name for the fork — user can rename before saving
let forkName = $state('');
// Name collision: warn if an item in the library already has this name
const forkNameCollision = $derived(
  library.some(i => i.name.toLowerCase() === forkName.trim().toLowerCase() && i.id !== selected?.id)
);
```

Add a helper to reload the library (needed after fork-save):
```typescript
async function reloadLibrary() {
  libraryLoaded = false;
  if (type === 'hop') library = (await ipc(listHopLibrary())) ?? [];
  else if (type === 'fermentable') library = (await ipc(listFermentableLibrary())) ?? [];
  else library = (await ipc(listYeastLibrary())) ?? [];
  libraryLoaded = true;
}
```

Add a function to enter fork mode when "Duplicate & Edit" is clicked:
```typescript
function enterForkMode() {
  if (!selected) return;
  forkMode = true;
  forkName = selected.name + ' (Custom)';
}
```

Add a function to save the fork:
```typescript
async function saveFork() {
  if (!selected || forkSaving || forkNameCollision || !forkName.trim()) return;
  forkSaving = true;
  let saved = null;
  if (type === 'hop') {
    const h = selected as Hop;
    saved = await ipc(createHop({
      name: forkName.trim(),
      forked_from_id: h.id,
      alpha_pct: h.alpha_pct,
      beta_pct: h.beta_pct ?? null,
      form: h.form,
      type_: h.type_ ?? null,
      origin: h.origin ?? null,
      notes: h.notes ?? null,
      substitutes: h.substitutes ?? null,
    } satisfies CreateHopInput));
  } else if (type === 'fermentable') {
    const f = selected as Fermentable;
    saved = await ipc(createFermentable({
      name: forkName.trim(),
      forked_from_id: f.id,
      type_: f.type_,
      yield_pct: f.yield_pct,
      color_lovibond: f.color_lovibond,
      origin: f.origin ?? null,
      notes: f.notes ?? null,
      add_after_boil: f.add_after_boil,
    } satisfies CreateFermentableInput));
  } else {
    const y = selected as Yeast;
    saved = await ipc(createYeast({
      name: forkName.trim(),
      forked_from_id: y.id,
      type_: y.type_,
      form: y.form,
      laboratory: y.laboratory ?? null,
      product_id: y.product_id ?? null,
      attenuation_pct: y.attenuation_pct ?? null,
      flocculation: y.flocculation ?? null,
      notes: y.notes ?? null,
      add_to_secondary: y.add_to_secondary,
    } satisfies CreateYeastInput));
  }
  forkSaving = false;
  if (saved) {
    await reloadLibrary();
    selected = library.find(i => i.id === saved.id) ?? saved;
    forkMode = false;
  }
}
```

Reset `forkMode` when `selected` changes:
```typescript
$effect(() => {
  if (selected !== null) forkMode = false;
});
```

Also update the existing `$effect` that resets fields when `open` changes to also reset `forkMode`:
```typescript
$effect(() => {
  if (!dialog) return;
  if (open) {
    loadLibrary();
    dialog.showModal();
    query = '';
    selected = null;
    forkMode = false;
    setTimeout(() => searchInput?.focus(), 0);
  } else if (dialog.open) {
    dialog.close();
  }
});
```

- [ ] **Step 2: Add "Duplicate & Edit" button and fork form to detail panel**

In the template of `src/lib/desktop/IngredientPicker.svelte`, inside the right detail panel (`<!-- Right: detail -->`), in each `{:else if type === 'hop'}`, `{:else if type === 'fermentable'}`, and `{:else}` (yeast) block, add a "Duplicate & Edit" button in the footer area and the fork-mode form overlay.

For the hop detail block, replace the footer `<div>` (the one with "Add to Recipe" button) with:

```svelte
{#if forkMode}
  <!-- Fork name form overlaying the footer -->
  <div style="border-top: 1px solid var(--color-border); padding: 12px 16px; background: var(--color-bg-surface); flex-shrink: 0; display: flex; flex-direction: column; gap: 8px;">
    <div style="font-size: 12px; color: var(--color-text-secondary);">Save a custom copy with a new name:</div>
    {#if forkNameCollision}
      <div style="font-size: 11px; color: #fca5a5; background: #7f1d1d20; border: 1px solid #dc262650; padding: 4px 8px; border-radius: 4px;">
        Name already exists — choose a different name.
      </div>
    {/if}
    <div style="display: flex; gap: 8px; align-items: center;">
      <input bind:value={forkName}
             style="flex: 1; background: var(--color-bg-elevated); border: 1px solid var(--color-border); border-radius: 5px; padding: 5px 8px; color: var(--color-text-primary); font-size: 13px;" />
      <button onclick={() => { forkMode = false; }} style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); border-radius: 6px; padding: 6px 12px; font-size: 13px; color: var(--color-text-secondary); cursor: pointer;">Cancel</button>
      <button onclick={saveFork} disabled={forkSaving || forkNameCollision || !forkName.trim()}
              style="background: {forkSaving || forkNameCollision || !forkName.trim() ? 'var(--color-bg-elevated)' : 'var(--color-accent)'}; color: {forkSaving || forkNameCollision || !forkName.trim() ? 'var(--color-text-muted)' : '#fff'}; border: none; border-radius: 6px; padding: 6px 14px; font-size: 13px; font-weight: 600; cursor: pointer;">
        {forkSaving ? 'Saving…' : 'Save Copy'}
      </button>
    </div>
  </div>
{:else}
  <div style="border-top: 1px solid var(--color-border); padding: 12px 16px; display: flex; gap: 10px; align-items: flex-end; background: var(--color-bg-surface); flex-shrink: 0;">
    <!-- ... all existing amount/use/time inputs ... -->
    {#if hop.source === 'seeded'}
      <button onclick={enterForkMode}
              style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); border-radius: 6px; padding: 6px 12px; font-size: 12px; color: var(--color-text-secondary); cursor: pointer; flex-shrink: 0;">
        Duplicate & Edit
      </button>
    {/if}
    <button onclick={handleAdd} disabled={!canAdd}
      style="margin-left: auto; background: {canAdd ? 'var(--color-accent)' : 'var(--color-bg-elevated)'}; color: {canAdd ? '#fff' : 'var(--color-text-muted)'}; border: none; border-radius: 6px; padding: 8px 18px; font-size: 13px; font-weight: 600; cursor: {canAdd ? 'pointer' : 'default'};">
      Add to Recipe
    </button>
  </div>
{/if}
```

Apply the same fork-mode footer pattern to the fermentable and yeast detail blocks.

**Important:** The existing footer content (amount inputs, use select, time input, hopstand temp input, Add to Recipe button) must be preserved exactly — only wrap it in `{:else}` and add the `forkMode` branch and "Duplicate & Edit" button.

- [ ] **Step 3: Update table imports**

In each of the three table files, update the IngredientPicker import:

`src/lib/components/ingredients/HopsTable.svelte`:
```typescript
import IngredientPicker, { type AddPayload } from "$platform/IngredientPicker.svelte";
```

`src/lib/components/ingredients/FermentablesTable.svelte`:
```typescript
import IngredientPicker, { type AddPayload } from "$platform/IngredientPicker.svelte";
```

`src/lib/components/ingredients/YeastsTable.svelte`:
```typescript
import IngredientPicker, { type AddPayload } from "$platform/IngredientPicker.svelte";
```

- [ ] **Step 4: Verify TypeScript compilation**

Run: `just check-ts`

Expected: No errors.

- [ ] **Step 5: Commit**

```bash
git add src/lib/desktop/IngredientPicker.svelte \
        src/lib/components/ingredients/HopsTable.svelte \
        src/lib/components/ingredients/FermentablesTable.svelte \
        src/lib/components/ingredients/YeastsTable.svelte
git commit -m "feat: add desktop IngredientPicker with Duplicate & Edit flow"
```

---

### Task 6: Mobile IngredientPicker

**Files:**
- Create: `src/lib/mobile/IngredientPicker.svelte`

The mobile picker is a full-screen overlay (not a `<dialog>`). It uses a `screen` state (`'list' | 'detail'`) to push between panels instead of side-by-side layout. It exports the same `AddPayload` type and the same props interface as the desktop picker so the table components work unchanged.

- [ ] **Step 1: Create `src/lib/mobile/IngredientPicker.svelte`**

```svelte
<script lang="ts">
  import BrewingIcon from "$lib/components/BrewingIcon.svelte";
  import type { Hop, Fermentable, Yeast } from '$lib/api';
  import { listHopLibrary, listFermentableLibrary, listYeastLibrary } from '$lib/api';
  import { ipc } from '$lib/stores/error';
  import { settings } from '$lib/stores/settings';
  import {
    kgToHopDisplay, hopDisplayToKg, hopWeightLabel,
    lbToKg, weightLabel,
    cToF, fToC, tempLabel,
    type Units,
  } from '$lib/units';
  import type { BrewingIconName } from "$lib/icons";

  export type AddPayload =
    | { type: 'hop'; item: Hop; amount_kg: number; use_: string; time_min: number; hopstand_temp_c: number | null }
    | { type: 'fermentable'; item: Fermentable; amount_kg: number }
    | { type: 'yeast'; item: Yeast; amount: number };

  const HOP_USES = ['boil', 'aroma', 'dry hop', 'first wort', 'hopstand'] as const;

  let {
    type,
    open,
    onclose,
    onadd,
  }: {
    type: 'hop' | 'fermentable' | 'yeast';
    open: boolean;
    onclose: () => void;
    onadd: (payload: AddPayload) => void;
  } = $props();

  let screen = $state<'list' | 'detail'>('list');
  let query = $state('');
  let library = $state<(Hop | Fermentable | Yeast)[]>([]);
  let libraryLoaded = $state(false);
  let selected = $state<Hop | Fermentable | Yeast | null>(null);
  let amount = $state(0);
  let use_ = $state('boil');
  let time = $state(60);
  let hopstand_temp_c = $state(80);

  const units = $derived<Units>($settings.units === 'imperial' ? 'imperial' : 'metric');

  async function loadLibrary() {
    if (libraryLoaded) return;
    if (type === 'hop') library = (await ipc(listHopLibrary())) ?? [];
    else if (type === 'fermentable') library = (await ipc(listFermentableLibrary())) ?? [];
    else library = (await ipc(listYeastLibrary())) ?? [];
    libraryLoaded = true;
  }

  $effect(() => {
    if (open) {
      loadLibrary();
      query = '';
      selected = null;
      screen = 'list';
    }
  });

  $effect(() => {
    if (!selected) return;
    if (type === 'hop') { amount = hopDisplayToKg(units === 'imperial' ? 1 : 28, units); use_ = 'boil'; time = 60; hopstand_temp_c = 80; }
    else if (type === 'fermentable') { amount = units === 'imperial' ? lbToKg(2) : 1.0; }
    else { amount = 1; }
  });

  const filtered = $derived(
    query.trim() === ''
      ? library
      : library.filter(item => item.name.toLowerCase().includes(query.trim().toLowerCase()))
  );

  const canAdd = $derived(selected !== null && amount > 0);

  function selectItem(item: Hop | Fermentable | Yeast) {
    selected = item;
    screen = 'detail';
  }

  function handleAdd() {
    if (!selected || amount <= 0) return;
    if (type === 'hop') {
      onadd({ type: 'hop', item: selected as Hop, amount_kg: amount, use_, time_min: time, hopstand_temp_c: use_ === 'hopstand' ? hopstand_temp_c : null });
    } else if (type === 'fermentable') {
      onadd({ type: 'fermentable', item: selected as Fermentable, amount_kg: amount });
    } else {
      onadd({ type: 'yeast', item: selected as Yeast, amount });
    }
  }

  function rowSubtext(item: Hop | Fermentable | Yeast): string {
    if (type === 'hop') return `${(item as Hop).alpha_pct}% AA`;
    if (type === 'fermentable') {
      const f = item as Fermentable;
      return `${f.yield_pct.toFixed(0)}% · ${f.color_lovibond}°L`;
    }
    const y = item as Yeast;
    return y.laboratory ?? y.form;
  }

  function fmt(val: number | null, digits = 1): string {
    return val == null ? '—' : val.toFixed(digits);
  }

  const headerIcon = $derived<BrewingIconName>(
    type === "hop" ? "hop" : type === "fermentable" ? "fermentable" : "yeast"
  );
  const headerTitle = $derived(
    type === "hop" ? "Add Hop" : type === "fermentable" ? "Add Fermentable" : "Add Yeast"
  );
</script>

{#if open}
  <div class="fixed inset-0 flex flex-col" style="z-index: 200; background: var(--color-bg-base); color: var(--color-text-primary);">

    {#if screen === 'list'}
      <!-- List screen -->
      <div class="flex items-center gap-3 px-4 py-3 flex-shrink-0"
           style="border-bottom: 1px solid var(--color-border); background: var(--color-bg-surface);">
        <button onclick={onclose} style="background: none; border: none; cursor: pointer; color: var(--color-text-secondary); padding: 4px; font-size: 20px; line-height: 1;">×</button>
        <span style="font-size: 16px; display: inline-flex;"><BrewingIcon name={headerIcon} /></span>
        <h2 class="text-base font-semibold flex-1" style="color: var(--color-text-primary); margin: 0;">{headerTitle}</h2>
      </div>
      <div class="px-4 py-2 flex-shrink-0">
        <div class="relative">
          <svg style="position: absolute; left: 8px; top: 50%; transform: translateY(-50%); pointer-events: none; color: var(--color-text-muted);"
               width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
            <circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
          </svg>
          <input bind:value={query}
                 placeholder="Search {type === 'hop' ? 'hops' : type === 'fermentable' ? 'fermentables' : 'yeasts'}…"
                 class="w-full text-sm"
                 style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); border-radius: 8px; padding: 8px 12px 8px 28px; color: var(--color-text-primary); outline: none;" />
        </div>
      </div>
      <div class="flex-1 overflow-y-auto px-4 pb-4 flex flex-col gap-1">
        {#if filtered.length === 0}
          <p class="text-sm text-center mt-8" style="color: var(--color-text-muted);">No results for "{query}"</p>
        {:else}
          {#each filtered as item (item.id)}
            <button onclick={() => selectItem(item)}
                    class="flex items-center justify-between w-full text-left px-3 py-3 rounded-lg"
                    style="background: var(--color-bg-elevated); border: 1px solid var(--color-border);">
              <span class="text-sm" style="color: var(--color-text-primary);">{item.name}</span>
              <span class="text-xs flex-shrink-0 ml-3" style="color: var(--color-text-secondary);">{rowSubtext(item)}</span>
            </button>
          {/each}
        {/if}
      </div>

    {:else}
      <!-- Detail screen -->
      <div class="flex items-center gap-3 px-4 py-3 flex-shrink-0"
           style="border-bottom: 1px solid var(--color-border); background: var(--color-bg-surface);">
        <button onclick={() => { screen = 'list'; }}
                style="background: none; border: none; cursor: pointer; color: var(--color-accent); font-size: 14px; padding: 4px 0; display: flex; align-items: center; gap: 4px;">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
          Back
        </button>
        <h2 class="text-base font-semibold flex-1 truncate" style="color: var(--color-text-primary); margin: 0;">{selected?.name ?? ''}</h2>
      </div>

      <div class="flex-1 overflow-y-auto p-4 flex flex-col gap-3">
        {#if selected && type === 'hop'}
          {@const hop = selected as Hop}
          <div style="display: flex; gap: 6px; flex-wrap: wrap;">
            {#if hop.origin}<span style="background: var(--color-bg-elevated); color: var(--color-text-secondary); padding: 3px 10px; border-radius: 99px; font-size: 12px;">{hop.origin}</span>{/if}
            {#if hop.type_}<span style="background: var(--color-bg-elevated); color: var(--color-text-secondary); padding: 3px 10px; border-radius: 99px; font-size: 12px;">{hop.type_}</span>{/if}
            <span style="background: var(--color-bg-elevated); color: var(--color-text-secondary); padding: 3px 10px; border-radius: 99px; font-size: 12px;">{hop.form}</span>
          </div>
          <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 6px;">
            {#each [['Alpha', hop.alpha_pct + '%'], ['Beta', fmt(hop.beta_pct ?? null) + '%'], ['Cohumulone', fmt(hop.cohumulone_pct ?? null) + '%']] as [label, value]}
              <div style="background: var(--color-bg-elevated); border-radius: 6px; padding: 8px 10px;">
                <div style="font-size: 11px; color: var(--color-text-secondary);">{label}</div>
                <div style="font-size: 14px; font-weight: 600;">{value}</div>
              </div>
            {/each}
          </div>
          {#if hop.notes}<p style="font-size: 13px; color: var(--color-text-secondary); line-height: 1.5; margin: 0;">{hop.notes}</p>{/if}

        {:else if selected && type === 'fermentable'}
          {@const ferm = selected as Fermentable}
          <div style="display: flex; gap: 6px; flex-wrap: wrap;">
            <span style="background: var(--color-bg-elevated); color: var(--color-text-secondary); padding: 3px 10px; border-radius: 99px; font-size: 12px;">{ferm.type_}</span>
          </div>
          <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 6px;">
            {#each [['Yield', ferm.yield_pct.toFixed(0) + '%'], ['Color', ferm.color_lovibond + '°L'], ['Origin', ferm.origin ?? '—']] as [label, value]}
              <div style="background: var(--color-bg-elevated); border-radius: 6px; padding: 8px 10px;">
                <div style="font-size: 11px; color: var(--color-text-secondary);">{label}</div>
                <div style="font-size: 14px; font-weight: 600;">{value}</div>
              </div>
            {/each}
          </div>
          {#if ferm.notes}<p style="font-size: 13px; color: var(--color-text-secondary); line-height: 1.5; margin: 0;">{ferm.notes}</p>{/if}

        {:else if selected && type === 'yeast'}
          {@const yeast = selected as Yeast}
          <div style="display: flex; gap: 6px; flex-wrap: wrap;">
            <span style="background: var(--color-bg-elevated); color: var(--color-text-secondary); padding: 3px 10px; border-radius: 99px; font-size: 12px;">{yeast.type_}</span>
            <span style="background: var(--color-bg-elevated); color: var(--color-text-secondary); padding: 3px 10px; border-radius: 99px; font-size: 12px;">{yeast.form}</span>
            {#if yeast.laboratory}<span style="background: var(--color-bg-elevated); color: var(--color-text-secondary); padding: 3px 10px; border-radius: 99px; font-size: 12px;">{yeast.laboratory}</span>{/if}
          </div>
          <div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 6px;">
            {#each [['Attenuation', fmt(yeast.attenuation_pct ?? null, 0) + '%'], ['Flocculation', yeast.flocculation ?? '—']] as [label, value]}
              <div style="background: var(--color-bg-elevated); border-radius: 6px; padding: 8px 10px;">
                <div style="font-size: 11px; color: var(--color-text-secondary);">{label}</div>
                <div style="font-size: 14px; font-weight: 600;">{value}</div>
              </div>
            {/each}
          </div>
          {#if yeast.notes}<p style="font-size: 13px; color: var(--color-text-secondary); line-height: 1.5; margin: 0;">{yeast.notes}</p>{/if}
        {/if}
      </div>

      <!-- Add controls footer -->
      <div class="flex-shrink-0 p-4 flex flex-col gap-3"
           style="border-top: 1px solid var(--color-border); background: var(--color-bg-surface);">
        {#if type === 'hop' && selected}
          <div class="flex gap-3">
            <div class="flex flex-col gap-1 flex-1">
              <label class="text-xs" style="color: var(--color-text-secondary);">Amount ({hopWeightLabel(units)})</label>
              <input type="number" inputmode="decimal" step={units === 'imperial' ? 0.1 : 1}
                     value={kgToHopDisplay(amount, units).toFixed(units === 'imperial' ? 2 : 0)}
                     onblur={(e) => { const v = parseFloat((e.target as HTMLInputElement).value); if (!isNaN(v)) amount = hopDisplayToKg(v, units); }}
                     min="0.001" class="px-3 py-2 rounded-lg text-sm"
                     style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </div>
            <div class="flex flex-col gap-1 flex-1">
              <label class="text-xs" style="color: var(--color-text-secondary);">Use</label>
              <select bind:value={use_} class="px-3 py-2 rounded-lg text-sm"
                      style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); color: var(--color-text-primary);">
                {#each HOP_USES as u}<option value={u}>{u}</option>{/each}
              </select>
            </div>
            <div class="flex flex-col gap-1" style="width: 70px;">
              <label class="text-xs" style="color: var(--color-text-secondary);">Time (min)</label>
              <input type="number" inputmode="decimal" step="5" bind:value={time} min="0" class="px-3 py-2 rounded-lg text-sm"
                     style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </div>
          </div>
          {#if use_ === 'hopstand'}
            <div class="flex flex-col gap-1" style="max-width: 120px;">
              <label class="text-xs" style="color: var(--color-text-secondary);">Temp ({tempLabel(units)})</label>
              <input type="number" inputmode="decimal" step="1"
                     value={units === 'imperial' ? cToF(hopstand_temp_c).toFixed(0) : hopstand_temp_c}
                     onblur={(e) => { const v = parseFloat((e.target as HTMLInputElement).value); if (!isNaN(v)) hopstand_temp_c = units === 'imperial' ? fToC(v) : v; }}
                     min="0" class="px-3 py-2 rounded-lg text-sm"
                     style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </div>
          {/if}
        {:else if type === 'fermentable' && selected}
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Amount ({weightLabel(units)})</label>
            <input type="number" inputmode="decimal" step="0.1" min="0"
                   value={units === 'imperial' ? (amount / 0.453592).toFixed(2) : amount.toFixed(2)}
                   onblur={(e) => { const v = parseFloat((e.target as HTMLInputElement).value); if (!isNaN(v)) amount = units === 'imperial' ? v * 0.453592 : v; }}
                   class="px-3 py-2 rounded-lg text-sm"
                   style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); color: var(--color-text-primary); max-width: 150px;" />
          </div>
        {:else if type === 'yeast' && selected}
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Packs / Units</label>
            <input type="number" inputmode="decimal" step="1" min="1" bind:value={amount}
                   class="px-3 py-2 rounded-lg text-sm"
                   style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); color: var(--color-text-primary); max-width: 100px;" />
          </div>
        {/if}
        <button onclick={handleAdd} disabled={!canAdd}
                class="w-full py-3 rounded-lg text-sm font-semibold"
                style="background: {canAdd ? 'var(--color-accent)' : 'var(--color-bg-elevated)'}; color: {canAdd ? '#fff' : 'var(--color-text-muted)'}; border: none; cursor: {canAdd ? 'pointer' : 'default'};">
          Add to Recipe
        </button>
      </div>
    {/if}
  </div>
{/if}
```

- [ ] **Step 2: Verify TypeScript compilation**

Run: `just check-ts`

Expected: No errors.

- [ ] **Step 3: Commit**

```bash
git add src/lib/mobile/IngredientPicker.svelte
git commit -m "feat: add mobile IngredientPicker with full-screen push navigation"
```

---

## Self-Review

**Spec coverage:**
- ✅ API types and CRUD functions for all 5 ingredient types (Task 1)
- ✅ Desktop Library nav icon (Task 2)
- ✅ Mobile More tab updated + settings page Library link (Task 3)
- ✅ Library screen with tabs, search, source badges, create/edit/delete (Task 4)
- ✅ IngredientEditModal for all 5 types with name collision warning (Task 4)
- ✅ Desktop picker: Duplicate & Edit flow with fork-save and name collision warning (Task 5)
- ✅ Table imports updated to `$platform/IngredientPicker.svelte` (Task 5)
- ✅ Mobile picker: full-screen with list→detail navigation (Task 6)

**Type consistency:**
- `AddPayload` is defined identically in both `src/lib/desktop/IngredientPicker.svelte` and `src/lib/mobile/IngredientPicker.svelte`, matching the existing definition in `src/lib/components/ingredients/IngredientPicker.svelte`.
- The old `src/lib/components/ingredients/IngredientPicker.svelte` file is no longer imported by any table component after Task 5 updates the imports. It can be left in place (it's harmless) or deleted. The plan does not delete it to minimize blast radius, but the agent may delete it.

**Notes for the implementer:**
- The `$platform` alias in vite resolves `$platform/IngredientPicker.svelte` to `src/lib/desktop/IngredientPicker.svelte` (desktop builds) or `src/lib/mobile/IngredientPicker.svelte` (iOS/Android builds), based on `TAURI_ENV_PLATFORM`.
- The `ipc()` helper from `$lib/stores/error` wraps any `Promise<T>` — on error it sets the error toast and returns `null`. Always check for `null` on the result.
- All `source` field checks are done against the string `'seeded'` (not a boolean). Seeded rows have `item.source === 'seeded'`; user rows have `item.source === 'user'`.
- The backend enforces the source guard — `update_hop` and `delete_hop` will return an error for seeded ingredients. The UI should prevent calling them (hide Edit/Delete buttons for seeded rows), but the backend is the authoritative guard.
