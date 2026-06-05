# Notes Image Insertion Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a 🖼 toolbar button to the batch notes Markdown editor that opens a photo picker showing existing batch attachments; selecting one inserts `![name](attachment://id)` at the cursor, and the Preview tab resolves those references to real images.

**Architecture:** `BatchOverviewTab` loads image attachments on mount via `listBatchAttachments`, resolves each to a Tauri asset URL with `convertFileSrc`, and passes them as `ImageRef[]` to `MarkdownEditor`. The editor shows a 🖼 toolbar button only when the `images` prop is provided, and post-processes rendered preview HTML to swap `attachment://id` URLs for resolved asset URLs. A new `ImagePickerModal` component handles the selection dialog. No backend changes are needed — the path construction pattern (`${appDataDir}/attachments/${batchId}/${filename}`) already exists in `BatchAttachmentsTab`, and `tauri.conf.json` already scopes the asset protocol to `$APPDATA/attachments/**`.

**Tech Stack:** Svelte 5 (runes), TypeScript, Vitest, `@testing-library/svelte`, `@tauri-apps/api/core` (`convertFileSrc`), `@tauri-apps/api/path` (`appDataDir`)

---

## Files

| Action | Path | Responsibility |
|---|---|---|
| Modify | `src/lib/api.ts` | Add `ImageRef` interface |
| Create | `src/lib/components/ImagePickerModal.svelte` | Photo grid selection modal |
| Modify | `src/lib/components/MarkdownEditor.svelte` | `images` prop, 🖼 button, modal, preview resolution |
| Modify | `src/lib/components/batch/BatchOverviewTab.svelte` | Load attachments, resolve to `ImageRef[]`, pass to editor |
| Create | `tests/ImagePickerModal.test.ts` | Tests for the modal |
| Create | `tests/MarkdownEditor.test.ts` | Tests for new toolbar and preview behaviour |

---

### Task 1: Add `ImageRef` type

**Files:**
- Modify: `src/lib/api.ts`

- [ ] **Step 1: Add the interface**

Open `src/lib/api.ts`. After the `export type BatchAttachment = ...` line (line 60), insert:

```typescript
export interface ImageRef {
  id: string;
  name: string;
  assetUrl: string;
}
```

- [ ] **Step 2: Verify compilation**

```bash
npm run check
```
Expected: no TypeScript errors.

- [ ] **Step 3: Commit**

```bash
git add src/lib/api.ts
git commit -m "feat: add ImageRef interface"
```

---

### Task 2: `ImagePickerModal.svelte`

**Files:**
- Create: `src/lib/components/ImagePickerModal.svelte`
- Create: `tests/ImagePickerModal.test.ts`

- [ ] **Step 1: Write the failing tests**

Create `tests/ImagePickerModal.test.ts`:

```typescript
import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import ImagePickerModal from '$lib/components/ImagePickerModal.svelte';
import type { ImageRef } from '$lib/api';

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn(), convertFileSrc: vi.fn((p) => p) }));

const images: ImageRef[] = [
  { id: 'id-1', name: 'brew-day.jpg', assetUrl: 'asset://brew-day.jpg' },
  { id: 'id-2', name: 'grain-bill.jpg', assetUrl: 'asset://grain-bill.jpg' },
];

describe('ImagePickerModal', () => {
  it('renders a thumbnail for each image', () => {
    render(ImagePickerModal, { images, onInsert: vi.fn(), onClose: vi.fn() });
    expect(screen.getByAltText('brew-day.jpg')).toBeTruthy();
    expect(screen.getByAltText('grain-bill.jpg')).toBeTruthy();
  });

  it('Insert button is disabled until a photo is selected', () => {
    render(ImagePickerModal, { images, onInsert: vi.fn(), onClose: vi.fn() });
    expect(screen.getByRole('button', { name: 'Insert' })).toBeDisabled();
  });

  it('calls onInsert with the selected image then closes', async () => {
    const user = userEvent.setup();
    const onInsert = vi.fn();
    const onClose = vi.fn();
    render(ImagePickerModal, { images, onInsert, onClose });
    await user.click(screen.getByAltText('brew-day.jpg'));
    await user.click(screen.getByRole('button', { name: 'Insert' }));
    expect(onInsert).toHaveBeenCalledWith(images[0]);
    expect(onClose).toHaveBeenCalled();
  });

  it('calls onClose when Cancel is clicked', async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    render(ImagePickerModal, { images, onInsert: vi.fn(), onClose });
    await user.click(screen.getByRole('button', { name: 'Cancel' }));
    expect(onClose).toHaveBeenCalled();
  });

  it('shows empty state and no Insert button when images is empty', () => {
    render(ImagePickerModal, { images: [], onInsert: vi.fn(), onClose: vi.fn() });
    expect(screen.getByText(/No photos yet/)).toBeTruthy();
    expect(screen.queryByRole('button', { name: 'Insert' })).toBeNull();
  });
});
```

