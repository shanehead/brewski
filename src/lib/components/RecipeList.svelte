<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { recipeList, refreshRecipeList } from "$lib/stores/recipes";
  import { createRecipe, deleteRecipe } from "$lib/api";
  import type { RecipeSummary } from "$lib/api";
  import { settings } from "$lib/stores/settings";
  import { type Units, lToGal, volumeLabel } from "$lib/units";

  let { selectedId = $bindable<string | null>(null) } = $props();
  let search = $state("");

  const units = $derived<Units>($settings.units === "imperial" ? "imperial" : "metric");

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
            {recipe.style_name ?? recipe.type_} · {(units === "imperial" ? lToGal(recipe.batch_size_l) : recipe.batch_size_l).toFixed(1)}{volumeLabel(units)}
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
