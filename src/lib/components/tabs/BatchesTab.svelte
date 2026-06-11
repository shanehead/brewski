<!-- src/lib/components/tabs/BatchesTab.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import type { BatchSummary, RecipeVersionStatus, RecipeVersionSummary } from "$lib/api";
  import { listBatchesForRecipe } from "$lib/api";
  import { startBrew, brewCurrent, brewVersion } from "$lib/brewFlow";
  import { ipc } from "$lib/stores/error";
  import BatchList from "$lib/components/BatchList.svelte";
  import BrewVersionModal from "$lib/components/BrewVersionModal.svelte";

  let { recipeId }: { recipeId: string } = $props();

  let batches = $state<BatchSummary[]>([]);
  let promptStatus = $state<RecipeVersionStatus | null>(null);
  let promptVersions = $state<RecipeVersionSummary[]>([]);

  async function load() {
    batches = (await ipc(listBatchesForRecipe(recipeId))) ?? [];
  }

  onMount(load);

  async function handleBrew() {
    const result = await startBrew(recipeId);
    if (!result) return;
    promptStatus = result.status;
    promptVersions = result.versions;
  }

  async function finishBrew(batch: { id: string } | null) {
    promptStatus = null;
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
    <div class="text-sm text-text-muted">
      {batches.length} batch{batches.length === 1 ? "" : "es"}
      {#if avgRating !== null}· avg rating {avgRating}/10{/if}
    </div>
    <button
      onclick={handleBrew}
      class="px-3 py-1.5 rounded text-sm bg-accent"
      style="color: #fff;"
    >Brew this Recipe</button>
  </div>
  <BatchList {batches} onRefresh={load} />
</div>

{#if promptStatus}
  <BrewVersionModal
    status={promptStatus}
    versions={promptVersions}
    onBrewCurrent={async (name) => finishBrew(await brewCurrent(recipeId, name))}
    onBrewVersion={async (vid) => finishBrew(await brewVersion(recipeId, vid))}
    onCancel={() => (promptStatus = null)} />
{/if}