- [ ] **Step 2: Run tests to confirm they fail**

```bash
npm test -- --reporter=verbose tests/ImagePickerModal.test.ts
```
Expected: FAIL — component file not found.

- [ ] **Step 3: Create `ImagePickerModal.svelte`**

Create `src/lib/components/ImagePickerModal.svelte`:

```svelte
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
```

- [ ] **Step 4: Run tests to confirm they pass**

```bash
npm test -- --reporter=verbose tests/ImagePickerModal.test.ts
```
Expected: 5 tests pass.

- [ ] **Step 5: Commit**

```bash
git add src/lib/components/ImagePickerModal.svelte tests/ImagePickerModal.test.ts
git commit -m "feat: add ImagePickerModal component"
```

---

### Task 3: Update `MarkdownEditor.svelte`

**Files:**
- Modify: `src/lib/components/MarkdownEditor.svelte`
- Create: `tests/MarkdownEditor.test.ts`

- [ ] **Step 1: Write the failing tests**

Create `tests/MarkdownEditor.test.ts`:

```typescript
import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { tick } from 'svelte';
import MarkdownEditor from '$lib/components/MarkdownEditor.svelte';
import type { ImageRef } from '$lib/api';

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn(), convertFileSrc: vi.fn((p) => p) }));

const images: ImageRef[] = [
  { id: 'abc-123', name: 'brew.jpg', assetUrl: 'asset://brew.jpg' },
];

describe('MarkdownEditor image toolbar', () => {
  it('does not show the image button when images prop is absent', () => {
    render(MarkdownEditor, { value: '', onchange: vi.fn() });
    expect(screen.queryByTitle('Insert image')).toBeNull();
  });

  it('shows the image button when images prop is provided', () => {
    render(MarkdownEditor, { value: '', onchange: vi.fn(), images });
    expect(screen.getByTitle('Insert image')).toBeTruthy();
  });
});

describe('MarkdownEditor preview image resolution', () => {
  it('resolves attachment:// src to the matching assetUrl', async () => {
    const user = userEvent.setup();
    const { container } = render(MarkdownEditor, {
      value: '![brew.jpg](attachment://abc-123)',
      onchange: vi.fn(),
      images,
    });
    await user.click(screen.getByRole('button', { name: 'Preview' }));
    await tick();
    const img = container.querySelector('img');
    expect(img?.getAttribute('src')).toBe('asset://brew.jpg');
  });

  it('sets src to empty string when attachment id is not in images', async () => {
    const user = userEvent.setup();
    const { container } = render(MarkdownEditor, {
      value: '![missing.jpg](attachment://unknown-id)',
      onchange: vi.fn(),
      images,
    });
    await user.click(screen.getByRole('button', { name: 'Preview' }));
    await tick();
    const img = container.querySelector('img');
    expect(img?.getAttribute('src')).toBe('');
  });
});
```

- [ ] **Step 2: Run tests to confirm they fail**

```bash
npm test -- --reporter=verbose tests/MarkdownEditor.test.ts
```
Expected: FAIL — `images` prop not accepted, no image button, `attachment://` not resolved.

- [ ] **Step 3: Replace `MarkdownEditor.svelte` with the updated version**

