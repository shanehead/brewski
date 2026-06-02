# Markdown Editor Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace all textarea fields in Brewski with a reusable `MarkdownEditor` component featuring Write/Preview tabs, a 6-button formatting toolbar, and task list checkbox support via `markdown-it`.

**Architecture:** A single `MarkdownEditor.svelte` component manages a local `draft` state variable (synced from the `value` prop via `$effect`) so the Preview tab always reflects the latest typed content without requiring a blur. Pure text-insertion helpers live in `src/lib/markdown.ts` so they can be unit-tested without the DOM. The toolbar reads cursor position from `textarea.selectionStart/End`, applies the helper, then writes directly to the textarea DOM and updates `draft` state.

**Tech Stack:** markdown-it, markdown-it-task-lists, @testing-library/svelte, vitest, Svelte 5

---

## File Map

| File | Action | Purpose |
|------|--------|---------|
| `src/lib/markdown.ts` | Create | Pure insert helpers: `wrapSelection`, `insertLinePrefix`, `insertBlock` |
| `src/lib/components/MarkdownEditor.svelte` | Create | Write/Preview tab component with 6-button toolbar and prose styles |
| `tests/markdown.test.ts` | Create | Unit tests for insert helpers |
| `tests/MarkdownEditor.test.ts` | Create | Component tests: tab switching, onchange, task list rendering |
| `src/lib/components/tabs/NotesTab.svelte` | Modify | Replace 2 textareas (Recipe Notes, Taste Notes) |
| `src/lib/components/batch/BatchOverviewTab.svelte` | Modify | Replace 1 textarea (Batch Notes) |
| `src/lib/components/ingredients/IngredientEditModal.svelte` | Modify | Replace 5 textareas (hop, fermentable, yeast, misc, water notes) |

---

## Task 1: Install dependencies

**Files:**
- Modify: `package.json` (via npm)

- [ ] **Step 1: Install the two npm packages**

```bash
npm install markdown-it markdown-it-task-lists
```

Expected: both appear under `"dependencies"` in `package.json`.

- [ ] **Step 2: Check for missing TypeScript types**

```bash
npx tsc --noEmit 2>&1 | grep -i "markdown-it-task-lists"
```

If you see `Could not find a declaration file for module 'markdown-it-task-lists'`, create this file:

```typescript
// src/lib/markdown-it-task-lists.d.ts
import type MarkdownIt from 'markdown-it';
declare module 'markdown-it-task-lists' {
  export default function taskLists(
    md: MarkdownIt,
    options?: { enabled?: boolean }
  ): void;
}
```

- [ ] **Step 3: Commit**

```bash
git add package.json package-lock.json
git commit -m "chore: add markdown-it and markdown-it-task-lists"
```

---

## Task 2: Write failing tests for insert helpers

**Files:**
- Create: `tests/markdown.test.ts`

- [ ] **Step 1: Create the test file**

```typescript
// tests/markdown.test.ts
import { describe, it, expect } from 'vitest';
import { wrapSelection, insertLinePrefix, insertBlock } from '$lib/markdown';

describe('wrapSelection', () => {
  it('wraps selected text in before/after markers', () => {
    const result = wrapSelection('hello world', 6, 11, '**', '**');
    expect(result.value).toBe('hello **world**');
    expect(result.selStart).toBe(8);   // after opening **
    expect(result.selEnd).toBe(13);    // before closing **
  });

  it('inserts markers at cursor with no selection', () => {
    const result = wrapSelection('hello', 5, 5, '**', '**');
    expect(result.value).toBe('hello****');
    expect(result.selStart).toBe(7);   // cursor between the **
    expect(result.selEnd).toBe(7);
  });

  it('works at start of string', () => {
    const result = wrapSelection('world', 0, 5, '*', '*');
    expect(result.value).toBe('*world*');
    expect(result.selStart).toBe(1);
    expect(result.selEnd).toBe(6);
  });
});

describe('insertLinePrefix', () => {
  it('inserts prefix at start of the current line', () => {
    // cursor at pos 8, single line — lineStart is 0
    const result = insertLinePrefix('hello world', 8, '## ');
    expect(result.value).toBe('## hello world');
    expect(result.selStart).toBe(3);  // after '## '
    expect(result.selEnd).toBe(3);
  });

  it('inserts prefix at the correct line in multiline text', () => {
    // 'line two' starts at pos 9; cursor is at pos 12 (inside 'line two')
    const text = 'line one\nline two\nline three';
    const result = insertLinePrefix(text, 12, '- ');
    expect(result.value).toBe('line one\n- line two\nline three');
    expect(result.selStart).toBe(11); // 9 (lineStart) + 2 (prefix)
    expect(result.selEnd).toBe(11);
  });

  it('handles cursor at start of file', () => {
    const result = insertLinePrefix('hello', 0, '- ');
    expect(result.value).toBe('- hello');
    expect(result.selStart).toBe(2);
  });
});

describe('insertBlock', () => {
  it('inserts block with a leading newline when not at line start', () => {
    const result = insertBlock('hello', 5, '---');
    expect(result.value).toBe('hello\n---\n');
    expect(result.selStart).toBe(10);
    expect(result.selEnd).toBe(10);
  });

  it('inserts block without extra newline when already at line start', () => {
    const result = insertBlock('hello\n', 6, '---');
    expect(result.value).toBe('hello\n---\n');
    expect(result.selStart).toBe(10);
    expect(result.selEnd).toBe(10);
  });

  it('inserts block without extra newline when string is empty', () => {
    const result = insertBlock('', 0, '---');
    expect(result.value).toBe('---\n');
    expect(result.selStart).toBe(4);
  });
});
```

