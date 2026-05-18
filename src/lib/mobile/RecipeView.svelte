<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { getRecipe, getRecipeStats } from "$lib/api";
  import type { Recipe, RecipeStats } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { settings } from "$lib/stores/settings";
  import OverviewTab from "$lib/components/tabs/OverviewTab.svelte";
  import IngredientsTab from "$lib/components/tabs/IngredientsTab.svelte";
  import MashTab from "$lib/components/tabs/MashTab.svelte";
  import WaterTab from "$lib/components/tabs/WaterTab.svelte";
  import FermentationTab from "$lib/components/tabs/FermentationTab.svelte";
  import NotesTab from "$lib/components/tabs/NotesTab.svelte";

  let { id }: { id: string } = $props();

  let recipe = $state<Recipe | null>(null);
  let stats = $state<RecipeStats | null>(null);

  async function load() {
    recipe = await ipc(getRecipe(id)) ?? null;
    if (recipe) stats = await ipc(getRecipeStats(recipe.id)) ?? null;
  }

  onMount(load);
  $effect(() => { if (id) load(); });

  function fmt(val: number | undefined, decimals = 3): string {
    if (val === undefined || val === null) return "—";
    return val.toFixed(decimals);
  }
</script>

{#if recipe}
  <div class="flex flex-col h-full overflow-hidden" style="background: var(--color-bg-base);">
    <!-- Header with back button -->
    <div class="flex items-center gap-3 px-4 py-3 border-b flex-shrink-0"
         style="background: var(--color-bg-surface); border-color: var(--color-border);">
      <button
        onclick={() => goto("/")}
        class="text-sm"
        style="color: var(--color-accent);"
      >‹ Recipes</button>
      <span class="flex-1 font-semibold text-base truncate"
            style="color: var(--color-text-primary);">{recipe.name}</span>
    </div>

    <!-- Single scroll -->
    <div class="flex-1 overflow-y-auto">
      <div class="p-4 flex flex-col gap-6">

        <!-- Stats card -->
        {#if stats}
          <div class="rounded-lg p-4 grid grid-cols-5 gap-2"
               style="background: var(--color-bg-elevated); border: 1px solid var(--color-border);">
            {#each [
              { label: "OG", value: fmt(stats.og, 3) },
              { label: "FG", value: fmt(stats.fg, 3) },
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
          <OverviewTab {recipe} onchange={load} />
        </section>

        <!-- Ingredients -->
        <section>
          <div class="text-xs font-semibold uppercase tracking-wider mb-3"
               style="color: var(--color-text-secondary);">Ingredients</div>
          <IngredientsTab {recipe} onchange={load} />
        </section>

        <!-- Mash -->
        <section>
          <div class="text-xs font-semibold uppercase tracking-wider mb-3"
               style="color: var(--color-text-secondary);">Mash</div>
          <MashTab {recipe} {stats} onchange={load} />
        </section>

        <!-- Water -->
        <section>
          <div class="text-xs font-semibold uppercase tracking-wider mb-3"
               style="color: var(--color-text-secondary);">Water</div>
          <WaterTab {recipe} onchange={load} />
        </section>

        <!-- Fermentation -->
        <section>
          <div class="text-xs font-semibold uppercase tracking-wider mb-3"
               style="color: var(--color-text-secondary);">Fermentation</div>
          <FermentationTab {recipe} onchange={load} />
        </section>

        <!-- Notes -->
        <section>
          <div class="text-xs font-semibold uppercase tracking-wider mb-3"
               style="color: var(--color-text-secondary);">Notes</div>
          <NotesTab {recipe} onchange={load} />
        </section>

      </div>
    </div>
  </div>
{:else}
  <div class="flex-1 flex items-center justify-center">
    <p class="text-sm" style="color: var(--color-text-muted);">Loading…</p>
  </div>
{/if}
