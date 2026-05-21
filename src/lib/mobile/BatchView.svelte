<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import type { Batch, UpdateBatchInput } from "$lib/api";
  import { getBatch, updateBatch } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { refreshBatchList } from "$lib/stores/batches";
  import BatchOverviewTab from "$lib/components/batch/BatchOverviewTab.svelte";
  import BatchGravityTab from "$lib/components/batch/BatchGravityTab.svelte";
  import BatchTastingTab from "$lib/components/batch/BatchTastingTab.svelte";

  let { id }: { id: string } = $props();

  let batch = $state<Batch | null>(null);

  async function loadBatch() {
    batch = await ipc(getBatch(id)) ?? null;
  }

  onMount(async () => {
    await refreshBatchList();
    await loadBatch();
  });

  $effect(() => { if (id) loadBatch(); });

  async function handleUpdate(input: UpdateBatchInput) {
    if (!batch) return;
    batch = await ipc(updateBatch(batch.id, input)) ?? batch;
  }
</script>

{#if batch}
  <div class="flex flex-col h-full overflow-hidden" style="background: var(--color-bg-base);">
    <!-- Header with back button -->
    <div class="flex items-center gap-3 px-4 py-3 border-b flex-shrink-0"
         style="background: var(--color-bg-surface); border-color: var(--color-border);">
      <button
        onclick={() => goto("/batches")}
        class="text-sm"
        style="color: var(--color-accent);"
      >‹ Batches</button>
      <span class="flex-1 font-semibold text-base truncate"
            style="color: var(--color-text-primary);">{batch.recipe_name}</span>
    </div>

    <!-- Single scroll -->
    <div class="flex-1 overflow-y-auto">
      <div class="p-4 flex flex-col gap-6">

        <!-- Overview (dates + measurements, includes status) -->
        <section>
          <BatchOverviewTab {batch} onUpdate={handleUpdate} />
        </section>

        <!-- Gravity Log -->
        <section>
          <div class="text-xs font-semibold uppercase tracking-wider mb-3"
               style="color: var(--color-text-secondary);">Gravity Log</div>
          <BatchGravityTab {batch} onRefresh={loadBatch} />
        </section>

        <!-- Tasting -->
        <section>
          <div class="text-xs font-semibold uppercase tracking-wider mb-3"
               style="color: var(--color-text-secondary);">Tasting</div>
          <BatchTastingTab {batch} onUpdate={handleUpdate} />
        </section>

      </div>
    </div>
  </div>
{:else}
  <div class="flex-1 flex items-center justify-center">
    <p class="text-sm" style="color: var(--color-text-muted);">Loading…</p>
  </div>
{/if}
