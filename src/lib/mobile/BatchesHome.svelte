<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
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

<div class="flex flex-col h-full overflow-hidden" style="background: var(--color-bg-surface);">
  <div class="p-3 border-b" style="border-color: var(--color-border);">
    <button
      onclick={handleNew}
      class="w-full py-3 rounded text-sm font-medium"
      style="background: var(--color-accent); color: #fff;"
    >+ New Batch</button>
  </div>
  <div class="flex-1 overflow-y-auto">
    {#each $batchList as batch (batch.id)}
      <a
        href="/batches/{batch.id}"
        class="flex items-center justify-between px-4 py-3 border-b text-sm"
        style="border-color: var(--color-border); color: var(--color-text-primary);"
      >
        <div class="flex flex-col gap-0.5 min-w-0">
          <span class="truncate font-medium">{batch.recipe_name}</span>
          <span class="text-xs truncate" style="color: var(--color-text-muted);">
            {batch.name ?? "Batch"} · {batch.status}
          </span>
        </div>
        <span style="color: var(--color-text-muted);">›</span>
      </a>
    {:else}
      <p class="p-4 text-sm" style="color: var(--color-text-muted);">No batches yet. Tap + to start one.</p>
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
    <div class="rounded-lg p-4 mx-4 max-h-96 flex flex-col gap-2 overflow-hidden"
         style="background: var(--color-bg-surface); border: 1px solid var(--color-border); width: calc(100% - 2rem);">
      {#if step === "recipe"}
        <div class="font-medium text-sm">Choose a recipe to brew</div>
        <div class="flex-1 overflow-y-auto flex flex-col gap-1">
          {#each recipes as r (r.id)}
            <button
              onclick={() => handlePickRecipe(r)}
              class="text-left px-3 py-3 rounded text-sm"
              style="background: var(--color-bg-elevated); color: var(--color-text-primary);"
            >{r.name}</button>
          {/each}
        </div>
        <button onclick={() => showPicker = false}
          class="text-xs py-2" style="color: var(--color-text-muted);">Cancel</button>
      {:else}
        <button
          onclick={handleBack}
          class="text-xs text-left font-medium py-1"
          style="color: var(--color-accent);"
        >← {pickedRecipe?.name}</button>
        <div class="font-medium text-sm">Choose a version</div>
        <div class="flex-1 overflow-y-auto flex flex-col gap-1">
          {#each versions as v, i (v.id)}
            <button
              onclick={() => handlePickVersion(v)}
              class="text-left px-3 py-3 rounded text-sm"
              style="background: {i === 0 ? 'var(--color-accent)' : 'var(--color-bg-elevated)'}; color: {i === 0 ? '#fff' : 'var(--color-text-primary)'};"
            >
              <span class="font-mono">v{v.version_number}</span>
              {#if v.name}<span class="ml-1">· {v.name}</span>{/if}
              <span class="ml-1 text-xs opacity-60">{formatDate(v.created_at)}</span>
            </button>
          {/each}
        </div>
        <button onclick={() => showPicker = false}
          class="text-xs py-2" style="color: var(--color-text-muted);">Cancel</button>
      {/if}
    </div>
  </div>
{/if}
