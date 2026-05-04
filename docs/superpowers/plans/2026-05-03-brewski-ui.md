# Brewski UI Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the complete Svelte 5 frontend — typed API wrappers, app shell with icon rail + recipe list, full-screen recipe editor with tabs (Overview, Ingredients, Mash, Fermentation, Notes) and persistent stats sidebar, plus settings page.

**Architecture:** Pure UI layer. No business logic in TypeScript. `src/lib/api.ts` is the only file that calls `invoke()`. Components call api.ts functions and render returned data. Stats sidebar calls `get_recipe_stats` after every ingredient change.

**Tech Stack:** Svelte 5, SvelteKit, TypeScript, Tailwind CSS v4 (CSS custom properties), `@tauri-apps/api`

**Prerequisite:** `2026-05-03-brewski-backend.md` must be complete.

---

### Task 1: Typed API wrappers (`api.ts`)

**Files:**
- Create: `src/lib/api.ts`

- [ ] **Step 1: Create `src/lib/api.ts`**

```typescript
import { invoke } from "@tauri-apps/api/core";

export interface RecipeSummary {
  id: string;
  name: string;
  style_name: string | null;
  type_: string;
  batch_size_l: number;
  created_at: number;
  updated_at: number;
}

export interface EquipmentProfile {
  id: string;
  name: string;
  notes: string | null;
  boil_size_l: number;
  batch_size_l: number;
  boil_time_min: number;
  evap_rate_pct_hr: number;
  trub_chiller_loss_l: number;
  fermenter_loss_l: number;
  efficiency_pct: number;
  created_at: number;
  updated_at: number;
}

export interface Style {
  id: string;
  name: string;
  category: string;
  og_min: number;
  og_max: number;
  fg_min: number;
  fg_max: number;
  ibu_min: number;
  ibu_max: number;
  color_min_srm: number;
  color_max_srm: number;
  abv_min_pct: number | null;
  abv_max_pct: number | null;
}

export interface RecipeAdditionFermentable {
  id: string;
  recipe_id: string;
  fermentable_id: string | null;
  name: string;
  type_: string;
  yield_pct: number;
  color_lovibond: number;
  amount_kg: number;
  add_after_boil: boolean;
  addition_order: number;
}

export interface RecipeAdditionHop {
  id: string;
  recipe_id: string;
  hop_id: string | null;
  name: string;
  alpha_pct: number;
  form: string;
  amount_kg: number;
  use_: string;
  time_min: number;
  addition_order: number;
}

export interface RecipeAdditionYeast {
  id: string;
  recipe_id: string;
  yeast_id: string | null;
  name: string;
  type_: string;
  form: string;
  laboratory: string | null;
  product_id: string | null;
  attenuation_pct: number | null;
  amount: number | null;
}

export interface RecipeAdditionMisc {
  id: string;
  recipe_id: string;
  name: string;
  type_: string;
  use_: string;
  amount: number;
  time_min: number;
  addition_order: number;
}

export interface RecipeAdditionWater {
  id: string;
  recipe_id: string;
  name: string;
  amount_l: number;
}

export interface MashStep {
  id: string;
  mash_id: string;
  name: string;
  type_: string;
  infuse_amount_l: number | null;
  step_temp_c: number;
  step_time_min: number;
  ramp_time_min: number | null;
  step_order: number;
}

export interface Mash {
  id: string;
  recipe_id: string;
  name: string;
  grain_temp_c: number;
  sparge_temp_c: number | null;
  ph: number | null;
  notes: string | null;
  steps: MashStep[];
}

export interface Recipe {
  id: string;
  name: string;
  type_: string;
  brewer: string | null;
  batch_size_l: number;
  boil_size_l: number;
  boil_time_min: number;
  efficiency_pct: number | null;
  style_id: string | null;
  equipment_profile_id: string | null;
  notes: string | null;
  taste_notes: string | null;
  taste_rating: number | null;
  fermentation_stages: number;
  primary_age_days: number | null;
  primary_temp_c: number | null;
  secondary_age_days: number | null;
  secondary_temp_c: number | null;
  carbonation_vols: number | null;
  forced_carbonation: boolean;
  date: string | null;
  equipment_profile: EquipmentProfile | null;
  style: Style | null;
  fermentables: RecipeAdditionFermentable[];
  hops: RecipeAdditionHop[];
  yeasts: RecipeAdditionYeast[];
  miscs: RecipeAdditionMisc[];
  waters: RecipeAdditionWater[];
  mash: Mash | null;
}

export interface RecipeStats {
  og: number;
  fg: number;
  abv_pct: number;
  ibu: number;
  srm: number;
  calories_per_355ml: number;
  bu_gu_ratio: number;
  pre_boil_gravity: number;
  pre_boil_volume_l: number;
  post_boil_volume_l: number;
}

export interface Fermentable {
  id: string;
  name: string;
  type_: string;
  yield_pct: number;
  color_lovibond: number;
}

export interface Hop {
  id: string;
  name: string;
  alpha_pct: number;
  form: string;
}

export interface Yeast {
  id: string;
  name: string;
  type_: string;
  form: string;
  laboratory: string | null;
  product_id: string | null;
  attenuation_pct: number | null;
}

// --- Recipes ---
export const listRecipes = () => invoke<RecipeSummary[]>("list_recipes");
export const getRecipe = (id: string) => invoke<Recipe>("get_recipe", { id });
export const createRecipe = (input: {
  name: string;
  type_?: string;
  batch_size_l?: number;
  boil_size_l?: number;
  boil_time_min?: number;
  equipment_profile_id?: string;
  source_id?: string;
}) => invoke<Recipe>("create_recipe", { input });
export const updateRecipe = (id: string, input: Partial<Recipe>) =>
  invoke<Recipe>("update_recipe", { id, input });
export const deleteRecipe = (id: string) => invoke<void>("delete_recipe", { id });
export const getRecipeStats = (recipeId: string) =>
  invoke<RecipeStats>("get_recipe_stats", { recipeId });

// --- Recipe additions ---
export const createRecipeFermentable = (recipeId: string, input: object) =>
  invoke<RecipeAdditionFermentable>("create_recipe_fermentable", { recipeId, input });
export const updateRecipeFermentable = (id: string, input: object) =>
  invoke<RecipeAdditionFermentable>("update_recipe_fermentable", { id, input });
export const deleteRecipeFermentable = (id: string) =>
  invoke<void>("delete_recipe_fermentable", { id });

export const createRecipeHop = (recipeId: string, input: object) =>
  invoke<RecipeAdditionHop>("create_recipe_hop", { recipeId, input });
export const updateRecipeHop = (id: string, input: object) =>
  invoke<RecipeAdditionHop>("update_recipe_hop", { id, input });
export const deleteRecipeHop = (id: string) => invoke<void>("delete_recipe_hop", { id });

export const createRecipeYeast = (recipeId: string, input: object) =>
  invoke<RecipeAdditionYeast>("create_recipe_yeast", { recipeId, input });
export const updateRecipeYeast = (id: string, input: object) =>
  invoke<RecipeAdditionYeast>("update_recipe_yeast", { id, input });
export const deleteRecipeYeast = (id: string) => invoke<void>("delete_recipe_yeast", { id });

export const createRecipeMisc = (recipeId: string, input: object) =>
  invoke<RecipeAdditionMisc>("create_recipe_misc", { recipeId, input });
export const updateRecipeMisc = (id: string, input: object) =>
  invoke<RecipeAdditionMisc>("update_recipe_misc", { id, input });
export const deleteRecipeMisc = (id: string) => invoke<void>("delete_recipe_misc", { id });

export const createRecipeWater = (recipeId: string, input: object) =>
  invoke<RecipeAdditionWater>("create_recipe_water", { recipeId, input });
export const updateRecipeWater = (id: string, input: object) =>
  invoke<RecipeAdditionWater>("update_recipe_water", { id, input });
export const deleteRecipeWater = (id: string) => invoke<void>("delete_recipe_water", { id });

// --- Mash ---
export const getMash = (recipeId: string) => invoke<Mash>("get_mash", { recipeId });
export const updateMash = (recipeId: string, input: object) =>
  invoke<Mash>("update_mash", { recipeId, input });
export const createMashStep = (mashId: string, input: object) =>
  invoke<MashStep>("create_mash_step", { mashId, input });
export const updateMashStep = (id: string, input: object) =>
  invoke<MashStep>("update_mash_step", { id, input });
export const deleteMashStep = (id: string) => invoke<void>("delete_mash_step", { id });
export const updateMashStepOrder = (orderedIds: string[]) =>
  invoke<void>("update_mash_step_order", { orderedIds });

// --- Equipment + library ---
export const listEquipmentProfiles = () => invoke<EquipmentProfile[]>("list_equipment_profiles");
export const createEquipmentProfile = (input: object) =>
  invoke<EquipmentProfile>("create_equipment_profile", { input });
export const updateEquipmentProfile = (id: string, input: object) =>
  invoke<EquipmentProfile>("update_equipment_profile", { id, input });
export const deleteEquipmentProfile = (id: string) =>
  invoke<void>("delete_equipment_profile", { id });

export const listStyles = () => invoke<Style[]>("list_styles");
export const listFermentableLibrary = () => invoke<Fermentable[]>("list_fermentable_library");
export const listHopLibrary = () => invoke<Hop[]>("list_hop_library");
export const listYeastLibrary = () => invoke<Yeast[]>("list_yeast_library");

// --- Settings ---
export const getSettings = () => invoke<Record<string, string>>("get_settings");
export const updateSetting = (key: string, value: string) =>
  invoke<void>("update_setting", { key, value });

// --- Import / export ---
export const getRecipeBeerxml = (recipeId: string) =>
  invoke<string>("get_recipe_beerxml", { recipeId });
export const createRecipesFromBeerxml = (xml: string) =>
  invoke<RecipeSummary[]>("create_recipes_from_beerxml", { xml });
```

