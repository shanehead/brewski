<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { getRecipe, getRecipeStats, createRecipe } from "$lib/api";
  import type { Recipe, RecipeStats } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import RecipeList from "$lib/components/RecipeList.svelte";
  import StatsSidebar from "$lib/components/StatsSidebar.svelte";
  import OverviewTab from "$lib/components/tabs/OverviewTab.svelte";
  import IngredientsTab from "$lib/components/tabs/IngredientsTab.svelte";
  import MashTab from "$lib/components/tabs/MashTab.svelte";
  import WaterTab from "$lib/components/tabs/WaterTab.svelte";
  import FermentationTab from "$lib/components/tabs/FermentationTab.svelte";
  import NotesTab from "$lib/components/tabs/NotesTab.svelte";
  import TabBar from "$lib/components/TabBar.svelte";
  import type { BrewingIconName } from "$lib/icons";

  let { id }: { id: string } = $props();

  let recipe = $state<Recipe | null>(null);
  let stats = $state<RecipeStats | null>(null);
  let activeTab = $state<"overview" | "ingredients" | "mash" | "water" | "fermentation" | "notes">("overview");
  let cloning = $state(false);

  onMount(async () => {
    recipe = await ipc(getRecipe(id)) ?? null;
    if (recipe) stats = await ipc(getRecipeStats(id)) ?? null;
  });

  $effect(() => {
    if (id) {
      (async () => {
        recipe = await ipc(getRecipe(id)) ?? null;
        if (recipe) stats = await ipc(getRecipeStats(id)) ?? null;
      })();
    }
  });

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

  const TABS: { key: typeof activeTab; label: string; icon: BrewingIconName }[] = [
    { key: "overview", label: "Overview", icon: "overview" },
    { key: "ingredients", label: "Ingredients", icon: "ingredients" },
    { key: "mash", label: "Mash", icon: "mash" },
    { key: "water", label: "Water", icon: "water" },
    { key: "fermentation", label: "Fermentation", icon: "fermentation" },
    { key: "notes", label: "Notes", icon: "notes" },
  ] as const;
</script>

<RecipeList selectedId={id} />

{#if recipe}
  <div class="flex flex-1 flex-col overflow-hidden">
    <!-- Header -->
    <header
      class="flex items-center px-4 py-2 border-b gap-3 flex-shrink-0"
      style="background: var(--color-bg-surface); border-color: var(--color-border);"
    >
      <button
        onclick={() => goto("/")}
        class="text-xs px-2 py-1 rounded"
        style="color: var(--color-text-secondary); background: var(--color-bg-elevated); border: 1px solid var(--color-border);"
      >
        ← Recipes
      </button>
      <div class="flex-1">
        <h1 class="text-base font-semibold" style="color: var(--color-text-primary);">{recipe.name}</h1>
        <p class="text-xs" style="color: var(--color-text-muted);">Example Recipe — read only</p>
      </div>
      <button
        onclick={handleClone}
        disabled={cloning}
        class="px-4 py-1.5 rounded text-sm font-semibold transition-colors disabled:opacity-50"
        style="background: var(--color-accent); color: #fff;"
      >
        {cloning ? "Cloning…" : "Clone to My Recipes"}
      </button>
    </header>

    <!-- Tab bar -->
    <nav class="px-4 pt-1 flex-shrink-0"
         style="background: var(--color-bg-surface);">
      <TabBar tabs={TABS} active={activeTab} onchange={(key) => activeTab = key as typeof activeTab} />
    </nav>

    <!-- Tab content + stats sidebar -->
    <div class="flex flex-1 overflow-hidden">
      <div class="flex-1 overflow-y-auto p-4">
        {#if activeTab === "overview"}
          <OverviewTab {recipe} onchange={() => {}} />
        {:else if activeTab === "ingredients"}
          <IngredientsTab {recipe} {stats} onchange={() => {}} />
        {:else if activeTab === "mash"}
          <MashTab {recipe} {stats} onchange={() => {}} />
        {:else if activeTab === "water"}
          <WaterTab {recipe} {stats} onchange={() => {}} />
        {:else if activeTab === "fermentation"}
          <FermentationTab {recipe} onchange={() => {}} />
        {:else if activeTab === "notes"}
          <NotesTab {recipe} onchange={() => {}} />
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