- [ ] **Step 2: Run the tests and confirm they fail**

```bash
npx vitest run tests/markdown.test.ts
```

Expected: FAIL with `Cannot find module '$lib/markdown'`.

---

## Task 3: Create src/lib/markdown.ts and pass the tests

**Files:**
- Create: `src/lib/markdown.ts`

- [ ] **Step 1: Create the helper module**

```typescript
// src/lib/markdown.ts

export function wrapSelection(
  text: string,
  selStart: number,
  selEnd: number,
  before: string,
  after: string
): { value: string; selStart: number; selEnd: number } {
  const selected = text.slice(selStart, selEnd);
  const value = text.slice(0, selStart) + before + selected + after + text.slice(selEnd);
  return {
    value,
    selStart: selStart + before.length,
    selEnd: selStart + before.length + selected.length,
  };
}

export function insertLinePrefix(
  text: string,
  cursorPos: number,
  prefix: string
): { value: string; selStart: number; selEnd: number } {
  const lineStart = text.lastIndexOf('\n', cursorPos - 1) + 1;
  const value = text.slice(0, lineStart) + prefix + text.slice(lineStart);
  const newPos = lineStart + prefix.length;
  return { value, selStart: newPos, selEnd: newPos };
}

export function insertBlock(
  text: string,
  cursorPos: number,
  block: string
): { value: string; selStart: number; selEnd: number } {
  const before = text.slice(0, cursorPos);
  const after = text.slice(cursorPos);
  const newLine = before.endsWith('\n') || before === '' ? '' : '\n';
  const insertion = newLine + block + '\n';
  const value = before + insertion + after;
  const newPos = cursorPos + insertion.length;
  return { value, selStart: newPos, selEnd: newPos };
}
```

- [ ] **Step 2: Run the tests and confirm they pass**

```bash
npx vitest run tests/markdown.test.ts
```

Expected: 9 tests PASS.

- [ ] **Step 3: Commit**

```bash
git add src/lib/markdown.ts tests/markdown.test.ts
git commit -m "feat: add markdown insert helpers with unit tests"
```

---

## Task 4: Write failing component tests for MarkdownEditor

**Files:**
- Create: `tests/MarkdownEditor.test.ts`

- [ ] **Step 1: Create the test file**

