<script lang="ts">
  import type { RecipeVersionSummary } from "$lib/api";

  let {
    versions,
    viewingVersionId,
    onview,
    onbranch,
    ondelete,
    onclose,
  }: {
    versions: RecipeVersionSummary[];
    viewingVersionId: string | null;
    onview: (version: RecipeVersionSummary) => void;
    onbranch: (version: RecipeVersionSummary) => void;
    ondelete: (version: RecipeVersionSummary) => void;
    onclose: () => void;
  } = $props();

  function formatDate(ts: number): string {
    return new Date(ts * 1000).toLocaleDateString(undefined, {
      month: "short",
      day: "numeric",
      year: "numeric",
    });
  }

  function indentLevel(version: RecipeVersionSummary, visited = new Set<string>()): number {
    if (!version.parent_version_id) return 0;
    if (visited.has(version.id)) return 0; // cycle detected
    visited.add(version.id);
    const parent = versions.find((v) => v.id === version.parent_version_id);
    if (!parent) return 1;
    return indentLevel(parent, visited) + 1;
  }

  function handleDeleteClick(version: RecipeVersionSummary, e: MouseEvent) {
    e.stopPropagation();
    console.log('VersionHistoryPanel: delete clicked', version.id);
    if (typeof ondelete === 'function') {
      try {
        ondelete(version);
      } catch (err) {
        console.warn('delete handler threw', err);
      }
    } else {
      console.warn('ondelete handler not provided');
    }
  }
</script>

<div
  class="flex flex-col h-full border-l overflow-hidden"
  style="background: var(--color-bg-surface); border-color: var(--color-border); min-width: 220px; max-width: 260px; position: relative; z-index: 50; pointer-events: auto;"
>
  <div
    class="flex items-center justify-between px-3 py-2 border-b flex-shrink-0"
    style="border-color: var(--color-border);"
  >
    <span class="text-xs font-semibold" style="color: var(--color-text-secondary);">
      VERSION HISTORY
    </span>
    <button
      onclick={onclose}
      class="text-xs px-1"
      style="color: var(--color-text-muted);"
    >✕</button>
  </div>

  <div class="flex-1 overflow-y-auto">
    {#each versions as version}
      {@const indent = Math.min(indentLevel(version), 3)}
      <div
        onclick={() => onview(version)}
        onkeydown={(e) => {
          if (e.key === 'Enter' || e.key === ' ') {
            e.preventDefault();
            onview(version);
          }
        }}
        role="button"
        tabindex="0"
        class="w-full text-left px-3 py-2 border-b transition-colors cursor-pointer"
        style="
          padding-left: {0.75 + indent * 0.75}rem;
          border-color: var(--color-border);
          background: {viewingVersionId === version.id
            ? 'var(--color-bg-elevated)'
            : 'transparent'};
        "
      >
        <div class="flex items-center gap-1.5">
          {#if indent > 0}
            <span style="color: var(--color-text-muted); font-size: 0.6rem;">↳</span>
          {/if}
          <span class="text-xs font-mono" style="color: var(--color-accent);">
            v{version.version_number}
          </span>
          {#if version.name}
            <span class="text-xs truncate" style="color: var(--color-text-primary);">
              {version.name}
            </span>
          {/if}
        </div>
        <div class="text-xs mt-0.5" style="color: var(--color-text-muted);">
          {formatDate(version.created_at)}
        </div>
        {#if viewingVersionId === version.id}
          <div class="mt-1 flex gap-1">
            <button
              onclick={() => onbranch(version)}
              class="text-xs px-2 py-0.5 rounded"
              style="background: var(--color-accent); color: #fff;"
            >
              Branch from here
            </button>
            <button
              onclick={(e) => handleDeleteClick(version, e)}
              class="text-xs px-2 py-0.5 rounded"
              style="background: var(--color-bg-elevated); color: var(--color-text-muted); border: 1px solid var(--color-border); cursor: pointer;"
            >
              Delete
            </button>
          </div>
        {/if}
      </div>
    {/each}
  </div>
</div>
