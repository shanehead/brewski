<!-- src/lib/components/batch/BatchAttachmentsTab.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { open as openDialog } from "@tauri-apps/plugin-dialog";
  import { appDataDir as getAppDataDir } from "@tauri-apps/api/path";
  import type { Batch, BatchAttachment } from "$lib/api";
  import {
    listBatchAttachments,
    addBatchAttachment,
    deleteBatchAttachment,
    openBatchAttachment,
  } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import ConfirmModal from "$lib/components/ConfirmModal.svelte";

  let { batch, onAttachmentsChange }: { batch: Batch; onAttachmentsChange?: () => void } = $props();

  let attachments = $state<BatchAttachment[]>([]);
  let appDataDir = $state("");
  let adding = $state(false);
  let deleteCandidate = $state<BatchAttachment | null>(null);

  const photos = $derived(
    attachments.filter((a) => a.mime_type?.startsWith("image/")),
  );
  const files = $derived(
    attachments.filter((a) => !a.mime_type?.startsWith("image/")),
  );

  async function load() {
    attachments = (await ipc(listBatchAttachments(batch.id))) ?? [];
  }

  onMount(async () => {
    appDataDir = await getAppDataDir();
    await load();
  });

  function attachmentSrc(a: BatchAttachment): string {
    return convertFileSrc(`${appDataDir}/attachments/${batch.id}/${a.filename}`);
  }

  async function handleAdd() {
    const result = await openDialog({ multiple: true });
    if (!result) return;
    adding = true;
    try {
      const paths = Array.isArray(result) ? result : [result];
      for (const p of paths) {
        const name =
          p.split("/").pop() ?? p.split("\\").pop() ?? "attachment";
        await ipc(addBatchAttachment(batch.id, p, name));
      }
      await load();
      onAttachmentsChange?.();
    } finally {
      adding = false;
    }
  }

  async function handleDelete(id: string) {
    const attachment = attachments.find((a) => a.id === id);
    if (attachment) {
      deleteCandidate = attachment;
    }
  }

  async function confirmDelete() {
    if (!deleteCandidate) return;
    await ipc(deleteBatchAttachment(deleteCandidate.id));
    deleteCandidate = null;
    await load();
    onAttachmentsChange?.();
  }

  async function handleOpen(id: string) {
    await ipc(openBatchAttachment(id));
  }

  function fileIcon(mimeType: string | null | undefined): string {
    if (!mimeType) return "📄";
    if (mimeType === "application/pdf") return "📄";
    if (
      mimeType.includes("spreadsheet") ||
      mimeType.includes("excel") ||
      mimeType.includes("xlsx")
    )
      return "📊";
    return "📄";
  }

  function formatSize(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  }
</script>

<div class="p-4 flex flex-col gap-4">
  <!-- Header row -->
  <div class="flex items-center justify-between">
    <span class="text-xs text-text-muted">
      {attachments.length} attachment{attachments.length === 1 ? "" : "s"}
    </span>
    <button
      onclick={handleAdd}
      disabled={adding}
      class="px-3 py-1.5 rounded text-sm font-medium bg-accent"
      style="color: #fff;"
    >
      {adding ? "Adding…" : "+ Add"}
    </button>
  </div>

  <!-- Photo wall -->
  {#if photos.length > 0}
    <div
      class="photo-grid"
      style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 4px;"
    >
      {#each photos as photo (photo.id)}
        <div
          class="photo-cell bg-bg-elevated"
          role="button"
          tabindex="0"
          style="position: relative; aspect-ratio: 1; overflow: hidden; border-radius: 4px; cursor: pointer;"
          onclick={() => handleOpen(photo.id)}
          onkeydown={(e) => e.key === "Enter" && handleOpen(photo.id)}
        >
          <img
            src={attachmentSrc(photo)}
            alt={photo.original_name}
            onload={(e) => {
              const img = e.currentTarget as HTMLImageElement;
              if (img.naturalHeight > img.naturalWidth * 1.3) {
                (img.closest(".photo-cell") as HTMLElement).style.gridRow =
                  "span 2";
                (img.closest(".photo-cell") as HTMLElement).style.aspectRatio =
                  "auto";
              }
            }}
            style="width: 100%; height: 100%; object-fit: cover; display: block;"
          />
          <button
            onclick={(e) => { e.stopPropagation(); handleDelete(photo.id); }}
            aria-label="Delete {photo.original_name}"
            class="delete-btn"
            style="
              position: absolute; top: 4px; right: 4px;
              width: 22px; height: 22px;
              background: rgba(0,0,0,0.6); color: #fff;
              border-radius: 50%; border: none; cursor: pointer;
              font-size: 12px; display: flex; align-items: center; justify-content: center;
            "
          >×</button>
        </div>
      {/each}
    </div>
  {/if}

  <!-- File list -->
  {#if files.length > 0}
    <div class="flex flex-col gap-1">
      {#each files as file (file.id)}
        <div
          class="flex items-center gap-2 px-3 py-2 rounded cursor-pointer bg-bg-elevated border border-border"
         
          role="button"
          tabindex="0"
          onclick={() => handleOpen(file.id)}
          onkeydown={(e) => e.key === "Enter" && handleOpen(file.id)}
        >
          <span class="text-base leading-none">{fileIcon(file.mime_type)}</span>
          <span class="flex-1 text-sm truncate text-text-secondary">
            {file.original_name}
          </span>
          <span class="text-xs flex-shrink-0 text-text-muted">
            {formatSize(file.size_bytes)}
          </span>
          <button
            onclick={(e) => { e.stopPropagation(); handleDelete(file.id); }}
            aria-label="Delete {file.original_name}"
            class="text-sm flex-shrink-0 text-text-muted"
           
          >×</button>
        </div>
      {/each}
    </div>
  {/if}

  {#if attachments.length === 0 && !adding}
    <p class="text-sm text-center py-8 text-text-muted">
      No attachments yet. Add photos, PDFs, or any files.
    </p>
  {/if}
</div>

{#if deleteCandidate}
  <ConfirmModal
    message={`Delete "${deleteCandidate.original_name}"? This cannot be undone.`}
    confirmLabel="Delete"
    dangerous={true}
    onconfirm={confirmDelete}
    oncancel={() => { deleteCandidate = null; }}
  />
{/if}

<style>
  @media (max-width: 640px) {
    .photo-grid {
      grid-template-columns: repeat(2, 1fr) !important;
    }
    .delete-btn {
      width: 28px !important;
      height: 28px !important;
      font-size: 14px !important;
    }
  }
</style>
