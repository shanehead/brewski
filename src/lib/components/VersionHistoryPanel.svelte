<script lang="ts">
  import type { RecipeVersionSummary } from "$lib/api";
  import { saveRecipeVersion } from "$lib/api";
  import { ipc } from "$lib/stores/error";

  let {
    versions,
    viewingVersionId,
    onview,
    onbranch,
    ondelete,
    onclose,
    recipeId,
    hasUnversionedChanges,
    onSaved,
  }: {
    versions: RecipeVersionSummary[];
    viewingVersionId: string | null;
    onview: (version: RecipeVersionSummary) => void;
    onbranch: (version: RecipeVersionSummary) => void;
    ondelete: (version: RecipeVersionSummary) => void;
    onclose: () => void;
    recipeId: string;
    hasUnversionedChanges: boolean;
    onSaved: () => void;
  } = $props();

  let saving = $state(false);
  let saveName = $state("");
  let inFlight = $state(false);

  async function confirmSave() {
    if (inFlight) return;
    inFlight = true;
    try {
      const v = await ipc(saveRecipeVersion({ recipe_id: recipeId, name: saveName.trim() || null }));
      if (v) { saving = false; saveName = ""; onSaved(); }
    } finally {
      inFlight = false;
    }
  }

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
    ondelete(version);
  }
</script>

<div
  class="flex flex-col h-full border-l overflow-hidden bg-bg-surface border-border"
  style="min-width: 220px; max-width: 260px;"
>
  <div
    class="flex items-center justify-between px-3 py-2 border-b flex-shrink-0 border-border"
   
  >
    <span class="text-xs font-semibold text-text-secondary">
      VERSION HISTORY
    </span>
    <button
      onclick={onclose}
      class="text-xs px-1 text-text-muted"
     
    >✕</button>
  </div>

  {#if hasUnversionedChanges}
    <div class="flex flex-col gap-1 mx-2 mt-2 mb-1 px-2 py-1.5 rounded bg-bg-elevated border border-border">
      <span class="text-xs text-text-secondary">⚠ un-versioned changes</span>
      {#if saving}
        <div class="flex gap-1">
          <input
            class="flex-1 px-2 py-1 rounded text-xs bg-bg-base text-text-primary border border-border"
            placeholder="Name (optional)"
            bind:value={saveName}
            onkeydown={(e) => { if (e.key === "Enter") confirmSave(); }}
          />
          <button class="text-xs px-2 py-1 rounded bg-accent" style="color:#fff;" disabled={inFlight} onclick={confirmSave}>Save</button>
          <button class="text-xs px-2 py-1 rounded text-text-secondary" onclick={() => { saving = false; saveName = ""; }}>Cancel</button>
        </div>
      {:else}
        <button class="text-xs px-2 py-1 rounded bg-accent self-start" style="color:#fff;" onclick={() => (saving = true)}>Save as version</button>
      {/if}
    </div>
  {/if}

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
            <span class="text-text-muted" style="font-size: 0.6rem;">↳</span>
          {/if}
          <span class="text-xs font-mono text-accent">
            v{version.version_number}
          </span>
          {#if version.name}
            <span class="text-xs truncate text-text-primary">
              {version.name}
            </span>
          {/if}
        </div>
        <div class="text-xs mt-0.5 text-text-muted">
          {formatDate(version.created_at)}
        </div>
        {#if viewingVersionId === version.id}
          <div class="mt-1 flex gap-1">
            <button
              onclick={(e) => { e.stopPropagation(); onbranch(version); }}
              class="text-xs px-2 py-0.5 rounded bg-accent"
              style="color: #fff;"
            >
              Branch from here
            </button>
            <button
              onclick={(e) => handleDeleteClick(version, e)}
              class="text-xs px-2 py-0.5 rounded bg-bg-elevated text-text-muted border border-border"
              style="cursor: pointer;"
            >
              Delete
            </button>
          </div>
        {/if}
      </div>
    {/each}
  </div>
</div>
