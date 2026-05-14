<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import BatchList from "$lib/components/BatchList.svelte";
  import { batchList, refreshBatchList } from "$lib/stores/batches";
  import { createBatch, listRecipes } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import type { RecipeSummary } from "$lib/api";

  let showPicker = $state(false);
  let recipes = $state<RecipeSummary[]>([]);

  onMount(() => ipc(refreshBatchList()));

  async function handleNew() {
    recipes = (await ipc(listRecipes())) ?? [];
    showPicker = true;
  }

  async function handlePickRecipe(recipeId: string) {
    showPicker = false;
    const batch = await ipc(createBatch({ recipe_id: recipeId, name: null }));
    if (!batch) return;
    await ipc(refreshBatchList());
    goto(`/batches/${batch.id}`);
  }
</script>

<aside class="w-56 flex flex-col flex-shrink-0 border-r overflow-hidden"
       style="background: var(--color-bg-surface); border-color: var(--color-border);">
  <div class="p-2 border-b" style="border-color: var(--color-border);">
    <button
      onclick={handleNew}
      class="w-full px-2 py-1.5 rounded text-sm text-left"
      style="background: var(--color-accent); color: #fff;"
    >+ New Batch</button>
  </div>
  <div class="flex-1 overflow-y-auto">
    <BatchList batches={$batchList} onRefresh={() => ipc(refreshBatchList())} />
  </div>
</aside>

<div class="flex-1 flex items-center justify-center" style="color: var(--color-text-muted);">
  <p class="text-sm">Select a batch to view</p>
</div>

{#if showPicker}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center"
    style="background: rgba(0,0,0,0.5);"
    role="dialog"
    aria-modal="true"
  >
    <div class="rounded-lg p-4 w-80 max-h-96 flex flex-col gap-2 overflow-hidden"
         style="background: var(--color-bg-surface); border: 1px solid var(--color-border);">
      <div class="font-medium text-sm">Choose a recipe to brew</div>
      <div class="flex-1 overflow-y-auto flex flex-col gap-1">
        {#each recipes as r (r.id)}
          <button
            onclick={() => handlePickRecipe(r.id)}
            class="text-left px-3 py-2 rounded text-sm hover:opacity-80"
            style="background: var(--color-bg-elevated); color: var(--color-text-primary);"
          >{r.name}</button>
        {/each}
      </div>
      <button onclick={() => showPicker = false}
        class="text-xs" style="color: var(--color-text-muted);">Cancel</button>
    </div>
  </div>
{/if}
