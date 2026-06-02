# Markdown Editor Design

**Date:** 2026-06-02
**Feature:** Markdown support for all notes/text fields across the app

## Overview

Replace every plain `<textarea>` in Brewski with a reusable `MarkdownEditor` component that provides a Write/Preview tab toggle and a formatting toolbar. Markdown is rendered using `markdown-it` with task list checkbox support.

## UX

**Editing mode:** Tab toggle (Write / Preview). Write tab shows the raw markdown textarea; Preview tab shows rendered output. Both tabs are always reachable with a single click.

**Toolbar:** Visible on the Write tab only. 6 buttons in two groups:

- Group 1: **Bold** (wraps selection in `**`), *Italic* (wraps in `*`), **H** (inserts `## ` at line start)
- Group 2: List (inserts `- `), Task (inserts `- [ ] `), Rule (inserts `---`)

Each toolbar action inserts syntax at the cursor position using `selectionStart`/`selectionEnd`, then restores textarea focus. If text is selected, Bold and Italic wrap the selection; others insert at the start of the selection.

## Component API

**File:** `src/lib/components/MarkdownEditor.svelte`

```ts
let {
  value,        // string | null
  onchange,     // (value: string | null) => void — called on blur
  placeholder,  // string | undefined
  rows = 6,     // number — height hint
  id,           // string | undefined — for label association
}: { ... } = $props();
```

Internal state: `let activeTab: 'write' | 'preview' = $state('write')`.

The Write tab renders a `<textarea>` with the raw markdown. The Preview tab renders `{@html md.render(value ?? '')}` inside a styled `<div class="prose">`. `onchange` is called `onblur` with the new value, or `null` if the value is empty.

## Dependencies

Two new npm packages:

- `markdown-it` — markdown renderer, configured with `{ linkify: true, breaks: true }`
- `markdown-it-task-lists` — plugin for `- [ ]` / `- [x]` syntax

A single `md` instance is created at module scope and reused across all component instances:

```ts
import MarkdownIt from 'markdown-it';
import taskLists from 'markdown-it-task-lists';

const md = new MarkdownIt({ linkify: true, breaks: true }).use(taskLists);
```

Task list checkboxes render as `<input type="checkbox" disabled>` in the Preview — read-only. The raw markdown string is always the source of truth.

No HTML sanitization library is needed: this is a local Tauri app rendering the user's own notes with no external input.

## Prose Styles

Scoped CSS inside `MarkdownEditor.svelte` styles the `.prose` preview container to match Brewski's theme using CSS custom properties:

- Headings (`h1`–`h3`): larger font size, `var(--color-text-primary)`
- `strong`, `em`: standard weight/style
- `ul`, `ol`: indented, with appropriate list markers
- Task list `li`: `list-style: none`, flex row with checkbox and label
- `hr`: `var(--color-border)`
- `a`: `var(--color-accent)` with underline

Styles are scoped so they do not affect anything outside the component.

## Rollout

8 textareas replaced across 3 files:

| File | Fields | rows |
|------|--------|------|
| `src/lib/components/tabs/NotesTab.svelte` | Recipe Notes | 8 |
| `src/lib/components/tabs/NotesTab.svelte` | Taste Notes | 4 |
| `src/lib/components/batch/BatchOverviewTab.svelte` | Batch Notes | 4 |
| `src/lib/components/ingredients/IngredientEditModal.svelte` | Hop Notes | 3 |
| `src/lib/components/ingredients/IngredientEditModal.svelte` | Fermentable Notes | 3 |
| `src/lib/components/ingredients/IngredientEditModal.svelte` | Yeast Notes | 3 |
| `src/lib/components/ingredients/IngredientEditModal.svelte` | Misc Notes | 3 |
| `src/lib/components/ingredients/IngredientEditModal.svelte` | Water Notes | 2 |

Each replacement is a direct prop-for-prop substitution: `value`, `onchange` (mapped from the existing `onblur` pattern), `placeholder`, `rows`, and `id` where present.

## Data Model

No database changes required. Notes fields are already stored as plain text strings (`TEXT NULL`). Markdown is plain text — no migration needed.

## Testing

- Unit test the toolbar insert functions with various cursor positions (start, mid-selection, end)
- Unit test that `onchange` fires with `null` when the field is cleared
- Verify Preview renders headings, bold, italic, lists, task checkboxes, and `<hr>` correctly
- Smoke test each of the 8 callsites: type content, blur, reload, confirm value persists
