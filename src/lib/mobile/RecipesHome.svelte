<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { recipeList, refreshRecipeList, baselineRecipeList, refreshBaselineRecipeList } from "$lib/stores/recipes";
  import { createRecipe, createRecipesFromBeerxml } from "$lib/api";
  import type { RecipeSummary } from "$lib/api";
  import { ipc, setSuccess } from "$lib/stores/error";
  import { settings, saveSetting } from "$lib/stores/settings";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { appDataDir as getAppDataDir } from "@tauri-apps/api/path";
  import { srmToHex } from "$lib/utils/srm";

  let fileInput: HTMLInputElement;
  let importing = $state(false);
  let appDataDir = $state("");
  let searchEl = $state<HTMLInputElement | null>(null);
  let search = $state("");

  const filtered = $derived(
    search.trim() === ""
      ? $recipeList
      : $recipeList.filter((r) => r.name.toLowerCase().includes(search.trim().toLowerCase()))
  );

  const startersCollapsed = $derived($settings.starters_collapsed ?? false);

  onMount(async () => {
    appDataDir = await getAppDataDir();
    ipc(refreshRecipeList());
    ipc(refreshBaselineRecipeList());
  });

  function thumbnailSrc(recipe: RecipeSummary): string | null {
    return recipe.image_path
      ? convertFileSrc(`${appDataDir}/images/${recipe.image_path}`)
      : null;
  }

  async function handleNew() {
    const recipe = await ipc(createRecipe({ name: "New Recipe" }));
    if (!recipe) return;
    await ipc(refreshRecipeList());
    goto(`/recipe/${recipe.id}`);
  }

  async function handleImport(event: Event) {
    const file = (event.target as HTMLInputElement).files?.[0];
    if (!file) return;
    importing = true;
    try {
      const xml = await file.text();
      const imported = await ipc(createRecipesFromBeerxml(xml));
      if (!imported) return;
      setSuccess(`${imported.length} recipe${imported.length === 1 ? "" : "s"} imported`);
      await ipc(refreshRecipeList());
      fileInput.value = "";
    } finally {
      importing = false;
    }
  }

  function toggleStarters() {
    saveSetting("starters_collapsed", startersCollapsed ? "false" : "true");
  }
</script>

<div class="flex flex-col h-full overflow-hidden bg-bg-surface">
  <div class="p-3 border-b flex flex-col gap-2 border-border">
    <button
      onclick={handleNew}
      class="w-full py-3 rounded text-sm font-medium bg-accent"
      style="color: #fff;"
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
      disabled={importing}
      class="w-full py-3 rounded text-sm font-medium border border-accent text-accent"
      style="background: transparent;"
    >
      {importing ? "Importing…" : "Import BeerXML"}
    </button>
    <div class="relative">
      <svg class="text-text-muted" style="position: absolute; left: 8px; top: 50%; transform: translateY(-50%); pointer-events: none;"
           width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
      </svg>
      <input
        bind:this={searchEl}
        type="search"
        placeholder="Search recipes…"
        bind:value={search}
        class="w-full pl-8 pr-2.5 py-2 rounded text-sm outline-none bg-bg-elevated text-text-primary border border-border"
      />
    </div>
  </div>
  <div class="flex-1 overflow-y-auto">
    {#each filtered as recipe (recipe.id)}
      {@const thumb = thumbnailSrc(recipe)}
      <a
        href="/recipe/{recipe.id}"
        class="flex items-center gap-3 px-4 py-3 border-b text-sm border-border text-text-primary"
       
      >
        {#if thumb}
          <img src={thumb} alt="" class="w-8 h-8 rounded flex-shrink-0 object-cover" />
        {:else}
          <div class="w-8 h-8 rounded flex-shrink-0" style="background: linear-gradient(135deg, {srmToHex(4)}, {srmToHex(16)});"></div>
        {/if}
        <span class="truncate flex-1">{recipe.name}</span>
        <span class="text-text-muted">›</span>
      </a>
    {:else}
      <p class="p-4 text-sm text-text-muted">
        {search ? "No matches" : "No recipes yet. Tap + to create one."}
      </p>
    {/each}

    <!-- Example Recipes section -->
    {#if $baselineRecipeList.length > 0 && !$settings.hide_example_recipes}
      <button
        onclick={toggleStarters}
        class="w-full flex items-center justify-between px-4 py-2 border-b bg-bg-base border-border"
       
      >
        <span class="text-xs font-semibold uppercase tracking-wider text-text-muted">
          Example Recipes
        </span>
        <span class="text-xs text-text-muted">
          {startersCollapsed ? "▸" : "▾"}
        </span>
      </button>
      {#if !startersCollapsed}
        {#each $baselineRecipeList as recipe (recipe.id)}
          {@const thumb = thumbnailSrc(recipe)}
          <a
            href="/baseline-recipe/{recipe.id}"
            class="flex items-center gap-3 px-4 py-3 border-b text-sm border-border text-text-secondary"
           
          >
            {#if thumb}
              <img src={thumb} alt="" class="w-8 h-8 rounded flex-shrink-0 object-cover" />
            {:else}
              <div class="w-8 h-8 rounded flex-shrink-0" style="background: linear-gradient(135deg, {srmToHex(4)}, {srmToHex(16)});"></div>
            {/if}
            <span class="truncate flex-1">{recipe.name}</span>
            <span class="text-text-muted">›</span>
          </a>
        {/each}
      {/if}
    {/if}
  </div>
</div>