Replace the entire file at `src/lib/components/MarkdownEditor.svelte`:

```svelte
<!-- src/lib/components/MarkdownEditor.svelte -->
<script lang="ts">
  import MarkdownIt from 'markdown-it';
  import taskLists from 'markdown-it-task-lists';
  import { untrack } from 'svelte';
  import { wrapSelection, insertLinePrefix, insertBlock } from '$lib/markdown';
  import type { ImageRef } from '$lib/api';
  import ImagePickerModal from '$lib/components/ImagePickerModal.svelte';

  const md = new MarkdownIt({ linkify: true, breaks: true }).use(taskLists);

  let {
    value,
    onchange,
    placeholder,
    rows = 6,
    id,
    images,
  }: {
    value: string | null;
    onchange: (value: string | null) => void;
    placeholder?: string;
    rows?: number;
    id?: string;
    images?: ImageRef[];
  } = $props();

  let activeTab: 'write' | 'preview' = $state('write');
  let textarea: HTMLTextAreaElement | undefined = $state();
  let pickerOpen = $state(false);

  let draft = $state(untrack(() => value ?? ''));
  $effect(() => {
    draft = value ?? '';
  });

  function applyEdit(result: { value: string; selStart: number; selEnd: number }) {
    if (!textarea) return;
    draft = result.value;
    textarea.value = result.value;
    textarea.setSelectionRange(result.selStart, result.selEnd);
    textarea.focus();
    onchange(draft || null);
  }

  function insertImage(image: ImageRef) {
    if (!textarea) return;
    applyEdit(insertBlock(
      textarea.value,
      textarea.selectionStart,
      `![${image.name}](attachment://${image.id})`,
    ));
  }

  function renderPreview(text: string): string {
    const html = md.render(text);
    if (!images?.length) return html;
    const map = new Map(images.map((img) => [img.id, img.assetUrl]));
    return html.replace(/src="attachment:\/\/([^"]+)"/g, (_, imgId) => {
      return `src="${map.get(imgId) ?? ''}"`;
    });
  }
</script>

