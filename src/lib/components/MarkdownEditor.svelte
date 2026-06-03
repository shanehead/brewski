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