```typescript
// tests/MarkdownEditor.test.ts
import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { tick } from 'svelte';
import MarkdownEditor from '$lib/components/MarkdownEditor.svelte';

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));

describe('MarkdownEditor', () => {
  it('shows a textarea in Write mode by default', () => {
    render(MarkdownEditor, { value: 'hello', onchange: vi.fn() });
    expect(screen.getByRole('textbox')).toBeTruthy();
    expect(screen.getByRole('button', { name: 'Write' })).toBeTruthy();
    expect(screen.getByRole('button', { name: 'Preview' })).toBeTruthy();
  });

  it('switches to Preview and renders markdown as HTML', async () => {
    const user = userEvent.setup();
    render(MarkdownEditor, { value: '**bold text**', onchange: vi.fn() });
    await user.click(screen.getByRole('button', { name: 'Preview' }));
    await tick();
    expect(screen.queryByRole('textbox')).toBeNull();
    const preview = document.querySelector('.md-preview');
    expect(preview?.innerHTML).toContain('<strong>bold text</strong>');
  });

  it('switches back to Write from Preview', async () => {
    const user = userEvent.setup();
    render(MarkdownEditor, { value: 'hello', onchange: vi.fn() });
    await user.click(screen.getByRole('button', { name: 'Preview' }));
    await tick();
    await user.click(screen.getByRole('button', { name: 'Write' }));
    await tick();
    expect(screen.getByRole('textbox')).toBeTruthy();
  });

  it('calls onchange with the typed value on blur', async () => {
    const user = userEvent.setup();
    const onchange = vi.fn();
    render(MarkdownEditor, { value: null, onchange });
    const textarea = screen.getByRole('textbox');
    await user.click(textarea);
    await user.keyboard('my notes');
    await user.tab();
    expect(onchange).toHaveBeenCalledWith('my notes');
  });

  it('calls onchange with null when value is cleared and blurred', async () => {
    const user = userEvent.setup();
    const onchange = vi.fn();
    render(MarkdownEditor, { value: 'existing', onchange });
    const textarea = screen.getByRole('textbox') as HTMLTextAreaElement;
    await user.clear(textarea);
    await user.tab();
    expect(onchange).toHaveBeenCalledWith(null);
  });

  it('renders task list checkboxes in Preview', async () => {
    const user = userEvent.setup();
    render(MarkdownEditor, { value: '- [ ] todo\n- [x] done', onchange: vi.fn() });
    await user.click(screen.getByRole('button', { name: 'Preview' }));
    await tick();
    const checkboxes = document.querySelectorAll('.md-preview input[type="checkbox"]');
    expect(checkboxes.length).toBe(2);
    expect((checkboxes[0] as HTMLInputElement).checked).toBe(false);
    expect((checkboxes[1] as HTMLInputElement).checked).toBe(true);
  });

  it('renders null value as empty textarea with placeholder', () => {
    render(MarkdownEditor, { value: null, onchange: vi.fn(), placeholder: 'Add notes…' });
    const textarea = screen.getByRole('textbox') as HTMLTextAreaElement;
    expect(textarea.value).toBe('');
    expect(textarea.placeholder).toBe('Add notes…');
  });
});
```

- [ ] **Step 2: Run the tests and confirm they fail**

```bash
npx vitest run tests/MarkdownEditor.test.ts
```

Expected: FAIL with `Cannot find module '$lib/components/MarkdownEditor.svelte'`.

---

## Task 5: Create MarkdownEditor.svelte and pass the tests

**Files:**
- Create: `src/lib/components/MarkdownEditor.svelte`

- [ ] **Step 1: Create the component**

```svelte
<!-- src/lib/components/MarkdownEditor.svelte -->
<script lang="ts">
  import MarkdownIt from 'markdown-it';
  import taskLists from 'markdown-it-task-lists';
  import { wrapSelection, insertLinePrefix, insertBlock } from '$lib/markdown';

  const md = new MarkdownIt({ linkify: true, breaks: true }).use(taskLists);

  let {
    value,
    onchange,
    placeholder,
    rows = 6,
    id,
  }: {
    value: string | null;
    onchange: (value: string | null) => void;
    placeholder?: string;
    rows?: number;
    id?: string;
  } = $props();

  let activeTab: 'write' | 'preview' = $state('write');
  let textarea: HTMLTextAreaElement | undefined = $state();

  // Local draft tracks the in-progress content so Preview reflects typed
  // text before the user blurs (which is when onchange fires).
  let draft = $state(value ?? '');
  $effect(() => {
    draft = value ?? '';
  });

  function applyEdit(result: { value: string; selStart: number; selEnd: number }) {
    if (!textarea) return;
    draft = result.value;
    textarea.value = result.value;
    textarea.setSelectionRange(result.selStart, result.selEnd);
    textarea.focus();
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
          onclick={() => textarea && applyEdit(wrapSelection(textarea.value, textarea.selectionStart, textarea.selectionEnd, '**', '**'))}
          class="toolbar-btn" style="font-weight: 700;" title="Bold">B</button>
        <button type="button"
          onclick={() => textarea && applyEdit(wrapSelection(textarea.value, textarea.selectionStart, textarea.selectionEnd, '*', '*'))}
          class="toolbar-btn" style="font-style: italic;" title="Italic">I</button>
        <button type="button"
          onclick={() => textarea && applyEdit(insertLinePrefix(textarea.value, textarea.selectionStart, '## '))}
          class="toolbar-btn" title="Heading">H</button>
        <div class="toolbar-sep"></div>
        <button type="button"
          onclick={() => textarea && applyEdit(insertLinePrefix(textarea.value, textarea.selectionStart, '- '))}
          class="toolbar-btn" title="List">≡</button>
        <button type="button"
          onclick={() => textarea && applyEdit(insertLinePrefix(textarea.value, textarea.selectionStart, '- [ ] '))}
          class="toolbar-btn" title="Task list">☑</button>
        <button type="button"
          onclick={() => textarea && applyEdit(insertBlock(textarea.value, textarea.selectionStart, '---'))}
          class="toolbar-btn" title="Rule">—</button>
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
        {@html md.render(draft)}
      {:else}
        <span style="color: var(--color-text-secondary);">{placeholder ?? ''}</span>
      {/if}
    </div>
  {/if}
</div>

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
  .toolbar-btn:hover {
    background: var(--color-bg-elevated);
  }
  .toolbar-sep {
    width: 1px;
    height: 14px;
    background: var(--color-border);
    margin: 0 2px;
  }

  /* Prose styles for the Preview div. :global is required because {@html}
     content does not receive Svelte's scoped attribute. The .md-preview
     class is unique to this component so leakage risk is minimal. */
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
</style>
```

