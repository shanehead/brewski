<script lang="ts">
  import { untrack } from "svelte";
  import type { RecipeVersionSummary, RecipeVersionStatus } from "$lib/api";

  let {
    status,
    versions,
    onBrewCurrent,
    onBrewVersion,
    onCancel,
  }: {
    status: RecipeVersionStatus;
    versions: RecipeVersionSummary[];
    onBrewCurrent: (name: string | null) => void;
    onBrewVersion: (versionId: string) => void;
    onCancel: () => void;
  } = $props();

  let newName = $state("");
  let selected = $state(untrack(() => status.latest_version_id ?? versions[0]?.id ?? ""));

  function label(v: RecipeVersionSummary): string {
    return v.name ? `v${v.version_number} · ${v.name}` : `v${v.version_number}`;
  }
</script>

<div class="fixed inset-0 z-50 flex items-center justify-center" style="background: rgba(0,0,0,0.4);">
  <div class="w-[420px] max-w-[92vw] rounded-lg p-4 flex flex-col gap-3 bg-bg-surface border border-border">
    <h2 class="text-base font-semibold text-text-primary">Choose a version to brew</h2>

    {#if status.has_unversioned_changes || status.version_count === 0}
      <div class="flex flex-col gap-2 p-3 rounded bg-bg-elevated border border-border">
        <div class="text-sm text-text-primary">
          {#if status.version_count === 0}
            ⚠ This recipe isn't saved as a version yet.
          {:else}
            ⚠ This recipe has un-versioned changes.
          {/if}
        </div>
        <input
          class="px-2 py-1.5 rounded text-sm bg-bg-base text-text-primary border border-border"
          placeholder="Name (optional)"
          bind:value={newName} />
        <button class="px-3 py-1.5 rounded text-sm bg-accent self-start" style="color: #fff;"
                onclick={() => onBrewCurrent(newName.trim() || null)}>
          Brew with current changes
        </button>
      </div>
    {/if}

    {#if versions.length > 0}
      <div class="flex flex-col gap-1">
        <div class="text-xs text-text-secondary">Saved versions</div>
        <select bind:value={selected}
                class="px-2 py-1.5 rounded text-sm bg-bg-elevated text-text-primary border border-border">
          {#each versions as v}
            <option value={v.id}>{label(v)}</option>
          {/each}
        </select>
        <button class="px-3 py-1.5 rounded text-sm mt-1 bg-bg-elevated text-text-primary border border-border self-start"
                disabled={!selected} onclick={() => onBrewVersion(selected)}>
          Brew a saved version
        </button>
      </div>
    {/if}

    <button class="text-xs text-text-secondary self-end" onclick={onCancel}>Cancel</button>
  </div>
</div>