- [ ] **Step 2: Verify TypeScript compiles**

```bash
npm run check
```

Expected: no type errors.

- [ ] **Step 3: Commit**

```bash
git add src/lib/api.ts
git commit -m "feat: typed api.ts wrappers for all Tauri commands"
```

---

### Task 2: Svelte stores

**Files:**
- Create: `src/lib/stores/settings.ts`
- Create: `src/lib/stores/recipes.ts`

- [ ] **Step 1: Create `src/lib/stores/settings.ts`**

```typescript
import { writable } from "svelte/store";
import { getSettings, updateSetting } from "$lib/api";

export const settings = writable<Record<string, string>>({});

export async function loadSettings() {
  const s = await getSettings();
  settings.set(s);
}

export async function saveSetting(key: string, value: string) {
  await updateSetting(key, value);
  settings.update((s) => ({ ...s, [key]: value }));
}
```

- [ ] **Step 2: Create `src/lib/stores/recipes.ts`**

```typescript
import { writable } from "svelte/store";
import type { RecipeSummary } from "$lib/api";
import { listRecipes } from "$lib/api";

export const recipeList = writable<RecipeSummary[]>([]);

export async function refreshRecipeList() {
  const list = await listRecipes();
  recipeList.set(list);
}
```

- [ ] **Step 3: Commit**

```bash
git add src/lib/stores/
git commit -m "feat: settings and recipe list stores"
```

---

### Task 3: App shell — icon rail + recipe list

**Files:**
- Modify: `src/routes/+layout.svelte`
- Modify: `src/routes/+page.svelte`
- Create: `src/lib/components/RecipeList.svelte`
- Create: `src/lib/components/AppShell.svelte`

- [ ] **Step 1: Create `src/lib/components/AppShell.svelte`**

```svelte
<script lang="ts">
  import { page } from "$app/stores";
  let { children } = $props();

  const isRecipes = $derived($page.url.pathname === "/" || $page.url.pathname.startsWith("/recipe"));
</script>

<div class="flex h-screen overflow-hidden" style="background: var(--color-bg-base); color: var(--color-text-primary);">
  <!-- Icon rail -->
  <nav class="flex flex-col items-center w-12 py-3 gap-2 border-r flex-shrink-0"
       style="background: var(--color-bg-surface); border-color: var(--color-border);">
    <!-- Recipes icon -->
    <a href="/" class="w-8 h-8 flex items-center justify-center rounded transition-colors"
       style={isRecipes ? "background: var(--color-accent); color: #fff;" : "color: var(--color-text-secondary);"}>
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
        <polyline points="14 2 14 8 20 8"/>
        <line x1="16" y1="13" x2="8" y2="13"/>
        <line x1="16" y1="17" x2="8" y2="17"/>
        <polyline points="10 9 9 9 8 9"/>
      </svg>
    </a>

    <div class="flex-1"></div>

    <!-- Settings icon -->
    <a href="/settings" class="w-8 h-8 flex items-center justify-center rounded transition-colors"
       style={$page.url.pathname === '/settings' ? "background: var(--color-accent); color: #fff;" : "color: var(--color-text-secondary);"}>
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="12" cy="12" r="3"/>
        <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/>
      </svg>
    </a>
  </nav>

  <!-- Main content area -->
  <div class="flex flex-1 overflow-hidden">
    {@render children()}
  </div>
</div>
```

- [ ] **Step 2: Create `src/lib/components/RecipeList.svelte`**

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { recipeList, refreshRecipeList } from "$lib/stores/recipes";
  import { createRecipe, deleteRecipe } from "$lib/api";
  import type { RecipeSummary } from "$lib/api";

  let { selectedId = $bindable<string | null>(null) } = $props();
  let search = $state("");

  const filtered = $derived(
    search.trim()
      ? $recipeList.filter((r) => r.name.toLowerCase().includes(search.toLowerCase()))
      : $recipeList
  );

  onMount(refreshRecipeList);

  async function handleNew() {
    const recipe = await createRecipe({ name: "New Recipe" });
    await refreshRecipeList();
    goto(`/recipe/${recipe.id}`);
  }

  async function handleDelete(e: MouseEvent, id: string) {
    e.stopPropagation();
    e.preventDefault();
    if (!confirm("Delete this recipe?")) return;
    await deleteRecipe(id);
    await refreshRecipeList();
    if (selectedId === id) goto("/");
  }
</script>

