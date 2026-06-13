<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import type { Batch, UpdateBatchInput, ImageRef } from "$lib/api";
  import { getBatch, updateBatch, listBatchAttachments } from "$lib/api";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { appDataDir as getAppDataDir } from "@tauri-apps/api/path";
  import { ipc } from "$lib/stores/error";
  import BatchOverviewTab from "$lib/components/batch/BatchOverviewTab.svelte";
  import BatchAttachmentsTab from "$lib/components/batch/BatchAttachmentsTab.svelte";

  let { id }: { id: string } = $props();

  let batch = $state<Batch | null>(null);
  let showAttachments = $state(false);
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

<svelte:window onkeydown={(e) => e.key === "Escape" && (showAttachments = false)} />

<div class="flex flex-1 flex-col overflow-hidden">
  {#if batch}
    <!-- Header -->
    <div class="px-4 pt-3 pb-2 flex-shrink-0 flex items-start justify-between gap-2 border-b border-border"
        >
      <div class="min-w-0">
        <div class="font-semibold text-base truncate">{batch.recipe_name}</div>
        <div class="text-xs text-text-muted">
          {batch.name ?? "Batch"} · v{batch.recipe_version_id.slice(0, 6)}
        </div>
      </div>
      <button
        onclick={() => showAttachments = true}
        class="flex-shrink-0 px-2 py-1 rounded text-xs mt-0.5 bg-bg-elevated text-text-muted border border-border"
       
      >📎 Attachments</button>
    </div>

    <!-- Stage content -->
    <div class="flex-1 overflow-y-auto">
      <BatchOverviewTab {batch} onUpdate={handleUpdate} onRefresh={loadBatch} images={imageRefs} />
    </div>

    <!-- Attachments modal -->
    {#if showAttachments}
      <div
        class="fixed inset-0 z-50 flex items-center justify-center"
        style="background: rgba(0,0,0,0.5);"
        role="none"
        onclick={(e) => e.target === e.currentTarget && (showAttachments = false)}
        onkeydown={(e) => e.key === "Escape" && (showAttachments = false)}
      >
        <div class="rounded-lg w-full max-w-2xl max-h-[80vh] flex flex-col overflow-hidden bg-bg-surface border border-border"
             role="dialog" aria-modal="true">
          <div class="flex items-center justify-between px-4 py-3 border-b flex-shrink-0 border-border"
              >
            <div class="font-medium text-sm">Attachments</div>
            <button
              onclick={() => showAttachments = false}
              class="text-xs px-2 py-1 rounded text-text-muted bg-bg-elevated"

            >Close</button>
          </div>
          <div class="flex-1 overflow-y-auto">
            <BatchAttachmentsTab {batch} onAttachmentsChange={loadImageRefs} />
          </div>
        </div>
      </div>
    {/if}
  {:else}
    <div class="flex-1 flex items-center justify-center text-text-muted">
      <p class="text-sm">Loading…</p>
    </div>
  {/if}
</div>
