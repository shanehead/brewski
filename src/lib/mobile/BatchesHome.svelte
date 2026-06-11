<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { batchList, refreshBatchList } from "$lib/stores/batches";
  import { listRecipes } from "$lib/api";
  import { startBrew, brewCurrent, brewVersion } from "$lib/brewFlow";
  import { ipc } from "$lib/stores/error";
  import BrewVersionModal from "$lib/components/BrewVersionModal.svelte";
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
    const decision = await startBrew(recipe.id);
    if (!decision) return;
    if (decision.kind === "auto") {
      await ipc(refreshBatchList());
      goto(`/batches/${decision.batch.id}`);
      return;
    }
    promptRecipeId = recipe.id;
    promptStatus = decision.status;
    promptVersions = decision.versions;
  }

  async function finishBrew(batch: { id: string } | null) {
    promptStatus = null;
    if (!batch) return;
    await ipc(refreshBatchList());
    goto(`/batches/${batch.id}`);
  }
</script>

<div class="flex flex-col h-full overflow-hidden bg-bg-surface">
  <div class="p-3 border-b border-border">
    <button
      onclick={handleNew}
      class="w-full py-3 rounded text-sm font-medium bg-accent"
      style="color: #fff;"
    >+ New Batch</button>
  </div>
  <div class="flex-1 overflow-y-auto">
    {#each $batchList as batch (batch.id)}
      <a
        href="/batches/{batch.id}"
        class="flex items-center justify-between px-4 py-3 border-b text-sm border-border text-text-primary"

      >
        <div class="flex flex-col gap-0.5 min-w-0">
          <span class="truncate font-medium">{batch.recipe_name}</span>
          <span class="text-xs truncate text-text-muted">
            {batch.name ?? "Batch"} · {batch.status}
          </span>
        </div>
        <span class="text-text-muted">›</span>
      </a>
    {:else}
      <p class="p-4 text-sm text-text-muted">No batches yet. Tap + to start one.</p>
    {/each}
  </div>
</div>

{#if showPicker}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center"
    style="background: rgba(0,0,0,0.5);"
    role="dialog"
    aria-modal="true"
  >
    <div class="rounded-lg p-4 mx-4 max-h-96 flex flex-col gap-2 overflow-hidden bg-bg-surface border border-border"
         style="width: calc(100% - 2rem);">
      <div class="font-medium text-sm">Choose a recipe to brew</div>
      <div class="flex-1 overflow-y-auto flex flex-col gap-1">
        {#each recipes as r (r.id)}
          <button
            onclick={() => handlePickRecipe(r)}
            class="text-left px-3 py-3 rounded text-sm bg-bg-elevated text-text-primary"

          >{r.name}</button>
        {/each}
      </div>
      <button onclick={() => showPicker = false}
        class="text-xs py-2 text-text-muted">Cancel</button>
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
