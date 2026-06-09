<!-- src/lib/components/tabs/BatchesTab.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import type { BatchSummary } from "$lib/api";
  import { listBatchesForRecipe, createBatch } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import BatchList from "$lib/components/BatchList.svelte";

  let { recipeId }: { recipeId: string } = $props();

  let batches = $state<BatchSummary[]>([]);

  async function load() {
    batches = (await ipc(listBatchesForRecipe(recipeId))) ?? [];
  }

  onMount(load);

  async function handleBrew() {
    const batch = await ipc(createBatch({ recipe_id: recipeId, name: null }));
    if (!batch) return;
    goto(`/batches/${batch.id}`);
  }

  const avgRating = $derived(
    (() => {
      const rated = batches.filter((b) => b.rating != null);
      if (!rated.length) return null;
      return (rated.reduce((s, b) => s + (b.rating ?? 0), 0) / rated.length).toFixed(1);
    })()
  );
</script>

<div class="p-4 flex flex-col gap-4 overflow-y-auto h-full max-w-2xl">
  <div class="flex items-center justify-between">
    <div class="text-sm" style="color: var(--color-text-muted);">
      {batches.length} batch{batches.length === 1 ? "" : "es"}
      {#if avgRating !== null}· avg rating {avgRating}/10{/if}
    </div>
    <button
      onclick={handleBrew}
      class="px-3 py-1.5 rounded text-sm"
      style="background: var(--color-accent); color: #fff;"
    >Brew this Recipe</button>
  </div>
  <BatchList {batches} onRefresh={load} />
</div>