- [ ] **Step 2: Run the component tests**

```bash
npx vitest run tests/MarkdownEditor.test.ts
```

Expected: 7 tests PASS.

- [ ] **Step 3: Run the full test suite to confirm no regressions**

```bash
npx vitest run
```

Expected: all tests PASS.

- [ ] **Step 4: Commit**

```bash
git add src/lib/components/MarkdownEditor.svelte
git commit -m "feat: add MarkdownEditor component with Write/Preview tabs and toolbar"
```

---

## Task 6: Replace textareas in NotesTab.svelte and BatchOverviewTab.svelte

**Files:**
- Modify: `src/lib/components/tabs/NotesTab.svelte`
- Modify: `src/lib/components/batch/BatchOverviewTab.svelte`

- [ ] **Step 1: Replace the entire content of NotesTab.svelte**

```svelte
<!-- src/lib/components/tabs/NotesTab.svelte -->
<script lang="ts">
  import type { Recipe } from "$lib/api";
  import { updateRecipe } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import MarkdownEditor from "$lib/components/MarkdownEditor.svelte";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  async function save(field: string, value: unknown) {
    await ipc(updateRecipe(recipe.id, { [field]: value } as any));
    onchange();
  }
</script>

<div class="flex flex-col gap-4 max-w-2xl">
  <div class="flex flex-col gap-1">
    <label for="notes-recipe" class="text-sm font-medium" style="color: var(--color-text-secondary);">Recipe Notes</label>
    <MarkdownEditor
      id="notes-recipe"
      value={recipe.notes ?? null}
      onchange={(v) => save("notes", v)}
      rows={8}
      placeholder="Process notes, observations…"
    />
  </div>

  <div class="flex flex-col gap-1">
    <label for="notes-taste" class="text-sm font-medium" style="color: var(--color-text-secondary);">Taste Notes</label>
    <MarkdownEditor
      id="notes-taste"
      value={recipe.taste_notes ?? null}
      onchange={(v) => save("taste_notes", v)}
      rows={4}
      placeholder="Aroma, flavor, appearance, mouthfeel…"
    />
  </div>

  <div class="flex flex-col gap-1">
    <label for="notes-rating" class="text-sm font-medium" style="color: var(--color-text-secondary);">Taste Rating (0–50)</label>
    <input id="notes-rating" type="number" inputmode="decimal" step="1" min="0" max="50"
           value={recipe.taste_rating ?? ""}
           onblur={(e) => {
             const v = (e.target as HTMLInputElement).value;
             save("taste_rating", v ? parseFloat(v) : null);
           }}
           class="w-24 px-2 py-1.5 rounded text-sm"
           style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
  </div>
</div>
```

- [ ] **Step 2: Update BatchOverviewTab.svelte**

Add the import at the top of the `<script>` block in `src/lib/components/batch/BatchOverviewTab.svelte`:

```svelte
import MarkdownEditor from "$lib/components/MarkdownEditor.svelte";
```

Then find the Notes section (around line 262) and replace the textarea:

