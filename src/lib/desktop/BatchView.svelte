<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import type { Batch, UpdateBatchInput } from "$lib/api";
  import { getBatch, updateBatch } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { batchList, refreshBatchList } from "$lib/stores/batches";
  import BatchList from "$lib/components/BatchList.svelte";
  import BatchOverviewTab from "$lib/components/batch/BatchOverviewTab.svelte";
  import BatchAttachmentsTab from "$lib/components/batch/BatchAttachmentsTab.svelte";

  let { id }: { id: string } = $props();

  let batch = $state<Batch | null>(null);
  let showAttachments = $state(false);

  async function loadBatch() {
    batch = await ipc(getBatch(id)) ?? null;
  }

  onMount(async () => {
    await refreshBatchList();
    await loadBatch();
  });

  $effect(() => {
    if (id) loadBatch();
  });

  async function handleUpdate(input: UpdateBatchInput) {
    if (!batch) return;
    batch = await ipc(updateBatch(batch.id, input)) ?? batch;
  }
</script>

<aside class="w-56 flex flex-col flex-shrink-0 border-r overflow-hidden"
       style="background: var(--color-bg-surface); border-color: var(--color-border);">
  <div class="p-2 border-b" style="border-color: var(--color-border);">
    <button
      onclick={() => goto("/batches")}
      class="w-full px-2 py-1.5 rounded text-sm text-left"
      style="background: var(--color-accent); color: #fff;"
    >+ New Batch</button>
  </div>
  <div class="flex-1 overflow-y-auto">
    <BatchList batches={$batchList} onRefresh={async () => { await ipc(refreshBatchList()); }} />
  </div>
</aside>

<div class="flex flex-1 flex-col overflow-hidden">
  {#if batch}
    <!-- Header -->
    <div class="px-4 pt-3 pb-2 flex-shrink-0 flex items-start justify-between gap-2"
         style="border-bottom: 1px solid var(--color-border);">
      <div class="min-w-0">
        <div class="font-semibold text-base truncate">{batch.recipe_name}</div>
        <div class="text-xs" style="color: var(--color-text-muted);">
          {batch.name ?? "Batch"} · v{batch.recipe_version_id.slice(0, 6)}
        </div>
      </div>
      <button
        onclick={() => showAttachments = true}
        class="flex-shrink-0 px-2 py-1 rounded text-xs mt-0.5"
        style="background: var(--color-bg-elevated); color: var(--color-text-muted); border: 1px solid var(--color-border);"
      >📎 Attachments</button>
    </div>

    <!-- Stage content -->
    <div class="flex-1 overflow-y-auto">
      <BatchOverviewTab {batch} onUpdate={handleUpdate} onRefresh={loadBatch} />
    </div>

    <!-- Attachments modal -->
    {#if showAttachments}
      <div
        class="fixed inset-0 z-50 flex items-center justify-center"
        style="background: rgba(0,0,0,0.5);"
        role="dialog"
        aria-modal="true"
      >
        <div class="rounded-lg w-full max-w-2xl max-h-[80vh] flex flex-col overflow-hidden"
             style="background: var(--color-bg-surface); border: 1px solid var(--color-border);">
          <div class="flex items-center justify-between px-4 py-3 border-b flex-shrink-0"
               style="border-color: var(--color-border);">
            <div class="font-medium text-sm">Attachments</div>
            <button
              onclick={() => showAttachments = false}
              class="text-xs px-2 py-1 rounded"
              style="color: var(--color-text-muted); background: var(--color-bg-elevated);"
            >Close</button>
          </div>
          <div class="flex-1 overflow-y-auto">
            <BatchAttachmentsTab {batch} />
          </div>
        </div>
      </div>
    {/if}
  {:else}
    <div class="flex-1 flex items-center justify-center" style="color: var(--color-text-muted);">
      <p class="text-sm">Loading…</p>
    </div>
  {/if}
</div>
