<!-- src/lib/components/ImagePickerModal.svelte -->
<script lang="ts">
  import type { ImageRef } from '$lib/api';

  let {
    images,
    onInsert,
    onClose,
  }: {
    images: ImageRef[];
    onInsert: (image: ImageRef) => void;
    onClose: () => void;
  } = $props();

  let selected = $state<ImageRef | null>(null);

  function handleInsert() {
    if (!selected) return;
    onInsert(selected);
    onClose();
  }
</script>

<div
  class="backdrop"
  role="presentation"
  onclick={(e) => e.target === e.currentTarget && onClose()}
  onkeydown={(e) => e.key === 'Escape' && onClose()}
>
  <div class="modal" role="dialog" aria-modal="true" aria-label="Insert image">
    <div class="modal-header">
      <span class="modal-title">Insert image</span>
      <button type="button" onclick={onClose} class="close-btn" aria-label="Close">✕</button>
    </div>

    {#if images.length === 0}
      <p class="empty-state">No photos yet. Add photos in the Attachments tab first.</p>
    {:else}
      <div class="photo-grid">
        {#each images as image}
          <button
            type="button"
            class="photo-cell"
            class:selected={selected?.id === image.id}
            onclick={() => (selected = image)}
            aria-label={image.name}
          >
            <img src={image.assetUrl} alt={image.name} class="photo-thumb" />
            <span class="photo-name">{image.name}</span>
          </button>
        {/each}
      </div>
    {/if}

    <div class="modal-footer">
      <button type="button" onclick={onClose} class="btn-cancel">Cancel</button>
      {#if images.length > 0}
        <button
          type="button"
          onclick={handleInsert}
          class="btn-insert"
          disabled={!selected}
        >Insert</button>
      {/if}
    </div>
  </div>
</div>

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 50;
  }
  .modal {
    background: var(--color-bg-elevated);
    border: 1px solid var(--color-border);
    border-radius: 8px;
    padding: 16px;
    width: min(480px, 90vw);
    max-height: 80vh;
    overflow-y: auto;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
  }
  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
  }
  .modal-title {
    font-size: 14px;
    font-weight: 600;
    color: var(--color-text-primary);
  }
  .close-btn {
    background: none;
    border: none;
    color: var(--color-text-muted);
    font-size: 16px;
    cursor: pointer;
    padding: 2px 6px;
    line-height: 1;
  }
  .close-btn:hover { color: var(--color-text-primary); }
  .photo-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 8px;
    margin-bottom: 12px;
  }
  @media (max-width: 640px) {
    .photo-grid { grid-template-columns: repeat(2, 1fr); }
  }
  .photo-cell {
    background: var(--color-bg-surface);
    border: 2px solid transparent;
    border-radius: 6px;
    overflow: hidden;
    cursor: pointer;
    padding: 0;
    text-align: left;
    min-height: 44px;
    width: 100%;
  }
  .photo-cell:hover { border-color: var(--color-border); }
  .photo-cell.selected { border-color: var(--color-accent); }
  .photo-thumb {
    width: 100%;
    height: 72px;
    object-fit: cover;
    display: block;
  }
  .photo-name {
    display: block;
    padding: 3px 6px;
    font-size: 10px;
    color: var(--color-text-muted);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .photo-cell.selected .photo-name { color: var(--color-text-primary); }
  .empty-state {
    text-align: center;
    padding: 24px 0;
    font-size: 13px;
    color: var(--color-text-muted);
    margin-bottom: 12px;
  }
  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    margin-top: 4px;
  }
  .btn-cancel {
    background: var(--color-bg-surface);
    border: 1px solid var(--color-border);
    border-radius: 4px;
    padding: 6px 14px;
    font-size: 12px;
    color: var(--color-text-secondary);
    cursor: pointer;
  }
  .btn-cancel:hover { color: var(--color-text-primary); }
  .btn-insert {
    background: var(--color-accent);
    border: none;
    border-radius: 4px;
    padding: 6px 14px;
    font-size: 12px;
    color: #fff;
    font-weight: 500;
    cursor: pointer;
  }
  .btn-insert:disabled { opacity: 0.4; cursor: default; }
</style>
