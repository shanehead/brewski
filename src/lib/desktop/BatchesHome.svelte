<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import BatchList from "$lib/components/BatchList.svelte";
  import { batchList, refreshBatchList } from "$lib/stores/batches";
  import { createBatch, listRecipes, listRecipeVersions } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import type { RecipeSummary, RecipeVersionSummary } from "$lib/api";

  let showPicker = $state(false);
  let step = $state<"recipe" | "version">("recipe");
  let recipes = $state<RecipeSummary[]>([]);
  let pickedRecipe = $state<RecipeSummary | null>(null);
  let versions = $state<RecipeVersionSummary[]>([]);

  onMount(() => ipc(refreshBatchList()));

  async function handleNew() {
    recipes = (await ipc(listRecipes())) ?? [];
    step = "recipe";
    pickedRecipe = null;
    versions = [];
    showPicker = true;
  }

  async function handlePickRecipe(recipe: RecipeSummary) {
    const vers = (await ipc(listRecipeVersions(recipe.id))) ?? [];
    if (vers.length >= 2) {
      pickedRecipe = recipe;
      versions = vers;
      step = "version";
    } else {
      showPicker = false;
      const batch = await ipc(createBatch({ recipe_id: recipe.id, name: null }));
      if (!batch) return;
      await ipc(refreshBatchList());
      goto(`/batches/${batch.id}`);
    }
  }

  async function handlePickVersion(version: RecipeVersionSummary) {
    showPicker = false;
    const batch = await ipc(
      createBatch({ recipe_id: pickedRecipe!.id, version_id: version.id, name: null })
    );
    if (!batch) return;
    await ipc(refreshBatchList());
    goto(`/batches/${batch.id}`);
  }

  function handleBack() {
    step = "recipe";
    pickedRecipe = null;
    versions = [];
  }

  function formatDate(ts: number): string {
    return new Date(ts * 1000).toLocaleDateString(undefined, {
      month: "short",
      day: "numeric",
      year: "numeric",
    });
  }
</script>

<aside class="w-56 flex flex-col flex-shrink-0 border-r overflow-hidden bg-bg-surface border-border"
      >
  <div class="p-2 border-b border-border">
    <button
      onclick={handleNew}
      class="w-full px-2 py-1.5 rounded text-sm text-left bg-accent"
      style="color: #fff;"
    >+ New Batch</button>
  </div>
  <div class="flex-1 overflow-y-auto">
    <BatchList batches={$batchList} onRefresh={() => ipc(refreshBatchList())} />
  </div>
</aside>

<div class="flex-1 flex items-center justify-center text-text-muted">
  <p class="text-sm">Select a batch to view</p>
</div>

{#if showPicker}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center"
    style="background: rgba(0,0,0,0.5);"
    role="dialog"
    aria-modal="true"
  >
    <div class="rounded-lg p-4 w-80 max-h-96 flex flex-col gap-2 overflow-hidden bg-bg-surface border border-border"
        >
      {#if step === "recipe"}
        <div class="font-medium text-sm">Choose a recipe to brew</div>
        <div class="flex-1 overflow-y-auto flex flex-col gap-1">
          {#each recipes as r (r.id)}
            <button
              onclick={() => handlePickRecipe(r)}
              class="text-left px-3 py-2 rounded text-sm hover:opacity-80 bg-bg-elevated text-text-primary"
             
            >{r.name}</button>
          {/each}
        </div>
        <button onclick={() => showPicker = false}
          class="text-xs text-text-muted">Cancel</button>
      {:else}
        <button
          onclick={handleBack}
          class="text-xs text-left font-medium text-accent"
         
        >← {pickedRecipe?.name}</button>
        <div class="font-medium text-sm">Choose a version</div>
        <div class="flex-1 overflow-y-auto flex flex-col gap-1">
          {#each versions as v, i (v.id)}
            <button
              onclick={() => handlePickVersion(v)}
              class="text-left px-3 py-2 rounded text-sm hover:opacity-80"
              style="background: {i === 0 ? 'var(--color-accent)' : 'var(--color-bg-elevated)'}; color: {i === 0 ? '#fff' : 'var(--color-text-primary)'};"
            >
              <span class="font-mono">v{v.version_number}</span>
              {#if v.name}<span class="ml-1">· {v.name}</span>{/if}
              <span class="ml-1 text-xs opacity-60">{formatDate(v.created_at)}</span>
            </button>
          {/each}
        </div>
        <button onclick={() => showPicker = false}
          class="text-xs text-text-muted">Cancel</button>
      {/if}
    </div>
  </div>
{/if}