<div class="markdown-editor" style="border: 1px solid var(--color-border); border-radius: 4px; overflow: hidden;">
  <div style="display: flex; align-items: center; border-bottom: 1px solid var(--color-border); height: 32px; background: var(--color-bg-elevated);">
    <button
      type="button"
      onclick={() => activeTab = 'write'}
      class="tab-btn"
      class:active={activeTab === 'write'}
    >Write</button>
    <button
      type="button"
      onclick={() => activeTab = 'preview'}
      class="tab-btn"
      class:active={activeTab === 'preview'}
    >Preview</button>

    {#if activeTab === 'write'}
      <div style="flex: 1;"></div>
      <div class="toolbar">
        <button type="button"
          onmousedown={(e) => e.preventDefault()}
          onclick={() => textarea && applyEdit(wrapSelection(textarea.value, textarea.selectionStart, textarea.selectionEnd, '**', '**'))}
          class="toolbar-btn" style="font-weight: 700;" title="Bold">B</button>
        <button type="button"
          onmousedown={(e) => e.preventDefault()}
          onclick={() => textarea && applyEdit(wrapSelection(textarea.value, textarea.selectionStart, textarea.selectionEnd, '*', '*'))}
          class="toolbar-btn" style="font-style: italic;" title="Italic">I</button>
        <button type="button"
          onmousedown={(e) => e.preventDefault()}
          onclick={() => textarea && applyEdit(insertLinePrefix(textarea.value, textarea.selectionStart, '## '))}
          class="toolbar-btn" title="Heading">H</button>
        <div class="toolbar-sep"></div>
        <button type="button"
          onmousedown={(e) => e.preventDefault()}
          onclick={() => textarea && applyEdit(insertLinePrefix(textarea.value, textarea.selectionStart, '- '))}
          class="toolbar-btn" title="List">≡</button>
        <button type="button"
          onmousedown={(e) => e.preventDefault()}
          onclick={() => textarea && applyEdit(insertLinePrefix(textarea.value, textarea.selectionStart, '- [ ] '))}
          class="toolbar-btn" title="Task list">☑</button>
        <button type="button"
          onmousedown={(e) => e.preventDefault()}
          onclick={() => textarea && applyEdit(insertBlock(textarea.value, textarea.selectionStart, '---'))}
          class="toolbar-btn" title="Rule">—</button>
        {#if images !== undefined}
          <div class="toolbar-sep"></div>
          <button type="button"
            onmousedown={(e) => e.preventDefault()}
            onclick={() => (pickerOpen = true)}
            class="toolbar-btn"
            title="Insert image"
            aria-label="Insert image">🖼</button>
        {/if}
      </div>
    {/if}
  </div>

  {#if activeTab === 'write'}
    <textarea
      bind:this={textarea}
      {id}
      {placeholder}
      {rows}
      value={draft}
      oninput={(e) => { draft = (e.target as HTMLTextAreaElement).value; }}
      onblur={() => onchange(draft || null)}
      class="w-full px-3 py-2 text-sm resize-none outline-none"
      style="background: var(--color-bg-elevated); color: var(--color-text-primary); font-family: monospace; display: block; box-sizing: border-box;"
    ></textarea>
  {:else}
    <div
      class="md-preview px-3 py-2 text-sm"
      style="background: var(--color-bg-elevated); color: var(--color-text-primary); min-height: {rows * 1.6}em;"
    >
      {#if draft}
        {@html renderPreview(draft)}
      {:else}
        <span style="color: var(--color-text-secondary);">{placeholder ?? ''}</span>
      {/if}
    </div>
  {/if}
</div>

{#if pickerOpen && images !== undefined}
  <ImagePickerModal
    {images}
    onInsert={insertImage}
    onClose={() => (pickerOpen = false)}
  />
{/if}

<style>
  .tab-btn {
    padding: 0 12px;
    height: 100%;
    font-size: 11px;
    background: transparent;
    border: none;
    border-right: 1px solid var(--color-border);
    border-bottom: 2px solid transparent;
    color: var(--color-text-secondary);
    cursor: pointer;
  }
  .tab-btn.active {
    color: var(--color-text-primary);
    font-weight: 600;
    background: var(--color-bg-surface);
    border-bottom-color: var(--color-accent);
  }
  .toolbar {
    display: flex;
    align-items: center;
    gap: 2px;
    padding: 0 8px;
  }
  .toolbar-btn {
    background: var(--color-bg-surface);
    border: 1px solid var(--color-border);
    color: var(--color-text-primary);
    width: 22px;
    height: 22px;
    border-radius: 3px;
    font-size: 11px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .toolbar-btn:hover { background: var(--color-bg-elevated); }
  .toolbar-sep {
    width: 1px;
    height: 14px;
    background: var(--color-border);
    margin: 0 2px;
  }

  :global(.md-preview h1) { font-size: 1.25em; font-weight: 700; margin: 0.75em 0 0.25em; }
  :global(.md-preview h2) { font-size: 1.1em; font-weight: 700; margin: 0.75em 0 0.25em; }
  :global(.md-preview h3) { font-size: 1em; font-weight: 700; margin: 0.5em 0 0.2em; }
  :global(.md-preview p) { margin: 0.5em 0; }
  :global(.md-preview ul),
  :global(.md-preview ol) { margin: 0.5em 0; padding-left: 1.5em; }
  :global(.md-preview li) { margin: 0.2em 0; }
  :global(.md-preview li.task-list-item) {
    list-style: none;
    margin-left: -1.5em;
    display: flex;
    align-items: center;
    gap: 0.5em;
  }
  :global(.md-preview hr) { border: none; border-top: 1px solid var(--color-border); margin: 0.75em 0; }
  :global(.md-preview a) { color: var(--color-accent); text-decoration: underline; }
  :global(.md-preview code) {
    background: var(--color-bg-surface);
    padding: 0.1em 0.3em;
    border-radius: 3px;
    font-family: monospace;
    font-size: 0.9em;
  }
  :global(.md-preview strong) { font-weight: 700; }
  :global(.md-preview em) { font-style: italic; }
  :global(.md-preview blockquote) {
    border-left: 3px solid var(--color-border);
    margin: 0.5em 0;
    padding-left: 1em;
    color: var(--color-text-secondary);
  }
  :global(.md-preview img) {
    max-width: 100%;
    height: auto;
    border-radius: 4px;
    margin: 8px 0;
    display: block;
  }
</style>
```

- [ ] **Step 4: Run tests to confirm they pass**

```bash
npm test -- --reporter=verbose tests/MarkdownEditor.test.ts
```
Expected: 4 tests pass.

- [ ] **Step 5: Commit**

```bash
git add src/lib/components/MarkdownEditor.svelte tests/MarkdownEditor.test.ts
git commit -m "feat: add image insertion and preview resolution to MarkdownEditor"
```

---

### Task 4: Update `BatchOverviewTab.svelte`

**Files:**
- Modify: `src/lib/components/batch/BatchOverviewTab.svelte`

- [ ] **Step 1: Add imports**

In the `<script>` block, add after the existing imports (after line 15):

```typescript
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { appDataDir as getAppDataDir } from "@tauri-apps/api/path";
  import type { ImageRef } from "$lib/api";
  import { listBatchAttachments } from "$lib/api";
```

- [ ] **Step 2: Add `imageRefs` state**

After `let recipe = $state<Recipe | null>(null);` (line 26), add:

```typescript
  let imageRefs = $state<ImageRef[]>([]);
```

- [ ] **Step 3: Extend `onMount` to load image attachments**

Replace the existing `onMount` block (lines 43–54) with:

```typescript
  onMount(async () => {
    const [versions, fetchedRecipe, attachments, appDataDir] = await Promise.all([
      ipc(listRecipeVersions(batch.recipe_id)),
      ipc(getRecipe(batch.recipe_id)),
      ipc(listBatchAttachments(batch.id)),
      getAppDataDir(),
    ]);
    if (versions) {
      batchVersion = versions.find((v) => v.id === batch.recipe_version_id) ?? null;
    }
    if (fetchedRecipe) {
      recipe = fetchedRecipe;
    }
    if (attachments) {
      imageRefs = attachments
        .filter((a) => a.mime_type?.startsWith('image/'))
        .map((a) => ({
          id: a.id,
          name: a.original_name,
          assetUrl: convertFileSrc(`${appDataDir}/attachments/${batch.id}/${a.filename}`),
        }));
    }
  });
```

- [ ] **Step 4: Pass `images` to `MarkdownEditor`**

In the Notes section (around line 291), update `MarkdownEditor` to pass `images`:

```svelte
    <MarkdownEditor
      value={batch.notes ?? null}
      onchange={(v) => onUpdate({ notes: v })}
      rows={4}
      placeholder="Brew day observations, gravity readings, anything worth remembering…"
      images={imageRefs}
    />
```

- [ ] **Step 5: Run the full test suite**

```bash
npm test
```
Expected: all tests pass, no regressions.

- [ ] **Step 6: Verify manually in the running app**

```bash
npm run tauri dev
```

Test the following:

1. **Batch with photos** — open a batch that has images in its Attachments tab. Go to Overview. The notes editor shows a 🖼 button in the toolbar.
2. **Open picker** — click 🖼. The "Insert image" modal opens showing photo thumbnails. The Insert button is disabled.
3. **Select and insert** — click a photo (border highlights). Click Insert. The modal closes and `![name](attachment://id)` appears in the editor at the cursor.
4. **Preview** — switch to Preview tab. The image renders inline at full container width.
5. **Batch with no photos** — open a batch with no attachments. The 🖼 button still appears (the prop is `[]`, not `undefined`). Clicking it shows "No photos yet. Add photos in the Attachments tab first." with no Insert button.
6. **Mobile** — resize the window to a narrow width. The photo grid switches to 2 columns.

- [ ] **Step 7: Commit**

```bash
git add src/lib/components/batch/BatchOverviewTab.svelte
git commit -m "feat: wire image attachments from BatchOverviewTab into MarkdownEditor"
```
