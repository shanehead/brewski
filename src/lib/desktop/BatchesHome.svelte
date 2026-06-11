<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import BatchList from "$lib/components/BatchList.svelte";
  import BrewVersionModal from "$lib/components/BrewVersionModal.svelte";
  import { batchList, refreshBatchList } from "$lib/stores/batches";
  import { listRecipes } from "$lib/api";
  import { startBrew, brewCurrent, brewVersion } from "$lib/brewFlow";
  import { ipc } from "$lib/stores/error";
  import type { RecipeSummary, RecipeVersionStatus, RecipeVersionSummary } from "$lib/api";

  let showPicker = $state(false);
  let recipes = $state<RecipeSummary[]>([]);
  let promptStatus = $state<RecipeVersionStatus | null>(null);
  let promptVersions = $state<RecipeVersionSummary[]>([]);
  let promptRecipeId = $state<string | null>(null);

  onMount(() => ipc(refreshBatchList()));

  async function handleNew() {
    recipes = (await ipc(listRecipes())) ?? [];
    showPicker = true;
  }

  async function handlePickRecipe(recipe: RecipeSummary) {
    showPicker = false;
    const result = await startBrew(recipe.id);
    if (!result) return;
    promptRecipeId = recipe.id;
    promptStatus = result.status;
    promptVersions = result.versions;
  }

  async function finishBrew(batch: { id: string } | null) {
    promptStatus = null;
    if (!batch) return;
    await ipc(refreshBatchList());
    goto(`/batches/${batch.id}`);
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
    </div>
  </div>
{/if}

{#if promptStatus && promptRecipeId}
  <BrewVersionModal
    status={promptStatus}
    versions={promptVersions}
    onBrewCurrent={async (name) => finishBrew(await brewCurrent(promptRecipeId!, name))}
    onBrewVersion={async (vid) => finishBrew(await brewVersion(promptRecipeId!, vid))}
    onCancel={() => (promptStatus = null)} />
{/if}
