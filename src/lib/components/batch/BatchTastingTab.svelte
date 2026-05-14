<!-- src/lib/components/batch/BatchTastingTab.svelte -->
<script lang="ts">
  import type { Batch, UpdateBatchInput } from "$lib/api";

  let { batch, onUpdate }: { batch: Batch; onUpdate: (input: UpdateBatchInput) => void } = $props();

  let ratingInput = $state(batch.rating?.toString() ?? "");
</script>

<div class="p-4 flex flex-col gap-4 overflow-y-auto">
  <div class="flex flex-col gap-1">
    <label class="text-xs" style="color: var(--color-text-muted);">RATING (1–10)</label>
    <input
      type="number"
      min="1"
      max="10"
      step="1"
      bind:value={ratingInput}
      onblur={() => {
        const v = parseInt(ratingInput);
        onUpdate({ rating: isNaN(v) ? null : Math.min(10, Math.max(1, v)) });
      }}
      placeholder="—"
      class="w-24 px-2 py-1.5 rounded text-sm outline-none"
      style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);"
    />
  </div>
  <div class="flex flex-col gap-1 flex-1">
    <label class="text-xs" style="color: var(--color-text-muted);">TASTING NOTES</label>
    <textarea
      value={batch.tasting_notes ?? ""}
      onblur={(e) => onUpdate({ tasting_notes: e.currentTarget.value || null })}
      placeholder="Appearance, aroma, flavor, mouthfeel…"
      class="p-2 rounded text-sm outline-none resize-none min-h-48"
      style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);"
    ></textarea>
  </div>
</div>
