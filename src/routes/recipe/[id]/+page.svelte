<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import type { PageData } from "./$types";
  import { getRecipe, getRecipeStats, updateRecipe } from "$lib/api";
  import type { Recipe, RecipeStats } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import RecipeList from "$lib/components/RecipeList.svelte";
  import StatsSidebar from "$lib/components/StatsSidebar.svelte";
  import BrewingIcon from "$lib/components/BrewingIcon.svelte";
  import OverviewTab from "$lib/components/tabs/OverviewTab.svelte";
  import IngredientsTab from "$lib/components/tabs/IngredientsTab.svelte";
  import MashTab from "$lib/components/tabs/MashTab.svelte";
  import WaterTab from "$lib/components/tabs/WaterTab.svelte";
  import FermentationTab from "$lib/components/tabs/FermentationTab.svelte";
  import NotesTab from "$lib/components/tabs/NotesTab.svelte";
  import BatchesTab from "$lib/components/tabs/BatchesTab.svelte";
  import type { BrewingIconName } from "$lib/icons";

  let { data }: { data: PageData } = $props();

  let recipe = $state<Recipe | null>(null);
  let stats = $state<RecipeStats | null>(null);
  let activeTab = $state<"overview" | "ingredients" | "mash" | "water" | "fermentation" | "notes" | "batches">("overview");
  let saving = $state(false);

  const TABS: { key: "overview" | "ingredients" | "mash" | "water" | "fermentation" | "notes" | "batches"; label: string; icon: BrewingIconName }[] = [
    { key: "overview", label: "Overview", icon: "overview" },
    { key: "ingredients", label: "Ingredients", icon: "ingredients" },
    { key: "mash", label: "Mash", icon: "mash" },
    { key: "water", label: "Water", icon: "water" },
    { key: "fermentation", label: "Fermentation", icon: "fermentation" },
    { key: "batches", label: "Batches", icon: "batches" },
    { key: "notes", label: "Notes", icon: "notes" },
  ] as const;

  async function loadRecipeById(id: string) {
    recipe = await ipc(getRecipe(id)) ?? null;
    if (recipe) {
      stats = await ipc(getRecipeStats(recipe.id)) ?? null;
    } else {
      stats = null;
    }
  }

  onMount(async () => {
    await loadRecipeById(data.id);
  });

  $effect(() => {
    if (data?.id) {
      (async () => {
        await loadRecipeById(data.id);
      })();
    }
  });

  async function refreshStats() {
    if (!recipe) return;
    stats = await ipc(getRecipeStats(recipe.id)) ?? null;
  }

  async function refreshRecipe() {
    await loadRecipeById(data.id);
  }

  async function handleNameBlur(e: FocusEvent) {
    const target = e.currentTarget as HTMLInputElement;
    if (!recipe || target.value === recipe.name) return;
    saving = true;
    recipe = await ipc(updateRecipe(recipe.id, { name: target.value })) ?? recipe;
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
          class="px-4 py-2 text-sm border-b-2 transition-colors inline-flex items-center gap-2"
          style={activeTab === tab.key
            ? "border-color: var(--color-accent); color: var(--color-text-primary);"
            : "border-color: transparent; color: var(--color-text-secondary);"}
        >
          <BrewingIcon name={tab.icon} />
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
          <MashTab {recipe} {stats} onchange={refreshRecipe} />
        {:else if activeTab === "water"}
          <WaterTab {recipe} onchange={refreshRecipe} />
        {:else if activeTab === "fermentation"}
          <FermentationTab {recipe} onchange={refreshRecipe} />
        {:else if activeTab === "notes"}
          <NotesTab {recipe} onchange={refreshRecipe} />
        {:else if activeTab === "batches"}
          <BatchesTab recipeId={recipe.id} />
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