<aside class="w-56 flex flex-col flex-shrink-0 border-r overflow-hidden"
       style="background: var(--color-bg-surface); border-color: var(--color-border);">
  <!-- Search + New -->
  <div class="p-2 flex flex-col gap-1.5 border-b" style="border-color: var(--color-border);">
    <input
      type="search"
      placeholder="Search recipes…"
      bind:value={search}
      class="w-full px-2.5 py-1.5 rounded text-sm outline-none"
      style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);"
    />
    <button
      onclick={handleNew}
      class="w-full py-1.5 rounded text-sm font-medium transition-colors"
      style="background: var(--color-accent); color: #fff;"
    >
      + New Recipe
    </button>
  </div>

  <!-- Recipe list -->
  <ul class="flex-1 overflow-y-auto py-1">
    {#each filtered as recipe (recipe.id)}
      <li>
        <a
          href="/recipe/{recipe.id}"
          class="group flex flex-col px-3 py-2 cursor-pointer transition-colors"
          style={selectedId === recipe.id
            ? "background: var(--color-bg-elevated);"
            : "color: var(--color-text-primary);"}
        >
          <span class="text-sm font-medium truncate" style="color: var(--color-text-primary);">{recipe.name}</span>
          <span class="text-xs truncate mt-0.5" style="color: var(--color-text-secondary);">
            {recipe.style_name ?? recipe.type_} · {recipe.batch_size_l}L
          </span>
        </a>
      </li>
    {:else}
      <li class="px-3 py-6 text-center text-sm" style="color: var(--color-text-muted);">
        {search ? "No matches" : "No recipes yet"}
      </li>
    {/each}
  </ul>
</aside>
```

- [ ] **Step 3: Update `src/routes/+layout.svelte`**

```svelte
<script lang="ts">
  import "../app.css";
  import "../themes/midnight.css";
  import AppShell from "$lib/components/AppShell.svelte";

  let { children } = $props();
</script>

<AppShell>
  {@render children()}
</AppShell>
```

- [ ] **Step 4: Update `src/routes/+page.svelte`** (recipe list home)

```svelte
<script lang="ts">
  import RecipeList from "$lib/components/RecipeList.svelte";
</script>

<RecipeList />

<div class="flex-1 flex items-center justify-center" style="color: var(--color-text-muted);">
  <p class="text-sm">Select a recipe to edit</p>
</div>
```

- [ ] **Step 5: Start dev server and verify layout**

```bash
npm run tauri dev
```

Expected: dark window with icon rail on the left, recipe list panel (empty), and "Select a recipe" message in center. "+ New Recipe" button visible. No console errors.

- [ ] **Step 6: Commit**

```bash
git add src/
git commit -m "feat: app shell with icon rail and recipe list"
```

---

### Task 4: Stats sidebar component

**Files:**
- Create: `src/lib/components/StatsSidebar.svelte`

- [ ] **Step 1: Create `src/lib/components/StatsSidebar.svelte`**

```svelte
<script lang="ts">
  import type { RecipeStats } from "$lib/api";

  let { stats }: { stats: RecipeStats | null } = $props();

  function fmt(val: number | undefined, decimals = 3): string {
    if (val === undefined || val === null) return "—";
    return val.toFixed(decimals);
  }

  function srmToHex(srm: number): string {
    // Approximate SRM to hex color
    const clamp = Math.min(Math.max(srm, 1), 40);
    const stops: [number, string][] = [
      [1, "#FFE699"], [2, "#FFD878"], [3, "#FFCA5A"], [4, "#FFBF42"],
      [6, "#FBB123"], [8, "#F8A600"], [10, "#F39C00"], [13, "#EA8F00"],
      [17, "#D77200"], [20, "#CF6900"], [24, "#BB5100"], [29, "#A13600"],
      [35, "#8D1D00"], [40, "#611200"],
    ];
    for (let i = stops.length - 1; i >= 0; i--) {
      if (clamp >= stops[i][0]) return stops[i][1];
    }
    return "#FFE699";
  }
</script>

<aside class="w-40 flex-shrink-0 flex flex-col border-l p-3 gap-3"
       style="background: var(--color-bg-surface); border-color: var(--color-border);">
  <h3 class="text-xs font-semibold uppercase tracking-wider" style="color: var(--color-text-muted);">Stats</h3>

  {#if stats}
    <div class="flex flex-col gap-2">
      <StatRow label="OG" value={fmt(stats.og, 3)} />
      <StatRow label="FG" value={fmt(stats.fg, 3)} />
      <StatRow label="ABV" value="{fmt(stats.abv_pct, 1)}%" />
      <StatRow label="IBU" value={fmt(stats.ibu, 0)} />
      <div class="flex items-center justify-between">
        <span class="text-xs" style="color: var(--color-text-secondary);">SRM</span>
        <div class="flex items-center gap-1.5">
          <div class="w-3 h-3 rounded-full border border-white/20"
               style="background: {srmToHex(stats.srm)};"></div>
          <span class="text-xs font-mono" style="color: var(--color-text-primary);">{fmt(stats.srm, 1)}</span>
        </div>
      </div>
      <StatRow label="BU:GU" value={fmt(stats.bu_gu_ratio, 2)} />
      <StatRow label="Cal" value="{fmt(stats.calories_per_355ml, 0)} kcal" />
    </div>

    <div class="border-t pt-2" style="border-color: var(--color-border);">
      <h4 class="text-xs font-semibold uppercase tracking-wider mb-2" style="color: var(--color-text-muted);">Volumes</h4>
      <StatRow label="Pre-boil" value="{fmt(stats.pre_boil_volume_l, 1)}L" />
      <StatRow label="Post-boil" value="{fmt(stats.post_boil_volume_l, 1)}L" />
      <StatRow label="Pre-boil G" value={fmt(stats.pre_boil_gravity, 3)} />
    </div>
  {:else}
    <p class="text-xs" style="color: var(--color-text-muted);">Add ingredients to see stats</p>
  {/if}
</aside>

{#snippet StatRow(label: string, value: string)}
  <div class="flex items-center justify-between">
    <span class="text-xs" style="color: var(--color-text-secondary);">{label}</span>
    <span class="text-xs font-mono" style="color: var(--color-text-primary);">{value}</span>
  </div>
{/snippet}
```

- [ ] **Step 2: Commit**

```bash
git add src/lib/components/StatsSidebar.svelte
git commit -m "feat: stats sidebar component with SRM color swatch"
```

---

### Task 5: Recipe editor shell

**Files:**
- Create: `src/routes/recipe/[id]/+page.svelte`
- Create: `src/routes/recipe/[id]/+page.ts`

- [ ] **Step 1: Create `src/routes/recipe/[id]/+page.ts`**

```typescript
import type { PageLoad } from "./$types";

export const load: PageLoad = ({ params }) => {
  return { id: params.id };
};
```

- [ ] **Step 2: Create `src/routes/recipe/[id]/+page.svelte`**

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import type { PageData } from "./$types";
  import { getRecipe, getRecipeStats, updateRecipe } from "$lib/api";
  import type { Recipe, RecipeStats } from "$lib/api";
  import RecipeList from "$lib/components/RecipeList.svelte";
  import StatsSidebar from "$lib/components/StatsSidebar.svelte";
  import OverviewTab from "$lib/components/tabs/OverviewTab.svelte";
  import IngredientsTab from "$lib/components/tabs/IngredientsTab.svelte";
  import MashTab from "$lib/components/tabs/MashTab.svelte";
  import FermentationTab from "$lib/components/tabs/FermentationTab.svelte";
  import NotesTab from "$lib/components/tabs/NotesTab.svelte";

  let { data }: { data: PageData } = $props();

  let recipe = $state<Recipe | null>(null);
  let stats = $state<RecipeStats | null>(null);
  let activeTab = $state<"overview" | "ingredients" | "mash" | "fermentation" | "notes">("overview");
  let saving = $state(false);

  const TABS = [
    { key: "overview", label: "Overview" },
    { key: "ingredients", label: "Ingredients" },
    { key: "mash", label: "Mash" },
    { key: "fermentation", label: "Fermentation" },
    { key: "notes", label: "Notes" },
  ] as const;

  onMount(async () => {
    recipe = await getRecipe(data.id);
    await refreshStats();
  });

  async function refreshStats() {
    if (!recipe) return;
    stats = await getRecipeStats(recipe.id);
  }

  async function refreshRecipe() {
    recipe = await getRecipe(data.id);
    await refreshStats();
  }

  async function handleNameBlur(e: FocusEvent) {
    const target = e.currentTarget as HTMLInputElement;
    if (!recipe || target.value === recipe.name) return;
    saving = true;
    recipe = await updateRecipe(recipe.id, { name: target.value } as any);
    saving = false;
  }
</script>

<RecipeList selectedId={data.id} />

{#if recipe}
  <div class="flex flex-1 flex-col overflow-hidden">
    <!-- Header -->
    <header class="flex items-center px-4 py-2 border-b gap-3 flex-shrink-0"
            style="background: var(--color-bg-surface); border-color: var(--color-border);">
      <button onclick={() => goto("/")} class="text-xs px-2 py-1 rounded"
              style="color: var(--color-text-secondary); background: var(--color-bg-elevated);">
        ← Recipes
      </button>
      <input
        value={recipe.name}
        onblur={handleNameBlur}
        class="flex-1 text-base font-semibold bg-transparent outline-none"
        style="color: var(--color-text-primary);"
      />
      {#if saving}
        <span class="text-xs" style="color: var(--color-text-muted);">Saving…</span>
      {/if}
    </header>

    <!-- Tab bar -->
    <nav class="flex border-b flex-shrink-0"
         style="background: var(--color-bg-surface); border-color: var(--color-border);">
      {#each TABS as tab}
        <button
          onclick={() => activeTab = tab.key}
          class="px-4 py-2 text-sm border-b-2 transition-colors"
          style={activeTab === tab.key
            ? "border-color: var(--color-accent); color: var(--color-text-primary);"
            : "border-color: transparent; color: var(--color-text-secondary);"}
        >
          {tab.label}
        </button>
      {/each}
    </nav>

    <!-- Tab content + stats sidebar -->
    <div class="flex flex-1 overflow-hidden">
      <div class="flex-1 overflow-y-auto p-4">
        {#if activeTab === "overview"}
          <OverviewTab {recipe} onchange={refreshRecipe} />
        {:else if activeTab === "ingredients"}
          <IngredientsTab {recipe} onchange={refreshRecipe} />
        {:else if activeTab === "mash"}
          <MashTab {recipe} onchange={refreshRecipe} />
        {:else if activeTab === "fermentation"}
          <FermentationTab {recipe} onchange={refreshRecipe} />
        {:else if activeTab === "notes"}
          <NotesTab {recipe} onchange={refreshRecipe} />
        {/if}
      </div>
      <StatsSidebar {stats} />
    </div>
  </div>
{:else}
  <div class="flex-1 flex items-center justify-center">
    <p class="text-sm" style="color: var(--color-text-muted);">Loading…</p>
  </div>
{/if}
```

- [ ] **Step 3: Create stub tab components** (populated in later tasks)

Create these files with minimal content:

`src/lib/components/tabs/OverviewTab.svelte`:
```svelte
<script lang="ts">
  import type { Recipe } from "$lib/api";
  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();
</script>
<p style="color: var(--color-text-muted)">Overview — coming soon</p>
```

`src/lib/components/tabs/IngredientsTab.svelte`:
```svelte
<script lang="ts">
  import type { Recipe } from "$lib/api";
  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();
</script>
<p style="color: var(--color-text-muted)">Ingredients — coming soon</p>
```

`src/lib/components/tabs/MashTab.svelte`:
```svelte
<script lang="ts">
  import type { Recipe } from "$lib/api";
  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();
</script>
<p style="color: var(--color-text-muted)">Mash — coming soon</p>
```

`src/lib/components/tabs/FermentationTab.svelte`:
```svelte
<script lang="ts">
  import type { Recipe } from "$lib/api";
  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();
</script>
<p style="color: var(--color-text-muted)">Fermentation — coming soon</p>
```

`src/lib/components/tabs/NotesTab.svelte`:
```svelte
<script lang="ts">
  import type { Recipe } from "$lib/api";
  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();
</script>
<p style="color: var(--color-text-muted)">Notes — coming soon</p>
```

- [ ] **Step 4: Test in browser**

```bash
npm run tauri dev
```

Click "+ New Recipe" → should navigate to editor with tab bar and stats sidebar. Editing the recipe name and tabbing out should save it (check the recipe list panel updates).

- [ ] **Step 5: Commit**

```bash
git add src/routes/recipe/ src/lib/components/tabs/
git commit -m "feat: recipe editor shell with tabs and stats sidebar"
```

---

### Task 6: Overview tab

**Files:**
- Modify: `src/lib/components/tabs/OverviewTab.svelte`

- [ ] **Step 1: Implement Overview tab**

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import type { Recipe, Style, EquipmentProfile } from "$lib/api";
  import { updateRecipe, listStyles, listEquipmentProfiles } from "$lib/api";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  let styles = $state<Style[]>([]);
  let equipmentProfiles = $state<EquipmentProfile[]>([]);

  onMount(async () => {
    [styles, equipmentProfiles] = await Promise.all([listStyles(), listEquipmentProfiles()]);
  });

  async function save(field: string, value: unknown) {
    await updateRecipe(recipe.id, { [field]: value } as any);
    onchange();
  }

  const RECIPE_TYPES = ["all_grain", "extract", "partial_mash"] as const;
</script>

<div class="grid grid-cols-2 gap-4 max-w-2xl">
  <Field label="Recipe Type">
    <select value={recipe.type_} onchange={(e) => save("type_", (e.target as HTMLSelectElement).value)}
            class="w-full px-2 py-1.5 rounded text-sm"
            style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);">
      {#each RECIPE_TYPES as t}
        <option value={t}>{t.replace("_", " ")}</option>
      {/each}
    </select>
  </Field>

  <Field label="Brewer">
    <input type="text" value={recipe.brewer ?? ""}
           onblur={(e) => save("brewer", (e.target as HTMLInputElement).value)}
           class="w-full px-2 py-1.5 rounded text-sm"
           style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
  </Field>

  <Field label="Batch Size (L)">
    <input type="number" step="0.1" value={recipe.batch_size_l}
           onblur={(e) => save("batch_size_l", parseFloat((e.target as HTMLInputElement).value))}
           class="w-full px-2 py-1.5 rounded text-sm"
           style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
  </Field>

  <Field label="Boil Size (L)">
    <input type="number" step="0.1" value={recipe.boil_size_l}
           onblur={(e) => save("boil_size_l", parseFloat((e.target as HTMLInputElement).value))}
           class="w-full px-2 py-1.5 rounded text-sm"
           style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
  </Field>

  <Field label="Boil Time (min)">
    <input type="number" step="5" value={recipe.boil_time_min}
           onblur={(e) => save("boil_time_min", parseFloat((e.target as HTMLInputElement).value))}
           class="w-full px-2 py-1.5 rounded text-sm"
           style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
  </Field>

  <Field label="Efficiency (%)">
    <input type="number" step="1" value={recipe.efficiency_pct ?? ""}
           placeholder="From equipment profile"
           onblur={(e) => {
             const v = (e.target as HTMLInputElement).value;
             save("efficiency_pct", v ? parseFloat(v) : null);
           }}
           class="w-full px-2 py-1.5 rounded text-sm"
           style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
  </Field>

  <Field label="Equipment Profile">
    <select value={recipe.equipment_profile_id ?? ""}
            onchange={(e) => save("equipment_profile_id", (e.target as HTMLSelectElement).value || null)}
            class="w-full px-2 py-1.5 rounded text-sm"
            style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);">
      <option value="">None</option>
      {#each equipmentProfiles as ep}
        <option value={ep.id}>{ep.name}</option>
      {/each}
    </select>
  </Field>

  <Field label="Style">
    <select value={recipe.style_id ?? ""}
            onchange={(e) => save("style_id", (e.target as HTMLSelectElement).value || null)}
            class="w-full px-2 py-1.5 rounded text-sm"
            style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);">
      <option value="">None</option>
      {#each styles as s}
        <option value={s.id}>{s.name}</option>
      {/each}
    </select>
  </Field>

  <Field label="Date">
    <input type="date" value={recipe.date ?? ""}
           onblur={(e) => save("date", (e.target as HTMLInputElement).value || null)}
           class="w-full px-2 py-1.5 rounded text-sm"
           style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
  </Field>
</div>

{#snippet Field(label: string, children: any)}
  <div class="flex flex-col gap-1">
    <label class="text-xs font-medium" style="color: var(--color-text-secondary);">{label}</label>
    {@render children()}
  </div>
{/snippet}
```

- [ ] **Step 2: Test in browser**

Open a recipe, click Overview tab. Verify all fields render, changing batch size updates the stats sidebar values (via `onchange` → `getRecipeStats`).

- [ ] **Step 3: Commit**

```bash
git add src/lib/components/tabs/OverviewTab.svelte
git commit -m "feat: Overview tab with all recipe fields"
```

---

### Task 7: Ingredients tab

**Files:**
- Modify: `src/lib/components/tabs/IngredientsTab.svelte`
- Create: `src/lib/components/ingredients/FermentablesTable.svelte`
- Create: `src/lib/components/ingredients/HopsTable.svelte`
- Create: `src/lib/components/ingredients/YeastsTable.svelte`

- [ ] **Step 1: Implement `FermentablesTable.svelte`**

```svelte
<script lang="ts">
  import type { Recipe, RecipeAdditionFermentable, Fermentable } from "$lib/api";
  import { listFermentableLibrary, createRecipeFermentable, updateRecipeFermentable, deleteRecipeFermentable } from "$lib/api";
  import { onMount } from "svelte";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  let library = $state<Fermentable[]>([]);
  let adding = $state(false);
  let selectedLibId = $state("");
  let amount = $state(1.0);

  onMount(async () => { library = await listFermentableLibrary(); });

  const selectedLib = $derived(library.find((f) => f.id === selectedLibId));

  async function handleAdd() {
    if (!selectedLib) return;
    await createRecipeFermentable(recipe.id, {
      fermentable_id: selectedLib.id,
      name: selectedLib.name,
      type_: selectedLib.type_,
      yield_pct: selectedLib.yield_pct,
      color_lovibond: selectedLib.color_lovibond,
      amount_kg: amount,
    });
    adding = false;
    selectedLibId = "";
    amount = 1.0;
    onchange();
  }

  async function handleAmountChange(f: RecipeAdditionFermentable, value: string) {
    const kg = parseFloat(value);
    if (!isNaN(kg) && kg > 0) {
      await updateRecipeFermentable(f.id, { amount_kg: kg });
      onchange();
    }
  }

  async function handleDelete(id: string) {
    await deleteRecipeFermentable(id);
    onchange();
  }
</script>

<div class="flex flex-col gap-2">
  <div class="flex items-center justify-between">
    <h3 class="text-sm font-semibold" style="color: var(--color-text-primary);">Fermentables</h3>
    <button onclick={() => adding = !adding} class="text-xs px-2 py-1 rounded"
            style="background: var(--color-accent); color: #fff;">+ Add</button>
  </div>

  {#if adding}
    <div class="flex gap-2 items-end p-2 rounded" style="background: var(--color-bg-elevated);">
      <div class="flex-1">
        <label class="text-xs mb-1 block" style="color: var(--color-text-secondary);">Fermentable</label>
        <select bind:value={selectedLibId} class="w-full px-2 py-1.5 rounded text-sm"
                style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);">
          <option value="">Choose…</option>
          {#each library as f}
            <option value={f.id}>{f.name}</option>
          {/each}
        </select>
      </div>
      <div class="w-24">
        <label class="text-xs mb-1 block" style="color: var(--color-text-secondary);">Amount (kg)</label>
        <input type="number" step="0.1" bind:value={amount} min="0.01"
               class="w-full px-2 py-1.5 rounded text-sm"
               style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </div>
      <button onclick={handleAdd} class="text-xs px-3 py-1.5 rounded"
              style="background: var(--color-accent); color: #fff;">Add</button>
      <button onclick={() => adding = false} class="text-xs px-2 py-1.5 rounded"
              style="color: var(--color-text-secondary);">Cancel</button>
    </div>
  {/if}

  {#if recipe.fermentables.length > 0}
    <table class="w-full text-sm">
      <thead>
        <tr style="color: var(--color-text-muted);">
          <th class="text-left py-1 font-medium text-xs">Name</th>
          <th class="text-right py-1 font-medium text-xs">Lovibond</th>
          <th class="text-right py-1 font-medium text-xs">kg</th>
          <th class="w-6"></th>
        </tr>
      </thead>
      <tbody>
        {#each recipe.fermentables as f (f.id)}
          <tr class="border-t" style="border-color: var(--color-border);">
            <td class="py-1.5" style="color: var(--color-text-primary);">{f.name}</td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">{f.color_lovibond}°L</td>
            <td class="text-right py-1.5">
              <input type="number" step="0.05" value={f.amount_kg}
                     onblur={(e) => handleAmountChange(f, (e.target as HTMLInputElement).value)}
                     class="w-16 text-right px-1 rounded text-xs"
                     style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid transparent;" />
            </td>
            <td class="pl-1">
              <button onclick={() => handleDelete(f.id)} class="text-xs opacity-40 hover:opacity-100"
                      style="color: var(--color-text-secondary);">×</button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>
```

- [ ] **Step 2: Implement `HopsTable.svelte`**

```svelte
<script lang="ts">
  import type { Recipe, RecipeAdditionHop, Hop } from "$lib/api";
  import { listHopLibrary, createRecipeHop, updateRecipeHop, deleteRecipeHop } from "$lib/api";
  import { onMount } from "svelte";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  let library = $state<Hop[]>([]);
  let adding = $state(false);
  let selectedLibId = $state("");
  let amount = $state(0.028);
  let use_ = $state("boil");
  let time = $state(60);

  onMount(async () => { library = await listHopLibrary(); });

  const selectedLib = $derived(library.find((h) => h.id === selectedLibId));

  async function handleAdd() {
    if (!selectedLib) return;
    await createRecipeHop(recipe.id, {
      hop_id: selectedLib.id,
      name: selectedLib.name,
      alpha_pct: selectedLib.alpha_pct,
      form: selectedLib.form,
      amount_kg: amount,
      use_,
      time_min: time,
    });
    adding = false;
    selectedLibId = "";
    onchange();
  }

  async function handleDelete(id: string) {
    await deleteRecipeHop(id);
    onchange();
  }

  const HOP_USES = ["boil", "aroma", "dry hop", "first wort", "whirlpool"] as const;
</script>

<div class="flex flex-col gap-2">
  <div class="flex items-center justify-between">
    <h3 class="text-sm font-semibold" style="color: var(--color-text-primary);">Hops</h3>
    <button onclick={() => adding = !adding} class="text-xs px-2 py-1 rounded"
            style="background: var(--color-accent); color: #fff;">+ Add</button>
  </div>

  {#if adding}
    <div class="flex flex-wrap gap-2 items-end p-2 rounded" style="background: var(--color-bg-elevated);">
      <div class="flex-1 min-w-32">
        <label class="text-xs mb-1 block" style="color: var(--color-text-secondary);">Hop</label>
        <select bind:value={selectedLibId} class="w-full px-2 py-1.5 rounded text-sm"
                style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);">
          <option value="">Choose…</option>
          {#each library as h}
            <option value={h.id}>{h.name} ({h.alpha_pct}% AA)</option>
          {/each}
        </select>
      </div>
      <div class="w-20">
        <label class="text-xs mb-1 block" style="color: var(--color-text-secondary);">Amount (kg)</label>
        <input type="number" step="0.001" bind:value={amount} min="0.001"
               class="w-full px-2 py-1.5 rounded text-sm"
               style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </div>
      <div class="w-28">
        <label class="text-xs mb-1 block" style="color: var(--color-text-secondary);">Use</label>
        <select bind:value={use_} class="w-full px-2 py-1.5 rounded text-sm"
                style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);">
          {#each HOP_USES as u}
            <option value={u}>{u}</option>
          {/each}
        </select>
      </div>
      <div class="w-16">
        <label class="text-xs mb-1 block" style="color: var(--color-text-secondary);">Time (min)</label>
        <input type="number" step="5" bind:value={time} min="0"
               class="w-full px-2 py-1.5 rounded text-sm"
               style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </div>
      <button onclick={handleAdd} class="text-xs px-3 py-1.5 rounded self-end"
              style="background: var(--color-accent); color: #fff;">Add</button>
      <button onclick={() => adding = false} class="text-xs px-2 py-1.5 rounded self-end"
              style="color: var(--color-text-secondary);">Cancel</button>
    </div>
  {/if}

  {#if recipe.hops.length > 0}
    <table class="w-full text-sm">
      <thead>
        <tr style="color: var(--color-text-muted);">
          <th class="text-left py-1 font-medium text-xs">Name</th>
          <th class="text-right py-1 font-medium text-xs">AA%</th>
          <th class="text-right py-1 font-medium text-xs">g</th>
          <th class="text-right py-1 font-medium text-xs">Use</th>
          <th class="text-right py-1 font-medium text-xs">Time</th>
          <th class="w-6"></th>
        </tr>
      </thead>
      <tbody>
        {#each recipe.hops as h (h.id)}
          <tr class="border-t" style="border-color: var(--color-border);">
            <td class="py-1.5" style="color: var(--color-text-primary);">{h.name}</td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">{h.alpha_pct}%</td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">{(h.amount_kg * 1000).toFixed(0)}g</td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">{h.use_}</td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">{h.time_min}min</td>
            <td class="pl-1">
              <button onclick={() => handleDelete(h.id)} class="text-xs opacity-40 hover:opacity-100"
                      style="color: var(--color-text-secondary);">×</button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>
```

- [ ] **Step 3: Implement `YeastsTable.svelte`**

```svelte
<script lang="ts">
  import type { Recipe, RecipeAdditionYeast, Yeast } from "$lib/api";
  import { listYeastLibrary, createRecipeYeast, deleteRecipeYeast } from "$lib/api";
  import { onMount } from "svelte";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  let library = $state<Yeast[]>([]);
  let adding = $state(false);
  let selectedLibId = $state("");

  onMount(async () => { library = await listYeastLibrary(); });

  const selectedLib = $derived(library.find((y) => y.id === selectedLibId));

  async function handleAdd() {
    if (!selectedLib) return;
    await createRecipeYeast(recipe.id, {
      yeast_id: selectedLib.id,
      name: selectedLib.name,
      type_: selectedLib.type_,
      form: selectedLib.form,
      laboratory: selectedLib.laboratory,
      product_id: selectedLib.product_id,
      attenuation_pct: selectedLib.attenuation_pct,
      amount: 1,
    });
    adding = false;
    selectedLibId = "";
    onchange();
  }

  async function handleDelete(id: string) {
    await deleteRecipeYeast(id);
    onchange();
  }
</script>

<div class="flex flex-col gap-2">
  <div class="flex items-center justify-between">
    <h3 class="text-sm font-semibold" style="color: var(--color-text-primary);">Yeast</h3>
    <button onclick={() => adding = !adding} class="text-xs px-2 py-1 rounded"
            style="background: var(--color-accent); color: #fff;">+ Add</button>
  </div>

  {#if adding}
    <div class="flex gap-2 items-end p-2 rounded" style="background: var(--color-bg-elevated);">
      <div class="flex-1">
        <label class="text-xs mb-1 block" style="color: var(--color-text-secondary);">Yeast</label>
        <select bind:value={selectedLibId} class="w-full px-2 py-1.5 rounded text-sm"
                style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);">
          <option value="">Choose…</option>
          {#each library as y}
            <option value={y.id}>{y.name} ({y.laboratory ?? y.form})</option>
          {/each}
        </select>
      </div>
      <button onclick={handleAdd} class="text-xs px-3 py-1.5 rounded"
              style="background: var(--color-accent); color: #fff;">Add</button>
      <button onclick={() => adding = false} class="text-xs px-2 py-1.5 rounded"
              style="color: var(--color-text-secondary);">Cancel</button>
    </div>
  {/if}

  {#each recipe.yeasts as y (y.id)}
    <div class="flex items-center justify-between py-1.5 border-t" style="border-color: var(--color-border);">
      <div>
        <p class="text-sm" style="color: var(--color-text-primary);">{y.name}</p>
        <p class="text-xs" style="color: var(--color-text-secondary);">
          {y.laboratory ?? ""} {y.product_id ?? ""} · {y.attenuation_pct ?? "?"}% attenuation
        </p>
      </div>
      <button onclick={() => handleDelete(y.id)} class="text-xs opacity-40 hover:opacity-100"
              style="color: var(--color-text-secondary);">×</button>
    </div>
  {/each}
</div>
```

- [ ] **Step 4: Implement `IngredientsTab.svelte`**

```svelte
<script lang="ts">
  import type { Recipe } from "$lib/api";
  import FermentablesTable from "$lib/components/ingredients/FermentablesTable.svelte";
  import HopsTable from "$lib/components/ingredients/HopsTable.svelte";
  import YeastsTable from "$lib/components/ingredients/YeastsTable.svelte";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();
</script>

<div class="flex flex-col gap-8">
  <FermentablesTable {recipe} {onchange} />
  <HopsTable {recipe} {onchange} />
  <YeastsTable {recipe} {onchange} />
</div>
```

- [ ] **Step 5: Test in browser**

Open a recipe → Ingredients tab. Add a fermentable — stats sidebar should update immediately. Add a hop — IBU should update. Add a yeast — FG/ABV should update (uses attenuation_pct).

- [ ] **Step 6: Commit**

```bash
git add src/lib/components/tabs/IngredientsTab.svelte src/lib/components/ingredients/
git commit -m "feat: Ingredients tab with fermentables, hops, yeast tables"
```

---

### Task 8: Mash tab

**Files:**
- Modify: `src/lib/components/tabs/MashTab.svelte`

- [ ] **Step 1: Implement `MashTab.svelte`**

```svelte
<script lang="ts">
  import type { Recipe, MashStep } from "$lib/api";
  import { updateMash, createMashStep, updateMashStep, deleteMashStep, updateMashStepOrder } from "$lib/api";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  let addingStep = $state(false);
  let stepName = $state("Mash In");
  let stepType = $state("infusion");
  let stepTemp = $state(67.0);
  let stepTime = $state(60);
  let stepInfuse = $state<number | null>(null);

  const mash = $derived(recipe.mash);

  async function ensureMash() {
    if (!mash) {
      await updateMash(recipe.id, { name: "Single Infusion", grain_temp_c: 21 });
      onchange();
    }
  }

  async function handleAddStep() {
    await ensureMash();
    const currentMash = recipe.mash!;
    await createMashStep(currentMash.id, {
      name: stepName,
      type_: stepType,
      step_temp_c: stepTemp,
      step_time_min: stepTime,
      infuse_amount_l: stepInfuse,
    });
    addingStep = false;
    onchange();
  }

  async function handleDeleteStep(id: string) {
    await deleteMashStep(id);
    onchange();
  }

  async function handleMashField(field: string, value: unknown) {
    await ensureMash();
    await updateMash(recipe.id, { [field]: value });
    onchange();
  }

  const STEP_TYPES = ["infusion", "temperature", "decoction"] as const;
</script>

<div class="flex flex-col gap-4 max-w-xl">
  <!-- Mash profile settings -->
  <div class="grid grid-cols-2 gap-3">
    <div class="flex flex-col gap-1">
      <label class="text-xs font-medium" style="color: var(--color-text-secondary);">Profile Name</label>
      <input type="text" value={mash?.name ?? "Single Infusion"}
             onblur={(e) => handleMashField("name", (e.target as HTMLInputElement).value)}
             class="px-2 py-1.5 rounded text-sm"
             style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
    </div>
    <div class="flex flex-col gap-1">
      <label class="text-xs font-medium" style="color: var(--color-text-secondary);">Grain Temp (°C)</label>
      <input type="number" step="1" value={mash?.grain_temp_c ?? 21}
             onblur={(e) => handleMashField("grain_temp_c", parseFloat((e.target as HTMLInputElement).value))}
             class="px-2 py-1.5 rounded text-sm"
             style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
    </div>
    <div class="flex flex-col gap-1">
      <label class="text-xs font-medium" style="color: var(--color-text-secondary);">Sparge Temp (°C)</label>
      <input type="number" step="1" value={mash?.sparge_temp_c ?? ""}
             placeholder="75"
             onblur={(e) => {
               const v = (e.target as HTMLInputElement).value;
               handleMashField("sparge_temp_c", v ? parseFloat(v) : null);
             }}
             class="px-2 py-1.5 rounded text-sm"
             style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
    </div>
    <div class="flex flex-col gap-1">
      <label class="text-xs font-medium" style="color: var(--color-text-secondary);">Mash pH</label>
      <input type="number" step="0.1" value={mash?.ph ?? ""}
             placeholder="5.4"
             onblur={(e) => {
               const v = (e.target as HTMLInputElement).value;
               handleMashField("ph", v ? parseFloat(v) : null);
             }}
             class="px-2 py-1.5 rounded text-sm"
             style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
    </div>
  </div>

  <!-- Mash steps -->
  <div class="flex flex-col gap-2">
    <div class="flex items-center justify-between">
      <h3 class="text-sm font-semibold" style="color: var(--color-text-primary);">Steps</h3>
      <button onclick={() => addingStep = !addingStep} class="text-xs px-2 py-1 rounded"
              style="background: var(--color-accent); color: #fff;">+ Add Step</button>
    </div>

    {#if addingStep}
      <div class="flex flex-wrap gap-2 p-2 rounded" style="background: var(--color-bg-elevated);">
        <input type="text" bind:value={stepName} placeholder="Step name"
               class="flex-1 min-w-24 px-2 py-1.5 rounded text-sm"
               style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
        <select bind:value={stepType} class="w-28 px-2 py-1.5 rounded text-sm"
                style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);">
          {#each STEP_TYPES as t}
            <option value={t}>{t}</option>
          {/each}
        </select>
        <input type="number" bind:value={stepTemp} step="0.5" placeholder="Temp °C"
               class="w-20 px-2 py-1.5 rounded text-sm"
               style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
        <input type="number" bind:value={stepTime} step="5" placeholder="Time min"
               class="w-20 px-2 py-1.5 rounded text-sm"
               style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
        <button onclick={handleAddStep} class="text-xs px-3 py-1.5 rounded"
                style="background: var(--color-accent); color: #fff;">Add</button>
      </div>
    {/if}

    {#if mash && mash.steps.length > 0}
      <div class="flex flex-col gap-1">
        {#each mash.steps as step (step.id)}
          <div class="flex items-center gap-3 py-2 border-t" style="border-color: var(--color-border);">
            <div class="flex-1">
              <p class="text-sm" style="color: var(--color-text-primary);">{step.name}</p>
              <p class="text-xs" style="color: var(--color-text-secondary);">
                {step.step_temp_c}°C · {step.step_time_min} min · {step.type_}
                {#if step.infuse_amount_l} · {step.infuse_amount_l}L{/if}
              </p>
            </div>
            <button onclick={() => handleDeleteStep(step.id)} class="text-xs opacity-40 hover:opacity-100"
                    style="color: var(--color-text-secondary);">×</button>
          </div>
        {/each}
      </div>
    {:else}
      <p class="text-xs py-2" style="color: var(--color-text-muted);">No mash steps yet</p>
    {/if}
  </div>
</div>
```

- [ ] **Step 2: Test in browser**

Open Mash tab. Add a mash step — it should appear in the list with temp and time. Editing grain temp should persist on blur.

- [ ] **Step 3: Commit**

```bash
git add src/lib/components/tabs/MashTab.svelte
git commit -m "feat: Mash tab with profile settings and step list"
```

---

### Task 9: Fermentation and Notes tabs

**Files:**
- Modify: `src/lib/components/tabs/FermentationTab.svelte`
- Modify: `src/lib/components/tabs/NotesTab.svelte`

- [ ] **Step 1: Implement `FermentationTab.svelte`**

```svelte
<script lang="ts">
  import type { Recipe } from "$lib/api";
  import { updateRecipe } from "$lib/api";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  async function save(field: string, value: unknown) {
    await updateRecipe(recipe.id, { [field]: value } as any);
    onchange();
  }
</script>

<div class="grid grid-cols-2 gap-4 max-w-xl">
  <div class="col-span-2">
    <h3 class="text-sm font-semibold mb-3" style="color: var(--color-text-primary);">Fermentation Schedule</h3>
  </div>

  {#each [
    { field: "primary_age_days", label: "Primary (days)" },
    { field: "primary_temp_c", label: "Primary Temp (°C)" },
    { field: "secondary_age_days", label: "Secondary (days)" },
    { field: "secondary_temp_c", label: "Secondary Temp (°C)" },
    { field: "tertiary_age_days", label: "Tertiary (days)" },
    { field: "tertiary_temp_c", label: "Tertiary Temp (°C)" },
  ] as row}
    <div class="flex flex-col gap-1">
      <label class="text-xs font-medium" style="color: var(--color-text-secondary);">{row.label}</label>
      <input type="number" step="1"
             value={(recipe as any)[row.field] ?? ""}
             onblur={(e) => {
               const v = (e.target as HTMLInputElement).value;
               save(row.field, v ? parseFloat(v) : null);
             }}
             class="px-2 py-1.5 rounded text-sm"
             style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
    </div>
  {/each}

  <div class="col-span-2 border-t pt-4" style="border-color: var(--color-border);">
    <h3 class="text-sm font-semibold mb-3" style="color: var(--color-text-primary);">Carbonation</h3>
  </div>

  <div class="flex flex-col gap-1">
    <label class="text-xs font-medium" style="color: var(--color-text-secondary);">CO₂ Volumes</label>
    <input type="number" step="0.1" value={recipe.carbonation_vols ?? ""}
           onblur={(e) => {
             const v = (e.target as HTMLInputElement).value;
             save("carbonation_vols", v ? parseFloat(v) : null);
           }}
           class="px-2 py-1.5 rounded text-sm"
           style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
  </div>

  <div class="flex flex-col gap-1">
    <label class="text-xs font-medium" style="color: var(--color-text-secondary);">Forced Carbonation</label>
    <label class="flex items-center gap-2 mt-1 cursor-pointer">
      <input type="checkbox" checked={recipe.forced_carbonation}
             onchange={(e) => save("forced_carbonation", (e.target as HTMLInputElement).checked)}
             class="rounded" />
      <span class="text-sm" style="color: var(--color-text-secondary);">Yes (kegged)</span>
    </label>
  </div>
</div>
```

- [ ] **Step 2: Implement `NotesTab.svelte`**

```svelte
<script lang="ts">
  import type { Recipe } from "$lib/api";
  import { updateRecipe } from "$lib/api";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  async function save(field: string, value: unknown) {
    await updateRecipe(recipe.id, { [field]: value } as any);
    onchange();
  }
</script>

<div class="flex flex-col gap-4 max-w-2xl">
  <div class="flex flex-col gap-1">
    <label class="text-xs font-medium" style="color: var(--color-text-secondary);">Recipe Notes</label>
    <textarea value={recipe.notes ?? ""}
              onblur={(e) => save("notes", (e.target as HTMLTextAreaElement).value || null)}
              rows="8"
              placeholder="Process notes, observations…"
              class="px-3 py-2 rounded text-sm resize-none outline-none"
              style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);"></textarea>
  </div>

  <div class="flex flex-col gap-1">
    <label class="text-xs font-medium" style="color: var(--color-text-secondary);">Taste Notes</label>
    <textarea value={recipe.taste_notes ?? ""}
              onblur={(e) => save("taste_notes", (e.target as HTMLTextAreaElement).value || null)}
              rows="4"
              placeholder="Aroma, flavor, appearance, mouthfeel…"
              class="px-3 py-2 rounded text-sm resize-none outline-none"
              style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);"></textarea>
  </div>

  <div class="flex flex-col gap-1">
    <label class="text-xs font-medium" style="color: var(--color-text-secondary);">Taste Rating (0–50)</label>
    <input type="number" step="1" min="0" max="50"
           value={recipe.taste_rating ?? ""}
           onblur={(e) => {
             const v = (e.target as HTMLInputElement).value;
             save("taste_rating", v ? parseFloat(v) : null);
           }}
           class="w-24 px-2 py-1.5 rounded text-sm"
           style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
  </div>
</div>
```

- [ ] **Step 3: Test fermentation and notes tabs in browser**

Open both tabs, verify field saves persist (navigate away and back; values should still be set).

- [ ] **Step 4: Commit**

```bash
git add src/lib/components/tabs/FermentationTab.svelte src/lib/components/tabs/NotesTab.svelte
git commit -m "feat: Fermentation and Notes tabs"
```

---

### Task 10: Settings page

**Files:**
- Create: `src/routes/settings/+page.svelte`

- [ ] **Step 1: Create `src/routes/settings/+page.svelte`**

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { settings, loadSettings, saveSetting } from "$lib/stores/settings";
  import { listEquipmentProfiles, createEquipmentProfile, deleteEquipmentProfile } from "$lib/api";
  import type { EquipmentProfile } from "$lib/api";

  let profiles = $state<EquipmentProfile[]>([]);
  let newProfileName = $state("");

  onMount(async () => {
    await loadSettings();
    profiles = await listEquipmentProfiles();
  });

  async function handleThemeChange(e: Event) {
    await saveSetting("theme", (e.target as HTMLSelectElement).value);
  }

  async function handleUnitsChange(e: Event) {
    await saveSetting("units", (e.target as HTMLSelectElement).value);
  }

  async function handleDefaultEquipChange(e: Event) {
    await saveSetting("default_equipment_profile_id", (e.target as HTMLSelectElement).value);
  }

  async function handleAddProfile() {
    if (!newProfileName.trim()) return;
    await createEquipmentProfile({
      name: newProfileName,
      boil_size_l: 27.0,
      batch_size_l: 23.0,
      efficiency_pct: 72.0,
    });
    profiles = await listEquipmentProfiles();
    newProfileName = "";
  }

  async function handleDeleteProfile(id: string) {
    if (!confirm("Delete this equipment profile?")) return;
    await deleteEquipmentProfile(id);
    profiles = await listEquipmentProfiles();
  }
</script>

<div class="flex-1 overflow-y-auto p-6" style="background: var(--color-bg-base);">
  <h1 class="text-lg font-semibold mb-6" style="color: var(--color-text-primary);">Settings</h1>

  <div class="flex flex-col gap-6 max-w-md">
    <!-- Appearance -->
    <section class="flex flex-col gap-3">
      <h2 class="text-sm font-semibold" style="color: var(--color-text-secondary);">Appearance</h2>
      <div class="flex items-center justify-between">
        <label class="text-sm" style="color: var(--color-text-primary);">Theme</label>
        <select value={$settings.theme ?? "midnight"} onchange={handleThemeChange}
                class="px-2 py-1.5 rounded text-sm"
                style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);">
          <option value="midnight">Midnight</option>
        </select>
      </div>
    </section>

    <!-- Units -->
    <section class="flex flex-col gap-3">
      <h2 class="text-sm font-semibold" style="color: var(--color-text-secondary);">Units</h2>
      <div class="flex items-center justify-between">
        <label class="text-sm" style="color: var(--color-text-primary);">Measurement System</label>
        <select value={$settings.units ?? "metric"} onchange={handleUnitsChange}
                class="px-2 py-1.5 rounded text-sm"
                style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);">
          <option value="metric">Metric (L, kg, °C)</option>
          <option value="imperial">Imperial (gal, lb, °F)</option>
        </select>
      </div>
    </section>

    <!-- Equipment profiles -->
    <section class="flex flex-col gap-3">
      <h2 class="text-sm font-semibold" style="color: var(--color-text-secondary);">Equipment Profiles</h2>
      <div class="flex items-center justify-between">
        <label class="text-sm" style="color: var(--color-text-primary);">Default Profile</label>
        <select value={$settings.default_equipment_profile_id ?? ""}
                onchange={handleDefaultEquipChange}
                class="px-2 py-1.5 rounded text-sm"
                style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);">
          <option value="">None</option>
          {#each profiles as p}
            <option value={p.id}>{p.name}</option>
          {/each}
        </select>
      </div>

      {#each profiles as p (p.id)}
        <div class="flex items-center justify-between py-1 border-t" style="border-color: var(--color-border);">
          <div>
            <p class="text-sm" style="color: var(--color-text-primary);">{p.name}</p>
            <p class="text-xs" style="color: var(--color-text-secondary);">
              {p.batch_size_l}L batch · {p.efficiency_pct}% efficiency
            </p>
          </div>
          <button onclick={() => handleDeleteProfile(p.id)} class="text-xs px-2 py-1 rounded"
                  style="color: var(--color-text-secondary); background: var(--color-bg-elevated);">Delete</button>
        </div>
      {/each}

      <div class="flex gap-2 pt-1">
        <input type="text" bind:value={newProfileName} placeholder="New profile name"
               class="flex-1 px-2 py-1.5 rounded text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
        <button onclick={handleAddProfile} class="text-xs px-3 py-1.5 rounded"
                style="background: var(--color-accent); color: #fff;">Add</button>
      </div>
    </section>
  </div>
</div>
```

- [ ] **Step 2: Test settings page**

Click the gear icon in the rail. Verify theme and units dropdowns show current values. Add and delete an equipment profile.

- [ ] **Step 3: Commit**

```bash
git add src/routes/settings/
git commit -m "feat: settings page with theme, units, and equipment profiles"
```

---

### Task 11: End-to-end smoke test

- [ ] **Step 1: Create a complete recipe**

```bash
npm run tauri dev
```

Perform these actions in order, verifying each step:

1. Click "+ New Recipe" → editor opens with empty recipe
2. Rename it to "Pacific Haze IPA"
3. Overview tab: set batch size 23L, boil size 27L, select "American IPA" style
4. Ingredients tab → Add fermentables: 4.5kg Pale Malt, 0.5kg Crystal 40L
5. Stats sidebar shows OG ~1.052, SRM changes to amber
6. Add hops: 28g Citra (12% AA), Boil, 60 min
7. IBU updates to ~35+
8. Add yeast: US-05 (77% attenuation)
9. FG and ABV update
10. Mash tab: add "Mash In" step at 67°C, 60 min
11. Notes tab: add a note, verify it persists on refresh
12. Fermentation: set 14 days primary at 19°C
13. Navigate away and back → all values persist

- [ ] **Step 2: Test BeerXML export**

Open browser devtools console and run:
```javascript
window.__TAURI__.core.invoke("get_recipe_beerxml", { recipeId: "<your-recipe-id>" }).then(console.log)
```

Expected: well-formed BeerXML string printed to console.

- [ ] **Step 3: Commit**

```bash
git add -A
git commit -m "chore: v1 walking skeleton complete"
```

---

## UI complete

The v1 walking skeleton is done:
- Full dark UI with icon rail, recipe list, tabbed editor, live stats sidebar
- All five ingredient types with library picker
- Mash profile with step list
- Fermentation schedule and carbonation fields
- Notes tab with taste rating
- Settings page (theme, units, equipment profiles)
- BeerXML export (importable via devtools; import UI is a future enhancement)

Next milestones:
- Unit conversion display (imperial/metric toggle respects user preference)
- BeerXML import UI (file picker button)
- Drag-to-reorder mash steps and ingredient rows
- Custom ingredient creation
- Batches module
