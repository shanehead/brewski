<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { recipeList, refreshRecipeList, baselineRecipeList, refreshBaselineRecipeList } from "$lib/stores/recipes";
  import { createRecipe, createRecipesFromBeerxml } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { settings } from "$lib/stores/settings";
  import { saveSetting } from "$lib/stores/settings";

  let fileInput: HTMLInputElement;

  const startersCollapsed = $derived($settings.starters_collapsed ?? false);

  onMount(() => {
    ipc(refreshRecipeList());
    ipc(refreshBaselineRecipeList());
  });

  async function handleNew() {
    const recipe = await ipc(createRecipe({ name: "New Recipe" }));
    if (!recipe) return;
    await ipc(refreshRecipeList());
    goto(`/recipe/${recipe.id}`);
  }

  async function handleImport(event: Event) {
    const file = (event.target as HTMLInputElement).files?.[0];
    if (!file) return;
    const xml = await file.text();
    const imported = await ipc(createRecipesFromBeerxml(xml));
    if (!imported) return;
    await ipc(refreshRecipeList());
    fileInput.value = "";
  }

  function toggleStarters() {
    saveSetting("starters_collapsed", startersCollapsed ? "false" : "true");
  }
</script>

<div class="flex flex-col h-full overflow-hidden" style="background: var(--color-bg-surface);">
  <div class="p-3 border-b flex flex-col gap-2" style="border-color: var(--color-border);">
    <button
      onclick={handleNew}
      class="w-full py-3 rounded text-sm font-medium"
      style="background: var(--color-accent); color: #fff;"
    >+ New Recipe</button>
    <input
      type="file"
      accept=".xml,.beerxml,text/xml,application/xml"
      bind:this={fileInput}
      onchange={handleImport}
      class="hidden"
    />
    <button
      onclick={() => fileInput.click()}
      class="w-full py-3 rounded text-sm font-medium"
      style="border: 1px solid var(--color-accent); color: var(--color-accent); background: transparent;"
    >Import BeerXML</button>
  </div>
  <div class="flex-1 overflow-y-auto">
    {#each $recipeList as recipe (recipe.id)}
      <a
        href="/recipe/{recipe.id}"
        class="flex items-center justify-between px-4 py-3 border-b text-sm"
        style="border-color: var(--color-border); color: var(--color-text-primary);"
      >
        <span class="truncate">{recipe.name}</span>
        <span style="color: var(--color-text-muted);">›</span>
      </a>
    {:else}
      <p class="p-4 text-sm" style="color: var(--color-text-muted);">No recipes yet. Tap + to create one.</p>
    {/each}

    <!-- Starter Recipes section -->
    {#if $baselineRecipeList.length > 0}
      <button
        onclick={toggleStarters}
        class="w-full flex items-center justify-between px-4 py-2 border-b"
        style="background: var(--color-bg-base); border-color: var(--color-border);"
      >
        <span class="text-xs font-semibold uppercase tracking-wider" style="color: var(--color-text-muted);">
          Starter Recipes
        </span>
        <span class="text-xs" style="color: var(--color-text-muted);">
          {startersCollapsed ? "▸" : "▾"}
        </span>
      </button>
      {#if !startersCollapsed}
        {#each $baselineRecipeList as recipe (recipe.id)}
          <a
            href="/baseline-recipe/{recipe.id}"
            class="flex items-center justify-between px-4 py-3 border-b text-sm"
            style="border-color: var(--color-border); color: var(--color-text-secondary);"
          >
            <span class="truncate">{recipe.name}</span>
            <span style="color: var(--color-text-muted);">›</span>
          </a>
        {/each}
      {/if}
    {/if}
  </div>
</div>