```svelte
<!-- Remove: -->
<textarea
  value={batch.notes ?? ""}
  onblur={(e) => onUpdate({ notes: e.currentTarget.value || null })}
  placeholder="Brew day observations, gravity readings, anything worth remembering…"
  rows="4"
  class="w-full px-3 py-2 rounded text-sm outline-none resize-y"
  style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border); font-family: inherit;"
></textarea>

<!-- Add: -->
<MarkdownEditor
  value={batch.notes ?? null}
  onchange={(v) => onUpdate({ notes: v })}
  rows={4}
  placeholder="Brew day observations, gravity readings, anything worth remembering…"
/>
```

- [ ] **Step 3: Run the full test suite**

```bash
npx vitest run
```

Expected: all tests PASS.

- [ ] **Step 4: Commit**

```bash
git add src/lib/components/tabs/NotesTab.svelte src/lib/components/batch/BatchOverviewTab.svelte
git commit -m "feat: replace Notes and Batch textareas with MarkdownEditor"
```

---

## Task 7: Replace textareas in IngredientEditModal.svelte

**Files:**
- Modify: `src/lib/components/ingredients/IngredientEditModal.svelte`

The modal reads `hopNotes`, `fermNotes`, etc. from reactive state on Save. These vars use `bind:value` today, updating on every keystroke. `MarkdownEditor` uses `onchange` (blur-based) instead. This is safe because when the user clicks the Save button, the browser fires `blur` on the focused textarea before firing the button's `click` — so the state is always up-to-date when `handleSave` runs.

- [ ] **Step 1: Add the import**

At the top of the `<script>` block, after the existing imports, add:

```svelte
import MarkdownEditor from "$lib/components/MarkdownEditor.svelte";
```

- [ ] **Step 2: Replace the hop Notes textarea (around line 384)**

Find:
```svelte
<textarea bind:value={hopNotes} rows="3" class="px-2 py-1.5 rounded text-sm resize-none"
          style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);"></textarea>
```

Replace with:
```svelte
<MarkdownEditor
  value={hopNotes || null}
  onchange={(v) => { hopNotes = v ?? ''; }}
  rows={3}
/>
```

- [ ] **Step 3: Replace the fermentable Notes textarea (around line 430)**

Find:
```svelte
<textarea bind:value={fermNotes} rows="3" class="px-2 py-1.5 rounded text-sm resize-none"
          style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);"></textarea>
```

Replace with:
```svelte
<MarkdownEditor
  value={fermNotes || null}
  onchange={(v) => { fermNotes = v ?? ''; }}
  rows={3}
/>
```

- [ ] **Step 4: Replace the yeast Notes textarea (around line 492)**

Find:
```svelte
<textarea bind:value={yeastNotes} rows="3" class="px-2 py-1.5 rounded text-sm resize-none"
          style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);"></textarea>
```

Replace with:
```svelte
<MarkdownEditor
  value={yeastNotes || null}
  onchange={(v) => { yeastNotes = v ?? ''; }}
  rows={3}
/>
```

- [ ] **Step 5: Replace the misc Notes textarea (around line 540)**

Find:
```svelte
<textarea bind:value={miscNotes} rows="3" class="px-2 py-1.5 rounded text-sm resize-none"
          style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);"></textarea>
```

Replace with:
```svelte
<MarkdownEditor
  value={miscNotes || null}
  onchange={(v) => { miscNotes = v ?? ''; }}
  rows={3}
/>
```

- [ ] **Step 6: Replace the water Notes textarea (around line 593)**

Find:
```svelte
<textarea bind:value={waterNotes} rows="2" class="px-2 py-1.5 rounded text-sm resize-none"
          style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);"></textarea>
```

Replace with:
```svelte
<MarkdownEditor
  value={waterNotes || null}
  onchange={(v) => { waterNotes = v ?? ''; }}
  rows={2}
/>
```

- [ ] **Step 7: Run the full test suite**

```bash
npx vitest run
```

Expected: all tests PASS.

- [ ] **Step 8: Type check**

```bash
npx svelte-check --tsconfig ./tsconfig.json 2>&1 | tail -5
```

Expected: `0 errors` (warnings about unused CSS selectors from the global styles are fine to ignore).

- [ ] **Step 9: Commit**

```bash
git add src/lib/components/ingredients/IngredientEditModal.svelte
git commit -m "feat: replace IngredientEditModal textareas with MarkdownEditor"
```
