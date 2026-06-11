<script lang="ts">
  import { onMount } from "svelte";
  import BatchList from "$lib/components/BatchList.svelte";
  import { batchList, refreshBatchList } from "$lib/stores/batches";
  import { ipc } from "$lib/stores/error";

  onMount(() => ipc(refreshBatchList()));
</script>

<aside class="w-56 flex flex-col flex-shrink-0 border-r overflow-hidden bg-bg-surface border-border"
      >
  <div class="flex-1 overflow-y-auto">
    <BatchList batches={$batchList} onRefresh={() => ipc(refreshBatchList())} />
  </div>
</aside>

<div class="flex-1 flex items-center justify-center text-text-muted">
  <p class="text-sm">Select a batch to view</p>
</div>
