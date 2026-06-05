# Notes Image Insertion — Design Spec
_2026-06-05_

## Overview

Allow images to be inserted inline into batch notes. A 🖼 toolbar button opens a modal image picker showing photos already in the batch's Attachments tab. Selecting one inserts a standard Markdown image reference at the cursor. The Preview tab resolves those references to real images.

Scope: **batch notes only**. Recipe notes have no attachment storage and are out of scope.

---

## Architecture

Four moving parts:

1. **Backend** — `BatchAttachment` response gains a `path: String` field (the full on-disk file path).
2. **`BatchOverviewTab`** — loads image attachments alongside batch data, resolves each to a Tauri `asset://` URL via `convertFileSrc(path)`, and passes the result as `images: ImageRef[]` to `MarkdownEditor`.
3. **`MarkdownEditor`** — gains an optional `images?: ImageRef[]` prop. When provided, the 🖼 button appears in the toolbar and opens `ImagePickerModal`.
4. **`ImagePickerModal`** — new standalone component. Renders the selection grid, emits the chosen attachment on confirm.

---

## Data Model

### New type: `ImageRef`

```typescript
interface ImageRef {
  id: string;        // batch_attachment.id (UUID)
  name: string;      // original_name for display and alt text
  assetUrl: string;  // convertFileSrc(path) — loadable in webview
}
```

### Backend change: `BatchAttachment`

Add `path: String` to the Rust struct and serialised response. Path is the full on-disk location: `{appDataDir}/attachments/{batch_id}/{filename}`.

No migration needed — this is a computed field added to the API response only.

---

## Components

### `MarkdownEditor.svelte`

New prop:
```typescript
images?: ImageRef[]   // undefined = no image button; [] = button shown, empty picker
```

Changes:
- Toolbar gains a 🖼 button after the `—` divider, rendered only when `images !== undefined`.
- Clicking the button sets `pickerOpen = true`.
- On insert, writes `![name](attachment://id)` at the cursor position (using the existing `insertBlock` / cursor utilities in `markdown.ts`).
- Preview rendering: after `markdown-it` produces HTML, replace every `src="attachment://id"` with the corresponding `assetUrl` from the `images` map. Images that have no match (deleted attachment) render as broken `<img>` — no special handling needed.
- Add `:global(.prose img) { max-width: 100%; height: auto; border-radius: 4px; margin: 8px 0; }` to the scoped preview styles.

### `ImagePickerModal.svelte` (new)

Props:
```typescript
images: ImageRef[]
onInsert: (image: ImageRef) => void
onClose: () => void
```

Behaviour:
- Centered modal overlay, same presentation on desktop and mobile.
- Photo grid: `grid-template-columns: repeat(3, 1fr)` on desktop, `repeat(2, 1fr)` on mobile (≤ 640 px).
- Each cell shows the image thumbnail (using `assetUrl`) with `original_name` below, truncated.
- Single selection — clicking a cell highlights it with an accent border.
- Insert button enabled only when a selection is made; calls `onInsert(selected)` and closes.
- Cancel / backdrop click calls `onClose()`.
- Empty state: "No photos yet. Add photos in the Attachments tab first." — no Insert button.

### `BatchOverviewTab.svelte`

On load (alongside existing batch fetch):
1. Call `listBatchAttachments(batch.id)`.
2. Filter to image attachments: `mime_type?.startsWith('image/')`.
3. For each, call `convertFileSrc(attachment.path)` to produce `assetUrl`.
4. Pass the resulting `ImageRef[]` as `images` prop to `MarkdownEditor`.
5. Images are fetched once on mount. New photos added in the Attachments tab during the same session will appear in the picker after the user navigates away and back to the Overview tab (acceptable V1 behaviour).

---

## Markdown Storage Format

```
![original_name](attachment://uuid)
```

- `uuid` is `batch_attachments.id` — stable, doesn't change if the file is renamed.
- `original_name` is used as the `alt` attribute.
- Standard Markdown — renders as a broken image in any other Markdown renderer (acceptable; this is a local app).

---

## Tauri Asset Protocol

`convertFileSrc()` requires the `asset` protocol scope in `tauri.conf.json` to include the attachments directory. Verify (or add) a scope entry for `$APPDATA/attachments/**`.

---

## Mobile

- Toolbar: 🖼 icon button, no text label — fits in a single row alongside the existing buttons on narrow screens.
- Modal: same centered design as desktop; photo grid switches to 2 columns at ≤ 640 px.
- Tap targets in the grid: minimum `44 × 44 px` touch area.

---

## Edge Cases

| Scenario | Behaviour |
|---|---|
| No images in attachments | Picker shows empty state message; Insert disabled |
| Attachment deleted after insertion | Preview shows broken image (browser default) |
| Non-image attachments (PDF, etc.) | Excluded from picker — images only |
| Multiple insertions | Each click of 🖼 inserts independently at current cursor |
