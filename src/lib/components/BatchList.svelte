<!-- src/lib/components/BatchList.svelte -->
<script lang="ts">
  import { goto } from "$app/navigation";
  import type { BatchSummary } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { deleteBatch } from "$lib/api";

  let {
    batches = [],
    onRefresh,
  }: {
    batches: BatchSummary[];
    onRefresh: () => void;
  } = $props();

  const STATUS_LABELS: Record<string, string> = {
    planned: "Planned",
    brewing: "Brewing",
    fermenting: "Fermenting",
    packaged: "Packaged",
  };

  const STATUS_COLORS: Record<string, string> = {
    planned: "var(--color-text-muted)",
    brewing: "#f59e0b",
    fermenting: "#10b981",
    packaged: "#3b82f6",
  };

  function formatDate(ts: number | null | undefined): string {
    if (!ts) return "—";
    return new Date(ts * 1000).toLocaleDateString();
  }

  async function handleDelete(id: string, e: MouseEvent) {
    e.stopPropagation();
    if (!confirm("Delete this batch?")) return;
    await ipc(deleteBatch(id));
    onRefresh();
  }
</script>

<div class="flex flex-col gap-1 p-2">
  {#each batches as batch (batch.id)}
    <div
      role="button"
      tabindex="0"
      onclick={() => goto(`/batches/${batch.id}`)}
      onkeydown={(e) => e.key === "Enter" && goto(`/batches/${batch.id}`)}
      class="flex items-center gap-3 px-3 py-2 rounded cursor-pointer transition-colors bg-bg-elevated"
     
    >
      <div class="flex-1 min-w-0">
        <div class="text-sm font-medium truncate">{batch.recipe_name}</div>
        <div class="flex items-center gap-2 mt-0.5 flex-wrap">
          <span class="text-xs truncate text-text-muted">
            {batch.name ?? "Batch"} · {formatDate(batch.brew_date)}
          </span>
          {#if batch.actual_og}
            <span class="text-xs text-text-muted">
              OG {batch.actual_og.toFixed(3)}
            </span>
          {/if}
          <span
            class="text-xs px-1.5 py-0.5 rounded"
            style="color: {STATUS_COLORS[batch.status] ?? 'var(--color-text-muted)'}; background: var(--color-bg-surface);"
          >
            {STATUS_LABELS[batch.status] ?? batch.status}
          </span>
        </div>
      </div>
      <button
        onclick={(e) => handleDelete(batch.id, e)}
        class="opacity-40 hover:opacity-100 text-xs px-1 flex-shrink-0 self-start text-text-muted"
       
      >✕</button>
    </div>
  {:else}
    <p class="text-sm px-3 py-4 text-center text-text-secondary">
      No batches yet. Open a recipe and click Brew to create one.
    </p>
  {/each}
</div>
