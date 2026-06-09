<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { getRecipe, getRecipeStats, createRecipe } from "$lib/api";
  import type { Recipe, RecipeStats } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { settings } from "$lib/stores/settings";
  import { formatSg } from "$lib/gravity-display";
  import OverviewTab from "$lib/components/tabs/OverviewTab.svelte";
  import IngredientsTab from "$lib/components/tabs/IngredientsTab.svelte";
  import MashTab from "$lib/components/tabs/MashTab.svelte";
  import WaterTab from "$lib/components/tabs/WaterTab.svelte";
  import FermentationTab from "$lib/components/tabs/FermentationTab.svelte";
  import NotesTab from "$lib/components/tabs/NotesTab.svelte";

  let { id }: { id: string } = $props();

  let recipe = $state<Recipe | null>(null);
  let stats = $state<RecipeStats | null>(null);
  let cloning = $state(false);

  const gravityUnit = $derived($settings.gravity_unit ?? "sg");
  const displayOg = $derived(stats?.og != null ? formatSg(stats.og, gravityUnit) : "—");
  const displayFg = $derived(stats?.fg != null ? formatSg(stats.fg, gravityUnit) : "—");

  async function load() {
    recipe = await ipc(getRecipe(id)) ?? null;
    if (recipe) stats = await ipc(getRecipeStats(recipe.id)) ?? null;
  }

  onMount(load);
  $effect(() => { if (id) load(); });

  async function handleClone() {
    if (!recipe || cloning) return;
    cloning = true;
    const cloned = await ipc(createRecipe({
      name: recipe.name,
      source_id: recipe.id,
      type_: recipe.type_ ?? undefined,
      batch_size_l: recipe.batch_size_l ?? undefined,
      boil_size_l: recipe.boil_size_l ?? undefined,
      boil_time_min: recipe.boil_time_min ?? undefined,
    }));
    cloning = false;
    if (!cloned) return;
    goto(`/recipe/${cloned.id}`);
  }

  function fmt(val: number | undefined, decimals = 3): string {
    if (val === undefined || val === null) return "—";
    return val.toFixed(decimals);
  }
</script>

{#if recipe}
  <div class="flex flex-col h-full overflow-hidden" style="background: var(--color-bg-base);">
    <!-- Header with back button and clone action -->
    <div class="flex items-center gap-3 px-4 py-3 border-b flex-shrink-0"
         style="background: var(--color-bg-surface); border-color: var(--color-border);">
      <button
        onclick={() => goto("/")}
        class="text-sm flex-shrink-0"
        style="color: var(--color-accent);"
      >‹ Recipes</button>
      <div class="flex-1 min-w-0">
        <span class="font-semibold text-base truncate block"
              style="color: var(--color-text-primary);">{recipe.name}</span>
        <p class="text-xs" style="color: var(--color-text-muted);">Example Recipe — read only</p>
      </div>
      <button
        onclick={handleClone}
        disabled={cloning}
        class="px-3 py-2 rounded text-sm font-semibold transition-colors disabled:opacity-50 flex-shrink-0"
        style="background: var(--color-accent); color: #fff; min-height: 44px; display: flex; align-items: center;"
      >
        {cloning ? "Cloning…" : "Clone"}
      </button>
    </div>

    <!-- Single scroll -->
    <div class="flex-1 overflow-y-auto">
      <div class="p-4 flex flex-col gap-6">

        <!-- Stats card -->
        {#if stats}
          <div class="rounded-lg p-4 grid grid-cols-5 gap-2"
               style="background: var(--color-bg-elevated); border: 1px solid var(--color-border);">
            {#each [
              { label: "OG", value: displayOg },
              { label: "FG", value: displayFg },
              { label: "ABV", value: fmt(stats.abv_pct, 1) + "%" },
              { label: "IBU", value: fmt(stats.ibu, 0) },
              { label: "SRM", value: fmt(stats.srm, 1) },
            ] as stat}
              <div class="text-center">
                <div class="text-sm font-semibold font-mono" style="color: var(--color-accent);">{stat.value}</div>
                <div class="text-xs" style="color: var(--color-text-muted);">{stat.label}</div>
              </div>
            {/each}
          </div>
        {/if}

        <!-- Overview -->
        <section>
          <div class="text-xs font-semibold uppercase tracking-wider mb-3"
               style="color: var(--color-text-secondary);">Overview</div>
          <OverviewTab {recipe} onchange={() => {}} />
        </section>

        <!-- Ingredients -->
        <section>
          <div class="text-xs font-semibold uppercase tracking-wider mb-3"
               style="color: var(--color-text-secondary);">Ingredients</div>
          <IngredientsTab {recipe} {stats} onchange={() => {}} />
        </section>

        <!-- Mash -->
        <section>
          <div class="text-xs font-semibold uppercase tracking-wider mb-3"
               style="color: var(--color-text-secondary);">Mash</div>
          <MashTab {recipe} {stats} onchange={() => {}} />
        </section>

        <!-- Water -->
        <section>
          <div class="text-xs font-semibold uppercase tracking-wider mb-3"
               style="color: var(--color-text-secondary);">Water</div>
          <WaterTab {recipe} {stats} onchange={() => {}} />
        </section>

        <!-- Fermentation -->
        <section>
          <div class="text-xs font-semibold uppercase tracking-wider mb-3"
               style="color: var(--color-text-secondary);">Fermentation</div>
          <FermentationTab {recipe} onchange={() => {}} />
        </section>

        <!-- Notes -->
        <section>
          <div class="text-xs font-semibold uppercase tracking-wider mb-3"
               style="color: var(--color-text-secondary);">Notes</div>
          <NotesTab {recipe} onchange={() => {}} />
        </section>

      </div>
    </div>
  </div>
{:else}
  <div class="flex-1 flex items-center justify-center">
    <p class="text-sm" style="color: var(--color-text-muted);">Loading…</p>
  </div>
{/if}
