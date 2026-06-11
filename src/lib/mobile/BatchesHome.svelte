<script lang="ts">
  import { onMount } from "svelte";
  import { batchList, refreshBatchList } from "$lib/stores/batches";
  import { ipc } from "$lib/stores/error";

  onMount(() => ipc(refreshBatchList()));
</script>

<div class="flex flex-col h-full overflow-hidden bg-bg-surface">
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
      <p class="p-4 text-sm text-text-secondary">No batches yet. Open a recipe and tap Brew to create one.</p>
    {/each}
  </div>
</div>
