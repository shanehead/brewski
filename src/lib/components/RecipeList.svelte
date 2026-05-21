<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { recipeList, refreshRecipeList } from "$lib/stores/recipes";
  import { createRecipe, deleteRecipe, createRecipesFromBeerxml } from "$lib/api";
  import type { RecipeSummary } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { settings } from "$lib/stores/settings";
  import { type Units, lToGal, volumeLabel } from "$lib/units";

  let { selectedId = $bindable<string | null>(null) } = $props();
  let search = $state("");
  let fileInput: HTMLInputElement;

  const units = $derived<Units>($settings.units === "imperial" ? "imperial" : "metric");

  const filtered = $derived(
    search.trim()
      ? $recipeList.filter((r) => r.name.toLowerCase().includes(search.toLowerCase()))
      : $recipeList
  );

  onMount(() => ipc(refreshRecipeList()));

  async function handleNew() {
    const recipe = await ipc(createRecipe({ name: "New Recipe" }));
    if (!recipe) return;
    await ipc(refreshRecipeList());
    goto(`/recipe/${recipe.id}`);
  }

  async function handleDelete(id: string) {
    await ipc(deleteRecipe(id));
    await ipc(refreshRecipeList());
    if (selectedId === id) goto("/");
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
</script>

<aside class="w-56 flex flex-col flex-shrink-0 border-r overflow-hidden"
       style="background: var(--color-bg-surface); border-color: var(--color-border);">
  <!-- Search + New -->
  <div class="p-2 flex flex-col gap-1.5 border-b" style="border-color: var(--color-border);">
    <div class="relative">
      <svg class="absolute left-2 top-1/2 -translate-y-1/2 pointer-events-none" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" style="color: var(--color-text-muted);">
        <circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
      </svg>
      <input
        type="search"
        placeholder="Search recipes…"
        bind:value={search}
        class="w-full pl-7 pr-2.5 py-1.5 rounded text-sm outline-none"
        style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);"
      />
    </div>
    <button
      onclick={handleNew}
      class="w-full py-1.5 rounded text-sm font-medium transition-colors"
      style="background: var(--color-accent); color: #fff;"
    >
      + New Recipe
    </button>
    <input
      type="file"
      accept=".xml,.beerxml,text/xml,application/xml"
      bind:this={fileInput}
      onchange={handleImport}
      class="hidden"
    />
    <button
      onclick={() => fileInput.click()}
      class="w-full py-1.5 rounded text-sm font-medium transition-colors"
      style="border: 1px solid var(--color-border); color: var(--color-text-secondary); background: transparent;"
    >
      Import BeerXML
    </button>
  </div>

  <!-- Recipe list -->
  <ul class="flex-1 overflow-y-auto py-1">
    {#each filtered as recipe (recipe.id)}
      <li class="group relative">
        <a
          href="/recipe/{recipe.id}"
          class="flex flex-col px-3 py-2 pr-7 cursor-pointer transition-colors hover:bg-[var(--color-bg-elevated)]"
          style={selectedId === recipe.id
            ? "background: var(--color-bg-elevated); border-left: 2px solid var(--color-accent); padding-left: calc(0.75rem - 2px);"
            : "color: var(--color-text-primary); border-left: 2px solid transparent; padding-left: calc(0.75rem - 2px);"}
        >
          <span class="text-sm font-medium truncate" style="color: var(--color-text-primary);">{recipe.name}</span>
          <span class="text-xs truncate mt-0.5" style="color: var(--color-text-secondary);">
            {recipe.style_name ?? recipe.type_} · {(units === "imperial" ? lToGal(recipe.batch_size_l) : recipe.batch_size_l).toFixed(1)}{volumeLabel(units)}
          </span>
        </a>
        <button
          onclick={() => handleDelete(recipe.id)}
          aria-label="Delete recipe"
          class="absolute right-2 top-1/2 -translate-y-1/2 opacity-0 group-hover:opacity-100 transition-opacity text-sm leading-none"
          style="color: var(--color-text-muted);"
        >×</button>
      </li>
    {:else}
      <li class="px-3 py-6 text-center text-sm" style="color: var(--color-text-muted);">
        {search ? "No matches" : "No recipes yet"}
      </li>
    {/each}
  </ul>
</aside>
