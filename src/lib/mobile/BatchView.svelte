<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import type { Batch, UpdateBatchInput, ImageRef } from "$lib/api";
  import { getBatch, updateBatch, listBatchAttachments } from "$lib/api";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { appDataDir as getAppDataDir } from "@tauri-apps/api/path";
  import { ipc } from "$lib/stores/error";
  import { refreshBatchList } from "$lib/stores/batches";
  import BatchOverviewTab from "$lib/components/batch/BatchOverviewTab.svelte";
  import BatchAttachmentsTab from "$lib/components/batch/BatchAttachmentsTab.svelte";

  let { id }: { id: string } = $props();

  let batch = $state<Batch | null>(null);
  let imageRefs = $state<ImageRef[]>([]);

  async function loadImageRefs() {
    if (!batch) return;
    const appDataDir = await getAppDataDir();
    const attachments = await ipc(listBatchAttachments(batch.id));
    if (attachments) {
      imageRefs = attachments
        .filter((a) => a.mime_type?.startsWith('image/'))
        .map((a) => ({
          id: a.id,
          name: a.original_name,
          assetUrl: convertFileSrc(`${appDataDir}/attachments/${batch!.id}/${a.filename}`),
        }));
    }
  }

  async function loadBatch() {
    batch = await ipc(getBatch(id)) ?? null;
    await loadImageRefs();
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

    <div class="flex-1 overflow-y-auto">
      <div class="flex flex-col gap-6 pb-6">
        <!-- Overview + stage content (includes gravity for fermenting, tasting for packaged) -->
        <BatchOverviewTab {batch} onUpdate={handleUpdate} onRefresh={loadBatch} images={imageRefs} />

        <!-- Attachments always accessible at bottom on mobile -->
        <section class="px-4">
          <div class="text-xs font-semibold uppercase tracking-wider mb-3"
               style="color: var(--color-text-secondary);">Attachments</div>
          <BatchAttachmentsTab {batch} onAttachmentsChange={loadImageRefs} />
        </section>
      </div>
    </div>
  </div>
{:else}
  <div class="flex-1 flex items-center justify-center">
    <p class="text-sm" style="color: var(--color-text-muted);">Loading…</p>
  </div>
{/if}
